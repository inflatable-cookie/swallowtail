# 127 Consumer Front Door And Release Copy

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../043-v0-1-0-source-release-readiness.md`
Depends on: card 126

## Goal

Make source-tag installation, package selection, prerequisites, support, and
release limits obvious without reading roadmap history.

## Scope

1. Replace the root README chronicle with a concise consumer front door.
2. Add exact Git-tag dependency and package-coordination guidance.
3. Update changelog and v0.1.0 release notes to current package and route truth.
4. Add security and contribution/support guidance.
5. Keep route-specific harness and sidecar prerequisites in canonical guides.

## Validation

- `effigy qa:docs`
- `effigy qa:guides`
- external link and source-install checks

## Completion

- Replaced the 877-line root chronicle with a 177-line consumer front door
  covering route choice, exact-tag installation, package roles, integration
  shape, prerequisites, compatibility, development, support, and security.
- Rewrote the changelog and `v0.1.0` release notes around the current
  27-package, 33-route source-tag candidate and its remaining gates.
- Added security, support, and contribution policies without duplicating
  route-specific prerequisites from canonical guides.
- Added `qa:consumer-docs` to validate the exact Git pin, package inventory,
  route inventory, and policy-file presence through ordinary docs QA.
- Docs, expanded links, guide coverage, all workspace examples, Northstar
  checks, and diff hygiene pass without authenticated or external effects.

## Auto-Continuation

Yes. Continue to card 128 after the consumer copy matches the reviewed API.
