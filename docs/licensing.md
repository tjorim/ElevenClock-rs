# Licensing and provenance

ElevenClock-rs (the Rust implementation, its tooling and the original artwork in
[`assets/`](../assets)) is licensed under the [Apache License 2.0](../LICENSE). The
inherited Python application and its imagery stay under GPLv3 in Git history; they are
not part of this tree, its packages or its source companion.

## Decision

On 2026-10-16 the owner stated: "We never modified or used the python code itself, we
can keep the fork in legacy gpl and change ours", and selected Apache 2.0. That
authorship statement is the basis for the license. It is not a relicensing of upstream
material, and no upstream permission is claimed.

- Root `LICENSE` is Apache 2.0; all crates inherit `license = "Apache-2.0"` from the
  workspace. Installer and ZIP copy it into `LICENSE.txt`.
- The previous GPLv3 text, the Python sources, upstream imagery and the original
  credits to Martí Climent and Xavier Delugo are kept in Git history (see the
  [migration guide](migration.md#legacy-reference)).
- Dependency licenses and original notices are unchanged and inventoried separately
  ([dependency policy](dependencies.md)).
- Apache 2.0 for this code grants no rights to the ElevenClock name; see the
  [product name review](product-name.md).

## Supporting review

A sampled technical review (2026-10-16) supported the authorship statement; it is not an
independent legal determination or an exhaustive audit.

- **Whole-line comparison** of the Rust sources (63 files) with the Python sources
  (15 files), whitespace-collapsed and ignoring lines under 32 characters: 7,037 Rust
  lines, 3,087 Python lines, **zero shared lines**. This cannot find translated syntax
  or adapted algorithms.
- **Targeted inspection** of structure, formatting/presets, constants and tables,
  comments and UI wording found no copied tables, paragraphs or preset lists. Shared
  items are platform identifiers (`Shell_TrayWnd`, `Shell_SecondaryTrayWnd`), strftime
  token names and short vocabulary such as Settings and Exit.

## Artwork

`assets/icon.svg` is original geometric artwork drawn directly in SVG, without tracing an
upstream image; `scripts/build-icon.cjs` renders the ICO and PNG (see
[`assets/README.md`](../assets/README.md)). It replaced an icon that was byte-identical to
the upstream one (SHA256 `ada188d1…21d08bc`).
