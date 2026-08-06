# 2026-08-05 Runtime Direct Continuation And Realtime Rustdoc Tranche

## Result

The eighth `swallowtail-runtime` public-API tranche now documents and locally
enforces resource-free direct continuation and realtime media session
coordination.

Direct continuation keeps authority explicit: a user turn permits one initial
inference attempt, and one complete correlated tool-result set permits one
further attempt. Tool execution remains consumer-owned. Provider-private
continuation remains route- and session-bound, redacted, non-serializable, and
unusable after invalidation or close.

Realtime media keeps bytes opaque and device, playback, conversion, and privacy
policy downstream. Session state enforces exact format, direction, stream,
sequence, commit, response, and terminal ordering. Planned connection rollover
remains bounded live-operation transport continuity, not resume, retry, or
durable reconstruction.

Module-root `deny(missing_docs)` covers both families. The tranche removes 125
warnings. `swallowtail-runtime` has 203 remaining; the workspace falls from
2,519 to 2,394 without suppression. Six of 27 packages remain closed under the
release documentation gate; runtime is not yet package-complete.

## Validation

- focused runtime validation passed 141 tests and warnings-denied clippy
- extracted runtime package proof passed
- module-root denied-missing-doc Rustdoc passed
- workspace all-feature Rustdoc completed with 2,394 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with the remaining runtime host-input and support records,
then close `swallowtail-runtime` under crate-root denied missing docs.
