//! Safe platform facade. Native resources stay on their owning thread/process.

#[cfg(windows)]
#[doc(hidden)]
#[allow(non_upper_case_globals, non_snake_case)]
pub mod bindings;

#[cfg(windows)]
mod settings;
pub mod taskbar;
#[cfg(windows)]
pub use settings::{PlatformError, SettingsWindow};

pub mod overlay;

pub mod logging;
pub mod managed;
pub mod preferences_store;
pub mod settings_model;
pub mod startup;
pub mod support;

pub mod render;
pub mod theme;

#[cfg(windows)]
pub(crate) mod accessibility;
pub mod clock_time;
pub mod events;
pub mod instance;
pub mod interaction;
pub mod runtime;
pub mod tray;
