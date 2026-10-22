# Application artwork

`icon.svg` is original geometric artwork created for ElevenClock-rs on 2026-10-16:
an ivory clock face, orange hands and a mint taskbar on a navy rounded tile.
It uses no upstream image, font, traced outline or external illustration.
The SVG and its generated `icon.ico` / `icon.png` are supplied under
[Apache License 2.0](../LICENSE), like the original Rust implementation.
This license choice does not grant rights to the ElevenClock name.

To regenerate, make the development-only Node package `sharp@0.35.5` available and run
`node scripts/build-icon.cjs`. Node and sharp are not application dependencies.
The ICO contains transparent 32-bit PNG frames at 16, 20, 24, 32, 40, 48, 64, 128
and 256 pixels. The resource compiler embeds it as resource 102, which Settings
and the tray already load. No Rust resource IDs or paths need to change.

The inherited icon, upstream banners, screenshots and artwork were removed from
the tree at cutover. They remain in Git history under
[`legacy/`](https://github.com/tjorim/ElevenClock-rs/tree/0ac31152e265bb5d6e02fcac18404362453670d2/legacy) with their GPLv3 license and
historical notices. See [licensing](../docs/licensing.md).
