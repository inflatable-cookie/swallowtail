# 2026-08-05 Runtime Interactive Coordination Rustdoc Tranche

## Result

The seventh `swallowtail-runtime` public-API tranche now documents and locally
enforces typed harness user input, transient subagent directory projection,
harness RPC scheduling and display observations, ordered event buffering, and
bounded runtime event delivery.

Harness questions and answers remain bounded, redacted, stable-id records with
exact shape and membership validation. They transport correlated consumer
input but grant no permission, tool-execution, or product authority. Harness
RPC acknowledgement remains transport acceptance rather than model lifecycle
or persistence evidence. Display observations require no response.

The subagent directory remains an operation-local transient reducer. It keeps
provider-supplied parentage and actor attribution honest, creates only
identity-only unknown placeholders where permitted, applies changes
transactionally, and grants no provider child-control authority. Event
delivery preserves start and monotonic ordering, activity lifecycle, and every
semantic event. Only explicitly coalescible events may replace earlier
coalescible entries under pressure; late events remain quarantined.

Module-local `deny(missing_docs)` covers all five modules. The tranche removes
115 warnings. `swallowtail-runtime` has 328 remaining; the workspace falls
from 2,634 to 2,519 without suppression. Six of 27 packages remain closed
under the release documentation gate; runtime is not yet package-complete.

## Validation

- focused runtime validation passed 141 tests and warnings-denied clippy
- extracted runtime package proof passed
- module-local denied-missing-doc Rustdoc passed
- workspace all-feature Rustdoc completed with 2,519 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with direct continuation and realtime media, then close the
remaining runtime host-input and support records under crate-root denial.
