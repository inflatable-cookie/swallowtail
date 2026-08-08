# 157 Prepared Plan Builder Extraction

Status: completed
Owner: Tom
Created: 2026-08-08
Milestone: `../052-shared-adapter-scaffolding.md`
Depends on: card 156

## Goal

Extract the prepared plan construction family into one shared builder so the
sixteen adapter copies of `instance_with_capabilities`, `requirements`, and
`build_plan` collapse into a parameterized helper.

## Scope

1. Extract the plan-family skeleton from the runtime's shared records into a
   `PreparedPlanBuilder` (or equivalent) in `swallowtail-runtime`, taking
   descriptor, access profile, evidence, capabilities, and host-service
   validation as parameters.
2. Migrate adapters in two tranches: first the hosted direct family
   (alibaba, anthropic, deepseek, kimi-platform, openai, xai), then the
   installed-harness family (codex, claude-agent, cursor, gemini, grok, kimi,
   qwen, pi, oh-my-pi, antigravity).
3. Keep the adapter-local pieces (claims, request builders, profile inputs)
   local; only the shared skeleton moves.

## Out Of Scope

- prepared operation behavior or evidence shape changes
- public API changes

## Acceptance

- [x] the shared builder has focused tests covering capability, requirement,
      and drift paths
- [x] every migrated adapter passes focused and extracted-package proof with
      an unchanged public API baseline
- [x] the plan-family duplication shrinks by the measured amount

## Stop Conditions

- stop if any migrated adapter changes its prepared plan, request, or
  preflight failure

## Auto-Continuation

Yes, to card 158 after acceptance.

## Validation

- focused validation per migrated adapter; `effigy package:verify-affected`
  per tranche
- `effigy package:api` after each tranche

## Completion Evidence

- new `swallowtail-runtime/src/prepared_plan.rs` owns three provider-neutral
  skeletons with three focused tests:
  - `instance_with_capabilities(base, capabilities)` — the configured-
    instance rebinding every adapter copied (adapter posture extensions
    chain on the result)
  - `base_requirements(layer, shape, role, instance, access_profile,
    credential_states, capabilities)` — the base operation-requirements
    record; credential states are an explicit parameter because claude-agent
    maps `LocalUnauthenticated` to `NotRequired`
  - `build_plan(descriptor, instance, route, requirements, access_profile,
    access_status, available_host_services)` — the preflight build with
    `PreflightStage::Preflight` failure mapping
- all sixteen plan-family adapters migrated in two tranches (hosted direct:
  alibaba, anthropic, deepseek, kimi-platform, openai, xai; installed
  harness: codex, claude-agent, cursor, gemini, grok, kimi, qwen, pi,
  oh-my-pi, antigravity); the claude-agent mechanism-derived credential
  state and cursor/antigravity ownership pre-checks stayed adapter-local,
  and openai's `openai_background_requirements` stayed in selection.rs
  because it takes a different parameter shape
- 573 adapter lines deleted across the fifteen plan modules; the shared
  module is one copy of each skeleton
- behavior parity: the previously-failing claude-agent local-subscription
  test was caught by the full round and fixed via the credential-state
  parameter; every migrated adapter's existing tests pass unchanged
- focused validation passes for all eighteen packages, affected-package
  proof passes for representative tranche pairs, the semantic API baseline
  is unchanged for the adapters (runtime additions captured in the
  regenerated v0.3.0 baseline), and the workspace round passes 1,502 tests
