---
title: g04.063 Kimi Code headless reasoning-effort worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-25
updated: 2026-08-25
planning_base: a00d92409565f5dfa32ef0a0275529c91d83a7c5
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260825-181108-g04-063-kimi-code-headless-reasoning-effort.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reconciled g04.062 after PR 61, reassessed the remaining
per-route feature inventory, and selected reasoning effort on
`kimi-code.headless`. g04.063 is compiled. Implementation has not started.
The runway begins with exact package/documentation evidence; cards 177-178 are
conditional on a non-empty Research 210 deliver-now set.

This file is the complete handoff from the planning/orchestrator thread to one
bounded implementation thread. Start from it without a copied transcript or
second prompt. Do not create internal subagents or parallel worker lanes; the
operator's harness owns dispatch.

Read the `northstar` skill, then `references/router.md` and
`references/modes/handoff.md` before task work. This metadata selects the
implementation worker loop. Follow the ready cards and completion protocol.

## Why It Matters

`kimi-code.headless` already selects an exact model and owns one prompt child,
but it exposes no reasoning control. Exact Kimi Code 0.38.0 documentation
describes `[thinking].effort`, per-model `support_efforts` and
`default_effort`, and a process-temporary `KIMI_MODEL_*` surface.

That is promising but not sufficient. The documentation also says an
unsupported configured effort can fall back to the model default. A naive
environment binding could claim one value while Kimi uses another, or could be
shadowed by ambient/user configuration. Research 210 must freeze the exact key,
parse point, precedence, supported version/model/value rows, and failure
behavior before code changes.

The deliverable is deliberately narrow: typed optional reasoning selection on
the existing selected-model headless route, passed only through an adapter-
owned child environment and rejected before spawn if exact agreement is not
proved. No user config mutation, raw environment map, public thought content,
or sibling-route promotion.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `a00d92409565f5dfa32ef0a0275529c91d83a7c5`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  the planning base before this handoff commit
- **Planning checkout:** clean after planning commit and push
- **Planning artifacts:** g04.063, cards 176-178, Research 210 reservation,
  compilation log, closeout reservation, programme/triage/index updates, and
  the sole Next Task
- **Worker mode:** implementation worker dispatched by the orchestrator
- **Worker branch:**
  `agent/g04-063-kimi-code-headless-reasoning-effort-20260825-181108`
- **Worker worktree:**
  `/Users/tom/Dev/worktrees/swallowtail-g04-063-kimi-code-headless-reasoning-effort-20260825-181108`
- **Worktree creation command:** `git worktree add
  /Users/tom/Dev/worktrees/swallowtail-g04-063-kimi-code-headless-reasoning-effort-20260825-181108
  -b agent/g04-063-kimi-code-headless-reasoning-effort-20260825-181108
  origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even when its generated path
  or branch differs from these placeholders. Record the actual path/branch and
  never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent. Never use `/tmp`, `TMPDIR`,
  or a guessed path.
- **Active lane:** per-route feature completion; Contracts 040 and 044 are
  authoritative; no contract edit is planned
- **Roadmap:**
  `docs/roadmaps/g04/063-kimi-code-headless-reasoning-effort.md`
- **Cards, in order:** card 176, conditional card 177, conditional card 178
- **Research:** `docs/research/210-kimi-code-headless-reasoning-effort-evidence.md`
- **Execution topology:** one serial worker lane; cards share model/profile
  input, child environment, fixtures, guide, research, and closeout
- **Route identity:** `kimi-code.headless`, driver
  `swallowtail.kimi.headless`, exact axis `kimi-code.executable`
- **Qualified executable range:** `0.29.0..=0.38.0`; exact 0.38.0 selected
  source is the first evidence candidate; Research 210 owns any lower floor or
  behavior revision split
- **Access:** delegated membership OAuth reference; ambient harness config;
  existing explicit durable retention and managed recovery
- **Current run shape:** one exact selected model, one prompt child,
  `--output-format stream-json`; reasoning currently `No` in the matrix
- **Candidate transport:** exact adapter-owned process-local child environment
  key only. No value or key is prequalified by planning.
- **Current evidence lead:** exact 0.38.0 configuration docs describe
  `[thinking].effort`, `support_efforts`, `default_effort`, temporary
  `KIMI_MODEL_*`, and fallback for unsupported effort. Exact headless
  renderer/options/run-prompt source was already found byte-identical from
  0.37.2 to 0.38.0 in Research 179.
- **Claim boundary:** planned/dispatched/accepted/effective/observed must remain
  separate. Output quality and prose never prove reasoning depth.
- **Required validation:** card-specific gates plus, if code executes, final
  `cargo fmt -p swallowtail-adapter-kimi`, `effigy validate:focused
  swallowtail-adapter-kimi`, `effigy package:verify-affected
  swallowtail-adapter-kimi`, `effigy check:examples`, `effigy qa:routes`,
  `effigy qa:northstar`, relevant research/logs/roadmaps/g04/batch-card/next-
  action index gates, `effigy package:api`, `effigy doctor`, and
  `git diff --check`
- **Known doctor baseline:** inherited 378 god-file findings: 332 warnings and
  46 errors; stale graph index; one generated-in-src warning. Do not increase
  those counts.
- **Planning validation:** `effigy tasks`, `effigy doctor`, `effigy test
  --plan`, `effigy qa:docs`, `effigy qa:northstar`, all docs index and next-
  action gates, and `git diff --check` ran before dispatch. Doctor reproduced
  the inherited baseline; planning gates passed.
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; do not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:** `crates/swallowtail-adapter-kimi/**` for exact headless
  prepared reasoning input/evidence, selected-model checks, child environment,
  failures, fixtures, tests, examples, and API baseline; the Kimi prepared
  guide; feature matrix only when warranted; Research 210; g04.063; cards
  176-178; reserved closeout; triage, programme, indexes, and sole Next Task;
  exact official Kimi Code docs and selected package sources; existing secret-
  free repository evidence
- **Out of scope:** Kimi ACP or local-server mutation; Python `kimi-cli`; raw
  config or environment maps; user config mutation; synthetic Kimi home/config
  roots; plan/yolo/agent/tool/permission/sandbox/memory/multi-agent controls;
  thought content or `ReasoningSummary`; new shared capability/contracts/
  runtime; another route or access profile; live account/login/credential/
  prompt/package-install work; currentness; `CHANGELOG.md`; release,
  publication, merge, generation rollover, or g04 closure
- The contracts at the planning base are the complete authorization boundary.
  If exact evidence requires shared authority or a provider-neutral surface,
  stop for orchestrator review.
- Current official documentation is a lead. Research 210 must record exact
  retrieved bodies/packages, dates, hashes, and decisive selected source.
- Identify the exact process-local key, its parser, precedence, scope, and
  lifetime. Distinguish child env, inherited env, user config, model config,
  CLI flags, and defaults.
- Freeze exact executable-version, selected-model, provider, value,
  `support_efforts`, and `default_effort` rows. Do not infer support from names
  or prequalify `off|on|low|medium|high|xhigh|max`.
- Missing, empty, malformed, unknown, unsupported, aliased, clamped, ignored,
  shadowed, or fallback values must be classified. Deliver only rows whose
  selected value cannot silently become another value or the model default.
- The adapter may set only the exact qualified key in the child environment.
  It must not mutate user configuration or expose generic settings.
- Inherited ambient configuration must not replace the prepared selection.
  Contradiction or unproved precedence fails before process creation.
- Preserve exact selected-model/provider agreement. A catalogue name or model-
  family label does not qualify a row.
- Preserve existing one-prompt arguments, delegated access, durable retention,
  managed recovery, retry, cancellation, deadline, terminal, and cleanup
  behavior. Omission must retain the current child launch.
- Freeze stream-json disclosure behavior. Thinking text and reasoning summaries
  remain outside public output, activity, callbacks, evidence, and diagnostics.
- Claim only the planned/dispatched/accepted/effective/observed layers that
  exact source and deterministic fixtures prove. Do not infer depth from
  tokens, prose, latency, or output quality.
- Default QA must not resolve credentials, install Kimi, run a prompt, contact
  a provider, or use paid inference.
- An honest evidence stop after card 176 is complete work. Mark cards 177-178
  blocked, update the reserved closeout and sole Next Task, and open the PR.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or clean/reset/stash over unrelated state.
- Do not merge the PR. Merge remains a separate operator-authorized action.
- Follow repository `AGENTS.md`, canonical authority, glue-light reporting,
  and Effigy selectors. Work in one meaningful batch.

## Important Context

- Research 017/046/056/066/068/074 establish the installed Kimi route,
  command/source corpus, access, retention, recovery, and headless behavior.
- Research 159 and 179 establish the exact executable axis and qualify
  `0.29.0..=0.38.0`. Research 179 is the current exact-source authority.
- Research 207 and 208 concern Kimi ACP effort/mode controls. They are sibling-
  route evidence only and do not qualify headless transport or confirmation.
- The headless route already has exact model selection. Research 210 must name
  the model rows rather than converting a global thinking setting into an
  unqualified route-wide claim.
- Official 0.38.0 configuration documentation says unsupported selected effort
  may fall back to model default. Treat that as a hard evidence gate, not an
  implementation detail.
- Official command documentation says stream-json does not write thinking
  content to JSONL. Verify exact selected source and avoid promoting that into
  a broader privacy claim than the evidence supports.
- A process-temporary model environment surface is not automatically safe. The
  exact key and its precedence over ambient/user state must be proved before it
  becomes an adapter-owned binding.
- Omission is a first-class compatibility case. Do not add a default effort or
  rewrite existing environment merely because a model has `default_effort`.
- The route's explicit retention and managed recovery contracts already exist.
  A new effort value must not alter durable state or restoration semantics.
- Generation boundary: do not close or roll over g04. After merge the
  orchestrator reconciles g04.063 and follows the sole Next Task.
- Decisions: manual operator-harness handoff only; no internal subagents. New-
  route research does not pre-empt per-route feature work.
- **Report after:** card 176/Research 210, then the binding/acceptance batch if
  the deliver-now set is non-empty
- **Report to:** the operator, who relays progress to the orchestrator

## Suggested Next Move

This handoff activates worker mode. Start by reading it from the top. Before
broad repository reads, run the quick startup worktree-safety preflight below.
If the current context is a clean, dedicated, non-`main` registered worktree,
use it immediately, record its actual path/branch, and do not create another
worktree because its generated name differs from this file.

Read `AGENTS.md`, g04.063, cards 176-178, Research
017/046/056/066/068/074/159/179/207/208/210, the Kimi guide, system
architecture, and Contracts 011/029/033/037/040/044/052. Execute card 176
first. Promote Research 210 with a non-empty exact table or honest empty set.
Continue automatically only when its gate is satisfied.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode.
2. Run one quick read-only safety probe: `git rev-parse --show-toplevel`, `git
   branch --show-current`, `git status --porcelain`, and `git worktree list
   --porcelain`.
3. Use the launcher-supplied clean, dedicated, non-`main` registered worktree.
   Confirm its base contains planning commit `a00d9240`. Never implement on
   `main`.
4. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Record the
   inherited doctor baseline; do not treat it as new work.

### Execute

1. Read the named authority and route-local source before editing.
2. Execute card 176 and promote Research 210. Official packages may be
   inspected in a disposable directory without install or prompt execution.
3. If the deliver-now set is empty or any decision gate fails, stop honestly:
   block cards 177-178, close the milestone as an evidence stop, update the
   reserved closeout/indexes/sole Next Task, validate, commit, push, and open
   the evidence PR.
4. If Research 210 admits exact rows, execute cards 177-178 serially in one
   meaningful implementation batch. Keep all changes inside the allowed
   surfaces.
5. Update Research 210, milestone/card status, guide/matrix/API as warranted,
   programme, triage, reserved closeout, indexes, and sole Next Task to match
   what was actually proved. Keep g04 open.

### Validate and report

1. Run every card-specific gate and the final validation set named in Current
   State. Default validation must remain account-, credential-, install-,
   prompt-, and provider-free.
2. Run `git diff --check` against the planning base and confirm no out-of-scope
   files, secrets, downloaded packages, or temporary research corpus remains.
3. Commit cohesive work, push the worker branch, and open one PR against
   `main`. Do not merge it.
4. Report exact branch, PR URL, head SHA, Research 210 disposition, delivered
   rows or stop reason, changed surfaces, validation, inherited baseline, and
   any review concern. State clearly that merge authority remains with the
   operator/orchestrator.
