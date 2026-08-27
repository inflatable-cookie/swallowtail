# 244 Claude Code Headless Spend-Cap Evidence

Status: complete
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.087 Fourth Parallel Per-Route Feature Qualification](../087-fourth-parallel-per-route-feature-qualification.md)
Depends on: g04.079; g04.083; g04.086 closeout
Research: [241 Claude Code Headless Spend-Cap Evidence](../../../research/241-claude-code-headless-spend-cap-evidence.md)

## Goal

Freeze exact Claude Code headless spend-cap version, access, units,
precedence, enforcement, terminal, billing, and omission truth, then promote
Research 241 with a closed deliver-now table or an honest empty set.

## Work

1. [x] Keep route `claude-code.headless`, exact published points in qualified
       `2.1.220..=2.1.241`, local subscription access, fixed read-only Plan
       tools, no session persistence, and current lifecycle unchanged.
2. [x] Freeze official `--max-budget-usd` documentation plus exact package
       declarations, parser, precedence, accounting source, loop enforcement,
       result schema, terminal/exit behavior, and access/billing branches.
3. [x] Build a closed version/access/value/unit table. Separate subscription
       allowance, API-key spend, provider-reported cost, local estimates, and
       a caller budget. Do not claim USD enforcement without exact evidence.
4. [x] Determine whether the selected subscription-only prepared route can
       bind a positive cap operation-privately and reject unsupported values
       before prompt effects without selecting a different billing profile.
5. [x] Prove exact argv/environment/settings precedence, repeated-turn or
       one-run scope, limit-reached result, exit, cleanup, and omission bytes.
6. [x] Separate requested, parsed, applied, accrued, provider-billed, enforced,
       returned, and observed truth.
7. [x] Audit prepared input/evidence, plan/request agreement, command builder,
       decoder, fixtures, guide, matrices, and API baseline without changing
       production surfaces.
8. [x] Promote Research 241 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [x] exact version/access/value table or honest empty set is recorded
- [x] a non-empty row proves units, billing source, precedence, enforcement,
      terminal mapping, and route-compatible access
- [x] local subscription allowance is not presented as API-key USD spend
- [x] omission retains current command and authority exactly
- [x] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Result

Honest empty set. Every published `2.1.220..=2.1.241` point advertises and
parses `--max-budget-usd`, enforces against a local catalog-priced USD ledger,
and shapes `error_max_budget_usd`. That meter does not close against the
selected local-subscription access/billing profile. Omission stays unchanged.
Details: Research 241 and
`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-spend-cap.json`.

## Validation

```sh
effigy validate:focused swallowtail-adapter-claude-agent
effigy qa:northstar
git diff --check
```

## Stop Conditions

- exact source cannot tie the cap to the selected access/billing profile
- enforcement or terminal truth requires a paid provider prompt
- ambient environment/settings can silently replace the requested cap
- proof needs login, credentials, account inspection, install/update, host
  mutation, production code, or a shared-contract change

## Out Of Scope

Claude response-only or ACP, advisor, permission modes, Fast, autocompaction,
maximum turns, API-key route creation, production binding, live provider work,
currentness, release, shared closeout, rollover, or g04 closure.
