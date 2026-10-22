# ADR 0001: fail-closed discovery in an isolated process

Date: 2026-10-14. Amended by [ADR 0002](0002-same-executable-discovery-helper.md),
which moves the isolated process into the app's own executable.

## Context

Windows 11 taskbars may expose the clock through native children, UI Automation,
or a provider subtree whose bounds also include notification and Show Desktop
controls. A taskbar rectangle or an accessibility button alone is insufficient
proof of a clock-only hit target. An in-process worker cannot promise cancellation
of a blocked COM provider call, and nothing shows that `CoCancelCall` or dropping
interfaces gives bounded teardown.

## Decision

**Identify the clock only from locale-independent properties.** Native observations
use bounded direct class lookup as well as `EnumWindows`, with physical rectangles;
UIA observations use AutomationId, ClassName, FrameworkId, ControlType, ProcessId,
IsOffscreen and rectangles. Discovery never reads localized Name properties, invokes
controls, alters Explorer, or derives a clock slot from offsets. The candidate is a
XAML clock button containing `TimeInnerTextBlock` and `DateInnerTextBlock` IDs (see
`taskbar/candidate.rs`).

**Fail closed.** Geometric validation checks generation, nonzero DPI, taskbar/monitor
containment, a complete adjacent-control inventory and strict intersection with
bell/Show Desktop/other controls. Touching edges are permitted. Missing or invalid
rectangles, incomplete inventory, provider errors, stale shell identity, changed
geometry/DPI/topology, truncation, deadline or generation mismatch cannot produce an
accepted clock rectangle; the native clock is then left alone. Whole-sample shell
identity is checked before and after discovery using process creation time as well
as PID, since process IDs alone can be reused. Native inventory is capped at 2,048
children; UIA traversal at 2,048 elements and depth 64.

**Isolate blocking work in a child process.** COM is initialized MTA, owned and
released entirely on the helper's main thread. Only owned value records are
serialized; HWNDs, HMONITORs and COM interfaces stay in the helper. The supervisor
assigns the helper to an unnamed, non-inherited Windows job with kill-on-close and a
one-process limit. A stdin handshake permits native/COM work only after job
assignment; losing the parent beforehand yields EOF. A result deadline includes
discovery, release, serialization and helper exit. After expiry the parent discards
output, terminates the helper and polls for exit for a bounded time. It stops on
failed teardown or failed job assignment, and never starts a replacement while an
earlier helper may remain active. These are polling deadlines, not real-time
guarantees or evidence that COM itself was cancelled.

There is one active generation and at most one pending request. New requests
replace the pending value and invalidate the active generation's result. Only
observed helper exit frees the active slot. No application callback or UI thread
runs discovery.

## Alternatives rejected

Guessed fixed slots, localized labels, Explorer injection, sending UIA interfaces
across threads, abandoning blocked threads and spawning replacements, and
interpreting a timeout as COM cancellation.
