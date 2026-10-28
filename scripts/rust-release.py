"""Development-only release assembly and fail-closed validation (Python 3.11+)."""
import argparse
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import shutil
import struct
import subprocess
import sys
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parent.parent
TARGET = "x86_64-pc-windows-msvc"
PORTABLE_FOLDER = "ElevenClock-rs"
# Each artifact type gets its own readme, shipped as README.txt.
README_SOURCES = {
    "installer": "packaging/INSTALLER-README.txt",
    "portable": "packaging/PORTABLE-README.txt",
}
PORTABLE_FILES = {
    f"{PORTABLE_FOLDER}/{file}" for file in (
        "elevenclock-rs.exe", "README.txt", "LICENSE.txt", "THIRD-PARTY-NOTICES.txt",
        "RUST-COPYRIGHT.html",
    )
}
# In-box DLLs the release executable imports, besides api-ms-win-* API sets. Keep
# this to what is actually imported: a new entry is a reviewed native dependency.
SYSTEM_DLLS = {
    "advapi32.dll", "bcryptprimitives.dll", "comctl32.dll", "d2d1.dll", "dwmapi.dll", "dwrite.dll",
    "kernel32.dll", "ntdll.dll", "ole32.dll", "oleaut32.dll", "shell32.dll",
    "uiautomationcore.dll", "user32.dll", "wtsapi32.dll",
}


def run(*args):
    return subprocess.check_output(args, cwd=ROOT).decode("utf-8").strip()


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def inspect_pe(data):
    """Read the PE32+ import table without executing the candidate."""
    if data[:2] != b"MZ":
        raise ValueError("Not a PE executable")
    pe = struct.unpack_from("<I", data, 0x3C)[0]
    if data[pe:pe + 4] != b"PE\0\0":
        raise ValueError("Invalid PE signature")
    machine, count = struct.unpack_from("<HH", data, pe + 4)
    size = struct.unpack_from("<H", data, pe + 20)[0]
    opt = pe + 24
    if machine != 0x8664 or struct.unpack_from("<H", data, opt)[0] != 0x20B:
        raise ValueError("Expected native x64 PE32+")
    if struct.unpack_from("<H", data, opt + 68)[0] != 2:
        raise ValueError("Expected GUI subsystem (no console)")
    sections = []
    for i in range(count):
        sections.append(struct.unpack_from("<IIII", data, opt + size + i * 40 + 8))

    def offset(rva):
        for virtual_size, address, raw_size, raw in sections:
            if address <= rva < address + max(virtual_size, raw_size):
                delta = rva - address
                if delta >= raw_size or raw + delta >= len(data):
                    break
                return raw + delta
        raise ValueError(f"Unmapped PE RVA: {rva:#x}")

    # Fail closed rather than silently omitting a second dependency mechanism.
    if struct.unpack_from("<I", data, opt + 112 + 13 * 8)[0]:
        raise ValueError("Delay imports require explicit runtime review")
    imports = set()
    rva = struct.unpack_from("<I", data, opt + 120)[0]
    if not rva:
        raise ValueError("Missing native imports")
    pos = offset(rva)
    while True:
        descriptor = struct.unpack_from("<IIIII", data, pos)
        if not any(descriptor):
            break
        name = offset(descriptor[3])
        imports.add(data[name:data.index(b"\0", name)].decode("ascii").lower())
        pos += 20
    for name in sorted(imports):
        api_set = name.startswith(("api-ms-win-", "ext-ms-win-")) and name.endswith(".dll")
        if "crt" in name or name.startswith(("msvcp", "vcruntime")):
            raise ValueError(f"Dynamic CRT dependency is forbidden: {name}")
        if name not in SYSTEM_DLLS and not api_set:
            raise ValueError(f"Unreviewed native dependency: {name}")
    return sorted(imports)


def zip_tree(source, destination):
    with zipfile.ZipFile(destination, "w", zipfile.ZIP_DEFLATED, strict_timestamps=False) as archive:
        for file in sorted(source.rglob("*")):
            if file.is_file():
                if file.is_symlink():
                    raise ValueError(f"Refusing symlink: {file}")
                archive.write(file, file.relative_to(source).as_posix())


def prepare(out, mode):
    if out.exists():
        raise ValueError("Output must be a new directory; stale artifacts are not reusable")
    if out.is_relative_to(ROOT) and not out.is_relative_to(ROOT / "dist"):
        raise ValueError("In-repository output must be under ignored dist/ to avoid capturing itself")
    dirty = bool(run("git", "status", "--porcelain"))
    if mode == "Distribution":
        if dirty:
            raise ValueError("Distribution requires a clean source checkout")
        # Development checkouts may carry blocked wording; a release must not.
        for readme in README_SOURCES.values():
            if "Distribution is blocked" in (ROOT / readme).read_text():
                raise ValueError("Distribution requires reviewed release notice wording")
    out.mkdir(parents=True)
    # The installer payload is flat; the portable ZIP is zipped from its parent so
    # that everything extracts into one ElevenClock-rs folder.
    payload = out / "payload"
    payload.mkdir()
    portable = out / "portable" / PORTABLE_FOLDER
    portable.mkdir(parents=True)
    symbols = out / "symbols"
    symbols.mkdir()
    version = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]["package"]["version"]
    if mode == "Distribution":
        name = f"ElevenClock-rs-{version}-x64"
    else:
        name = f"ElevenClock-rs-{version}-{run('git', 'rev-parse', '--short=12', 'HEAD')}-x64-dev"
    binary_dir = ROOT / "target/packaging" / TARGET / "release"
    binary = binary_dir / "elevenclock-rs.exe"
    inspect_pe(binary.read_bytes())  # Build-time gate: fails on an unreviewed or CRT import.
    pdb = binary_dir / "elevenclock_rs.pdb"
    if not pdb.is_file():
        raise ValueError("Missing release PDB")
    shutil.copy2(pdb, symbols / pdb.name)
    spec = importlib.util.spec_from_file_location("license_check", ROOT / "scripts/rust-license-check.py")
    checker = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(checker)
    metadata = json.loads(run("cargo", "metadata", "--locked", "--offline", "--format-version", "1"))
    notices = io.StringIO()
    if checker.inventory(metadata, out=notices):
        raise ValueError("Dependency license inventory failed")
    rust_docs = Path(run("rustc", "--print", "sysroot")) / "share/doc/rust"
    for directory, readme in (
        (payload, README_SOURCES["installer"]), (portable, README_SOURCES["portable"]),
    ):
        shutil.copy2(binary, directory / binary.name)
        shutil.copy2(ROOT / "LICENSE", directory / "LICENSE.txt")
        shutil.copy2(ROOT / readme, directory / "README.txt")
        (directory / "THIRD-PARTY-NOTICES.txt").write_text(notices.getvalue(), encoding="utf-8")
        shutil.copy2(rust_docs / "COPYRIGHT.html", directory / "RUST-COPYRIGHT.html")
    zip_tree(out / "portable", out / (name + "-portable.zip"))
    write_json(out / "package.json", {"name": name, "version": version})


def finish(out):
    package = json.loads((out / "package.json").read_text())
    name = package["name"]
    required = [out / (name + suffix) for suffix in ("-portable.zip", "-setup.exe")]
    required += [out / "payload/elevenclock-rs.exe"]
    for file in required:
        if not file.is_file():
            raise ValueError(f"Missing artifact: {file}")
    # The checksum file sits alongside the downloaded archives/installer.
    (out / "SHA256SUMS.txt").write_text("".join(
        f"{digest(file)}  {file.relative_to(out).as_posix()}\n" for file in required
    ), encoding="utf-8")
    symbols = out / "symbols"
    (symbols / "SHA256SUMS.txt").write_text("".join(
        f"{digest(file)}  {file.name}\n" for file in sorted(symbols.iterdir())
        if file.name != "SHA256SUMS.txt"
    ), encoding="utf-8")


def verify(out):
    package = json.loads((out / "package.json").read_text())
    name = package["name"]
    expected = {name + suffix for suffix in ("-portable.zip", "-setup.exe")}
    expected.add("payload/elevenclock-rs.exe")
    hashes = {}
    for line in (out / "SHA256SUMS.txt").read_text().splitlines():
        sha, relative = line.split("  ", 1)
        if relative not in expected or relative in hashes:
            raise ValueError(f"Unexpected/duplicate checksum entry: {relative}")
        if digest(out / relative) != sha:
            raise ValueError(f"Artifact checksum mismatch: {relative}")
        hashes[relative] = sha
    if set(hashes) != expected:
        raise ValueError("Incomplete artifact checksums")
    with zipfile.ZipFile(out / (name + "-portable.zip")) as archive:
        if set(archive.namelist()) != PORTABLE_FILES or len(archive.namelist()) != len(PORTABLE_FILES):
            raise ValueError("Portable ZIP file allowlist differs")
        executable = archive.read(f"{PORTABLE_FOLDER}/elevenclock-rs.exe")
        if hashlib.sha256(executable).hexdigest() != hashes["payload/elevenclock-rs.exe"]:
            raise ValueError("Portable executable differs from checksummed executable")
        # The shipped bytes must still pass the import and CRT gate.
        inspect_pe(executable)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("prepare", "finish", "verify"))
    parser.add_argument("--output", type=Path, default=ROOT / "dist/candidate")
    parser.add_argument("--mode", choices=("Development", "Distribution"), default="Development")
    args = parser.parse_args()
    if args.action == "prepare":
        prepare(args.output.resolve(), args.mode)
    elif args.action == "finish":
        finish(args.output.resolve())
    else:
        verify(args.output.resolve())


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError, struct.error, subprocess.CalledProcessError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        sys.exit(1)
