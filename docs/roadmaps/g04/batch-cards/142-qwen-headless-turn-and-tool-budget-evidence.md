# 142 Qwen Headless Turn And Tool Budget Evidence

Status: complete
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.051 Qwen Headless Turn And Tool Budgets](../051-qwen-headless-turn-and-tool-budgets.md)
Depends on: Research 017, 173, and 189; g04.026; g04.041

## Goal

Freeze exact current official and Qwen Code `0.21.15` turn/tool-budget
behavior, then define the smallest caller-decreasing subset whose dispatch,
lifetime, and terminal truth Swallowtail can represent exactly.

## Method

1. Fetch the current official headless documentation and exact official
   `v0.21.15` source. Record retrieval dates, page/source identity, stable URLs,
   complete specimen digests, and the decisive parser/controller/session paths.
2. Freeze current `qwen.headless` command construction, prepared input,
   immutable plan/evidence, driver agreement, structured run, session child,
   reasoning handshake, terminal, cancellation, deadline, failure, and cleanup
   behavior. Do not install Qwen Code or run a provider prompt.
3. Classify omission, turn values `1..=24`, zero, negative, fractional,
   overflow, and values above 24. Separate upstream parser acceptance from the
   proposed Swallowtail caller-decreasing domain.
4. Classify omission, tool-call values `0..=16`, negative, upstream unlimited
   `-1`, fractional, overflow, and values above 16. Prove whether zero is useful
   route truth and whether enforcement occurs before the first tool dispatch.
5. Establish exact counter definitions, increment points, reset points, and
   lifetime for one structured-run child, first turn, resumed turn, and fresh
   replacement. Do not infer operation-wide accounting from child-local state.
6. Establish the exact over-budget observable: process exit code, stderr,
   stream records, terminal classification, partial assistant/tool events,
   cleanup, and whether any work occurs before rejection. Keep upstream
   structured-output exemptions and subagent inner-call rules separate from
   the current route, which selects neither JSON-schema output nor agent tools.
7. Prove ordinary children and exact `0.21.15` reasoning-selected children can
   carry the same admitted budgets without changing initialize/set-effort,
   prompt timing, resume, replacement, or model qualification.
8. Decide the exact typed adapter-local input and immutable plan/evidence
   carrier needed for any admitted subset. Do not add production code or a
   shared capability on this card.
9. Decide whether exact `0.21.15` needs a feature-local behavior revision while
   leaving the wider Contract 029 package currentness claim unchanged.
10. Replace Research 198's reservation with source-backed domain, profile,
    lifetime, and terminal dispositions plus a deliver-now table or honest
    empty set. Do not edit shared closeout surfaces.

Current public documentation and secret-free exact-package/repository evidence
are authorized. No login, credential/account inspection, catalogue call,
provider request, prompt, paid work, package installation, or host
configuration change is authorized.

## Acceptance Criteria

- [x] exact official and `0.21.15` evidence is frozen with source identity and
      complete specimen digests
- [x] upstream parser domains and Swallowtail caller-decreasing domains are
      distinct
- [x] zero-tool usefulness and pre-dispatch enforcement are explicit
- [x] turn/tool counter definitions and child-local lifetimes are explicit
- [x] run, first, resumed, replacement, ordinary, and reasoning-selected
      profiles have dispositions
- [x] process exit, stderr, stream, terminal, partial-event, and cleanup truth
      is explicit or withheld
- [x] plan/evidence representation and compatibility revision are explicit
- [x] Research 198 is promoted with a deliver-now table or honest empty set
- [x] no production code, shared capability, matrix, contract, or currentness
      claim changes
- [x] `effigy validate:focused swallowtail-adapter-qwen` passes
- [x] `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- [x] `git diff --check` passes

Auto-continue to card 143 only when Research 198 admits a non-empty exact
value/profile set with truthful child and terminal semantics.

## Stop Conditions

- exact counting, enforcement, lifetime, or terminal behavior needs a live
  provider prompt or inference
- zero-tool or turn semantics cannot be mapped usefully and exactly
- selection needs a shared capability, contract/currentness change, synthetic
  config root, ambient setting, or breaking API
- ordinary or reasoning-selected child behavior would change

## Out Of Scope

- production binding, guide/matrix claims, another Qwen version/route, live
  work, wall-time selection, permission/tool expansion, release, or merge

## Closeout

Card 142 promoted Research 198 on 2026-08-23. The deliver-now set is
caller-decreasing per-child `--max-session-turns` `1..=24` and
`--max-tool-calls` `0..=16` on exact package `0.21.15`, including ordinary
and reasoning-selected structured runs plus first, resumed, and fresh
replacement children. Omission keeps current `24` / `16`. Zero-tool is
useful: first tool tick aborts before dispatch. Counters are process-local
and reset on every child. Terminal truth stays exit 53 / 55 with plain
stderr on `stream-json`; Swallowtail already classifies those exits.

Cards 143-144 are admitted because the deliver-now set is non-empty. No
shared capability, Contract 029 edit, or currentness-range change is
required on the worker branch.

