# 028 Pi Interpreted Installed Discovery Proof

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../011-host-approved-interpreted-executable-launch.md`
Depends on: card 027

## Goal

Prove exact installed Pi discovery through a host-approved interpreted launch
recipe without inheriting ambient environment or contacting the provider.

## Scope

1. Replace the direct-process live selector with the real Pi discovery role
   running through `LocalProcessHost`.
2. Keep selection test-owned: resolve the installed Pi script and Node
   interpreter before constructing the opaque target.
3. Assert exact `0.83.0` qualified classification and joined cleanup.
4. Keep the production Pi adapter free of launcher and PATH policy.

## Acceptance Criteria

- [x] the live selector uses `InstalledExecutableDiscoveryRequest`
- [x] the local host launches exact Node plus the exact selected Pi script
- [x] no ambient environment reaches the child
- [x] Pi exact `0.83.0` remains the observed harness version
- [x] the adapter contains no Node, npm-layout, PATH-search, or fallback rule
- [x] the selector is ignored and explicitly gated
- [x] no provider prompt, credential, or workspace effect occurs

## Validation

- `effigy validate:focused swallowtail-adapter-pi`
- `effigy probe:pi-installed`
- `cargo fmt --all --check`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

Yes. Continue directly to card 029 when the live installed proof passes.

## Evidence

- 41 deterministic Pi tests passed with the host-local test dependency
- the explicitly gated installed selector classified exact Pi `0.83.0` through
  `LocalProcessHost` in under one second
- the test selected exact Node and script paths before target construction;
  the child inherited no ambient environment
- no provider prompt, credential acquisition, network call, or workspace access
  ran
