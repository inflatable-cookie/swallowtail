# 142 Qwen Headless Turn And Tool Budget Evidence

Status: ready
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

- [ ] exact official and `0.21.15` evidence is frozen with source identity and
      complete specimen digests
- [ ] upstream parser domains and Swallowtail caller-decreasing domains are
      distinct
- [ ] zero-tool usefulness and pre-dispatch enforcement are explicit
- [ ] turn/tool counter definitions and child-local lifetimes are explicit
- [ ] run, first, resumed, replacement, ordinary, and reasoning-selected
      profiles have dispositions
- [ ] process exit, stderr, stream, terminal, partial-event, and cleanup truth
      is explicit or withheld
- [ ] plan/evidence representation and compatibility revision are explicit
- [ ] Research 198 is promoted with a deliver-now table or honest empty set
- [ ] no production code, shared capability, matrix, contract, or currentness
      claim changes
- [ ] `effigy validate:focused swallowtail-adapter-qwen` passes
- [ ] `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- [ ] `git diff --check` passes

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

