# v0.4.1 Live-Open Defect And v0.4.2 Lane

Date: 2026-09-05
Roadmap: `../roadmaps/g05/032-v0-4-2-release-readiness.md`
Source: Acowtancy Chatterbox report on bovine-accelerator-desktop PR 97 (`17321cc9`)

## What happened

The first application to open `claude-agent.sdk` for real, on `v0.4.1`,
got `swallowtail.claude-agent.sdk.open_rejected` with no further detail.
The `v0.4.1` open path maps every sidecar rejection to that one code and
discards the sidecar's own failure code, which the wire already decodes.
The sidecar also requires the SDK init's `system.model` to equal the
requested string exactly, and readiness pins Node to exact `22.23.2`;
neither check had ever met a real SDK init, because every proof used a fake
SDK that echoes the request and no live smoke ran before the tag.

## Lesson

This is the cost of dropping the consumer smoke for `v0.4.1`. The
compression was the operator's call under a deadline and the trade was
stated plainly at the time; it is now paid as a patch. `v0.4.2` keeps every
other compression but restores one live consumer open as the acceptance for
this route.

## Lane

Card 100 repairs diagnostics, effective-model evidence, and Node
newer-allowed at open, with one operator-authorized live open. Card 082 is
paused behind it (shared sidecar). Cards 101-102 prepare and prove the
candidate with a Bovine editing session, then stop for the tag decision.
