//! The managed launcher (ADR 0004): copies the centrally deployed app to a per-user
//! local folder, starts that copy and exits.
//!
//! Portable logic (arguments, source rules, validation, hashing, the decisions
//! themselves) is tested on any host. `native` is the Windows-only `Host`.
pub mod args;
pub mod fsops;
pub mod inventory;
pub mod inventory_config;
pub mod inventory_publish;
pub mod inventory_summary;
pub mod inventory_support;
pub mod launch;
pub mod pe;
pub mod sha256;
pub mod source;
pub mod uninstall;

#[cfg(windows)]
pub mod native;

#[cfg(windows)]
mod inventory_native;
