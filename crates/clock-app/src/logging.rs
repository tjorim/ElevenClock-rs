//! Application-side logging set-up (R11). The bounded writer, redaction and the
//! transition lines themselves live in `clock-windows`, next to the runtime that
//! produces them; this only turns logging on and records the outer lifecycle.
//!
//! Logging is best effort: it can never stop the clock, and nothing leaves the PC.
use clock_windows::{logging, preferences_store::StoreError};

/// Start the bounded local log and make an unexpected panic leave a line in it.
/// Call once, by the primary instance only.
pub fn start() {
    logging::init();
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let message = info
            .payload()
            .downcast_ref::<&str>()
            .map(|text| (*text).to_owned())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "no message".to_owned());
        logging::log(&format!("internal error: {message}"));
        previous(info);
    }));
    logging::log(&format!(
        "ElevenClock-rs {} started",
        env!("CARGO_PKG_VERSION")
    ));
}

/// The data folder could not be resolved: the clock runs on defaults and every save
/// fails visibly. Without this line the log would not say why.
pub fn unavailable_store(error: &StoreError) {
    logging::log(&format!(
        "preferences folder unavailable, using defaults: {}",
        error.log_text()
    ));
}

/// Saved preferences could not be loaded (unreadable, damaged or an unsupported version).
pub fn load_failed(error: &StoreError) {
    logging::log(&format!(
        "saved preferences not loaded: {}",
        error.log_text()
    ));
}

/// Stale temporary files removed at start (nothing is logged when there were none).
pub fn swept(count: usize) {
    if count > 0 {
        logging::log(&format!("removed {count} stale temporary files"));
    }
}

/// Record a failure that is about to be shown to the user.
pub fn failed(error: &dyn std::fmt::Display) {
    logging::log(&format!("stopped: {error}"));
}

pub fn stopped() {
    logging::log("ElevenClock-rs stopped");
}
