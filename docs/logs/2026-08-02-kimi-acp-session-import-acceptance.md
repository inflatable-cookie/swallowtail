# Kimi ACP Session Import Acceptance

Date: 2026-08-02
Roadmap: g03.021
Card: 057

## Change

Kimi ACP provider-session discovery/import now passes the common contract and
production-driver acceptance under local and remote-authoritative execution
host identities. Cursor pages remain plan-bound across fresh operation
attachments. Existing load replay and resume remain the only continuation
paths after import.

Catalogue and import now observe cancellation and deadline while waiting for
ACP responses. Either signal stops the child, joins the pump, and releases the
working-resource and credential leases. Process loss, malformed projection,
cleanup failure, cancellation, and timeout retain distinct failure stages.

An ACP agent that omits the independently negotiated list capability fails
before `session/list`. Stable protocol support did not add roles to Claude or
Cursor. Their catalogue/import posture remains unavailable pending exact
adapter evidence.

## Evidence

- `effigy validate:focused swallowtail-protocol-acp
  swallowtail-adapter-kimi swallowtail-testkit` passed 272 tests
- `effigy package:verify-affected swallowtail-protocol-acp
  swallowtail-adapter-kimi` passed
- `effigy qa:docs` and `git diff --check` passed
- no authentication mutation, live prompt, consumer edit, or broad suite

## Next

Card 058 freezes OpenCode's exact session catalogue/import operation closure
across every maintained server milestone before capability promotion.
