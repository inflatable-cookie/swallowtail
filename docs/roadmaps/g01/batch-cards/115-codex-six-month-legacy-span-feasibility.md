# 115 Codex Six-Month Legacy Span Feasibility

Status: completed
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

- [x] the six-month target is either compiled into exact proof work or rejected
      with a technical boundary
- [x] user config, rules, persistence, and sandbox behavior cannot drift
      silently
- [x] app-server v1 and v2 remain distinct protocol facades
- [x] no range crosses an unpublished or untested transition
- [x] any support-floor decision requiring product policy is returned to the
      operator
- [x] one sole next task remains

## Validation

- `effigy qa:docs`
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

No. Stop if legacy support would weaken current contracts or establish an
unfixed consumer support policy.

## Decision

Proceed with exact deprecated legacy segments. Do not use a container,
temporary credential home, v1 app-server driver, or implicit fallback.

Tagged `0.80.0` source already exposes the selected app-server v2 methods. Its
legacy difference is default stdio invocation; `--listen stdio://` begins at
`0.100.0`. The existing v2 facade can extend to January with private behavior
dispatch and a stable read-only capability subset.

Exec remains one structured-CLI driver with explicit behavior segments:

- `0.80.0..=0.81.0`: ambient config, durable retention, legacy search config
- `0.84.0..=0.98.0`: ambient config, durable retention, search-mode config
- `0.99.0..=0.121.0`: ambient config, ephemeral
- `0.122.0..=0.145.0`: suppressed config, ephemeral

Unpublished `0.82.0`, `0.83.0`, `0.108.0`, and `0.109.0` stay excluded.
Consumer warning, route preference, and acceptance of deprecated posture remain
downstream policy. Swallowtail only exposes exact requirements and capabilities.

## Continuation

- card 116: shared harness-configuration posture contract and records
- card 117: exact Codex legacy corpora
- card 118: private legacy version dispatch in both existing drivers
- card 119: six-month conformance and full closeout
