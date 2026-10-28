fn main() {
    // Same rule as clock-app: a release executable needs its native resources
    // (manifest, version info), which only a Windows MSVC/SDK host can compile.
    if !cfg!(windows)
        && std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("PROFILE").as_deref() == Ok("release")
    {
        panic!("Windows release builds require a Windows MSVC/SDK host for resources");
    }
    println!("cargo:rerun-if-changed=resources/launcher.rc");
    println!("cargo:rerun-if-changed=resources/launcher.manifest");
    println!("cargo:rerun-if-changed=../../Cargo.toml");
    // The version resource takes its numbers from Cargo, like the app's.
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
    #[cfg(windows)]
    embed_resource::compile("resources/launcher.rc", embed_resource::NONE)
        .manifest_required()
        .expect("Launcher resources and manifest must be linked");
}
