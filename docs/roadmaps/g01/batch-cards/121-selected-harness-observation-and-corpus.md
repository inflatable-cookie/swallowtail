# 121 Selected Harness Observation And Corpus

Status: planned
Owner: Tom
Updated: 2026-07-23
Milestone: `../040-cross-harness-compatibility-range-expansion.md`

## Objective

Freeze the selected harness range and realize any missing exact observation
boundary before production dispatch changes.

## Scope

- exact selected target and version axis
- baseline, latest boundary, milestones, exclusions, and rejection neighbors
- frozen protocol, schema, invocation, configuration, and lifecycle evidence
- safe host-authoritative observation using the selected route's real seam
- maintained and deprecated support status
- deterministic fixtures before production mapping

## Acceptance Criteria

- [ ] every claimed point has authoritative frozen evidence
- [ ] observation grants no install, auth, route, or execution authority
- [ ] corpus exposes no secret, path, or raw provider payload
- [ ] unknown and malformed versions fail closed
- [ ] production dispatch work is exact enough to compile

## Validation

- focused corpus and observation tests
- workspace all-target check
- workspace warnings-denied clippy
- `git diff --check`

## Auto-Continuation

Yes, only after card 120 makes the route and exact range unambiguous.
