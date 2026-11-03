# Clock templates and sharing

Templates carry the clock format, maximum font size (6–24 pt), and clock theme
(system, light or dark). Display selection, login startup, update-check consent,
onboarding, update history and organization configuration stay local. Templates do
not contain account identifiers or machine information, but format literals can
contain personal text; both the website and Settings explain this before sharing.

## Website editor

The landing page's **Try a format** editor has a token palette and an ordered list
of clock pieces. Click a token to add it, add literal text or a new line, and drag
pieces into order. Selecting a piece exposes **Move earlier**, **Move later** and
**Remove selected**, so dragging is optional and the same work is possible with a
keyboard or on a touch device. Literal percent signs are escaped automatically.
The plain format editor remains available and synchronizes the pieces.

The live browser preview shows the chosen size and theme. Invalid inputs explain
the problem and disable copy/open actions. **Copy format** copies only the format;
**Copy share link** includes appearance too. **Open in ElevenClock-rs** opens the
app for review when a compatible handler is registered. The app still measures fit
against real taskbar slots and DPI before Apply; the browser cannot establish fit.

## Links and import behavior

Version 1 uses these forms (the example is `%H:%M`, 9 pt, system theme):

```text
https://tjorim.github.io/ElevenClock-rs/#template=1&format=%25H%3A%25M&font=9&theme=system
elevenclock://template?v=1&format=%25H%3A%25M&font=9&theme=system
```

The website fragment is interpreted locally by JavaScript; its template contents
are not part of the HTTP request and there is no upload service or template fetch.
Sharing from the app copies the website form to the clipboard. It sends nothing
until the user shares that link through their chosen service. Opening **Website
editor** is an explicit browser action and puts the current draft in the fragment.

The core parser accepts at most 4,096 UTF-8 bytes, exact supported prefixes, one
supported version, and exactly the format/font/theme fields. Duplicates, unknown
fields, malformed percent/UTF-8 encoding, invalid formats, controls and out-of-range
appearance are rejected. Decoding occurs once. Links cannot select an executable,
file, download, startup action or private helper role. The app's `--template` action
accepts exactly one link argument; extra arguments cannot activate a private role.
Windows may normalize the app URL to `elevenclock://template/?…`; the parser accepts
that single optional trailing slash too, without decoding the payload twice.

A link received while the app is running is sent to its existing per-user/session
host using synchronous, system-marshalled `WM_COPYDATA` with a one-second send
timeout. The callback bounds and validates the bytes and retains only an owned
template. A one-slot mailbox keeps the newest pending request, and the ordinary
message pump opens/focuses Settings. No pointer survives the callback, no worker
owns a native handle, and no new timer or polling loop is introduced.

The **Templates** page displays the received link without replacing existing edits.
**Review template** explicitly stages format, font size and theme, then shows the
Date and time page. The clock changes only after the normal validation, fit check,
save and **Apply** path. Cancel discards the imported draft. Unsupported saved
preference versions continue to block Apply, including template imports.

## Windows link registration

The per-user installer registers `elevenclock:` under
`HKCU\Software\Classes\elevenclock`, with a quoted app path and quoted `%1` passed
after `--template`. It preserves an existing different per-user handler. Uninstall
removes the registration only if its command still exactly belongs to this app.
No HKLM key, elevation, browser setting, or Windows default-app choice is changed.
Registration changes notify the shell through
[`SHChangeNotify`](https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shchangenotify).

Portable and managed users can select **Enable template links** on the Templates
page. The same button disables this copy's registration. It acts immediately,
independently of Apply/Cancel and login startup, and refuses to replace another
copy/program's registration. Moving a portable executable requires disabling its
old registration before moving it, then enabling the new copy. The association
points to the local app, including in managed mode; opening a template does not
refresh the organization deployment. Managed uninstall removes a template handler
only when its command still belongs to that local app.

## Named local presets

The Templates page can save the current draft under a name, load a saved template
for review, or delete a saved preset. Saving a preset does not apply it to the
clock. Names contain 1–48 characters and are matched by their lowercased names;
saving an existing name replaces that preset, as the label explains. At most 50
presets are stored. Built-in format presets remain on Date and time.

The separate `%LOCALAPPDATA%\ElevenClock-rs\templates.json` library has a version-1
envelope containing names and validated share links. Active preferences keep their
existing schema. Reads are capped at 256 KiB; invalid/newer libraries are preserved
and preset writes are disabled rather than replacing them with an empty library.
Sharing and reviewing links remain available. Saves flush a create-new staging file
and atomically replace the library; failure preserves the in-memory library and
previous file. A killed process can leave an ignored staging file. The existing
application instance serializes normal writes; concurrent Windows sessions use
separate staging files and the last successful replacement wins. Local
presets survive app updates and removal along with other per-user data.
The retained synthetic `tests/fixtures/templates/v1-custom.json` fixture records the
version-1 persisted contract independently of the writer.

## Verification

Core tests cover link round trips, Unicode/literal text, invalid encodings, version
and field boundaries. Website tests use the same canonical link example. Portable
Settings tests verify that template staging preserves local-only settings and
changes neither active values nor storage before Apply. Library tests cover
replacement, reopening, deletion, unknown-version preservation and failed commits.
Native registry tests use throwaway HKCU keys to check exact ownership and cleanup.
The Settings harness checks controls, keyboard access and the native template flow.
