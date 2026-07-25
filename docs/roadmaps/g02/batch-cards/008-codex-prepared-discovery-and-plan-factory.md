# 008 Codex Prepared Discovery And Plan Factory

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../003-codex-prepared-integration-facade.md`

## Objective

Create the Codex prepared object that binds one target, exact version,
compatibility assessment, access provenance, and consistent preflight inputs.

## Governing Refs

- Contract 037 (active)
- Contracts 008-010, 023, 029, 032-033
- completed roadmap g02.002

## Scope

1. Add adapter-local setup inputs for stable instance identity, host, target,
   environment, access evidence, and selected Codex driver.
2. Run bounded installed discovery and retain the exact qualified, deprecated,
   or unverified-newer assessment.
3. Build consistent facade, ownership, configuration posture, access,
   instance, requirements, and route templates.
4. Expose safe preparation evidence and the expanded plan.
5. Bind one approved target for the prepared object's lifetime.
6. Preserve direct low-level drivers.

## Acceptance Criteria

- [x] one exact target reaches discovery and later execution
- [x] exact version and behavior assessment remain visible
- [x] access evidence or assertion provenance remains visible
- [x] adapter-fixed fields cannot be omitted or duplicated inconsistently
- [x] no model, workspace, network, search, or tool choice is implicit
- [x] low-level drivers remain independently callable

## Validation

- qualified, deprecated, excluded, malformed, and unverified-newer fixtures
- target and host drift rejection
- staged preparation failure and cleanup matrix
- local and remote-authoritative topology tests
- focused Codex and public API checks
- `git diff --check`

## Evidence Required

- prepared-object API example
- exact promotion and assessment assertions
- redaction audit
- card 009 readiness assessment

## Stop Conditions

- target identity can drift after observation
- access readiness must be fabricated
- setup merges exec and app-server behavior
- provider-specific records would enter core/runtime

## Auto-Continuation

Yes, only after card 009 is rebaselined to ready from the completed factory.

## Closeout

Completed 2026-07-24.

- `CodexPreparationInput` selects one driver, stable instance identity,
  authoritative host, opaque target, environment, access profile, and honest
  access evidence.
- `CodexPreparationProbe` carries only bounded lifecycle controls. The factory
  derives the discovery host and target, so callers cannot duplicate them.
- `CodexPreparedIntegration` retains the exact observation, compatibility
  behavior, access provenance, configured instance, environment, and target.
- The configured-instance target is derived from the same opaque executable
  reference sent to `--version`; later host or target substitution fails.
- Structured exec and app-server retain separate facade, ownership,
  configuration, capability, claim, and low-level driver identities.
- Maintained, deprecated, excluded, malformed, unverified-newer, host/access
  drift, spawn, output, exit, cleanup, topology, and redaction fixtures pass.
- A compile-checked prepared-discovery example records the public setup shape.

The factory intentionally selects no operation profile, model route, working
resource, network, search, tools, schema, attachments, reasoning, or deadline
beyond its own bounded version probe. Card 009 owns the separate expanded
catalogue, session, workspace, and exec plans built from these bound facts.

Public API classification: additive pre-1.0 adapter API plus more precise safe
Codex discovery diagnostics. Low-level drivers and signatures are unchanged.
