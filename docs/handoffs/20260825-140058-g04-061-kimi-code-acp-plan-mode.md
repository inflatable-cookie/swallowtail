---
title: g04.061 Kimi Code ACP plan-mode worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-25
updated: 2026-08-25
planning_base: 492e57faac552ea1caa9f801a72a59e7f404019b
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260825-140058-g04-061-kimi-code-acp-plan-mode.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reconciled the g04.060 merge, reassessed the remaining
per-route feature inventory, and selected negotiated plan mode on
`kimi-code.acp`. g04.061 is compiled. Implementation has not started. The
ready runway begins with exact version/source milestone evidence; cards
171-172 are conditional on a non-empty Research 208 deliver-now set.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. Start from this file without a copied transcript or a
second prompt. Do not create internal subagents or parallel worker lanes; the
operator's harness owns dispatch.

## Why It Matters

Production route `kimi-code.acp` already consumes the bounded ACP
`configOptions` snapshot, selects one reasoning value, and requires effective
response confirmation before returning a ready new session. Swallowtail also
already owns the typed portable `HarnessMode::Plan`, immutable
`HarnessModeSelection`, and Contract 034 negotiated-option boundary.

Exact official Kimi Code `0.38.0` source builds a `mode` select option with
`default|plan|auto|yolo`. Its exact dispatcher maps `plan` to plan mode plus
manual permission and rebuilds the option snapshot after selection. The
missing feature is a route-local typed binding, not raw provider configuration,
permission widening, or a new isolation claim. The exact version floor,
effective confirmation path, reasoning composition, and cleanup behavior still
need proof before production admission.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `492e57faac552ea1caa9f801a72a59e7f404019b`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `492e57faac552ea1caa9f801a72a59e7f404019b` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Planning artifacts included at the base:** g04.061, cards 170-172,
  Research 208 reservation, compilation log, route-local closeout reservation,
  g04.060 merge reconciliation, inventory reassessment, programme boundary,
  and the sole active Next Task
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker branch:** `agent/g04-061-kimi-acp-plan-mode-20260825-140058`
- **Worker worktree:**
  `/Users/tom/Dev/worktrees/swallowtail-g04-061-kimi-acp-plan-mode-20260825-140058`
- **Worktree creation command:** `git worktree add
  /Users/tom/Dev/worktrees/swallowtail-g04-061-kimi-acp-plan-mode-20260825-140058
  -b agent/g04-061-kimi-acp-plan-mode-20260825-140058 origin/main`
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
  `docs/roadmaps/g04/061-kimi-code-acp-plan-mode.md`
- **Ready cards, in order:** card 170, then conditional card 171, then
  conditional card 172
- **Allowed runway:** exact Kimi ACP plan-mode milestone evidence, then only
  Research 208-admitted typed binding and route-local acceptance
- **Remaining card budget:** three serial cards; cards 171-172 run only after
  their named evidence and implementation gates
- **Dispatch topology:** one serial worker lane. Do not use internal subagents;
  report through the operator's harness.
- **Parallel safety check:** cards share exact source evidence, compatibility
  segmentation, mode/reasoning ordering, fixtures, guide, research, and
  closeout; they are not parallel-safe
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  012, 017, 023, 029, 034, 037, 040, 041, and 052
- **Route identity:** `kimi-code.acp`, driver `swallowtail.kimi.acp`, axis
  `kimi-code.executable`, ACP v1 over stdio
- **Qualified versions:** deprecated exact `0.28.1`; maintained
  `0.29.0..=0.38.0`; later stable points visible as `UnverifiedNewer`
- **Current compatibility behavior:** `0.28.1`
  `kimi.acp.reasoning.legacy-select-v1`; `0.29.0..=0.38.0`
  `kimi.acp.reasoning.declared-effort-v2`
- **Current reasoning mapping:** new-session-only portable
  `off|on|low|medium|high|xhigh|max` where admitted by exact behavior; one
  valid `thinking` select snapshot; one set request; response snapshot
  confirmation; load/resume redeclaration rejected before host effects
- **Current plan-mode gap:** Kimi prepared-session validation accepts only
  developer-message, tool, and admitted reasoning options. Any harness-mode
  request is currently unsupported; the route advertises no harness-mode
  capability and sends no `mode` selection request.
- **Current access:** delegated Kimi membership OAuth through an opaque scoped
  credential lease; no token exposure or persistence
- **Current lifecycle:** one owned ACP child/session; load/resume/import exist
  but cannot redeclare negotiated reasoning; cancellation and close join owned
  protocol, process, resource, and credential work
- **Current isolation:** `AmbientHost`; provider plan/manual-permission state
  does not establish process or filesystem containment
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no installation, Kimi executable launch, OAuth/login
  mutation, account/credential inspection, provider prompt, external inference,
  or paid work. Current official public docs, exact public npm/source
  artifacts, existing fixtures, and secret-free source/unit probes are allowed
  by card 170.
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

- **In scope:** Kimi ACP prepared-session option validation, immutable plan,
  driver setup, exact compatibility selection, and diagnostics only as
  Research 208 requires; Kimi ACP mode/reasoning fixtures and tests; package API
  baseline only if changed; `docs/guides/kimi-prepared-integration.md`; feature
  matrix only when warranted; Research 208; g04.061; cards 170-172; reserved
  closeout; triage, programme, and sole Next Task; current official Kimi
  configuration docs; exact public Kimi Code npm/source tags inside qualified
  `0.29.0..=0.38.0`; existing secret-free fixtures and local source/unit
  evidence
- **Out of scope:** provider `default`, `auto`, or `yolo` as public selections;
  arbitrary option ids/values or raw config maps; display-label translation;
  aliases, fallback, or current-value inference; automatic approval, permission
  widening, callback authority, or tool-policy changes; provider- or
  host-enforced isolation claims; model selection; load/resume/import/recovery
  harness-mode mutation; Kimi headless/local-server, Python `kimi-cli`, Kimi
  Platform, or sibling routes; live OAuth/login/account work; currentness;
  `CHANGELOG.md`; shared architecture/contracts; release, publication, merge,
  generation rollover, or g04 closure
- The contracts at the planning base are the complete authorization boundary.
  Do not expand or rewrite them. If exact evidence requires a contract or
  shared runtime change, stop for orchestrator review.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, access, security, persistence, or compatibility decision.
- Current official docs are leads. Exact qualified npm/source evidence owns the
  version floor, option construction, request mapping, SDK application,
  fallback, permission, confirmation, and lifecycle claims.
- Only portable `HarnessMode::Plan` is a delivery candidate. Provider
  `default|auto|yolo` may coexist only as Research 208 permits; they never
  become public Swallowtail selections.
- A new session may prepare plan mode only when its immutable plan contains the
  exact portable plan constraint and the current provider snapshot contains one
  valid `mode` select option with exact `plan` membership.
- The route may send only the exact qualified selection request. The response
  or correlated update must expose a valid refreshed option whose effective
  `currentValue` is exactly `plan` before readiness. Dispatch is not enough.
- Research 208 must prove the exact provider application, including
  `setPlanMode(true)` and manual permission. Stop if the mapping falls back,
  substitutes another permission posture, or cannot be confirmed effectively.
- Caller omission must preserve the current wire and send no mode request.
- Plan plus reasoning requires two exact capability constraints, two prepared
  selections, separate requests, and separate effective confirmations. Research
  208 owns request ordering and joined failure cleanup. One confirmation never
  proves the other.
- Preserve every Research 207-admitted reasoning value applicable to the exact
  version. Do not alter model, access, resource, first-prompt, retention,
  callback, cancellation, deadline, terminal, or cleanup truth.
- If older qualified releases lack the complete mode behavior, split the
  compatibility revision at the exact milestone. Do not project the guarantee
  across the whole maintained segment or onto `UnverifiedNewer`.
- Selection remains new-session-only. Load and resume must reject harness-mode
  redeclaration before credential/resource/process effects. Import and recovery
  gain no mutation path.
- Manual permission and `AmbientHost` are independent truths. Never report
  plan mode as sandboxing or process/filesystem containment.
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
  `0.38.0`; Research 207 admitted `xhigh|max` through g04.060. PR 59
  fast-forwarded that exact reviewed work to `main` at `dc191750`.
- **Exact current source lead:** tag `@moonshot-ai/kimi-code@0.38.0` peels to
  commit `0999454bdcb5ddd98f39bffee434dcf0a810f394`. Research 179 freezes
  package and release identity. Reverify the artifact/source rather than
  copying this planning observation into promoted evidence.
- **Decisive exact files:** `packages/acp-adapter/src/modes.ts` defines four
  provider modes and maps `plan` to plan plus manual permission;
  `config-options.ts` builds the mode select; `server.ts` and `session.ts` own
  set dispatch and provider application; exact
  `set-session-config-option.test.ts` covers selection and refreshed snapshot
  truth. Freeze exact paths and digests in Research 208.
- **Version-floor tension:** the exact `0.38.0` source proves a candidate point,
  not the maintained range. Audit adjacent qualified tags and locate both the
  first complete milestone and its immediately preceding boundary.
- **Current production path:** `crates/swallowtail-adapter-kimi/src/driver.rs`
  consumes the new-session option snapshot and dispatches negotiated reasoning;
  `driver/validation.rs` rejects non-reasoning session options;
  `prepared_profile/session.rs` validates preparation and attachment rules;
  `selection.rs` owns exact compatibility behavior.
- **Current fixtures:** `tests/support/agent.rs` and included reasoning support
  emulate config-option snapshots and set requests; `tests/reasoning_dispatch.rs`
  covers qualified/unverified versions and mismatch/failure;
  `tests/reasoning_lifecycle.rs` proves attachment rejection and joined cleanup.
- **Prepared API:** `HarnessMode::Plan`, `HarnessModeSelection`, and
  `SessionOptions::with_harness_mode` already exist in the shared typed surface.
  A generic provider-option API is neither required nor authorized.
- **Matrix truth:** Kimi Code already has reasoning through ACP. Plan-mode
  delivery may warrant a focused feature note, but it must not imply broader
  provider configuration or isolation.
- **Confirmation truth:** current reasoning preparation checks snapshot
  membership and its confirmation checks returned effective value. Plan mode
  must preserve that distinction; provider acceptance or source inference is
  insufficient.
- **Lifecycle truth:** existing load/resume option restrictions run before host
  effects. Extend only the typed harness-mode rejection needed to preserve that
  boundary; do not replay provider state on attachment.
- **Research outcome:** an honest evidence stop after card 170 is complete work
  if the version floor, provider application, effective confirmation, reasoning
  composition, or cleanup gate fails. Do not force cards 171-172.
- **Report after:** card 170 and Research 208 are complete, then after the
  binding/acceptance batch if the deliver-now set is non-empty
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the
top. Before broad repository reads, run the quick startup worktree-safety
preflight in `## Completion Protocol`. If the current context is a clean,
dedicated, non-`main` registered worktree, use it immediately, record its actual
path/branch, and do not create another worktree because its generated name
differs from this file.

Read `AGENTS.md`, g04.061, cards 170-172, Research
006/086/165/179/207, the Kimi guide, system architecture, and Contracts
012/017/023/029/034/037/040/041/052. Execute card 170 first. Promote Research
208 with a non-empty exact table or an honest empty set. Continue automatically
only when its gate is satisfied.

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
   492e57faac552ea1caa9f801a72a59e7f404019b HEAD` succeeds, and confirm this
   handoff file exists in selected `HEAD`.
5. Read the active milestone, assigned cards, `AGENTS.md`, and canonical refs.
6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`; record the
   inherited doctor baseline and do not run the full planned workspace suite.

### While you work

- Execute cards 170-172 in order. Stop after card 170 when Research 208 is
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
   `effigy doctor`, and `git diff --check`. If card 170 stops with docs only,
   run its named focused/docs/diff gates and record why code-only gates did not
   apply.
2. Update Research 208, milestone/cards, Kimi guide/matrix only as warranted,
   reserved closeout, programme, triage, and sole Next Task. Keep g04 open.
3. Push the selected worker branch.
4. Open one reviewable PR against the current pushed `main` tip. The planning
   base above predates this handoff commit and is intentionally not
   self-referential.
5. In the PR body, link the milestone, cards, Research 208, changed surfaces,
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

- **Closeout refs:** Research 208; g04.061; cards 170-172; reserved g04.061
  closeout; Kimi guide/matrix only as warranted; triage; programme; sole Next
  Task
- **Merge conditions:** exact Research 208 deliver-now truth; all executed
  cards complete; required gates green; PR head reviewed; no unresolved drift,
  authority, compatibility, permission, composition, or confirmation issue;
  explicit operator merge command
- **After merge:** fast-forward only from the exact reviewed green head, then
  complete the post-merge closeout on `main`. Keep g04 open and reassess the
  remaining per-route inventory unless the operator supplies a different next
  direction.
