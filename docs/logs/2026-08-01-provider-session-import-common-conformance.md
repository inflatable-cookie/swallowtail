# 2026-08-01 Provider Session Import Common Conformance

## Result

Card 051 and roadmap g03.019 are complete. `swallowtail-testkit` now exposes a
provider-neutral catalogue/import fixture and one public assertion pack under
both local and remote-authoritative host identities.

## Proven Boundary

The fixture prepares exact catalogue and import plans with bounded scope,
content, provider references, pages, traversal, cursors, deadlines, access,
interface versions, model route, working resource, and session policy.

The conformance pack rejects duplicate traversal entries, copied candidates,
cross-plan cursors, oversized provider material, changed targets, and active
unavailable targets before a binding can escape. Provider title, preview,
reference, and cursor content remain absent from stable diagnostics and debug
surfaces.

A matching revalidation issues one `ExplicitlyImported`
`SessionResumeBinding`. A synthetic interactive driver then proves the binding
enters the existing load path, ordered replay completes before readiness, and
ordinary resume returns readiness without replay. Import itself still creates
no handle, prompt, consumer thread, persistence, provider mutation, or
background synchronization.

## Package Boundary Repair

Affected-package validation exposed a pre-existing testkit packaging defect:
activity corpus tests included fixture files from sibling workspace crates.
The exact 40 referenced evidence files now live in a package-local corpus, and
the tests reference those snapshots. The same validation also repaired stale
route-count expectations for already-recorded Antigravity and Cursor routes.
No production activity projection changed.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime
  swallowtail-testkit` passed 248 tests
- `effigy package:verify-affected swallowtail-core swallowtail-runtime
  swallowtail-testkit` passed
- no broad workspace, live-provider, or consumer suite ran

## Next

Execute card 052. Freeze exact Codex app-server thread-list, read, history,
pagination, status, and resume behavior across every maintained milestone
before adding a production catalogue/import claim.
