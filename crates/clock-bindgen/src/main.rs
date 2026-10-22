//! Regenerates `clock-windows/src/bindings/generated.rs` from `clock-windows/bindings.txt`.
//!
//! Run `cargo run -p clock-bindgen` after editing the API list; CI fails if the
//! checked-in bindings differ from a fresh generation.

use std::path::Path;

fn main() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../clock-windows");
    let filter = crate_dir.join("bindings.txt");
    let out = crate_dir.join("src/bindings/generated.rs");
    windows_bindgen::bindgen([
        "--out",
        out.to_str().expect("UTF-8 path"),
        "--flat",
        "--filter-file",
        filter.to_str().expect("UTF-8 path"),
    ]);
}
