# 009 Codex Prepared Operation Profiles

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../003-codex-prepared-integration-facade.md`

## Objective

Add separate prepared Codex catalogue, interactive-session, and structured-run
paths with named inspectable profiles.

## Governing Refs

- Contract 037 (active)
- Contracts 009, 012-013, 023, 029, and 033-034
- completed card 008

## Scope

1. Add app-server catalogue preparation.
2. Add read-only app-server sessions with explicit ambient configuration,
   provider-enforced isolation, approval never, denied network/search, and
   consumer-supplied tools.
3. Add bounded-workspace sessions as a separate writable opt-in.
4. Add structured Codex exec preparation with explicit reasoning, search,
   schema, attachments, working resource, and deadline.
5. Derive only adapter-owned and immutable plan-echo state.
6. Keep every expanded profile inspectable before effects.

## Acceptance Criteria

- [x] catalogue, read-only, bounded workspace, and exec remain separate
- [x] writable, network, search, and tools remain explicit opt-ins
- [x] runtime requests exactly match prepared plans
- [x] model and reasoning are never selected from catalogue defaults
- [x] legacy and current behavior segments remain adapter-private
- [x] cancellation, deadline, callback, and cleanup behavior is unchanged

## Validation

- prepared catalogue, session, workspace, and exec fixtures
- tool-free and tool-enabled session tests
- reasoning, search, schema, attachment, and policy mismatch tests
- legacy/current/unverified-newer version matrix
- focused Codex suite and warnings-denied clippy
- `git diff --check`

## Evidence Required

- expanded profile snapshots/assertions
- no-effect mismatch proof
- public usage examples
- card 010 readiness assessment

## Stop Conditions

- a named profile hides authority not visible in its expanded plan
- exec and app-server lifecycles converge
- provider defaults choose model, reasoning, network, or tools
- a supported low-level capability becomes inaccessible

## Auto-Continuation

Yes, only after card 010 is rebaselined to ready.

## Closeout

Completed 2026-07-24.

- `CodexPreparedIntegration` now prepares separate catalogue, read-only
  session, bounded-workspace session, and structured-exec values.
- Every prepared value retains exact installed-version and access-provenance
  evidence, an inspectable immutable plan, and the matching runtime request.
- Model route, model, working resource, writable authority, network, search,
  reasoning, schema, attachment, tools, and deadlines remain explicit.
- Provider-retention and harness-configuration behavior remain private
  exact-version dispatch.
- Unsupported exec tools and app-server session deadlines fail during
  preparation. Dynamic session tools are bounded, schema-validated, and cannot
  be redeclared on resume.
- Prepared profiles execute through the existing low-level drivers; those
  entry points remain public.
- The additive pre-1.0 API refreshes the superseded unreleased `0.1.0`
  baseline. It does not add a compatibility shim.
- Card 010 is ready for the full deterministic matrix, public guidance, and
  consumer migration inputs.
