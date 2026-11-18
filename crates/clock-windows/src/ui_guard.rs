//! One panic policy for native callbacks that run on the UI thread.
//!
//! A panic must never unwind into the system's frames (Rust aborts at an
//! `extern "system"` boundary, with no dialog and none of the ordered teardown in
//! `runtime::native::run`). [`run`] catches it instead, remembers it in a
//! thread-local flag and returns the callback's neutral default; the message loop
//! then stops the same way for every callback: it removes the overlays, shows the
//! Windows clock and reports an error.
//!
//! Callbacks on foreign threads (WinHTTP, UI Automation) and the window
//! enumerations (which also run in the discovery helper process, where aborting is
//! the correct fail-closed outcome) are deliberately not wrapped: there is no loop
//! to hand a failure to. The panic hook has already logged the location by then.
use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
};

thread_local! {
    static PANICKED: Cell<bool> = const { Cell::new(false) };
}

/// Run a UI-thread callback body. On a panic, record it and return `default`.
pub(crate) fn run<R>(default: R, body: impl FnOnce() -> R) -> R {
    catch_unwind(AssertUnwindSafe(body)).unwrap_or_else(|_| {
        PANICKED.set(true);
        default
    })
}

/// Whether a guarded callback panicked since the last call. Clears the flag.
pub(crate) fn take_panic() -> bool {
    PANICKED.take()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_panic_returns_the_default_and_is_reported_once() {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        assert_eq!(run(7, || 1), 1);
        assert!(!take_panic());
        let result = run(7, || -> i32 { panic!("callback failed") });
        std::panic::set_hook(previous);
        assert_eq!(result, 7);
        assert!(take_panic());
        assert!(!take_panic());
    }
}
