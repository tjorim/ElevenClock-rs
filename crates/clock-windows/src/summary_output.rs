//! Atomic summary publication through a held, validated output directory.
//! Windows is the product backend: ancestor and staging handles prevent
//! relocation/substitution. The Unix backend supports portable development
//! checks and requires an output directory whose entries/ancestry are trusted;
//! Unix directory descriptors alone cannot prevent another writer renaming them.
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io::{self, Read, Seek, SeekFrom, Write},
    path::{Component, Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
static STAGING_ID: AtomicU64 = AtomicU64::new(0);

pub struct Parent {
    // Keep every ancestor alive until publication/cleanup completes.
    directories: Vec<File>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn replaces_existing_output_and_cleans_staging_on_interruption_or_failed_commit() {
        let temp = tempfile::tempdir().unwrap();
        let parent = Parent::open(&temp.path().canonicalize().unwrap()).unwrap();
        let output = temp.path().join("summary.json");
        fs::write(&output, b"old").unwrap();
        parent
            .install(OsStr::new("summary.json"), b"new", || Ok(()))
            .unwrap();
        assert_eq!(fs::read(&output).unwrap(), b"new");
        assert!(
            parent
                .install(OsStr::new("summary.json"), b"interrupted", || Err(
                    io::Error::other("interrupted")
                ))
                .is_err()
        );
        assert_eq!(fs::read(&output).unwrap(), b"new");
        fs::create_dir(temp.path().join("directory.json")).unwrap();
        assert!(
            parent
                .install(
                    OsStr::new("directory.json"),
                    b"cannot replace directory",
                    || Ok(())
                )
                .is_err()
        );
        assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 2);
        for target in ["../escape", "x:y", "..", "x\\y", "x\0y", "x."] {
            assert!(
                parent
                    .install(OsStr::new(target), b"invalid", || Ok(()))
                    .is_err()
            );
        }
    }

    #[cfg(windows)]
    #[test]
    fn output_parent_and_ancestors_remain_pinned_before_staging_and_commit() {
        let temp = tempfile::tempdir().unwrap();
        let ancestor = temp.path().join("ancestor");
        let directory = ancestor.join("output");
        fs::create_dir_all(&directory).unwrap();
        let parent = Parent::open(&directory.canonicalize().unwrap()).unwrap();
        assert!(fs::rename(&ancestor, temp.path().join("moved")).is_err());
        parent
            .install(OsStr::new("summary.json"), b"safe", || {
                assert!(fs::rename(&ancestor, temp.path().join("moved")).is_err());
                assert!(fs::rename(&directory, ancestor.join("moved-output")).is_err());
                Ok(())
            })
            .unwrap();
        assert_eq!(fs::read(directory.join("summary.json")).unwrap(), b"safe");
    }

    #[cfg(windows)]
    #[test]
    fn in_place_output_junction_is_rejected_without_redirecting_publication() {
        use crate::bindings::*;
        use std::os::windows::{fs::OpenOptionsExt, io::AsRawHandle};
        let temp = tempfile::tempdir().unwrap();
        let directory = temp.path().join("output");
        let reports = temp.path().join("reports");
        let seed = temp.path().join("seed-junction");
        fs::create_dir(&directory).unwrap();
        fs::create_dir(&reports).unwrap();
        assert!(
            std::process::Command::new("cmd")
                .args(["/C", "mklink", "/J"])
                .arg(&seed)
                .arg(&reports)
                .output()
                .unwrap()
                .status
                .success()
        );
        let open = |path: &Path| {
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .share_mode((FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE) as u32)
                .custom_flags((FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT) as u32)
                .open(path)
                .unwrap()
        };
        let source = open(&seed);
        let mut data = [0u8; 16 * 1024];
        let mut returned = 0;
        // SAFETY: live directory handle, bounded output and synchronous request.
        unsafe {
            DeviceIoControl(
                HANDLE(source.as_raw_handle()),
                FSCTL_GET_REPARSE_POINT as u32,
                None,
                0,
                Some(data.as_mut_ptr().cast()),
                data.len() as u32,
                Some(&mut returned),
                None,
            )
            .ok()
            .unwrap();
        }
        let parent = Parent::open(&directory.canonicalize().unwrap()).unwrap();
        let destination = open(&directory);
        // SAFETY: copy an actual junction's bounded kernel-provided reparse data
        // onto our disposable empty directory while its parent handle stays live.
        unsafe {
            DeviceIoControl(
                HANDLE(destination.as_raw_handle()),
                FSCTL_SET_REPARSE_POINT as u32,
                Some(data.as_ptr().cast()),
                returned,
                None,
                0,
                Some(&mut returned),
                None,
            )
            .ok()
            .unwrap();
        }
        struct Reset(File, [u8; 8]);
        impl Drop for Reset {
            fn drop(&mut self) {
                let mut returned = 0;
                // SAFETY: live owned handle and tag-only reparse-delete buffer.
                unsafe {
                    let _ = DeviceIoControl(
                        HANDLE(self.0.as_raw_handle()),
                        FSCTL_DELETE_REPARSE_POINT as u32,
                        Some(self.1.as_ptr().cast()),
                        8,
                        None,
                        0,
                        Some(&mut returned),
                        None,
                    );
                }
            }
        }
        let mut tag = [0u8; 8];
        tag[..4].copy_from_slice(&data[..4]);
        let reset = Reset(destination, tag);
        assert!(
            parent
                .install(OsStr::new("summary.json"), b"safe", || Ok(()))
                .is_err()
        );
        assert!(!reports.join("summary.json").exists());
        drop(reset);
        assert_eq!(fs::read_dir(directory).unwrap().count(), 0);
    }
}
fn name(name: &OsStr) -> io::Result<()> {
    let text = name
        .to_str()
        .ok_or_else(|| io::Error::other("invalid output name"))?;
    if text.is_empty()
        || text == "."
        || text == ".."
        || text.contains(['/', '\\', ':', '\0'])
        || text.ends_with(['.', ' '])
    {
        return Err(io::Error::other("invalid output name"));
    }
    Ok(())
}
impl Parent {
    /// `path` is the canonical parent checked against the canonical report root.
    /// Walk it from its volume/share root without following substituted links.
    pub fn open(path: &Path) -> io::Result<Self> {
        if !path.is_absolute() {
            return Err(io::Error::other("output parent must be absolute"));
        }
        let mut anchor = PathBuf::new();
        let mut children = Vec::<OsString>::new();
        for component in path.components() {
            match component {
                Component::Prefix(_) | Component::RootDir => anchor.push(component.as_os_str()),
                Component::Normal(value) => {
                    name(value)?;
                    children.push(value.to_owned());
                }
                _ => return Err(io::Error::other("invalid output parent")),
            }
        }
        let mut directories = vec![native::root(&anchor)?];
        native::validate_directory(&directories[0])?;
        for child in children {
            let file = native::directory(directories.last().unwrap(), &child)?;
            native::validate_directory(&file)?;
            directories.push(file);
        }
        Ok(Self { directories })
    }
    /// No pathname is reopened for staging, verification, replacement or cleanup.
    pub fn install(
        &self,
        target: &OsStr,
        bytes: &[u8],
        before_replace: impl FnOnce() -> io::Result<()>,
    ) -> io::Result<()> {
        name(target)?;
        let parent = self.directories.last().unwrap();
        let mut stage = None;
        for _ in 0..8 {
            let temporary = OsString::from(format!(
                ".elevenclock-summary-{}-{}.tmp",
                std::process::id(),
                STAGING_ID.fetch_add(1, Ordering::Relaxed)
            ));
            match native::create(parent, &temporary) {
                Ok(file) => {
                    stage = Some((file, temporary));
                    break;
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error),
            }
        }
        let (mut file, temporary) =
            stage.ok_or_else(|| io::Error::other("no free summary staging name"))?;
        let result = (|| {
            file.write_all(bytes)?;
            file.sync_all()?;
            file.seek(SeekFrom::Start(0))?;
            let mut verified = Vec::new();
            (&mut file)
                .take(bytes.len() as u64 + 1)
                .read_to_end(&mut verified)?;
            if verified != bytes {
                return Err(io::Error::other("summary copy mismatch"));
            }
            before_replace()?;
            native::publish(parent, &file, &temporary, target)
        })();
        if result.is_err() {
            native::cleanup(parent, &file, &temporary);
        }
        result
    }
}

#[cfg(windows)]
mod native {
    use super::*;
    use crate::bindings::*;
    use std::{
        fs::OpenOptions,
        mem::{offset_of, size_of},
        os::windows::{
            ffi::OsStrExt,
            fs::{MetadataExt, OpenOptionsExt},
            io::{AsRawHandle, FromRawHandle},
        },
    };
    use windows_core::{NTSTATUS, PWSTR};
    fn handle(file: &File) -> HANDLE {
        HANDLE(file.as_raw_handle())
    }
    fn status(value: NTSTATUS) -> io::Result<()> {
        if value.0 >= 0 {
            Ok(())
        } else {
            Err(io::Error::from_raw_os_error(
                unsafe { RtlNtStatusToDosError(value) } as i32,
            ))
        }
    }
    pub fn root(path: &Path) -> io::Result<File> {
        OpenOptions::new()
            .access_mode((FILE_TRAVERSE | FILE_READ_ATTRIBUTES | SYNCHRONIZE) as u32)
            .share_mode((FILE_SHARE_READ | FILE_SHARE_WRITE) as u32)
            .custom_flags((FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT) as u32)
            .open(path)
    }
    fn relative(parent: &File, child: &OsStr, directory: bool) -> io::Result<File> {
        let mut wide: Vec<u16> = child.encode_wide().collect();
        let length = u16::try_from(wide.len() * 2).map_err(io::Error::other)?;
        let mut text = UNICODE_STRING {
            Length: length,
            MaximumLength: length,
            Buffer: PWSTR(wide.as_mut_ptr()),
        };
        let attributes = OBJECT_ATTRIBUTES {
            Length: size_of::<OBJECT_ATTRIBUTES>() as u32,
            RootDirectory: handle(parent),
            ObjectName: &mut text,
            Attributes: OBJ_CASE_INSENSITIVE as u32,
            ..Default::default()
        };
        let mut output = HANDLE::default();
        let mut iosb = IO_STATUS_BLOCK::default();
        // SAFETY: live parent, bounded single-component UTF-16 name and output
        // slots. Synchronous opens complete before any input buffers are dropped.
        status(unsafe {
            NtCreateFile(
                &mut output,
                ACCESS_MASK(if directory {
                    (FILE_TRAVERSE | FILE_READ_ATTRIBUTES | SYNCHRONIZE) as u32
                } else {
                    GENERIC_READ | (GENERIC_WRITE | DELETE | SYNCHRONIZE) as u32
                }),
                &attributes,
                &mut iosb,
                None,
                0,
                if directory {
                    (FILE_SHARE_READ | FILE_SHARE_WRITE) as u32
                } else {
                    FILE_SHARE_READ as u32
                },
                if directory {
                    FILE_OPEN as u32
                } else {
                    FILE_CREATE as u32
                },
                (FILE_SYNCHRONOUS_IO_NONALERT
                    | FILE_OPEN_REPARSE_POINT
                    | if directory {
                        FILE_DIRECTORY_FILE
                    } else {
                        FILE_NON_DIRECTORY_FILE
                    }) as u32,
                None,
                0,
            )
        })?;
        // SAFETY: a successful NtCreateFile returns one exclusively owned handle.
        Ok(unsafe { File::from_raw_handle(output.0) })
    }
    pub fn directory(parent: &File, child: &OsStr) -> io::Result<File> {
        relative(parent, child, true)
    }
    pub fn create(parent: &File, child: &OsStr) -> io::Result<File> {
        relative(parent, child, false)
    }
    pub fn validate_directory(file: &File) -> io::Result<()> {
        let metadata = file.metadata()?;
        if !metadata.is_dir()
            || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT as u32 != 0
        {
            return Err(io::Error::other("linked output parent"));
        }
        Ok(())
    }
    pub fn publish(parent: &File, file: &File, _: &OsStr, target: &OsStr) -> io::Result<()> {
        let wide: Vec<u16> = target.encode_wide().collect();
        let length = size_of::<FILE_RENAME_INFORMATION>() + wide.len() * 2;
        // usize storage gives the ABI structure its required pointer alignment.
        let mut storage = vec![0usize; length.div_ceil(size_of::<usize>())];
        let info = storage.as_mut_ptr().cast::<FILE_RENAME_INFORMATION>();
        let mut iosb = IO_STATUS_BLOCK::default();
        // SAFETY: aligned allocation covers the header and complete variable
        // UTF-16 tail; live parent/file handles and synchronous bounded call.
        unsafe {
            (*info).Anonymous.ReplaceIfExists = true;
            (*info).RootDirectory = handle(parent);
            (*info).FileNameLength = (wide.len() * 2) as u32;
            let tail = storage
                .as_mut_ptr()
                .cast::<u8>()
                .add(offset_of!(FILE_RENAME_INFORMATION, FileName))
                .cast::<u16>();
            std::ptr::copy_nonoverlapping(wide.as_ptr(), tail, wide.len());
            status(NtSetInformationFile(
                handle(file),
                &mut iosb,
                info.cast(),
                length as u32,
                FileRenameInformation,
            ))
        }
    }
    pub fn cleanup(_: &File, file: &File, _: &OsStr) {
        let disposition = FILE_DISPOSITION_INFORMATION { DeleteFile: true };
        let mut iosb = IO_STATUS_BLOCK::default();
        // SAFETY: marks only our held staging file for deletion on close.
        unsafe {
            let _ = NtSetInformationFile(
                handle(file),
                &mut iosb,
                (&disposition as *const FILE_DISPOSITION_INFORMATION).cast(),
                size_of::<FILE_DISPOSITION_INFORMATION>() as u32,
                FileDispositionInformation,
            );
        }
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
    fn c_name(name: &OsStr) -> io::Result<CString> {
        CString::new(name.as_bytes()).map_err(io::Error::other)
    }
    pub fn root(path: &Path) -> io::Result<File> {
        OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(path)
    }
    fn relative(parent: &File, child: &OsStr, flags: i32) -> io::Result<File> {
        let child = c_name(child)?;
        // SAFETY: live directory descriptor, single NUL-terminated basename;
        // mode is supplied for exclusive staging creation and ignored otherwise.
        let fd = unsafe {
            libc::openat(
                parent.as_raw_fd(),
                child.as_ptr(),
                flags | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                0o600,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: successful openat returns one exclusively owned descriptor.
        Ok(unsafe { File::from_raw_fd(fd) })
    }
    pub fn directory(parent: &File, child: &OsStr) -> io::Result<File> {
        relative(parent, child, libc::O_RDONLY | libc::O_DIRECTORY)
    }
    pub fn create(parent: &File, child: &OsStr) -> io::Result<File> {
        relative(parent, child, libc::O_RDWR | libc::O_CREAT | libc::O_EXCL)
    }
    pub fn validate_directory(file: &File) -> io::Result<()> {
        if file.metadata()?.is_dir() {
            Ok(())
        } else {
            Err(io::Error::other("invalid output directory"))
        }
    }
    pub fn publish(parent: &File, _: &File, staged: &OsStr, target: &OsStr) -> io::Result<()> {
        let staged = c_name(staged)?;
        let target = c_name(target)?;
        // SAFETY: live directory descriptor and validated single-component names.
        if unsafe {
            libc::renameat(
                parent.as_raw_fd(),
                staged.as_ptr(),
                parent.as_raw_fd(),
                target.as_ptr(),
            )
        } != 0
        {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    pub fn cleanup(parent: &File, _: &File, staged: &OsStr) {
        if let Ok(staged) = c_name(staged) {
            // SAFETY: remove only the relative staging name in our held directory.
            unsafe {
                libc::unlinkat(parent.as_raw_fd(), staged.as_ptr(), 0);
            }
        }
    }
}
