# Documentation

Docs for ElevenClock-rs. Using the app is covered in the [README](../README.md); moving
from the original Python ElevenClock is covered in the [migration guide](migration.md).
These Markdown files are the single source: the [website](https://tjorim.github.io/ElevenClock-rs/docs/)
renders them, with relative links rewritten to the published pages, and
[`site/README.md`](../site/README.md) explains the routes and how to preview them.

| Folder / file | What it is | Changes when |
| --- | --- | --- |
| [`migration.md`](migration.md) | Moving from the Python ElevenClock, removed features, rollback and where the legacy code and development history live in Git | Support or the legacy reference changes |
| [`managed-deployment.md`](managed-deployment.md) | Admin guide for the managed ZIP: central share, rollout, updates, offline behavior, removal and troubleshooting | The launcher or managed packaging changes |
| [`tested-configurations.md`](tested-configurations.md) | What has been run on real Windows, resource use and known limitations | Something new is tested |
| [`contributing.md`](contributing.md) | Workspace layout, commands, CI and the contracts contributors must keep | The workflow changes |
| [`dependencies.md`](dependencies.md), [`dependency-notices/`](dependency-notices) | Dependency policy and bundled third-party notices | A dependency is added, removed or updated |
| [`releasing.md`](releasing.md) | Native x64 packaging, installer ownership and publishing a release | Packaging changes |
| [`licensing.md`](licensing.md), [`product-name.md`](product-name.md) | The Apache 2.0 decision and artwork provenance; the product name, listing copy and distribution channels | The license or name decision changes |
| [`../site/`](../site/README.md) | The public website and these docs ([tjorim.github.io/ElevenClock-rs](https://tjorim.github.io/ElevenClock-rs/)): an Astro build that renders the Markdown in this repository, deployed from `main` by `pages.yml` | Features, install steps or the format language change; a page is added |
| [`design/`](design) | How each part behaves and why: the interfaces, states and limits a reader needs | The behaviour changes |
| [`adr/`](adr) | Architecture decisions that were weighed and settled | A decision is made or superseded |

## `design/`

| Topic | File |
| --- | --- |
| Clock state, scheduling and effects (pure core) | [`state-and-scheduling.md`](design/state-and-scheduling.md) |
| Format strings and presets | [`FORMAT.md`](../crates/clock-core/FORMAT.md) |
| Saved preferences: encoding, validation, recovery | [`preferences.md`](design/preferences.md) |
| DirectWrite rendering, DPI, theme, overflow | [`rendering.md`](design/rendering.md) |
| Runtime, monitors, Explorer recovery, shutdown | [`lifecycle.md`](design/lifecycle.md) |
| Clock actions, tray, keyboard and accessibility | [`interaction.md`](design/interaction.md) |
| Settings window and the preference flow | [`settings.md`](design/settings.md) |
| Support status, Copy diagnostics, local logs | [`diagnostics.md`](design/diagnostics.md) |
| Opt-in login startup and the uninstall contract | [`startup.md`](design/startup.md) |

## `adr/`

| Decision | File |
| --- | --- |
| Fail-closed taskbar discovery in an isolated process | [`0001-taskbar-discovery.md`](adr/0001-taskbar-discovery.md) |
| The discovery helper is the app's own executable | [`0002-same-executable-discovery-helper.md`](adr/0002-same-executable-discovery-helper.md) |
| Optional update checks against GitHub Releases | [`0003-update-checks.md`](adr/0003-update-checks.md) |
| Managed launcher that deploys a local copy from a central source | [`0004-managed-launcher.md`](adr/0004-managed-launcher.md) |

Screenshots used by a design file live next to it, in [`design/images/`](design/images).
Docs describe the app as it is now. Verification logs, screenshots and dated run records
from development are in Git history ([migration guide](migration.md#legacy-reference)).
