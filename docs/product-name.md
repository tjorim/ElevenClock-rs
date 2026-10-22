# Product name and listings

The owner keeps the name **ElevenClock-rs**: a community open-source Rust rewrite of
ElevenClock, credited to the original and not presented as an official successor.
No trademark clearance is sought. Distribution is **GitHub releases first; winget
later**; the Microsoft Store is outside the current plan. No name reservation, winget
submission or Store submission has been made.

## Existing use and package separation

| Surface | Observed upstream use | This project's identity |
| --- | --- | --- |
| Product name | ElevenClock, maintained by Martí Climent | ElevenClock-rs, maintained separately by tjorim |
| Microsoft Store | [ElevenClock, product XP89HZ8SVWTT0M](https://apps.microsoft.com/detail/XP89HZ8SVWTT0M) | No Store ID or name reservation established; do not reuse upstream's product ID |
| winget | `SomePythonThings.ElevenClock`, publisher Martí Climent, moniker `elevenclock` | Proposed `tjorim.ElevenClockRs`, publisher ElevenClock-rs contributors, proposed moniker `elevenclock-rs` |
| Installer AppId | `{D62480B8-71F1-48CE-BEEC-9D3E172C87B5}` | `{97911B67-52D7-4D9B-B833-29303BC540A7}` |
| Start menu AppUserModelID | Not reused | Existing `tjorim.ElevenClockRs` |
| Executable | Legacy ElevenClock | `elevenclock-rs.exe` |
| Install/data directories | Legacy paths retained separately | `%LOCALAPPDATA%\Programs\ElevenClock-rs` and `%LOCALAPPDATA%\ElevenClock-rs` |
| Login startup value | `elevenClock` | Existing `ElevenClockRust`; preserve this established ownership contract |

The [upstream winget manifest](https://github.com/microsoft/winget-pkgs/blob/master/manifests/s/SomePythonThings/ElevenClock/4.4.1.1/SomePythonThings.ElevenClock.locale.en-US.yaml)
was read directly from Microsoft's repository. It confirms the upstream ID,
publisher, product name, moniker and project URL; its tags also identify the Store
product. The public Store search returned the existing ElevenClock product page,
but the page content requires JavaScript; its complete current metadata was not
independently extracted in this review.

A direct GitHub Contents API lookup of
`manifests/t/tjorim/ElevenClockRs` in `microsoft/winget-pkgs` returned 404 on the
review date. This establishes only that no manifest existed at that exact path
at that time. It does not reserve an identifier, check pending submissions or
establish acceptance. Microsoft's [manifest guidance](https://learn.microsoft.com/en-us/windows/package-manager/package/manifest)
uses `Publisher.Package` identifiers; the proposed ID follows that form and is
distinct from upstream's. An AppUserModelID is not a winget registration or a
Microsoft Store identity, even when the strings match.

## Listing copy and intended identity

Use the following as a reviewable draft, subject to actual release validation:

- **Display name:** ElevenClock-rs.
- **Publisher:** ElevenClock-rs contributors. **Maintainer:** tjorim.
- **Short description:** A customizable native clock for the Windows 11 taskbar.
- **Project URL:** https://github.com/tjorim/ElevenClock-rs
- **Support URL:** https://github.com/tjorim/ElevenClock-rs/issues
- **License:** Apache-2.0 for the original implementation and artwork; third-party
  notices keep their own terms.
- **Relationship statement:** An independent Rust implementation maintained by
  tjorim. Affiliation with or endorsement by the original ElevenClock project is
  not claimed.
- **Description:** Choose a date/time format or ISO week-date preset, adjust the
  font and clock theme, and manage login startup through the app's native Settings window.
  Closing the app's Settings window keeps the clock running; Exit restores the
  native clock. This is a separate application from the original Python/Qt
  ElevenClock. Do not run both overlay applications simultaneously.

Do not claim monitor/OS compatibility beyond [tested configurations](tested-configurations.md).
Use the original icon and current-app screenshots only, not upstream imagery. Do not use upstream's Store badges,
package IDs, publisher identity or `elevenclock` moniker for this app.

## Later channels

- **winget:** recheck the proposed ID and moniker, use the actual released installer
  URL/SHA256/version, verify silent per-user install and uninstall detection, and
  validate the manifest against that artifact.
- **Microsoft Store**, if chosen: check **ElevenClock-rs** in the owner's Partner
  Center account first. Microsoft's [name-reservation instructions](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msi/reserve-your-apps-name)
  require a unique name and recommend avoiding trailing differentiators, so the
  hyphenated name may need checking. Keep the native installer identity until then.
