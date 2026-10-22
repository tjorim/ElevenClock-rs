# Supplemental dependency notices

The inventory checker normally reads original LICENSE/COPYING/NOTICE files
from Cargo's extracted packages. `manifest.json` names narrowly scoped exceptions
for upstream packaging and filename conventions. Each entry matches a package
name, exact version and exact license expression; each original notice is
verified by SHA-256. Missing files, changed license expressions and digest
mismatches fail the check. The checker never downloads fallback text in CI.

- **anes 0.1.6**: the published crate omits the upstream repository-root licenses.
  Its `.cargo_vcs_info.json` identifies commit
  `378ac7461ecac29fde9e10df7b08359d1315a9ad` (tag `v0.1.6`). The original
  LICENSE-MIT and LICENSE-APACHE from that commit are copied here, together with
  `anes/README.md`, which carries the contributor copyright declaration. Files
  are unmodified. Per-file source URLs and digests are in the manifest.
- **r-efi 6.0.0**: its published `AUTHORS` contains the original license grant,
  copyright notices and author list. The crate's recorded commit is
  `7e1b0322d31d625f81a5656096330934f9cd835d` (tag `v6.0.0`). The manifest
  explicitly recognizes and checks that bundled file; no substitute license
  text is generated or fetched. Generic AUTHORS or README files are not
  accepted as notices for other packages.
- **winapi-i686-pc-windows-gnu 0.4.0, winapi-x86_64-pc-windows-gnu 0.4.0**:
  the published import-library crates omit the repository-root licenses
  (upstream added them only after 0.4.0) and carry no `.cargo_vcs_info.json`.
  Both were published from winapi-rs commit
  `9497609ef44cc9bcd16cd2411c0ee6ccaf5483aa` ("Publish 0.3.4"), whose
  `i686/Cargo.toml` and `x86_64/Cargo.toml` declare version 0.4.0. The original
  LICENSE-MIT and LICENSE-APACHE from that commit are copied here unmodified.
  They are pulled in by criterion's `page_size` → `winapi` (bench-only, and only
  for `windows-gnu` targets).

The combined CI notice artifact includes all these original texts and their
source URLs. Future dependency versions require checking their packaged notices
and updating the mapping only when necessary, against their own immutable source.
