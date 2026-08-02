# Kimi ACP Session Catalogue And Import

Date: 2026-08-02
Roadmap: g03.021
Card: 056

## Change

Kimi Code ACP now exposes a prepared, resource-scoped session catalogue and
explicit import route. Catalogue initialization advertises read-only
filesystem capability. It negotiates `session/list`, applies common ACP bounds,
and projects only title, update time, opaque provider reference, and import
availability.

Import repeats the catalogue observation before issuing a binding. Exact host,
configured instance, executable version, access, Kimi state root, working
resource, model route, and session policy remain bound through preparation and
runtime validation. A changed or missing candidate issues no binding. A valid
binding enters the existing Kimi `session/load` replay and `session/resume`
path; Swallowtail does not create a second continuity mechanism.

## Compatibility

The deterministic corpus names exact Kimi Code `0.28.1`, `0.29.0`, `0.29.1`,
`0.29.2`, `0.30.0`, `0.31.0`, and `0.31.1`. All seven use the stable ACP v1
list/load/resume shapes selected by the route. Versions above `0.31.1` remain
visible as unverified newer but cannot inherit session-import support.

## Evidence

- `effigy validate:focused swallowtail-adapter-kimi swallowtail-protocol-acp`
  passed 186 tests
- `effigy package:verify-affected swallowtail-adapter-kimi
  swallowtail-protocol-acp` passed
- `git diff --check` passed
- no authentication mutation, live prompt, workspace write, or broad suite

## Next

Card 057 owns cross-host conformance, lifecycle failure boundaries, Claude and
Cursor route classification, public guidance, and package acceptance.
