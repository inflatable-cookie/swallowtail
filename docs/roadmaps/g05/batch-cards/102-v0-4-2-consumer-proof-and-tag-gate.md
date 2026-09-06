# 102 v0.4.2 Consumer Proof And Tag Gate

Status: complete; `v0.4.2` tagged at `f94dd16f2e4db79c5b7c4440cc1eb2d20f8b6af8`; consumer smoke inconclusive pre-window; Desktop lane owns the visibility defect
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-06
Milestone: `../032-v0-4-2-release-readiness.md`
Depends on: the preceding g05.032 card

## Goal

Run `effigy package:source-consumer` on the released `v0.4.2` tag and one Bovine Desktop editing session on `claude-agent.sdk` against it (the exact acceptance this defect class needs). The Bovine packet (checkout, command, retry budget) comes from the Acowtancy Chatterbox.

## Readiness

Chatterbox makes this card ready with its manifest when the preceding card
merges and the operator-authorized release tag exists.

## Standing Smoke Evidence

The v0.4.1 working-application smoke remains the standing accepted smoke for
the consumer launch path. The post-tag Bovine smoke is the next consumer
evidence to attach here; it must pin the released `v0.4.2` tag and is not
represented by the earlier untagged candidate attempt.

## Result

Two bounded Bovine smoke attempts are recorded as consumer evidence. The first
used the untagged candidate revision and the second used the released
`v0.4.2` tag; both failed before the isolated smoke window became visible.
Build and Vite succeeded, hidden-window restore converged, but neither attempt
produced a `bovine_window_page_ready` receipt, Chat surface, session open,
`open_rejected`, sidecar record, or turn. No Claude prompt was sent.

The tagged attempt used six direct crates and eight lock sources resolved to
`v0.4.2#f94dd16f2e4db79c5b7c4440cc1eb2d20f8b6af8`, with tag object
`927d14eccc16b5f24fc427913e087cd47fcfa499`. Its binary built successfully,
but the isolated window remained inaccessible. Cleanup left the port free and
no smoke processes; the primary Desktop window was not touched. Full evidence
is in the Acowtancy log
`docs/logs/2026-09/06-161100-desktop-swallowtail-v042-tagged-smoke.md`.

Disposition: consumer smoke inconclusive for the route, with no Swallowtail
defect indicated. The accepted working-application smoke remains the v0.4.1
smoke. The Desktop window-visibility issue belongs to the Acowtancy lane; this
card is closed and no v0.4.3 lane opens from this result.
