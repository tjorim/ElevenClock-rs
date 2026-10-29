# ADR 0003: optional update checks against GitHub Releases

Date: 2026-10-29. Decision for [#91](https://github.com/tjorim/ElevenClock-rs/issues/91),
part of #88. Implementation follows in #90 (release link), #92 (manual checks) and
#93 (automatic checks). This record authorizes that work; it does not enable it.

## Context

Self-managed users of the installer or portable ZIP cannot currently tell from the
app that a newer release exists. Managed users receive the version deployed by their
organisation through the [managed launcher](0004-managed-launcher.md), at launch and
login. They must not be offered a competing update source.

A check performed by the app introduces network traffic. The
[optional-feature policy](../dependencies.md#optional-features) requires an explicit
privacy, dependency and lifecycle decision before implementation. The clock must
remain fully usable offline, without an account.

## Decision

Ship one app executable for the installer, portable ZIP and managed ZIP. Update-check
code is always built in; there is no separate build or feature flag.

| Level | Self-managed behavior | Network performed by the app | Default |
| --- | --- | --- | --- |
| 0 | Version and releases link in Settings (#90) | None; a click opens the browser | Always shown |
| 1 | Check now in Settings (#92) | One request per click, subject to an in-flight check or rate-limit backoff | Always available |
| 2 | Automatic checks and one tray notification per new version (#93) | At most once per 24 hours | Off; explicit Settings opt-in |

The level 2 checkbox is the consent mechanism. It starts unchecked, with no first-run
prompt. Manual checks require no automatic-check consent. Both paths share one
request coordinator, so they never overlap or create a queue of requests. A manual
check also advances the automatic check's last-attempt time.

Managed mode is determined by the sibling launcher under ADR 0004. In that mode no
update UI (including the release link), scheduling, worker or request is reachable,
even if saved preferences enable automatic checks. The code remains in the binary.
Only the launcher obtains updates from the company's central location; this ADR adds
no GitHub access to the launcher or the discovery-helper role.

### Dependencies and request boundary

Use Windows WinHTTP through generated entries in `clock-windows/bindings.txt`, adding
`winhttp.dll` to the release DLL allowlist in `scripts/rust-release.py` with #92.
WinHTTP is a maintained Windows OS component, not a bundled library. No new crate,
async runtime, downloader or updater is introduced. Reuse serde_json for decoding.
There is no new third-party license or bundled notice; review the normal dependency
inventory and release imports to confirm that remains true. Binding generation and
FFI ownership follow the contributor guide.

Each check makes one unauthenticated HTTPS GET to
`https://api.github.com/repos/tjorim/ElevenClock-rs/releases/latest`, with a fixed
`User-Agent: ElevenClock-rs/<version>` using the app version. GitHub receives the
originating IP address, app version and normal request metadata. Send no identifier,
account credentials, cookies, telemetry, query parameters or preference data. Disable
cookies, automatic authentication and redirects; retain normal TLS certificate
validation. Do not probe another endpoint or fall back to HTTP.

GitHub controls processing and retention of its request records. Its
[privacy statement](https://docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement#security-and-retention)
describes purpose-based retention, rather than a fixed retention period for these
requests. We cannot control or promise deletion of GitHub's records. Opening a release
page uses the user's browser and GitHub's website privacy rules.

Decode a typed projection containing only required `tag_name`, `html_url`, `prerelease`
and `draft` fields, with strict field types. Ignore unrelated GitHub API fields (the
API's normal response contains many); do not retain them. Cap received bytes at 64 KiB,
including streamed responses without a Content-Length. Invalid JSON, missing fields,
wrong types, oversized data and invalid versions or URLs produce `Failed`, never an
update. Accept only an HTTPS release-page URL on `github.com` under
`/tjorim/ElevenClock-rs/releases/tag/`, without credentials, query or fragment, before
passing it to the browser.

### Versions and results

Strip one leading `v` from `tag_name` and compare Semantic Versioning major, minor and
patch numbers against `CARGO_PKG_VERSION`; compare numbers numerically, with checked
parsing, and ignore build metadata for precedence. Do not add a crate for this bounded
comparison. Ignore drafts and prereleases, including a prerelease suffix in the tag.
A valid stable release that is not newer yields `UpToDate`; a newer one yields
`Available { version, url }`. Malformed data yields `Failed(kind)` and Settings says
“Couldn't check”. A valid excluded draft/prerelease produces no update offer.

A result only offers the release page. There is no download, installation, executable
replacement or automatic browser launch. Explain there how to update: installer users
run the new setup; portable users exit the app and replace the exe. Managed users
follow their organisation's deployment process.

### Worker ownership, cancellation and offline behavior

The runtime owns a single bounded worker and result channel. Window callbacks only
queue owned actions; no request, DNS lookup or blocking wait runs in a callback or on
the login path. The worker owns all WinHTTP handles with explicit cleanup. Only a
plain result enum crosses back: `UpToDate`, `Available { version, url }` or
`Failed(kind)`. No native handles, COM pointers, response bodies or raw errors cross
the channel. Keep [ADR 0001](0001-taskbar-discovery.md) and
[ADR 0002](0002-same-executable-discovery-helper.md)'s callback and ownership rules.

Use asynchronous cancellable WinHTTP operations on the worker, connect and receive
timeouts, and a 10-second overall deadline covering resolution, connection, headers
and body. Per-operation timeouts alone must not permit an indefinitely trickling
response. Exit cancels pending work, closes request/session resources and discards
late results; shutdown never joins a blocked network operation. The implementation
must demonstrate cancellation and safe callback lifetime before shipping.

Automatic checks run only after startup has completed and when due. Persist the
attempt time before dispatch, so restarting or failing cannot cause repeated startup
traffic. Clock rollback must defer rather than loop; a future timestamp delays the
next automatic attempt. Lock/pause or Explorer transitions must not create extra
checks or revive cancelled work. Network failure never affects clock rendering.
Automatic failures are quiet and eligible again the next day; manual failure shows
“Couldn't check”. There are no immediate retries or connectivity probes.

Honor GitHub's [rate-limit rules](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api):
unauthenticated requests share 60 requests/hour per originating IP. On a rate-limit
response, defer both manual and automatic dispatch until the later applicable
Retry-After or exhausted-limit reset deadline; without a usable deadline defer at
least 24 hours. Show a local failure/backoff status for manual clicks during that
interval. Automatic dispatch still respects its daily limit. Do not authenticate to
raise the quota, retry in a loop or let repeated clicks bypass backoff.

Turning automatic checks off removes its scheduled wakeup, cancels an automatic
request and suppresses its late results and notifications. Release worker resources
when idle; manual Check now remains available. Exit cancels either kind of request.
Managed mode cancels all update work and suppresses all results. Beyond the private
preferences there is no response cache, downloaded file, service, hook or Run value.

### Preferences and notifications

Level 2 requires a version 2 preference encoding under the
[compatibility policy](../design/preferences.md#compatibility-and-retained-fixtures).
Read version 1 explicitly, retaining all existing fields and adding automatic checks
`false`, no last-attempt timestamp and no last-notified version. Preserve the original
bytes before committing version 2; publish migrated active state only after a
successful save. Preservation or commit failure leaves the original and active state
intact and prevents automatic dispatch. Keep unsupported-version write protection.

Retain unchanged version 1 fixtures and add version 2 expected fixtures and tests for
upgrade, interrupted/failed save, reopening and downgrade. An older executable sees
version 2 as unsupported, uses safe defaults and refuses to overwrite it; returning
to the newer executable retains the data.

Store the opt-in flag, last attempt time and last version notified. Persist the
notification marker before emitting one tray notification for that newer version;
a failed save suppresses the notification. Repeated checks or restarts must not
repeat it. Manual results remain visible in Settings. Serialize preference writes
with normal Settings commits so a worker completion cannot overwrite a newer draft
or opt-in choice. Persist any rate-limit deadline needed across restarts in the
version 2 contract; disabling checks leaves only this bounded preference state.

### Diagnostics and verification

Report managed yes/no, update-check mode (manual only or automatic; disabled in
managed mode), and last result kind, including not checked. Never report URLs, IPs,
headers, response bodies or raw network errors in diagnostics or logs. Extend the
golden text and redaction cases in `support/tests.rs` with the implementation.

Portable policy tests must prove zero requests, scheduling and notifications in
managed mode regardless of preferences; default-off behavior; manual-only dispatch;
daily throttling across restart and clock changes; backoff; notification deduplication;
malformed/oversized responses; numeric version ordering; URL validation; and late-result
suppression after disable or Exit. Windows tests must cover deadlines, cancellation,
handle cleanup and shutdown without waiting for the network. New controls need keyboard
and screen-reader checks, and background work needs the contributor guide's measured
resource evidence. Tests use scripted responses, not live GitHub rate-limit traffic.

## Documentation changes with implementation

As with ADR 0004, current-behavior docs change in the PR that makes each statement true:

* #90 documents the version and browser release link for self-managed users.
* #92 updates `docs/design/diagnostics.md`, the privacy policy in `README.md` and the
  shared site footer (`site/src/layouts/SiteLayout.astro`): the app makes no network
  requests except the update check described here, only on Check now or after the user
  enables automatic checks, and never in managed mode. Clarify that Copy diagnostics
  remains local and that there is still no telemetry, upload or crash reporter.
* #93 updates `docs/migration.md` to describe optional update notifications instead of
  listing them as unsupported. In-app installation remains unsupported. Update
  `docs/design/preferences.md`, Settings and lifecycle docs for the implemented
  version 2 contract, consent, scheduling and cancellation.

## Rejected alternatives

* Silent startup polling, default-on automatic checks or a first-run consent prompt:
  manual checks and an unchecked Settings option provide explicit user control.
* A separate managed app build: one executable is simpler; runtime policy must prove
  the network path unreachable in managed mode.
* A new HTTP/semver crate or async runtime: WinHTTP and a bounded version parser meet
  this narrow need without expanding the shipped dependency graph.
* Downloading or installing updates: this decision only informs users and opens the
  release page. Managed deployment remains the launcher's responsibility.
