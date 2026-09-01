---
title: Post-Claude Code Contract 029 currentness checkpoint worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-234026-post-claude-code-currentness-checkpoint.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, version-currentness]
---

## Objective

Run the required Contract 029 all-route checkpoint after Claude Code
`2.1.257`. Rebuild the complete production-family partition from current
claims, safe host observations, and current official stable channels. Record
one research checkpoint and select exactly one next family, or record why none
is material.

This is research and planning reconciliation only. It changes no compatibility
claim, fixture, matrix, runtime behavior, or provider surface.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base:** `e66d0279d2019b32f0447a28a24ac0fc155f40e4`
- **Pushed-main check:** local `main` and `origin/main` matched at that commit
  before this handoff was compiled.
- **Worker branch:** `worker/version-currentness-post-claude-code-checkpoint`
- **Worker worktree:** Paseo-managed worktree branched from pushed
  `origin/main`.
- **Authority:** Contract 029, `.cursor/skills/version-currentness/`, the
  checkpoint guide, `docs/roadmaps/standing-lanes.md`, the roadmaps Next Task,
  and this handoff.
- **Latest completed family:** Claude Code headless and response-only through
  official `2.1.257` in Research 273 and g05.019 cards 046-047. Watcher remains
  exact `2.1.251`; feature-specific exact sets remain bounded through
  `2.1.241`.
- **Other material boundaries:** Claude Agent ACP is qualified through
  `0.73.0`; Kimi headless through `0.39.1`; `kimi-code.acp` is `QualifiedOnly`
  at `0.38.0` and must not be reopened; Pi RPC is qualified through `0.84.4`;
  Gemini remains deferred.
- **Current generation:** g05 remains the active container. Currentness does
  not keep it open. g05.009/card 034 remains planned, not ready, at 249 proved
  / 518 remaining rows.
- **Ready-frontier shape:** this checkpoint is the sole currentness authority
  lane. The duplicate-roadmap-card papercut remains serial because it examines
  the same currentness allocation and roadmap front-door surfaces.
- **Worker class:** mechanical. This is broad, deterministic source collection
  and reconciliation with settled decision rules. Frontier-worker
  justification: none.

## Scope

In scope:

- re-derive every production route family exactly once from current adapter
  claims and the production feature matrix;
- record safe local `--version` observations for already-installed tools,
  without prompting, authenticating, installing, or upgrading;
- re-probe each family's documented primary official stable channel;
- classify the full family set with the checkpoint vocabulary and reconcile
  counts exactly;
- write the next numbered research record and index it;
- update only the currentness/front-door planning surfaces needed to name the
  one selected family or honest no-candidate result: `docs/roadmaps/README.md`,
  `docs/roadmaps/standing-lanes.md`, `docs/roadmaps/g05/README.md`, and
  `docs/roadmaps/generation-index.md`;
- keep g05.009/card 034, 249/518, watcher `2.1.251`, Kimi ACP A2, and Gemini's
  deferral unchanged.

Out of scope:

- compatibility claims, selection code, matrices, fixtures, contracts,
  architecture, logs, runtime code, or `PAPERCUTS.md`;
- qualifying the selected family or compiling identity/claim cards;
- provider prompts, authentication, live catalogue/session work, installs,
  upgrades, or execution of downloaded artifacts;
- preview, nightly, alpha, and unrelated development channels;
- selecting or changing more than one family;
- reopening `kimi-code.acp`, lifting Gemini's deferral, or promoting card 034.

## Acceptance

The PR is acceptable when:

1. Every production family appears exactly once, with safe host observation,
   official point, current Swallowtail boundary, and one allowed
   classification; totals reconcile without filters or exception lists.
2. External facts use primary official sources and record the observation
   date. Missing host tools are missing observations, not support gaps.
3. Every current host or official stable above a qualified boundary has a
   named material, deferred, exact/opaque, qualified-only, or incompatible
   reason; none is silently left unexplained.
4. Exactly one next family is ranked, or the checkpoint proves no family is
   currently material. The selected family is not Gemini, `kimi-code.acp`, an
   exact/opaque line, or a second flattened route family.
5. The checkpoint changes no claim, matrix, fixture, route, contract,
   architecture, log, runtime source, or papercut surface.
6. Current front doors agree on the checkpoint result, the sole Next Task, the
   g05 status/counts, the unchanged g05.009 pause, and the one-family rule.

## Stop Conditions

Stop and report evidence rather than choosing policy when:

- official sources disagree on package or release identity;
- a candidate depends on a preview channel, provider call, authentication,
  install, or host mutation;
- current claims cannot be reconciled without changing authority;
- ranking would reopen a qualified-only/exact-pin family or lift Gemini's
  deferral;
- a major-line reset or same-package route split cannot be classified without
  operator policy;
- the checkpoint would need to qualify a family rather than merely select it.

## Validation

Run only the checkpoint's docs gates:

- `effigy qa:docs:index:research`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g05`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy qa:northstar`
- `git diff --check`

Do not run workspace QA, package validation, live probes, or provider work.

## Completion Protocol

Before broad reads, confirm a clean registered non-`main` worktree, exact
branch, `HEAD == origin/main`, and that this handoff is loaded from `HEAD`.
Read `AGENTS.md`, Contract 029, the complete version-currentness skill and
reference, the checkpoint guide, standing lane, Research 273, and the latest
all-route checkpoint before inspecting current selection claims and feature
matrix rows. Required sibling worktree links: none.

Use the current official source for every row; frozen checkpoint tables are
not current authority. Keep the family partition exact and do not bulk-bump.
Commit the bounded docs diff, push the worker branch, and open one PR against
current pushed `main`. Report exact head/base, full partition counts, one
selected family or honest no-candidate result, material changed observations,
changed files, validation, and PR URL. Do not merge or start the selected
family.
