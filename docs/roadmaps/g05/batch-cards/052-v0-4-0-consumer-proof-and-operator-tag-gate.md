# 052 v0.4.0 Consumer Proof And Operator Tag Gate

Status: planned; exact candidate and canonical exact-SHA CI accepted; blocked only on the complete operator-authenticated application authority packet
Owner: Tom
Created: 2026-09-02
Milestone: `../021-v0-4-0-release-readiness.md`
Depends on: completed card 051 at candidate merge `56f3913a`; explicit operator-authorized authenticated working application

## Goal

Prove the frozen candidate through the current external source consumer and one
current normal authenticated working application, compile final release
evidence, and stop for the operator's exact tag decision.

## Operator Decision Required Before Ready

The operator must supply one complete Contract 036 application-smoke authority
packet naming:

- the working application and exact repository/checkout identity
- the normal Swallowtail route and smoke/rebuild command
- the exact dependency/source shape that consumes the candidate SHA, plus any
  later tag-resolution check requested after separately authorized tag creation
- the credential/account and provider-call authority required for the current
  authenticated normal product path
- every permitted temporary lock, build, cache, or repository mutation
- expected success, retained evidence, redaction rules, cleanup, stop
  conditions, and the exact retry budget

Do not infer Nucleus, Soundcheck, Monkey, or another application from precedent.
No credential-free rebuild, provider-free smoke, deterministic fixture, or
source-consumer result substitutes for the authenticated application path.
Keep this card planned and blocked until the complete packet is supplied.

## Scope

1. Reconfirm immutable candidate SHA
   `56f3913ac99af44b6ff45384cfc53a0adea587ba` is the accepted canonical
   Card 051 merge, remains in canonical `main` history, retains green exact-SHA
   workflow-dispatch CI run 33853812785, and has no tag. Later accepted
   planning-only closeout commits do not replace the candidate identity.
2. Rerun `effigy package:source-consumer` on the clean exact candidate. Record
   the external temporary Cargo consumer's exact revision resolution and normal
   prepared Codex path.
3. Run only the operator-authorized current authenticated working-application
   rebuild/smoke against that exact SHA. Use the application's normal public
   Swallowtail integration and exact named route. A provider-free or
   source-consumer substitute cannot complete this step. Do not broaden
   provider, credential, account, retry, write, persistence, or
   consumer-mutation authority.
4. Record command, source identity, selected packages/route, target, outcome,
   authenticated-path identity, attempt count, safe diagnostics, cleanup, and
   authorized provider evidence without secrets, prompts, private payloads,
   account data, or host paths.
5. Compile final release evidence: exact candidate SHA; 40-package `v0.4.0`
   baseline; the candidate route inventory versus `v0.3.3`'s 47, counting only
   routes the audit accepts; Rust `1.95.0`; Apple Silicon
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
- the operator-selected application passes through its current normal
  authenticated product path against the same SHA, within the authorized retry
  budget, with credential/provider authority, evidence, redaction, mutations,
  and cleanup recorded
- no provider-free rebuild, source-consumer result, or deterministic fixture is
  counted as the required application proof
- final evidence contains no result from another source tree and no secret,
  private payload, or host path
- release boundary and known limits match the milestone and release notes
- local and remote `v0.4.0` tags remain absent
- the card ends with the exact candidate SHA and an explicit operator authority
  request; it does not imply or execute that authority

## Validation

- `effigy package:source-consumer`
- operator-authorized authenticated application rebuild/smoke command
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
application builds against another tag/SHA, an unauthenticated or provider-free
substitute is accepted, a retry exceeds authority, a smoke mutates its
repository without approval, or final evidence contains a credential or
provider payload.

Required proof: Cargo metadata exact-revision assertion, the complete operator
authority packet, application dependency/source evidence, authenticated route,
command, attempts and outcome, clean cleanup, redacted final packet,
candidate/CI identity equality, and local/remote tag absence.

## Auto-Continuation

No. Stop for explicit operator authority. No roadmap card authorizes tag
creation or push.

## Stop Conditions

Stop if the application is unnamed, the current path is not authenticated, the
authority packet or retry budget is incomplete, either consumer uses another
source identity, a credential/provider call or repository mutation is not
explicitly authorized, evidence cannot be redacted safely, cleanup fails, the
candidate changes, CI is no longer exact and green, or any release mutation is
requested without the final exact operator authorization.
