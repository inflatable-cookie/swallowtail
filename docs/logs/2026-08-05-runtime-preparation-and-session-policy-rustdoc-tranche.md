# 2026-08-05 Runtime Preparation And Session Policy Rustdoc Tranche

## Result

The sixth `swallowtail-runtime` public-API tranche now documents and locally
enforces explicit operation policy, interactive session options, negotiated
session evidence, immutable session-plan agreement, preparation failures,
prepared access provenance, and prepared operation evidence.

The review preserves the no-default authority boundary. Operation policy keeps
provider network, search, reasoning, harness mode, retention, recovery,
reattachment, isolation, configuration, and runtime residency explicit.
Session options remain consumer-owned typed input. Negotiated provider option
values remain bounded session evidence and can only confirm an exact portable
selection already present in preflight.

Prepared access evidence retains observed versus caller-asserted provenance
without changing the supplied access status. Prepared operation evidence owns
an immutable expanded plan and safe compatibility evidence, but no request,
driver handle, credential, default, router, or fallback authority. Low-level
session agreements must exactly repeat preflight state.

Module-local `deny(missing_docs)` covers all seven modules and their private
children. The tranche removes 125 warnings. `swallowtail-runtime` has 443
remaining; the workspace falls from 2,759 to 2,634 without suppression. Six
of 27 packages remain closed under the release documentation gate; runtime is
not yet package-complete.

## Validation

- focused runtime validation passed 141 tests and warnings-denied clippy
- extracted runtime package proof passed
- module-local denied-missing-doc Rustdoc passed
- workspace all-feature Rustdoc completed with 2,634 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with runtime interactive support: harness user input,
subagent directory, harness RPC, event buffering, and the remaining session
request records.
