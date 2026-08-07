# 157 Prepared Plan Builder Extraction

Status: planned
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

- [ ] the shared builder has focused tests covering capability, requirement,
      and drift paths
- [ ] every migrated adapter passes focused and extracted-package proof with
      an unchanged public API baseline
- [ ] the plan-family duplication shrinks by the measured amount

## Stop Conditions

- stop if any migrated adapter changes its prepared plan, request, or
  preflight failure

## Auto-Continuation

Yes, to card 158 after acceptance.

## Validation

- focused validation per migrated adapter; `effigy package:verify-affected`
  per tranche
- `effigy package:api` after each tranche
