fn main() {
    // Cross-target type checking is useful on Linux, but must never produce a
    // release executable without the native resources and security manifest.
    if !cfg!(windows)
        && std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("PROFILE").as_deref() == Ok("release")
    {
        panic!("Windows release builds require a Windows MSVC/SDK host for resources");
    }
    println!("cargo:rerun-if-changed=resources/settings.rc");
    println!("cargo:rerun-if-changed=resources/app.manifest");
    println!("cargo:rerun-if-changed=../../assets/icon.ico");
    println!("cargo:rerun-if-changed=../../Cargo.toml");
    // The version resource takes the product version from Cargo, so every build of
    // a release carries the same version (code signing enforces this metadata).
    // embed-resource passes OUT_DIR to the resource compiler as an include path.
    let version = std::env::var("CARGO_PKG_VERSION").expect("Cargo sets the package version");
    let number = |part: &str| std::env::var(part).expect("Cargo sets the version parts");
    let header = format!(
        "#define ELEVENCLOCK_VERSION_NUMBER {},{},{},0\n#define ELEVENCLOCK_VERSION_TEXT \"{version}\"\n",
        number("CARGO_PKG_VERSION_MAJOR"),
        number("CARGO_PKG_VERSION_MINOR"),
        number("CARGO_PKG_VERSION_PATCH"),
    );
    let out_dir = std::env::var("OUT_DIR").expect("Cargo sets OUT_DIR");
    std::fs::write(
        std::path::Path::new(&out_dir).join("elevenclock_version.h"),
        header,
    )
    .expect("Version header must be written");
    // Development examples need the same DPI/security manifest; these resources
    // are linked into examples separately and never add examples to app payloads.
    #[cfg(windows)]
    embed_resource::compile_for_examples("resources/settings.rc", embed_resource::NONE)
        .manifest_required()
        .expect("Development example DPI manifest must be linked");
    #[cfg(windows)]
    embed_resource::compile("resources/settings.rc", embed_resource::NONE)
        .manifest_required()
        .expect("Settings resources and DPI manifest must be linked");
}
