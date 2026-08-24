---
title: g04.058 Antigravity headless agent-profile worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-24
updated: 2026-08-24
planning_base: 4ef1cc875ce30fbb3890f2f68d631083497c3238
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260824-224931-g04-058-antigravity-headless-agent-profile.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator reassessed the remaining promoted per-route feature inventory
after g04.057 and selected exact `antigravity.headless` agent-profile
selection. g04.058 is compiled. Implementation has not started. The ready
runway begins with exact listing, id, dispatch, fallback, init-confirmation,
authority, and lifecycle evidence; cards 162-163 are conditional on a non-empty
Research 205 deliver-now set.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. Start from this file without a copied transcript or a
second prompt. Do not create internal subagents or parallel worker lanes; the
operator's harness owns dispatch.

## Why It Matters

The route already selects exact model, optional effort and schema, resource
access, isolation, deadline, and exact-id continuation. Exact qualified CLI
help and current official `1.1.17` docs also expose `agy agents`, `--agent`, and
selected stream-JSON `init.agent`. That could support an exact route-local
profile selection instead of leaving a real per-route feature inaccessible.

It is not yet safe to bind. Agent ids may be custom, account-visible,
settings-backed, unavailable, or capable of changing instructions and tools.
The evidence card must prove bounded identity, no silent fallback, exact init
confirmation, and composition with the existing immutable authority. An empty
deliver-now set is a successful outcome when those facts cannot be frozen.

## Current State

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `4ef1cc875ce30fbb3890f2f68d631083497c3238`
- **Pushed main verification:** local `HEAD` and `origin/main` both resolved to
  `4ef1cc875ce30fbb3890f2f68d631083497c3238` before this handoff commit
- **Planning checkout:** clean after the planning commit and push
- **Planning artifacts included at the base:** g04.058, cards 161-163,
  Research 205 reservation, compilation log, route-local closeout reservation,
  inventory reassessment, and the sole Next Task
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight
- **Worker branch:** `agent/g04-058-antigravity-agent-profile-20260824-224931`
- **Worker worktree:** `/Users/tom/Dev/worktrees/swallowtail-g04-058-antigravity-agent-profile-20260824-224931`
- **Worktree creation command:** `git worktree add /Users/tom/Dev/worktrees/swallowtail-g04-058-antigravity-agent-profile-20260824-224931 -b agent/g04-058-antigravity-agent-profile-20260824-224931 origin/main`
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
  `docs/roadmaps/g04/058-antigravity-headless-agent-profile-selection.md`
- **Ready cards, in order:** card 161, then conditional card 162, then
  conditional card 163
- **Allowed runway:** exact Antigravity agent-profile evidence, then only
  Research 205 deliver-now prepared binding and route-local acceptance
- **Remaining card budget:** three serial cards; cards 162-163 run only after
  their named evidence and implementation gates
- **Dispatch topology:** one serial worker lane. Do not use internal subagents;
  report through the operator's harness.
- **Parallel safety check:** cards share prepared inputs, plan/request
  agreement, command construction, event parsing, continuation children,
  fixtures, guide, research, and closeout; they are not parallel-safe
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  011, 023, 029, 033, 037, 040, 041, and 052
- **Route identity:** `antigravity.headless`, driver
  `swallowtail.antigravity.headless`, axis `antigravity-cli.release`, exact
  qualified versions `1.1.9..=1.1.17`, later stable `UnverifiedNewer`
- **Current mapping:** command always selects exact `--model`; Read adds
  `--mode plan`; ProviderEnforced adds `--sandbox`; optional effort and schema
  have typed mappings; exact-id continuation adds `--conversation`; no
  `--agent` is emitted
- **Current init validation:** exact model, `permission_mode=request-review`,
  array-shaped tools, and string cwd; `init.agent` is not inspected
- **Model capability profile:** explicit-model headless structured run plus
  read-only exact-id continuation; no fallback; selected profile must stay
  route-local and cannot grant extra resource, permission, tool, isolation,
  subagent, account, or provider-session authority
- **Tool/runtime restrictions:** use Effigy selectors; no internal subagents or
  parallel worker lanes; no installation, login, account/config/profile
  mutation, credential or account-identity capture, authenticated provider
  prompt, external inference request, or paid work. Current official public
  docs, exact public artifacts/source, existing fixtures, and secret-free
  promptless help/listing are allowed by card 161.
- **Required validation:** card-specific gates plus, if code executes, final
  `cargo fmt -p swallowtail-adapter-antigravity`, `effigy validate:focused
  swallowtail-adapter-antigravity`, `effigy package:verify-affected
  swallowtail-adapter-antigravity`, `effigy check:examples`, `effigy qa:routes`,
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

- **In scope:** `crates/swallowtail-adapter-antigravity/**` for exact agent-id
  value, prepared run/session inputs, capability/constraint and plan/request
  agreement, one `--agent <id>` mapping, init confirmation, continuation-child
  propagation, safe failures, fixtures, examples, and package API baseline;
  `docs/guides/antigravity-prepared-integration.md`; Research 205; g04.058;
  cards 161-163; the reserved g04.058 route-local closeout; current official
  Antigravity CLI docs; exact public artifacts/source; existing secret-free
  fixtures and promptless listing evidence
- **Out of scope:** a generic provider settings map; arbitrary unbounded
  strings; profile definition/body, display-label translation, instructions,
  tools, files, paths, creation, mutation, import, deletion, or persistence;
  account/login work; model catalogue changes; addable-route changes; model,
  effort, schema, access, sandbox, permission, tool, subagent, background-task,
  or session expansion; `--dangerously-skip-permissions`; ambient `--continue`;
  provider fallback; another Antigravity route/product; Gemini; currentness;
  `CHANGELOG.md`; shared architecture/contracts; release, publication, merge,
  generation rollover, or g04 closure
- The contracts at the planning base are the complete authorization boundary.
  Do not expand or rewrite them. If exact evidence requires a contract or
  shared runtime change, stop for orchestrator review.
- Do not invent architecture, widen the roadmap, or choose an unresolved
  product, API, access, security, persistence, or compatibility decision.
- Do not treat `agy agents` output as a model catalogue or agent-definition
  API. Freeze only the bounded id evidence necessary for selection.
- Do not treat dispatch as acceptance or acceptance as effective selection.
  A delivered selection needs exact `init.agent` equality before output is
  accepted. Missing or foreign confirmation fails closed.
- Caller omission must preserve existing argv and behavior. Do not infer an
  ambient/default agent identity from omission.
- An agent profile does not grant additional resource, permission, tool,
  isolation, subagent, account, or provider-session authority. If exact
  composition cannot retain the prepared boundaries, stop with an empty set.
- Structured-run selection is operation-private. Continuation may carry one
  immutable id only if Research 205 proves exact reassertion and confirmation
  on every first, resumed, and fresh-replacement child.
- Failure and cancellation join every owned process/task and preserve current
  conversation advancement rules. Do not claim provider-state deletion.
- This handoff represents one worker lane. Do not edit another lane's scope.
  If shared mutable scope or a hidden dependency appears, stop and report it.
- Work only in the selected clean worker worktree. Never edit the
  orchestrator's planning checkout or clean, reset, stash over, or discard an
  unrelated checkout's state.
- Do not merge the PR. Merge remains a separate operator-authorised action.
- Follow repository `AGENTS.md`, the canonical architecture/contracts, and
  glue-light reporting. Work in one meaningful batch and use Effigy selectors.

## Important Context

- **Planning lineage:** Research 079 qualified exact headless stream JSON;
  Research 080 qualified exact-id continuation; Research 177 extended the
  unchanged selected route through `1.1.17`. g04.030 is the currentness base.
- **Official lead:** current `1.1.17` headless docs say `agy agents` lists
  available profiles, `--agent` selects one, and `init.agent` appears only when
  selected. Exact `1.1.9` help already names `--agent`, `agent`, and `agents`.
- **Why these cards are ready:** the candidate field is on the selected
  transport, the route owns its argv and init decoder, exact currentness is
  frozen, and confirmation has a named early stream location.
- **Existing command order:** `--print <prompt> --output-format stream-json
  --model <id>`, then plan/sandbox/effort/schema/conversation additions. Card
  162 owns exact placement only after Research 205 admits a row.
- **Current parser truth:** init already fails closed on wrong model or
  permission mode. It accepts any array-shaped tools and does not publish raw
  init data. Agent validation belongs beside these checks, not in output
  inference.
- **Continuation truth:** one prepared read-only ambient session starts one
  joined child per turn, privately carries only the last clean conversation id,
  and does not advance after failure/cancellation/mismatch. Profile selection
  must not weaken that rule.
- **Decision preference:** a narrow route-local `AntigravityAgentProfileId`-like
  value is acceptable only if Research 205 can bound it. Do not expose profile
  definitions or a generic setting bag.
- **Open tensions:** `agy agents` may be account/custom/config dependent;
  invalid agents may fail before init; profile choice may change tools; current
  fixtures lack `init.agent`; continuation may inherit or reject reassertion.
  Card 161 must settle each explicitly.
- **Report after:** card 161 and Research 205 are complete, then after the
  binding/acceptance batch if the deliver-now set is non-empty
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the
top. Before broad repository reads, run the quick startup worktree-safety
preflight in `## Completion Protocol`. If the current context is a clean,
dedicated, non-`main` registered worktree, use it immediately, record its actual
path/branch, and do not create another worktree because its generated name
differs from this file.

Read `AGENTS.md`, g04.058, cards 161-163, Research 079/080/177, the Antigravity
guide, system architecture, and Contracts 011/023/029/033/037/040/041/052.
Execute card 161 first. Promote Research 205 with a non-empty exact table or an
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
4. Confirm selected-worktree `HEAD == origin/main`, confirm `git merge-base
   --is-ancestor 4ef1cc875ce30fbb3890f2f68d631083497c3238 HEAD`, and confirm
   this handoff file exists in selected `HEAD`.
5. Read the milestone, cards, `AGENTS.md`, and canonical refs.
6. Run the repo's cheap orientation checks and record what actually ran.

### While you work

- Execute cards in order. Keep commits aligned with meaningful evidence and
  implementation chunks.
- After card 161, report changed files, evidence, validation, deliver-now set,
  remaining cards, risks, and blockers through the operator.
- Stop if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into new architecture.

### When the assigned runway is complete

1. Run the required final validation named in Current State.
2. Update Research 205, cards, milestone, closeout, guide/matrix when warranted,
   and honest next-task state. Record actual worktree/branch when relevant.
3. Push the selected worker branch.
4. Open one reviewable PR against current pushed `main`. The planning base is
   the pre-handoff commit, not a self-referential handoff hash.
5. Link the milestone, cards, changed surfaces, evidence, validation, and
   unresolved items in the PR body.
6. Report the PR URL and approved evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against canonical refs, diff, and checks.
Current review state: awaiting worker evidence and PR. Shared GitHub identity
may prevent formal approval; in that case the orchestrator's PR comment is the
canonical review record. Requested changes: none yet. The operator must
explicitly authorize merge.

- **Closeout refs:** Research 205; g04.058; cards 161-163; reserved g04.058
  closeout; research/log/roadmap/g04/batch-card indexes; sole Next Task

### Handoff closeout

Before calling the runway complete, leave research, cards, roadmap, log, and
next-task state honest. If evidence blocks delivery, record the empty set and
block cards 162-163 instead of making the handoff look more complete.
