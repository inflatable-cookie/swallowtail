# 2026-08-05 Runtime Observation And Callback Rustdoc Tranche

## Result

The second `swallowtail-runtime` public-API tranche now documents and locally
enforces observable activity, provider metadata, ordered events, terminal
outcomes, and callback exchange.

The review preserves the runtime boundaries encoded by the types. Activity
identity remains operation-local and separate from provider references.
Provider observations remain metadata, not fabricated agent activity.
Semantic events cannot be discarded as progress snapshots. Terminal status,
cleanup, cancellation, and remote deletion retain independent truth.
Callback requests carry exact operation and provider-request correlation;
responses remain exactly once and consumer-chosen.

Module-local `deny(missing_docs)` covers `activity`, `provider_observation`,
`event`, `outcome`, and `callback`, including their public child modules. The
tranche removes 359 warnings. `swallowtail-runtime` has 1,048 remaining; the
workspace falls from 3,598 to 3,239 without suppression. Six of 27 packages
remain closed under the release documentation gate; runtime is not yet
package-complete.

## Validation

- focused runtime validation passed 141 tests and warnings-denied clippy
- extracted runtime package proof passed
- module-local denied-missing-doc Rustdoc passed
- workspace all-feature Rustdoc completed with 3,239 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with runtime provider-session, reconciliation, checkpoint,
cleanup, and working-state restoration records.
