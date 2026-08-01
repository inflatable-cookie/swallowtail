# 2026-08-01 Provider Session Catalogue And Import Kernel

## Result

Card 049 is complete. Swallowtail now represents provider-session catalogue
and explicit import as separate provider-neutral operation shapes without
adding a driver or provider mapping.

## Realized Boundary

- independent catalogue/import capabilities, driver roles, operation shapes,
  and cancellation scopes
- working-resource catalogue scope and bounded page, traversal, cursor,
  content, and provider-reference limits
- opaque request-local catalogue and candidate identities
- bounded redacted cursors, provider-session candidates, optional display
  content, update time, activity state, and import availability
- immutable catalogue and import plans
- typed effect-free requests with plan-bound cancellation

Catalogue candidates retain their private provider reference but cannot become
a load or resume request. Import plans require the complete import, load, and
resume capability set, exact future model route, session access and provider
state policy, interface evidence, and matching route, host, access, instance,
resource, and source catalogue.

No runtime driver, import outcome, resume binding, provider protocol, history
replay, consumer persistence, or synchronization was added.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime` passed 163 tests
- `git diff --check` passed
- no broad workspace, package, live, or consumer suite ran

## Next

Execute card 050. Add the object-safe catalogue/import runtime roles, bounded
outcomes, explicit imported-binding origin, and host-service validation.
