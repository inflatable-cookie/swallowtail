# 115 Codex Six-Month Legacy Span Feasibility

Status: planned
Owner: Tom
Updated: 2026-07-23
Milestone: `../039-installed-harness-compatibility-range-audit.md`

## Objective

Determine whether the January-to-April Codex gap can be supported without
weakening bounded execution or pretending legacy app-server v1 is v2.

## Scope

- exec `0.80.0`, `0.99.0`, and `0.121.0` invocation and JSONL behavior
- isolated configuration and rules behavior before `0.122.0`
- ephemeral and web-search milestone at `0.99.0`
- app-server releases before the published v2-only `0.110.0` floor
- separate legacy behavior revision or driver identity where required
- exact exclusions, deprecation posture, and maintenance cost
- compile implementation cards only if the route is bounded and evidence-backed
- no production implementation, auth, consumer support-floor change, or
  lowest-common-denominator API

## Acceptance Criteria

- [ ] the six-month target is either compiled into exact proof work or rejected
      with a technical boundary
- [ ] user config, rules, persistence, and sandbox behavior cannot drift
      silently
- [ ] app-server v1 and v2 remain distinct protocol facades
- [ ] no range crosses an unpublished or untested transition
- [ ] any support-floor decision requiring product policy is returned to the
      operator
- [ ] one sole next task remains

## Validation

- `effigy qa:docs`
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

No. Stop if legacy support would weaken current contracts or establish an
unfixed consumer support policy.
