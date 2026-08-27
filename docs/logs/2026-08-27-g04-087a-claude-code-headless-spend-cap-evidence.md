# 2026-08-27 g04.087a Claude Code Headless Spend-Cap Evidence

Status: complete
Card: 244
Research: 241

## Boundary

Evidence only. This lane updated card 244, Research 241, this log, and
`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-spend-cap.json`
plus the fixture README note. Shared planning and production code stayed
unchanged.

## Evidence Target

Close exact qualified version, local-subscription access, positive value,
units, precedence, enforcement, accrued-cost source, terminal/exit, cleanup,
and omission truth for `--max-budget-usd` on `claude-code.headless`.

## Finding

Honest empty set.

- Every published point in `2.1.220..=2.1.241` advertises `--max-budget-usd`
  for API-call dollars in print mode and rejects non-positive parser inputs.
- Precedence is argv-only; no competing env var or settings key was observed.
- Accrued cost is a local model-catalog USD estimate on the session ledger,
  not subscription allowance, usage credits, or provider-billed API USD.
- Native guard emits `error_max_budget_usd`. Live limit-reached was not run;
  that needs provider turns this card forbids.
- Selected route stays local-subscription and rejects API-key billing. Units
  and access therefore do not close for a deliver-now binding.
- Current command builder omits the flag; omission remains exact.

## Output Contract

Promoted Research 241 with a closed empty deliver-now table. Did not create an
API-key route or equate subscription allowance with provider-billed USD.

## Validation

```sh
effigy validate:focused swallowtail-adapter-claude-agent
effigy qa:northstar
git diff --check
```

Doctor baseline unchanged in kind: 380 god-file findings (334 warnings, 46
errors), one generated-in-src warning. Recorded as inherited drift only.

## Next

Open the evidence PR against current pushed `main`. Do not merge or begin
production binding. Shared g04.087 / Next Task updates stay with the
orchestrator after the PR lands.
