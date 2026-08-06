# 2026-08-05 Runtime Rustdoc Closure

## Result

`swallowtail-runtime` now enforces denied missing public documentation at its
crate root.

The final tranche covers the execution-host input and support vocabulary:
portable attachments, scoped credentials and network grants, working-resource
callbacks, child-process requests and bounded I/O, installed-executable
discovery, schema transport, model-artifact and serving-endpoint leases,
monotonic deadlines, cancellation and detachment, session replay, and exact
session-policy validators.

The review preserves the existing authority boundaries. Opaque host references
do not become paths or ambient discovery. Leases remain scope-, host-, and
audience-bound. Cancellation and detachment acknowledgements do not become
terminal provider truth. Replay remains observation rather than continuation
authority. Schema transport remains separate from consumer validation.

Crate-root `deny(missing_docs)` removes the final 203 runtime warnings. The
workspace falls from 2,394 to 2,191 without suppression. Seven of 27 packages
are now closed under the release documentation gate. The remaining warnings
belong only to the 20 provider adapters.

## Validation

- focused runtime validation passed 141 tests and warnings-denied clippy
- extracted runtime package proof passed
- crate-root denied-missing-doc Rustdoc passed
- workspace all-feature Rustdoc completed with 2,191 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with package-family review of the hosted-direct adapters,
then move through installed harness adapters without weakening exact route
differences.
