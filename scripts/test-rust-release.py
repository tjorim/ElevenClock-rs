"""Regression tests for binary and packaging checks, without running an app."""
import hashlib
import importlib.util
from pathlib import Path
import struct
import os
import tempfile
import unittest
import zipfile

spec = importlib.util.spec_from_file_location("release", Path(__file__).with_name("rust-release.py"))
release = importlib.util.module_from_spec(spec)
spec.loader.exec_module(release)


def pe(dll=b"KERNEL32.dll", machine=0x8664, subsystem=2, delayed=False):
    data = bytearray(2048)
    data[:2] = b"MZ"
    struct.pack_into("<I", data, 0x3C, 0x80)
    data[0x80:0x84] = b"PE\0\0"
    struct.pack_into("<HH", data, 0x84, machine, 1)
    struct.pack_into("<H", data, 0x94, 240)
    opt = 0x98
    struct.pack_into("<H", data, opt, 0x20B)
    struct.pack_into("<H", data, opt + 68, subsystem)
    struct.pack_into("<II", data, opt + 120, 0x1000, 40)
    if delayed:
        struct.pack_into("<I", data, opt + 112 + 13 * 8, 0x1100)
    struct.pack_into("<IIII", data, opt + 240 + 8, 1024, 0x1000, 1024, 512)
    struct.pack_into("<IIIII", data, 512, 0, 0, 0, 0x1100, 0)
    data[768:768 + len(dll) + 1] = dll + b"\0"
    return data


class Gates(unittest.TestCase):
    def test_system_imports(self):
        self.assertEqual(release.inspect_pe(pe()), ["kernel32.dll"])
        self.assertEqual(release.inspect_pe(pe(b"api-ms-win-core-synch-l1-2-0.dll")), ["api-ms-win-core-synch-l1-2-0.dll"])

    def test_rejects_runtime_or_unreviewed_dll(self):
        for dll in (b"VCRUNTIME140.dll", b"ucrtbase.dll", b"api-ms-win-crt-runtime-l1-1-0.dll", b"Qt6Core.dll", b"python313.dll"):
            with self.subTest(dll=dll), self.assertRaises(ValueError):
                release.inspect_pe(pe(dll))

    def test_rejects_wrong_arch_console_and_unreviewed_delay_table(self):
        for args in ({"machine": 0xAA64}, {"subsystem": 3}, {"delayed": True}):
            with self.subTest(args=args), self.assertRaises(ValueError):
                release.inspect_pe(pe(**args))

    def test_rejects_invalid_pe(self):
        with self.assertRaises(ValueError):
            release.inspect_pe(b"not an executable")

    def test_zip_timestamps_before_1980_can_be_archived(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "source"
            source.mkdir()
            file = source / "LICENSE"
            file.write_bytes(b"original notice\n")
            os.utime(file, (1, 1))
            release.zip_tree(source, root / "package.zip")
            with zipfile.ZipFile(root / "package.zip") as archive:
                self.assertEqual(archive.read("LICENSE"), b"original notice\n")

    def test_finish_rejects_incomplete_artifact_set(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            release.write_json(root / "package.json", {"name": "candidate"})
            with self.assertRaisesRegex(ValueError, "Missing artifact"):
                release.finish(root)

    def package(self, root, files=None, managed=None, launcher=None):
        """A fake candidate: portable and managed ZIPs, installer, raw exes and checksums."""
        exe = bytes(pe())
        launcher = launcher or bytes(pe())
        files = files or {name: b"text" for name in release.PORTABLE_FILES}
        files[f"{release.PORTABLE_FOLDER}/elevenclock-rs.exe"] = exe
        if managed is None:
            managed = {name: b"text" for name in release.MANAGED_FILES}
            managed[f"{release.PORTABLE_FOLDER}/elevenclock-rs.data"] = exe
            managed[f"{release.PORTABLE_FOLDER}/SHA256SUMS.txt"] = (
                f"{hashlib.sha256(exe).hexdigest()}  elevenclock-rs.data\n".encode())
            managed[f"{release.PORTABLE_FOLDER}/elevenclock-rs-launcher.exe"] = launcher
        release.write_json(root / "package.json", {"name": "candidate"})
        for suffix, contents in (("portable", files), ("managed", managed)):
            with zipfile.ZipFile(root / f"candidate-{suffix}.zip", "w") as archive:
                for name, data in contents.items():
                    archive.writestr(name, data)
        (root / "candidate-setup.exe").write_bytes(b"installer")
        (root / "payload").mkdir()
        (root / "launcher").mkdir()
        (root / "symbols").mkdir()
        (root / "symbols/elevenclock_rs.pdb").write_bytes(b"pdb")
        (root / "payload/elevenclock-rs.exe").write_bytes(exe)
        (root / "launcher/elevenclock-rs-launcher.exe").write_bytes(launcher)
        release.finish(root)

    def test_verify_accepts_single_folder_portable_layout(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.package(root)
            release.verify(root)
            self.assertEqual(
                sorted(line.split("  ", 1)[1] for line in (root / "SHA256SUMS.txt").read_text().splitlines()),
                [
                    "candidate-managed.zip", "candidate-portable.zip", "candidate-setup.exe",
                    "launcher/elevenclock-rs-launcher.exe", "payload/elevenclock-rs.exe",
                ],
            )

    def test_verify_rejects_removed_evidence_and_flat_layout(self):
        base = {name: b"text" for name in release.PORTABLE_FILES}
        flat = {name.split("/", 1)[1]: data for name, data in base.items()}
        for label, extra in (
            ("provenance", {**base, "ElevenClock-rs/provenance.json": b"{}"}),
            ("dependencies", {**base, "ElevenClock-rs/runtime-dependencies.json": b"{}"}),
            ("flat", flat),
        ):
            with self.subTest(label), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                self.package(root, dict(extra))
                with self.assertRaisesRegex(ValueError, "allowlist"):
                    release.verify(root)

    def test_verify_rejects_unreviewed_import_in_shipped_exe(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.package(root)
            bad = bytes(pe(b"Qt6Core.dll"))
            with zipfile.ZipFile(root / "candidate-portable.zip", "w") as archive:
                for name in release.PORTABLE_FILES:
                    archive.writestr(name, bad if name.endswith(".exe") else b"text")
            (root / "payload/elevenclock-rs.exe").write_bytes(bad)
            release.finish(root)
            with self.assertRaisesRegex(ValueError, "Unreviewed native dependency"):
                release.verify(root)


    def test_managed_zip_is_the_portable_folder_plus_the_launcher_with_a_data_app(self):
        self.assertEqual(release.MANAGED_FILES - release.PORTABLE_FILES, {
            "ElevenClock-rs/elevenclock-rs-launcher.exe", "ElevenClock-rs/elevenclock-rs.data",
            "ElevenClock-rs/SHA256SUMS.txt",
        })
        self.assertEqual(release.PORTABLE_FILES - release.MANAGED_FILES, {"ElevenClock-rs/elevenclock-rs.exe"})

    def test_verify_rejects_a_managed_zip_with_other_files(self):
        base = {name: b"text" for name in release.MANAGED_FILES}
        for label, managed in (
            ("extra", {**base, "ElevenClock-rs/source.txt": b"x"}),
            ("no launcher", {k: v for k, v in base.items() if "launcher" not in k}),
        ):
            with self.subTest(label), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                self.package(root, managed=dict(managed))
                with self.assertRaisesRegex(ValueError, "Managed ZIP file allowlist"):
                    release.verify(root)

    def test_verify_rejects_a_different_app_executable_in_the_managed_zip(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.package(root)
            other = bytes(pe(b"USER32.dll"))
            with zipfile.ZipFile(root / "candidate-managed.zip", "w") as archive:
                for name in release.MANAGED_FILES:
                    archive.writestr(name, other if name.endswith("/elevenclock-rs.data") else (
                        bytes(pe()) if name.endswith("-launcher.exe") else b"text"))
            release.finish(root)
            with self.assertRaisesRegex(ValueError, "Managed ZIP app executable"):
                release.verify(root)

    def test_verify_rejects_a_managed_zip_whose_checksum_list_is_stale(self):
        for label, sums in (
            ("other hash", b"0" * 64 + b"  elevenclock-rs.data\n"),
            ("other name", None),
        ):
            with self.subTest(label), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                exe = bytes(pe())
                managed = {name: b"text" for name in release.MANAGED_FILES}
                managed["ElevenClock-rs/elevenclock-rs.data"] = exe
                managed["ElevenClock-rs/elevenclock-rs-launcher.exe"] = bytes(pe())
                managed["ElevenClock-rs/SHA256SUMS.txt"] = sums or (
                    f"{hashlib.sha256(exe).hexdigest()}  elevenclock-rs.exe\n".encode())
                self.package(root, managed=managed)
                with self.assertRaisesRegex(ValueError, "SHA256SUMS.txt does not list"):
                    release.verify(root)

    def test_managed_checksum_list_uses_plain_newlines(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            (folder / release.MANAGED_APP).write_bytes(b"app")
            release.write_managed_sums(folder)
            self.assertEqual(
                (folder / release.MANAGED_SUMS).read_bytes(),
                f"{hashlib.sha256(b'app').hexdigest()}  elevenclock-rs.data\n".encode())

    def test_app_may_import_winhttp_for_explicit_release_checks(self):
        release.inspect_pe(pe(b"WINHTTP.dll"))

    def test_launcher_must_not_import_the_network(self):
        release.inspect_pe(pe(b"KERNEL32.dll"), forbidden=release.NETWORK_DLLS)
        for dll in (b"WINHTTP.dll", b"WININET.dll", b"WS2_32.dll"):
            with self.subTest(dll=dll), self.assertRaisesRegex(ValueError, "Forbidden native dependency"):
                release.inspect_pe(pe(dll), forbidden=release.NETWORK_DLLS)

    def test_verify_rejects_a_launcher_with_network_imports(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            # winhttp.dll is not on the system allowlist either, so allow it to isolate the rule.
            bad = bytes(pe(b"WINHTTP.dll"))
            original = set(release.SYSTEM_DLLS)
            release.SYSTEM_DLLS.add("winhttp.dll")
            try:
                self.package(root, launcher=bad)
                with self.assertRaisesRegex(ValueError, "Forbidden native dependency"):
                    release.verify(root)
            finally:
                release.SYSTEM_DLLS.clear()
                release.SYSTEM_DLLS.update(original)


if __name__ == "__main__":
    unittest.main()
