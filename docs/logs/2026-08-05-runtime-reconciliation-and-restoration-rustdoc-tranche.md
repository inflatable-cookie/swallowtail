# 2026-08-05 Runtime Reconciliation And Restoration Rustdoc Tranche

## Result

The third `swallowtail-runtime` public-API tranche now documents and locally
enforces provider-session reconciliation, provider-run reconciliation, durable
operation checkpoints, working-state restoration, and settled observe-then-
attach sequencing.

The review preserves the recovery authority split. Checkpoints remain opaque,
integrity-checked records bound to one exact route or session attachment.
Reconciliation observes retained provider truth without prompt, callback,
cancellation, import, or management authority. Working-state restoration
executes one route-selected method and never falls through after failure.
Settled restoration completes read-only observation before considering its
separately prepared attachment and retains reconciliation evidence if that
attachment fails.

Module-local `deny(missing_docs)` covers both reconciliation modules, both
checkpoint modules, working-state restoration, and settled-session
restoration. The tranche removes 193 warnings. `swallowtail-runtime` has 855
remaining; the workspace falls from 3,239 to 3,046 without suppression. Six
of 27 packages remain closed under the release documentation gate; runtime is
not yet package-complete.

## Validation

- focused runtime validation passed 141 tests and warnings-denied clippy
- extracted runtime package proof passed
- module-local denied-missing-doc Rustdoc passed
- workspace all-feature Rustdoc completed with 3,046 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with runtime session bindings, provider-session management,
and recovered-resource cleanup before the remaining operation-policy and
provider-import families.
