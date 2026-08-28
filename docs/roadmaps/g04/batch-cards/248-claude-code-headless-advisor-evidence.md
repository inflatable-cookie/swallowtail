# 248 Claude Code Headless Advisor Evidence

Status: ready
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Milestone: [g04.088 Fifth Parallel Per-Route Feature Qualification](../088-fifth-parallel-per-route-feature-qualification.md)
Depends on: g04.079; g04.083; g04.087 closeout
Research: [245 Claude Code Headless Advisor Evidence](../../../research/245-claude-code-headless-advisor-evidence.md)

## Goal

Freeze exact Claude Code headless advisor version, model, access, selection,
spend, application, result, lifecycle, and omission truth. Promote Research
245 with a closed deliver-now table or an honest empty set.

## Work

1. [ ] Keep route `claude-code.headless`, every published qualified
       `2.1.220..=2.1.241` point, local-subscription access, fixed read-only Plan
       tools, no session persistence, and current lifecycle unchanged.
2. [ ] Freeze official `--advisor` documentation plus exact package
       declarations, parser, aliases, precedence, model resolution, access
       gates, request path, accounting, result shape, and failures.
3. [ ] Build a closed version/access/advisor-model/lifecycle table. Separate
       caller-selected advisor, ambient/default advisor, main model, subagent,
       model catalogue membership, entitlement, and billing.
4. [ ] Determine whether advisor selection is operation-private, immutable,
       pre-effect validated, and compatible with the selected subscription
       route without hidden account or model substitution.
5. [ ] Prove one-run and repeated-turn behavior, any extra provider requests or
       spend, terminal/error mapping, cleanup, and exact omission bytes.
6. [ ] Separate requested, parsed, resolved, dispatched, accepted, effective,
       returned, observed, and billed truth.
7. [ ] Audit prepared input/evidence, command builder, decoder, fixtures,
       guide, matrices, and API baseline without production changes.
8. [ ] Promote Research 245 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [ ] exact version/access/model/lifecycle table or honest empty set exists
- [ ] a non-empty row closes membership, entitlement, selection, application,
      extra-request/spend, terminal, cleanup, and omission truth
- [ ] advisor is not flattened into main-model, subagent, or portable vocabulary
- [ ] unsupported rows reject before prompt effects
- [ ] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-claude-agent
effigy qa:northstar
git diff --check
```

## Stop Conditions

- advisor membership, entitlement, application, or billed work remains live-only
- ambient settings or account defaults can silently replace the requested advisor
- proof needs login, credentials, account inspection, provider prompts, paid
  work, install/update, host mutation, or a shared-contract change

## Out Of Scope

Claude permission modes, response-only or ACP, Fast, compaction, spend cap,
maximum turns, API-key route creation, production binding, live provider work,
currentness, release, shared closeout, rollover, or g04 closure.
