# 052 v0.4.0 Consumer Proof And Operator Tag Gate

Status: planned; gated on exact candidate and operator application selection
Owner: Tom
Created: 2026-09-02
Milestone: `../021-v0-4-0-release-readiness.md`
Depends on: completed card 051; explicit operator-selected working application

## Goal

Prove the frozen candidate through the current external source consumer and one
normal working application, compile final release evidence, and stop for the
operator's exact tag decision.

## Operator Decision Required Before Ready

The operator must name:

- the working application and repository/checkout identity
- the normal Swallowtail route and smoke/rebuild command
- the exact way it consumes the candidate SHA
- whether credentials or a provider call are authorized; default is neither
- any allowed temporary lock/build mutation and its cleanup; default is no
  consumer-repo mutation
- expected success, redaction, retained evidence, and stop conditions

Do not infer Nucleus, Soundcheck, Monkey, or another application from precedent.
The source consumer is not a substitute for this decision.

## Scope

1. Reconfirm the candidate SHA equals canonical `origin/main`, remains clean,
   retains green exact-SHA CI, and has no later commit or tag.
2. Rerun `effigy package:source-consumer` on the clean exact candidate. Record
   the external temporary Cargo consumer's exact revision resolution and normal
   prepared Codex path.
3. Run only the operator-selected working-application rebuild/smoke against that
   exact SHA. Use the application's normal public Swallowtail integration and
   preserve its product authority. Do not broaden provider, credential, write,
   persistence, or consumer-mutation authority.
4. Record command, source identity, selected packages/route, target, outcome,
   safe diagnostics, cleanup, and any authorized provider evidence without
   secrets, prompts, payloads, account data, or host paths.
5. Compile final release evidence: exact candidate SHA; 40-package `v0.4.0`
   baseline; 48 routes versus `v0.3.3`'s 47; Rust `1.95.0`; Apple Silicon
   macOS; compatibility ledger; changelog/release notes; all 11 local gates;
   exact-SHA CI; clean source; external source consumer; application smoke;
   upgrade/rollback; known limits; freeze state; and tag absence.
6. Stop and request explicit operator authority for the exact source commit,
   canonical `main` and `origin`, tag name `v0.4.0`, exact annotated tag
   message, local annotated-tag creation, and tag push. Confirm the requested
   release contains no crates.io publication or GitHub Release object and no
   binaries, sidecars, or installers.

## Out Of Scope

Tag creation or push, release execution, registry or GitHub Release mutation,
binary/sidecar/installer publication, CI workflow change, candidate edit,
feature/currentness implementation, unselected provider work, or unapproved
consumer-repo mutation.

## Acceptance Criteria

- source consumer resolves every selected Swallowtail dependency at the exact
  candidate SHA and exercises the normal prepared path
- the operator-selected application rebuild/smoke passes against the same SHA
  with its authority and cleanup recorded
- final evidence contains no result from another source tree and no secret,
  private payload, or host path
- release boundary and known limits match the milestone and release notes
- local and remote `v0.4.0` tags remain absent
- the card ends with the exact candidate SHA and an explicit operator authority
  request; it does not imply or execute that authority

## Validation

- `effigy package:source-consumer`
- operator-selected application rebuild/smoke command
- `git status --porcelain`
- local/remote candidate identity, exact-SHA CI, and tag-absence checks
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g05`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:roadmaps:status`
- `effigy qa:docs:next-action:roadmaps`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: both consumers use the exact reviewed candidate while the application
keeps its own product and credential authority.

Smallest counterexample: the temporary consumer resolves a path checkout, the
application builds against another tag/SHA, a smoke mutates its repository
without approval, or final evidence contains a credential or provider payload.

Required proof: Cargo metadata exact-revision assertion, operator selection,
application dependency/source evidence, command and outcome, clean cleanup,
redacted final packet, candidate/CI identity equality, and local/remote tag
absence.

## Auto-Continuation

No. Stop for explicit operator authority. No roadmap card authorizes tag
creation or push.

## Stop Conditions

Stop if the application is unnamed, authority is incomplete, either consumer
uses another source identity, a credential/provider call or repository mutation
is not explicitly authorized, evidence cannot be redacted safely, the candidate
changes, CI is no longer exact and green, or any release mutation is requested
without the final exact operator authorization.
