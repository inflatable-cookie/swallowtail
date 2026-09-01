---
title: Contract 029 all-route currentness checkpoint worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: research
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-124336-version-currentness-all-route-checkpoint.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, version-currentness]
---

## Objective

Run the queued Contract 029 all-route version-currentness checkpoint. Rebuild
the full production-family table from current repository claims, safe host
version observations, and official stable channels. Write one research record,
index it, and name exactly one next family or record why none is material.

This is research only. It does not change a compatibility claim.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base:** `f56b54d6bc829975141088c4afa5ab2f5ab0454b`
- **Pushed-main check:** local `main` and `origin/main` matched at that commit
  before this handoff was compiled.
- **Worker branch:** `worker/version-currentness-all-route-checkpoint`
- **Worker worktree:** Paseo-managed worktree branched from pushed
  `origin/main`.
- **Authority:** Contract 029, the version-currentness runbook, the standing
  lane, the repository `version-currentness` skill, and this handoff.
- **Latest completed family:** Codex exec/app-server through official `0.151.0`
  in Research 262 and g05.012.
- **Other recent qualified points:** Claude Code `2.1.251`, Qwen headless
  `0.22.3`, and Kimi Code headless `0.38.0` on its v2 axis.
- **Ready-frontier shape:** independent research lane. It may run beside the
  watcher-host papercut because it owns only the research record and research
  index.
- **Serial edge:** any one-family qualification selected by this checkpoint
  waits for this PR to merge and for an operator-approved roadmap card.
- **Worker class:** mechanical. This is broad, methodical evidence collection,
  not frontier implementation reasoning.

## Scope

In scope:

- re-derive every production route family from current adapter claims and the
  production feature matrix;
- record safe local `--version` observations where a tool is already on PATH;
- check each family's documented official stable channel using primary sources;
- classify every family with the runbook vocabulary;
- write the next numbered research record and add exactly one research-index
  entry;
- rank one next family only when the evidence supports a material candidate.

Out of scope:

- claim, selection, matrix, fixture, contract, architecture, roadmap, log, or
  runtime changes;
- provider prompts, authentication, installs, upgrades, or live sessions;
- preview, nightly, alpha, and unrelated development channels;
- qualifying more than one family or compiling its implementation card;
- lifting the existing Gemini deferral.

## Acceptance

The PR is acceptable when:

1. Every production family is present exactly once, with local observation,
   official point, current Swallowtail boundary, and one allowed classification.
2. Every external fact cites the primary official source and records the
   observation date. Registry `latest` alone is never treated as compatibility.
3. Missing host tools are recorded as missing observations, not gaps.
4. Current host or official stable points above a qualified boundary have a
   named incompatible/deferred reason; none is silently left unexplained.
5. Exactly one next family is selected, or the record states why no family is
   currently material. The checkpoint itself makes no claim change.
6. The diff is limited to one new `docs/research/` record and
   `docs/research/README.md`.

## Stop Conditions

Stop and report evidence rather than choosing policy when:

- official sources disagree on package or release identity;
- the next family depends on a preview channel, live provider call, install, or
  authentication;
- current repository claims cannot be reconciled without changing authority;
- more than one family would need qualification in this PR;
- any result would require lifting the Gemini deferral.

## Validation

Run only the docs-only checkpoint gates:

- `effigy qa:docs:index:research`
- `effigy qa:northstar`
- `git diff --check`

Do not run workspace QA or provider probes.

## Completion Protocol

Before broad reads, confirm a clean registered non-`main` worktree, exact
branch, `HEAD == origin/main`, and that this handoff is loaded from `HEAD`.
Read `AGENTS.md`, Contract 029, the version-currentness skill and reference,
the runbook, standing lane, Research 127, Research 159, Research 262, current
selection claims, and the feature-matrix version columns.

Commit the bounded research diff, push the worker branch, and open one PR
against current pushed `main`. Report exact head/base, selected next family or
honest no-candidate result, source set, host observations, changed files,
validation, and PR URL. Do not merge.

