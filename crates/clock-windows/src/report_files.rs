//! Open-time confinement for manager reads of client-writable inventory reports.
//! Directory guards and checked file handles remain live through each read.
use std::{
    ffi::OsStr,
    fs::{self, File},
    io,
    path::{Component, Path, PathBuf},
};

pub struct Directory {
    file: File,
    path: PathBuf,
}

fn child_name(name: &OsStr) -> io::Result<()> {
    let mut components = Path::new(name).components();
    if !matches!(components.next(), Some(Component::Normal(_))) || components.next().is_some() {
        return Err(io::Error::other("invalid report name"));
    }
    Ok(())
}

impl Directory {
    pub fn open(path: &Path) -> io::Result<Self> {
        let file = native::open(path, true)?;
        native::validate(&file, true)?;
        Ok(Self {
            file,
            path: path.to_owned(),
        })
    }

    pub fn directory(&self, name: &OsStr) -> io::Result<Self> {
        child_name(name)?;
        let path = self.path.join(name);
        let file = native::child(self, name, true)?;
        native::validate(&file, true)?;
        Ok(Self { file, path })
    }

    pub fn file(&self, name: &OsStr) -> io::Result<File> {
        child_name(name)?;
        let file = native::child(self, name, false)?;
        native::validate(&file, false)?;
        Ok(file)
    }

    pub fn entries(&self) -> io::Result<fs::ReadDir> {
        #[cfg(unix)]
        {
            use std::os::fd::AsRawFd;
            // Enumerate the opened directory even if its original name changes.
            // Missing descriptor filesystems fail closed; do not fall back to
            // reopening a client-swappable pathname.
            let descriptors = if cfg!(target_os = "linux") {
                "/proc/self/fd"
            } else {
                "/dev/fd"
            };
            fs::read_dir(Path::new(descriptors).join(self.file.as_raw_fd().to_string()))
        }
        #[cfg(windows)]
        {
            // Windows has no descriptor-relative enumeration in std. This relies on `open`
            // sharing directories read-only (no FILE_SHARE_DELETE/WRITE): while this handle
            // is held the pinned directory cannot be renamed, deleted or replaced by a
            // junction, so the path still names the same directory. Keep that share mode.
            fs::read_dir(&self.path)
        }
    }
}

#[cfg(windows)]
mod native {
    use super::*;
    use crate::bindings::*;
    use std::{
        fs::OpenOptions,
        os::windows::fs::{MetadataExt, OpenOptionsExt},
    };
    pub fn open(path: &Path, directory: bool) -> io::Result<File> {
        OpenOptions::new()
            .read(true)
            .share_mode(if directory {
                FILE_SHARE_READ as u32
            } else {
                (FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE) as u32
            })
            .custom_flags(
                (FILE_FLAG_OPEN_REPARSE_POINT
                    | if directory {
                        FILE_FLAG_BACKUP_SEMANTICS
                    } else {
                        0
                    }) as u32,
            )
            .open(path)
    }
    pub fn child(parent: &Directory, name: &OsStr, directory: bool) -> io::Result<File> {
        // Holding the parent's no-write/no-delete handle prevents both renaming
        // it and converting it to a reparse point until the child has been read.
        let _guard = &parent.file;
        open(&parent.path.join(name), directory)
    }
    pub fn validate(file: &File, directory: bool) -> io::Result<()> {
        let metadata = file.metadata()?;
        if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT as u32 != 0
            || if directory {
                !metadata.is_dir()
            } else {
                !metadata.is_file()
            }
        {
            return Err(io::Error::other("linked or nonregular report object"));
        }
        Ok(())
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;
    #[test]
    fn held_directory_cannot_be_renamed() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("reports");
        fs::create_dir(&path).unwrap();
        let guard = Directory::open(&path).unwrap();
        assert!(fs::rename(&path, temp.path().join("moved")).is_err());
        drop(guard);
        fs::rename(&path, temp.path().join("moved")).unwrap();
    }
}

#[cfg(unix)]
mod native {
    use super::*;
    use std::{
        ffi::CString,
        fs::OpenOptions,
        os::{
            fd::{AsRawFd, FromRawFd},
            unix::{ffi::OsStrExt, fs::OpenOptionsExt},
        },
    };
    fn flags(directory: bool) -> i32 {
        libc::O_RDONLY
            | libc::O_CLOEXEC
            | libc::O_NOFOLLOW
            | if directory {
                libc::O_DIRECTORY
            } else {
                libc::O_NONBLOCK
            }
    }
    pub fn open(path: &Path, directory: bool) -> io::Result<File> {
        OpenOptions::new()
            .read(true)
            .custom_flags(flags(directory))
            .open(path)
    }
    pub fn child(parent: &Directory, name: &OsStr, directory: bool) -> io::Result<File> {
        let name = CString::new(name.as_bytes()).map_err(io::Error::other)?;
        // SAFETY: live directory descriptor and NUL-terminated single child name.
        let fd = unsafe { libc::openat(parent.file.as_raw_fd(), name.as_ptr(), flags(directory)) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: successful openat returns a new, exclusively owned descriptor.
        Ok(unsafe { File::from_raw_fd(fd) })
    }
    pub fn validate(file: &File, directory: bool) -> io::Result<()> {
        let metadata = file.metadata()?;
        if if directory {
            !metadata.is_dir()
        } else {
            !metadata.is_file()
        } {
            return Err(io::Error::other("nonregular report object"));
        }
        Ok(())
    }
}
