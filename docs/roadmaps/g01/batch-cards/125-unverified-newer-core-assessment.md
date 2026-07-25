# 125 Unverified Newer Core Assessment

Status: completed
Owner: Tom
Updated: 2026-07-24
Milestone: `../041-qualified-support-and-newer-version-execution.md`

## Objective

Represent and preflight qualified, unverified-newer, and incompatible exact
interface versions without consumer or provider identity.

## Scope

- explicit per-claim newer-version posture
- ordered exact-version assessment
- latest-qualified behavior revision for forward attempts
- exact exclusions beyond the qualified ceiling
- qualified-only opaque axes
- descriptor and immutable-plan assessment
- installed-executable observation and discovery status
- preflight permits qualified and explicitly allowed unverified-newer points
- no adapter dispatch

## Acceptance Criteria

- [x] qualified support remains distinguishable from execution permission
- [x] unverified-newer carries exact version and latest-qualified evidence
- [x] known-incompatible points remain closed
- [x] consumer presentation and policy remain downstream
- [x] no provider branch enters core, runtime, or testkit

## Validation

- focused core and testkit tests
- workspace all-target check
- workspace warnings-denied clippy
- `git diff --check`

## Auto-Continuation

Yes, after the three-way assessment and preflight behavior are deterministic.

## Outcome

Ordered claims now choose `QualifiedOnly` or `AllowUnverified`. Assessment
returns qualified, unverified-newer, or incompatible without changing the
qualified `supports` result. Unverified evidence retains the exact version,
latest-qualified boundary, and private behavior revision. Preflight permits
only qualified or explicitly allowed unverified points. Opaque claims,
prereleases, gaps, exclusions, malformed values, and older points remain
closed. Installed discovery preserves the same posture.
