# 2026-08-01 Provider Session Catalogue And Import Runtime

## Result

Card 050 is complete. Catalogue and import now have independent object-safe
executor-neutral runtime roles, bounded outcomes, prepared evidence, and exact
host-service validation.

## Runtime Boundary

Catalogue outcomes retain provider order while enforcing page and traversal
bounds. Their opaque cursors carry request-local seen identities, so duplicate
candidates fail across pages rather than being silently deduplicated.

Import execution must return exact revalidation evidence for the selected
candidate, provider session, working resource, activity, and availability.
Only the matching success constructor issues one ordinary
`SessionResumeBinding`, marked `ExplicitlyImported`. It returns no handle and
does not load or replay history.

Failure stages distinguish before-dispatch, catalogue dispatch/projection,
import revalidation/binding issue, cancellation, timeout, and cleanup. A
degraded or failed cleanup cannot accompany a successful page or imported
binding.

Prepared evidence binds access provenance to the immutable plan. Runtime
execution validation checks exact host identity and every planned host service
before driver effects.

No provider adapter, wire protocol, history replay, consumer persistence, or
background synchronization changed.

## Validation

- `effigy validate:focused swallowtail-runtime` passed 107 tests
- `git diff --check` passed
- no broad workspace, package, live, or consumer suite ran

## Next

Execute card 051. Add provider-neutral local and remote-authoritative
conformance plus import-to-load/resume boundary proof.
