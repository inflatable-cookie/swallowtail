# 034 Provider-Wide Facade Matrix And Guides

Status: completed
Owner: Tom
Created: 2026-07-25
Milestone: `../012-provider-wide-acceptance-and-candidate-return.md`

## Objective

Give consumers one exact route matrix and adapter-local normal-path guidance
for all production drivers.

## Governing Refs

- Contract 037
- completed roadmaps g02.007-g02.011
- existing Codex guide

## Scope

1. Publish a 22-route matrix covering crate, driver, role, transport, target,
   access, version axis, prepared constructor, bound operation, and low-level
   escape hatch.
2. Add compile-tested examples by runtime family and focused route examples
   where behavior differs.
3. Document guaranteed ranges versus unverified-newer attempts.
4. Document explicit inputs and non-goals.
5. Replace Codex-only front-door integration wording.

## Acceptance Criteria

- [x] every production route appears exactly once
- [x] no example implies automatic routing or credential discovery
- [x] operation-specific authority is visible at the call site
- [x] facade and low-level paths are both documented
- [x] version posture is clear to application integrators

## Validation

- docs and doctests
- example compilation from public packages
- route-matrix inventory check
- link and formatting checks

## Execution Evidence

`docs/guides/provider-route-matrix.md` now inventories all 22 production
routes across six families. Every row names its package, driver, roles,
transport, explicit target and access inputs, version axis, prepared
constructor, bound operations, and low-level escape hatch. Remote ACP remains
an explicit composable transport rather than a fabricated twenty-third route.

All adapter examples compile from public declarations under
`effigy check:examples`. `effigy qa:routes` proves the exact route set, count,
and uniqueness. `effigy qa:docs` passes. The guide states guaranteed,
deprecated, excluded, and unverified-newer behavior without implying provider,
model, endpoint, credential, or transport fallback.

## Auto-Continuation

Yes. Continue to card 035.
