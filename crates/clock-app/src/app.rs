//! Application lifecycle (R08): which role this process plays, single instance,
//! quiet versus interactive start, and hand-off to the runtime.
//!
//! There is no console, no public command suite and no usage text. Explorer
//! launches the same executable for the Start menu, a double-click and login
//! startup. Private markers select internal roles; --template accepts one bounded
//! presentation link for review in Settings, never an automatic Apply.
use crate::logging;
use clock_windows::{
    PlatformError,
    instance::{self, Acquired, LaunchMode, Role},
    preferences_store::{
        LocalPreferencesStore, PreferenceSession, PreferencesStore, UnavailableStore,
    },
    runtime::native::run,
    taskbar::helper,
};
use std::{process::ExitCode, time::Duration};

/// Long enough for a simultaneous first launch to finish creating its window.
const SIGNAL_WAIT: Duration = Duration::from_secs(5);

pub fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match Role::from_args(args.clone()) {
        Role::DiscoveryHelper { generation } => {
            ExitCode::from(u8::try_from(helper::run(generation)).unwrap_or(1))
        }
        Role::Application(launch) => match instance::template_from_args(&args)
            .map_err(|_| {
                PlatformError::message(
                    "The template link is invalid or unsupported. Your settings have not changed.",
                )
            })
            .and_then(|template| application(launch, template))
        {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                logging::failed(&error);
                error.show();
                ExitCode::FAILURE
            }
        },
    }
}

fn application(
    launch: LaunchMode,
    template: Option<clock_core::template::ClockTemplate>,
) -> Result<(), PlatformError> {
    match instance::acquire()? {
        Acquired::Existing(names) => {
            if let Some(template) = template {
                return if instance::signal_template(&names, &template, SIGNAL_WAIT) {
                    Ok(())
                } else {
                    Err(PlatformError::message(
                        "The running clock could not receive the template. Try again.",
                    ))
                };
            }
            // A second launch never adds a clock. An interactive one asks the
            // running instance to show Settings; a quiet startup launch does nothing.
            if launch == LaunchMode::Interactive && !instance::signal_existing(&names, SIGNAL_WAIT)
            {
                return Err(PlatformError::message(
                    "ElevenClock-rs is already running but did not respond",
                ));
            }
            Ok(())
        }
        Acquired::Primary(guard) => {
            if let Some(template) = template {
                clock_windows::templates::receive(template);
            }
            // Only the primary instance logs, so two processes never share a file.
            logging::start();
            // An unresolvable data folder must not stop the clock: run with defaults.
            // Every save then fails visibly in Settings rather than being lost.
            let result = match LocalPreferencesStore::new() {
                Ok(store) => start(store, launch, &guard),
                Err(_) => start(UnavailableStore, launch, &guard),
            };
            logging::stopped();
            result
        }
    }
}

fn start<S: PreferencesStore>(
    store: S,
    launch: LaunchMode,
    guard: &instance::InstanceGuard,
) -> Result<(), PlatformError> {
    run(PreferenceSession::open(store), launch, guard).map(|_| ())
}
