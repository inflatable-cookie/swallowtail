# 2026-08-05 Runtime Provider Catalogue And Import Rustdoc Tranche

## Result

The fifth `swallowtail-runtime` public-API tranche now documents and locally
enforces configured provider-instance catalogue admission plus provider-session
catalogue and explicit import.

The configured-instance projection remains consumer-assembled, bounded, and
non-executable. It exposes exact route, interface, capability, access, model,
and conservative readiness evidence without credential material, raw target
references, defaults, routing, or fallback policy. Unavailable instances and
model catalogues remain visible without becoming selectable.

Provider-session catalogue candidates remain bounded observations tied to one
exact plan and traversal. They grant no attachment or lifecycle authority.
Import requires explicit candidate selection, exact catalogue-to-import route
agreement, and read-only revalidation before it issues the ordinary
`SessionResumeBinding`. Import does not load history, resume work, create a
consumer thread, or issue provider-session management authority.

Module-local `deny(missing_docs)` covers both complete module families. The
tranche removes 171 warnings. `swallowtail-runtime` has 568 remaining; the
workspace falls from 2,930 to 2,759 without suppression. Six of 27 packages
remain closed under the release documentation gate; runtime is not yet
package-complete.

## Validation

- focused runtime validation passed 141 tests and warnings-denied clippy
- extracted runtime package proof passed
- module-local denied-missing-doc Rustdoc passed
- workspace all-feature Rustdoc completed with 2,759 remaining warnings

No authenticated provider, consumer, tag, push, GitHub Release, or registry
effect ran.

## Next

Continue card 126 with the remaining runtime operation-policy, interactive,
realtime, direct-continuation, and host-support families.
