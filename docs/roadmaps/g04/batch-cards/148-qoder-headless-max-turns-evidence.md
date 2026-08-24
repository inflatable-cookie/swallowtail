# 148 Qoder Headless Maximum-Turn Evidence

Status: blocked; awaiting operator claim reconciliation
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Milestone: [g04.053 Qoder Headless Maximum Turns](../053-qoder-headless-max-turns.md)
Depends on: Research 151; exact route `qoder.headless` `1.1.25`

## Goal

Freeze exact current official and Qoder `1.1.25` maximum-turn behavior, then
define the smallest caller-decreasing positive subset whose dispatch, counting,
limit terminal, and cleanup Swallowtail can represent exactly.

## Method

1. Fetch current official CLI/run-in-scripts documentation and the exact npm
   `@qoder-ai/qodercli@1.1.25` tarball. Record retrieval dates, identities,
   complete digests, and decisive parser/counter/enforcement/result paths.
2. Freeze current command, prepared input, immutable plan/evidence, driver,
   stream decoder, terminal, cancellation, deadline, failure, and cleanup.
3. Classify omission, `1..=8`, zero, negative, fractional, overflow, and values
   above eight. Separately classify omission of `--max-turns`.
4. Establish the exact turn definition, counter source, increment/check order,
   off-by-one boundary, child reset, and `result.num_turns` relationship.
5. Establish at-limit process exit, result subtype/error fields, partial events,
   terminal classification, cancellation, deadline, and joined cleanup.
6. Define the typed adapter-local carrier and feature-local revision required
   by any admitted subset. Do not add production code on this card.
7. Replace Research 200's reservation with a deliver-now table or honest empty
   set. Do not edit shared closeout surfaces.

No login, install, credential/account inspection, provider prompt, paid work,
or host configuration change is authorized.

## Acceptance Criteria

- [x] exact official and `1.1.25` evidence is frozen with identities and digests
- [x] parser domain and caller-decreasing public candidate are distinguished
- [ ] omission, `1..=8`, zero, negative, fractional, raised, overflow, and
      unbounded states have explicit dispositions reconciled with the qualified
      route (blocked: AgentLoop facts contradict prior omit-unbounded /
      required-bound-8 claims)
- [x] turn definition, check/increment order, off-by-one boundary, child-local
      lifetime, and `num_turns` relationship are explicit
- [x] process, result envelope, stream, terminal, partial-event, cancellation,
      deadline, failure, and cleanup truth is explicit or withheld
- [ ] plan/evidence representation and feature-local revision are explicit
      after claim reconciliation
- [x] Research 200 contains a deliver-now table or honest empty set
- [x] no production code, shared capability, matrix, contract, currentness, or
      sibling-route change lands on this card alone

## Evidence And Pause

Research 200 freezes exact digests and admits no deliver-now caller-decreasing
row: selected CLI headless QueryEngine factory (`entrypoint: "cli"`) hardcodes
AgentLoop `maxTurns: kN` (`1000`); CLI `--max-turns` only populates Config
`maxSessionTurns` for the text-error formatter.

That falsifies the AgentLoop reading of Research 151 / guide /
`require_max_turns` / `omit-max-turns-unbounded` / `command.rs` "required
positive CLI turn bound" claims. Decoder mapping of synthetic `error_max_turns`
(`limit.jsonl` `num_turns: 1`) remains distinct from proving argv `8` stops at
turn 8.

Card 148 is **not** complete. Cards 149-150 stay blocked. Production adapter,
guide, and fixtures are untouched pending operator planning: either rewrite
corpus claims to match factory `1000`, or change the qualified route.

## Validation

- `effigy qa:docs:index:research`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:index:logs`
- `git diff --check`

## Stop Conditions

- Stop after this card if Research 200 admits no exact deliver-now row.
- Stop if decisive evidence requires live provider work.
- Stop if the feature needs a shared capability, contract/currentness change,
  sibling route, or breaking API.
- Pause for operator if exact evidence contradicts the qualified route's
  existing max-turns claims rather than only a proposed caller option.
