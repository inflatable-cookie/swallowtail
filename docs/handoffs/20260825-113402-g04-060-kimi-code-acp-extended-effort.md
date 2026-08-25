---
title: g04.060 Kimi Code ACP extended-effort worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-25
updated: 2026-08-25
planning_base: cbc5ae7d518d12c37904cea792094c1fc178ccc5
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260825-113402-g04-060-kimi-code-acp-extended-effort.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reassessed the remaining per-route feature inventory after
g04.059 and selected Kimi Code ACP catalogue-declared `xhigh` and `max`
reasoning levels. g04.060 is compiled. Implementation has not started. The
ready runway begins with exact version/source milestone evidence; cards
168-169 are conditional on a non-empty Research 207 deliver-now set.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. Start from this file without a copied transcript or a
second prompt. Do not create internal subagents or parallel worker lanes; the
operator's harness owns dispatch.

## Why It Matters

Production route `kimi-code.acp` already negotiates reasoning from one exact
session `thinking` config option. Swallowtail sends one
`session/set_config_option` and requires the returned snapshot to confirm the
effective value before returning a ready session. Its parser accepts
`off|on|low|medium|high` and rejects any other advertised row as malformed.

Exact official Kimi Code `0.38.0` source constructs that option from the
current model's `support_efforts`; exact source tests exercise `xhigh` and
`max`. The missing feature is therefore a narrow route-local validation ceiling,
not a new route, UI-label translation, model-name guess, or authority change.
The exact version floor and end-to-end preservation still need proof before
production widening.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `cbc5ae7d518d12c37904cea792094c1fc178ccc5`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `cbc5ae7d518d12c37904cea792094c1fc178ccc5` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Planning artifacts included at the base:** g04.060, cards 167-169,
  Research 207 reservation, compilation log, route-local closeout reservation,
  inventory reassessment, programme boundary, and the sole Next Task
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker branch:** `agent/g04-060-kimi-acp-extended-effort-20260825-113402`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-060-kimi-acp-extended-effort-20260825-113402`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-060-kimi-acp-extended-effort-20260825-113402 -b agent/g04-060-kimi-acp-extended-effort-20260825-113402 origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and
  never create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent. Never use `/tmp`, `TMPDIR`,
  or a guessed path.
- **Active spec lane:** per-route feature completion; existing contracts are
  the complete authority and no contract edit is planned
- **Roadmap milestone:**
  `docs/roadmaps/g04/060-kimi-code-acp-catalogue-declared-effort-levels.md`
- **Ready cards, in order:** card 167, then conditional card 168, then
  conditional card 169
- **Allowed runway:** exact Kimi ACP extended-effort milestone evidence, then
  only Research 207-admitted parser/compatibility binding and acceptance
- **Remaining card budget:** three serial cards; cards 168-169 run only after
  their named evidence and implementation gates
- **Dispatch topology:** one serial worker lane. Do not use internal subagents;
  report through the operator's harness.
- **Parallel safety check:** cards share exact source evidence, compatibility
  segmentation, option validation, fixtures, guide, research, and closeout;
  they are not parallel-safe
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 023, 029, 034, 037, 040, 041, and 052
- **Route identity:** `kimi-code.acp`, driver `swallowtail.kimi.acp`, axis
  `kimi-code.executable`, ACP v1 over stdio
- **Qualified versions:** deprecated exact `0.28.1`; maintained
  `0.29.0..=0.38.0`; later stable points visible as `UnverifiedNewer`
- **Current compatibility behavior:** `0.28.1`
  `kimi.acp.reasoning.legacy-select-v1`; `0.29.0..=0.38.0`
  `kimi.acp.reasoning.declared-effort-v2`
- **Current reasoning mapping:** new-session-only portable
  `off|on|low|medium|high`; exact `thinking` / `thought_level` select snapshot;
  one set request; response snapshot confirmation; load/resume redeclaration
  rejected before host effects
- **Current parser gap:**
  `crates/swallowtail-adapter-kimi/src/driver/reasoning.rs` treats any declared
  effort outside `off|on|low|medium|high` as a malformed option
- **Current access:** delegated Kimi membership OAuth through an opaque scoped
  credential lease; no token exposure or persistence
- **Current lifecycle:** one owned ACP child/session; load/resume/import exist
  but cannot redeclare reasoning; cancellation and close join owned protocol,
  process, resource, and credential work
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no installation, Kimi executable launch, OAuth/login
  mutation, account/credential inspection, provider prompt, external inference,
  or paid work. Current official public docs, exact public npm/source
  artifacts, existing fixtures, and secret-free source/unit probes are allowed
  by card 167.
- **Required validation:** card-specific gates plus, if code executes, final
  `cargo fmt -p swallowtail-adapter-kimi`, `effigy validate:focused
  swallowtail-adapter-kimi`, `effigy package:verify-affected
  swallowtail-adapter-kimi`, `effigy check:examples`, `effigy qa:routes`,
  `effigy qa:northstar`, research/logs/roadmaps/g04/batch-card/next-action index
  gates, `effigy package:api`, `effigy doctor`, and `git diff --check`
- **Known doctor baseline:** inherited 378 god-file findings: 332 warnings and
  46 errors; stale graph index; one generated-in-src warning. New tests must be
  focused and must not increase the finding/error counts.
- **Planning validation:** `effigy test --plan`, `effigy qa:docs`, `effigy
  qa:northstar`, research/logs/roadmaps/g04/batch-card/next-action index gates,
  and `git diff --check` passed before the planning commit
- **PR base/head:** current pushed `main` / selected worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** none; do not merge

## Boundaries

Keep this run inside the named runway:

- **In scope:** `crates/swallowtail-adapter-kimi/src/driver/reasoning.rs` and
  exact compatibility selection only as Research 207 requires; Kimi ACP
  reasoning fixtures/tests; package API baseline only if changed;
  `docs/guides/kimi-prepared-integration.md`; feature-matrix note only when
  warranted; Research 207; g04.060; cards 167-169; reserved closeout; triage,
  programme, and sole Next Task; current official Kimi configuration docs;
  exact public Kimi Code npm/source tags inside qualified
  `0.29.0..=0.38.0`; existing secret-free fixtures and local source/unit
  evidence
- **Out of scope:** arbitrary effort strings or raw config maps; display-label
  translation; aliases, clamping, nearest-value fallback, or model-name
  inference; model selection/catalogue publication; load/resume/import/recovery
  reasoning mutation; Kimi headless/local-server/Python CLI/Platform; plan,
  YOLO/AFK, permissions, questions, tools, search, output/context controls,
  subagents, or filesystem widening; live OAuth/login/account work; currentness;
  `CHANGELOG.md`; shared architecture/contracts; release, publication, merge,
  generation rollover, or g04 closure
- The contracts at the planning base are the complete authorization boundary.
  Do not expand or rewrite them. If exact evidence requires a contract or
  shared runtime change, stop for orchestrator review.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, access, security, persistence, or compatibility decision.
- Current official docs are leads. Exact qualified npm/source evidence owns the
  version floor, option construction, selection, fallback, provider
  preservation, confirmation, and lifecycle claims.
- `xhigh` and `max` may prepare only when the current new-session snapshot
  advertises the exact requested value in one valid `thinking` select option.
- The set response must repeat a valid option and confirm the same effective
  `currentValue` before the session is returned. Dispatch is not enough.
- Do not accept arbitrary advertised values. Research 207 must decide whether
  a foreign row can coexist with a known subset or makes the full option
  malformed; implement exactly that disposition.
- Preserve `off|on|low|medium|high`, boolean and always-thinking shapes, and
  omission. Never infer support from a model alias, provider type, display
  label, or default.
- If older qualified releases lack the extended source behavior, split the
  compatibility revision at the exact milestone. Do not project the guarantee
  across the whole maintained segment or onto `UnverifiedNewer`.
- Selection remains new-session-only. Load and resume continue to reject
  reasoning redeclaration before credential/resource/process effects. Import
  and attachment recovery gain no mutation.
- Access, resource, model negotiation, provider-state, cancellation, terminal,
  and joined cleanup truth remain unchanged.
- Default QA must not launch Kimi or mutate provider/account state.
- This handoff represents one worker lane. Do not edit another lane's scope.
  If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.
- Follow repository `AGENTS.md`, the canonical architecture/contracts, and
  glue-light reporting. Work in one meaningful batch and use Effigy selectors.

## Important Context

- **Planning lineage:** Research 006 established Kimi ACP session and
  persistent-lifecycle evidence; Research 086 retained declared-effort behavior
  at `0.31.1`; Research 165 and 179 raised the route through exact `0.37.2` and
  `0.38.0`. g04.059 stopped a separate Deep Agents family.
- **Exact current source lead:** tag
  `@moonshot-ai/kimi-code@0.38.0` peels to commit
  `0999454bdcb5ddd98f39bffee434dcf0a810f394`. Research 179 freezes package and
  release identity. Reverify the artifact/source rather than copying this
  planning observation into promoted evidence.
- **Decisive exact files:**
  `packages/acp-adapter/src/config-options.ts` builds `off` plus each current
  model `supportEfforts`; `model-catalog.ts` derives those rows from effective
  `support_efforts`; `session.ts` validates and applies a chosen declared
  effort. Exact tests under `packages/acp-server/test/config.test.ts` and model
  catalogue suites include extended effort shapes.
- **Version-floor tension:** Research 179 records several selected ACP blobs as
  byte-identical from `0.31.1` through `0.38.0`, but it did not freeze
  `config-options.ts` or `model-catalog.ts` for this question. Audit adjacent
  tags across the qualified range and identify the real first milestone.
- **Current production path:**
  `crates/swallowtail-adapter-kimi/src/driver.rs` parses the new-session
  snapshot, prepares reasoning, sends one set request, confirms the response,
  then returns the handle. `driver/reasoning.rs` owns the restrictive shape.
- **Current fixtures:** `tests/support/agent/reasoning.rs` advertises only
  `off|low|medium|high`; `tests/reasoning_dispatch.rs` covers qualified and
  unverified versions plus mismatch/failure; `tests/reasoning_lifecycle.rs`
  proves load/resume rejection and joined cleanup.
- **Prepared API:** `ReasoningMode` and `SessionOptions` already carry arbitrary
  validated identifiers. A new public effort enum is not expected. Do not add
  one unless Research 207 proves the existing contract cannot express the
  selected surface; that is a stop/review condition, not implied authority.
- **Matrix truth:** the Kimi Code solution already has reasoning `Yes` through
  ACP. Delivery likely changes the guide/note, not the capability cell.
- **Confirmation truth:** `prepare_reasoning_selection` checks snapshot
  membership; `confirm` checks returned `currentValue` through negotiated
  reasoning. Preserve this ordering and do not weaken it to provider
  acceptance or source inference.
- **Lifecycle truth:** load/resume intentionally discard no selection; they
  reject non-empty reasoning through `SessionLifecycleOperation` before host
  effects. The guide already says reasoning is new-session-only.
- **Research outcome:** an honest evidence stop after card 167 is complete work
  if the version floor, provider preservation, or effective confirmation fails.
  Do not force cards 168-169.
- **Report after:** card 167 and Research 207 are complete, then after the
  binding/acceptance batch if the deliver-now set is non-empty
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the
top. Before broad repository reads, run the quick startup worktree-safety
preflight in `## Completion Protocol`. If the current context is a clean,
dedicated, non-`main` registered worktree, use it immediately, record its actual
path/branch, and do not create another worktree because its generated name
differs from this file.

Read `AGENTS.md`, g04.060, cards 167-169, Research 006/086/165/179, the Kimi
guide, system architecture, and Contracts 011/023/029/034/037/040/041/052.
Execute card 167 first. Promote Research 207 with a non-empty exact table or an
honest empty set. Continue automatically only when its gate is satisfied.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe before broad reads: `git rev-parse
   --show-toplevel`, `git branch --show-current`, `git status --porcelain`, and
   `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch. Do not compare it with the placeholders or create a
   second worktree merely because names differ.
3. Only if the current context is `main`, dirty, unregistered, or unusable,
   inspect the named worktree. If that also cannot be used, read
   `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`; ask the
   operator if absent. Create a unique worktree/branch there from pushed
   `origin/main`. Never use `/tmp`, `TMPDIR`, or a guessed path. Never clean,
   reset, stash-over, or discard another checkout. If the launcher supplied a
   dirty or `main` worktree, stop and report it instead of silently creating a
   second worktree.
4. From the selected worktree, confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`, confirm `git merge-base --is-ancestor
   cbc5ae7d518d12c37904cea792094c1fc178ccc5 HEAD` succeeds, and confirm this
   handoff file exists in selected `HEAD`.
5. Read the active milestone, assigned cards, `AGENTS.md`, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; record the
   inherited doctor baseline and do not run the full planned workspace suite.

### While you work

- Execute cards 167-169 in order. Stop after card 167 when Research 207 is
  empty or a named gate fails. An evidence stop is a complete worker outcome.
- Keep commits aligned with meaningful chunks, not arbitrary model turns.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into new architecture.

### When the assigned runway is complete

1. Run the card-specific final validation. If code executes, run
   `cargo fmt -p swallowtail-adapter-kimi`, `effigy validate:focused
   swallowtail-adapter-kimi`, `effigy package:verify-affected
   swallowtail-adapter-kimi`, `effigy check:examples`, `effigy qa:routes`,
   `effigy qa:northstar`, the relevant docs index gates, `effigy package:api`,
   `effigy doctor`, and `git diff --check`. If card 167 stops with docs only,
   run its named focused/docs/diff gates and record why code-only gates did not
   apply.
2. Update Research 207, milestone/cards, Kimi guide/matrix note only as
   warranted, reserved closeout, programme, triage, and sole Next Task. Keep
   g04 open.
3. Push the selected worker branch.
4. Open one reviewable PR against the current pushed `main` tip. The planning
   base above predates this handoff commit and is intentionally not
   self-referential.
5. In the PR body, link the milestone, cards, Research 207, changed surfaces,
   exact evidence, validation, and unresolved items.
6. Report the PR URL and exact head SHA to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and
checks. Current review state: awaiting worker evidence and PR.

The orchestrator and worker may share one GitHub identity. Formal self-approval
is then unavailable; the orchestrator posts the evidence-backed verdict as a
PR comment. If changes are requested, make only those changes on this branch,
push again, and report back through the operator. Requested changes: none yet.
The operator must explicitly authorise any merge.

- **Closeout refs:** Research 207; g04.060; cards 167-169; reserved g04.060
  closeout; Kimi guide/matrix note; triage; programme; sole Next Task
- **Merge conditions:** exact Research 207 deliver-now truth; all executed cards
  complete; required gates green; PR head reviewed; no unresolved drift,
  authority, compatibility, or confirmation issue; explicit operator merge
  command
- **After merge:** fast-forward only from the exact reviewed green head, then
  complete the post-merge closeout on `main`. Keep g04 open and reassess the
  remaining per-route inventory unless the operator supplies a different next
  direction.
