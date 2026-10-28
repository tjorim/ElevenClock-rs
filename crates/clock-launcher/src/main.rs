#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(windows)]
fn main() -> std::process::ExitCode {
    clock_launcher::native::main()
}

// Allows host workspace checks; Linux builds do not implement the launcher.
#[cfg(not(windows))]
fn main() {}
