# Opt-In Debug Observation Seam

Date: 2026-08-08
Roadmap: g03.055
Cards: 169-171 ready

## Outcome

Promoted the unfinished host diagnostic observer into a durable opt-in debug
observation seam.

Research 113 records the gap: safe diagnostics and bounded excerpts are live,
but `DiagnosticObserver` and `Diagnostic.internal_detail` are almost unused,
so harness drift remains hard to inspect from consumer evidence.

Contract 053 governs structured `DebugObservation` records through the host
observer, fail-soft emission, restricted/redacted detail, and hard
non-interference with safe diagnostics, public events, classification, and
lifecycle. Contract 010 now points at that sink; Contracts 003 and 004 carry
the ownership/redaction pointers. Architecture notes the observer as a host
sink, not a second event stream.

Spec 003 is promoted planning history. Roadmap g03.055 sequences runtime
records (card 169), Codex proof emissions (card 170), and guide/example
wiring (card 171). Card 169 is the active next task. The `v0.3.0` candidate
remains separately operator-authorized.

## Local Validation

Planning and contract surfaces only; no runtime package validation in this
closeout.

## Boundaries

No adapter emission, public API change, consumer-repo commit, tag, or release
in this planning promotion.
