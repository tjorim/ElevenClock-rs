//! `--uninstall`: take the managed copy off this account.
//!
//! Only the four known file names are removed (no wildcard, no folder removal), and
//! the app's `ElevenClockRust` Run value only when it starts this folder's app or
//! launcher: the same exact-match rule as the installer's uninstaller. Preferences
//! and logs are never touched. There is no message box: an administrator's script
//! may run this unattended, so the result is the exit code and the log.
use crate::{
    launch::{Claim, Context, Exit, Host, Layout, describe},
    source,
};
use clock_windows::startup::{RunKey, command_line};
use std::{fs, io, path::Path};

pub fn uninstall(host: &mut dyn Host, ctx: &Context) -> Exit {
    let layout = Layout::new(&ctx.destination);
    // The folder is shared with the installer, whose files are not ours to remove.
    if host.installer_entry_present() {
        host.log("refused: installed copy");
        return Exit::Refused;
    }
    // A running app holds its exe; stopping now is cleaner than a half removal.
    let claim = host.claim_instance();
    if !matches!(claim, Claim::Free(_)) {
        host.log("uninstall refused: running");
        return Exit::UninstallIncomplete;
    }
    let mut complete = true;
    let files = [
        layout.app(),
        layout.launcher(),
        layout.launcher_old(),
        layout.state(),
    ];
    for file in files {
        match fs::remove_file(&file) {
            Ok(()) => host.log("file removed"),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => {
                // The launcher cannot delete itself while it runs: run --uninstall
                // from the central copy to remove the local launcher too.
                let own = source::same_folder(&file, &ctx.own_exe);
                host.log(&format!(
                    "file not removed{}: {}",
                    if own {
                        " (this launcher is running)"
                    } else {
                        ""
                    },
                    describe(&error)
                ));
                complete = false;
            }
        }
    }
    match remove_startup_entry(host.run_key(), &layout.app(), &layout.launcher()) {
        Ok(true) => host.log("startup entry removed"),
        Ok(false) => host.log("startup entry left alone"),
        Err(message) => {
            host.log(&format!("startup entry not removed: {message}"));
            complete = false;
        }
    }
    if complete {
        host.log("uninstalled");
        Exit::Done
    } else {
        Exit::UninstallIncomplete
    }
}

/// Removes the Run value and its Windows flag when the value is exactly what the app
/// writes for this folder's app or launcher.
fn remove_startup_entry(key: &mut dyn RunKey, app: &Path, launcher: &Path) -> Result<bool, String> {
    let entry = key.read().map_err(|error| error.message())?;
    let Some(command) = entry.command else {
        return Ok(false);
    };
    let command = command.trim().to_lowercase();
    let ours = [app, launcher]
        .into_iter()
        .filter_map(|path| command_line(path).ok())
        .any(|expected| expected.to_lowercase() == command);
    if !ours {
        return Ok(false);
    }
    key.remove().map_err(|error| error.message())?;
    Ok(true)
}
