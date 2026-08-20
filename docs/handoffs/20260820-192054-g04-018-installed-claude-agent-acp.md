---
title: g04.018 installed Claude Agent ACP worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-20
updated: 2026-08-20
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260820-192054-g04-018-installed-claude-agent-acp.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

Hosted DeepSeek continuation is on `main`. Research 170 mapped Claude
Agent ACP as the second installed proof: a prepared facade, discovery,
and 029/032 classification already exist. A consumer still cannot list
or admit that route through Contract 057. Local subscription is
inherited login state, not hosted URL-open OAuth.

This is the handoff from the planning/orchestrator thread to one bounded
implementation thread. It is written so the worker can start from this file
without needing a copied transcript or a second prompt.

## Why It Matters

Without an adapter-local installed descriptor, the second-proof
programme has only hosted API-key coverage. Claude Agent ACP is the
installed analog of Codex app-server. Tokens must not enter portable
records. Do not reclassify this as hosted OAuth.

## Current State

Here is the state the worker is inheriting:

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Planning branch:** `main`
- **Planning base commit:** `fa33de150339ee203eff676b29f855e26f3fcfa6`
- **Pushed main verification:** run `git fetch origin`, then confirm local
  `HEAD == origin/main`; the current tip contains this handoff file after the
  later handoff commit. The recorded planning base above is the planning
  commit *before* this file existed.
- **Planning checkout:** clean on `main` after the planning commit; this
  handoff is a follow-up commit on the same branch
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates the worker-only worktree preflight.
- **Planning artifacts included at the base:** Contract 057; Research 170;
  realized Codex first-proof and DeepSeek second hosted proof; completed
  g04.016; milestone g04.018; ready cards 050-052
- **Worker branch:** `g04-018-installed-claude-agent-acp`
- **Worker worktree:** launcher-provided dedicated worktree first. Manual
  fallback path is
  `$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-018-installed-claude-agent-acp`
- **Worktree creation command:** only if the current context is unusable and
  `.agents.local.env` defines `AGENTS_WORKTREE_CONTAINER_DIR`:
  `git worktree add -b g04-018-installed-claude-agent-acp "$AGENTS_WORKTREE_CONTAINER_DIR/swallowtail-g04-018-installed-claude-agent-acp" origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these handoff placeholders. Record the actual
  path/branch and never create a second worktree for that reason. If the
  current context is unusable, use the named worktree when it matches; only
  then read `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and
  create a unique manual worktree/branch under that container from
  `origin/main`. Ask the operator first if the file or key is absent; never
  use `/tmp`, `TMPDIR`, or a guessed path.
- **Active spec lane:** none. Contract 057 is the authority. Research 170 is
  evidence, not a contract.
- **Roadmap milestone:** `docs/roadmaps/g04/018-installed-claude-agent-acp.md`
- **Ready cards, in order:**
  `docs/roadmaps/g04/batch-cards/050-claude-agent-acp-addable-descriptor.md`,
  then `docs/roadmaps/g04/batch-cards/051-claude-agent-acp-admission-and-prepare.md`,
  then `docs/roadmaps/g04/batch-cards/052-claude-agent-acp-refresh-update-and-subject.md`
- **Allowed runway:** g04.018 cards 050 → 051 → 052. Stop after refresh,
  update observation, and subject. Do not start llama.cpp or hosted OAuth.
- **Remaining card budget:** three cards, one PR
- **Dispatch topology:** serial; one worker, one worktree, one PR
- **Parallel safety check:** admission and refresh write the same admitted
  Claude Agent record. No parallel lane.
- **Canonical refs:** Contract 057; Contracts 011, 014, 029, 032, 037, 047;
  Research 170; `claude_agent_acp_descriptor`; `claude_agent_acp_claim`;
  `prepare_claude_agent`; `AddableRouteDescriptor`; `admit_instance`;
  `refresh_readiness`; `observe_instance_update`;
  `observe_authenticated_subject`
- **Model capability profile:** capable coding model, medium reasoning
- **Tool/runtime restrictions:** no live provider, install, login, or
  billing work. No hosted OAuth. No DeepSeek, Codex, or llama.cpp
  descriptor edits. No OpenHands production route. Do not invent a
  catalogue `provider_id`. Do not extract keychain bytes. Do not advertise
  `claude-code.headless` or `claude-code.response-only`. No 047 snapshot
  field additions. No GitHub Release, crates.io, or tag mutation. Do not
  rewrite `release-baselines/public-api-0.3.3/`.
- **Required validation:** card 050:
  `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-runtime`,
  `git diff --check`. Card 051:
  `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-runtime swallowtail-host-local`,
  `git diff --check`. Card 052:
  `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-runtime swallowtail-testkit`,
  `git diff --check`. If public types are added, update
  `release-baselines/public-api-unreleased/` and run `effigy package:api`
  before opening the PR.
- **PR base/head:** `main` / selected worker branch
  (`g04-018-installed-claude-agent-acp` unless the launcher supplied a
  different dedicated branch)
- **PR URL:** pending
- **Review state:** awaiting worker PR
- **Merge authorisation:** not granted; merge is a separate operator-authorised
  action

## Boundaries

Please keep this run inside the named runway:

- **In scope:** adapter-local installed addable descriptor for
  `claude-agent.acp`; opaque binary-path and env config fields; 057
  admission of the local subscription profile with no `CredentialRef`;
  reuse of `prepare_claude_agent`; host-supplied refresh; 029/032 update
  observation; subject Absent; unmarked overlay.
- **Out of scope:** hosted interactive OAuth; `claude-code.headless`;
  `claude-code.response-only`; API-key billing as this addable row;
  llama.cpp; OpenHands production wiring; inventing a catalogue
  `provider_id`; adding overlay metadata to 047; rewriting
  `public-api-0.3.3`; GitHub Release; crates.io; tag mutation.
- Do not invent architecture or change Contract 057.
- Topology is installed. Do not fold it into `ExecutionLayer`.
- Additive public API belongs in `release-baselines/public-api-unreleased/`.
- This handoff represents one worker lane. Do not edit another lane's
  assigned scope; if shared mutable scope or a hidden dependency appears,
  stop and report it through the operator.
- Work only in the selected clean worker worktree: prefer the current
  launcher-provided worktree and record its actual path/branch; otherwise use
  the named fallback created by the startup preflight. Never edit the
  orchestrator's planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Planning lineage:** first-proofs, Contract 052 consumer path, DeepSeek
  hosted API-key (PR 13), hosted OAuth gate held, Research 170. Claude
  Agent ACP is the next installed shape.
- **Why these cards are ready:** Research 170 mapped the prepared facade
  and dual access profiles onto 057. The addable row is subscription-only,
  matching Codex ChatGPT. Hosted OAuth stays gated.
- **Decisions and preferences:** do not extract keychain bytes. Do not
  advertise the API-key profile on this row. Do not treat local
  subscription as hosted URL-open OAuth.
- **Open tensions:** `prepare_claude_agent` still takes host executable
  and environment refs, not stored `ConfigFieldRef` values. Session-
  negotiated models have no catalogue `provider_id`; leave them unmarked.
  There is no `.agents.local.env` on the planning machine; if the launcher
  does not supply a worktree, ask the operator for
  `AGENTS_WORKTREE_CONTAINER_DIR`.
- **Report after:** card 050 descriptor; card 051 admission and prepare;
  card 052 refresh/update/subject and the PR
- **Report to:** the operator, who will relay progress to the orchestrator

## Suggested Next Move

This handoff explicitly activates worker mode. Start by reading it from the top.
Before broad repository reads, run the quick startup worktree-safety preflight in
`## Completion Protocol`. If the
current context is a clean, dedicated, non-`main` registered worktree, it is the
launcher-provided worktree: use it immediately, record its actual path/branch,
and do not compare its generated path/branch with this handoff or create another
worktree. If it is `main`, dirty, unregistered, or otherwise unusable, use the
named worktree if it matches; only then read `.agents.local.env`, require a valid
`AGENTS_WORKTREE_CONTAINER_DIR`, ask the operator if it is absent, and create a
unique manual worktree and branch under that container from pushed `origin/main`.
Never fall back to `/tmp` or `TMPDIR`. Do not run broad repo orientation before
this decision. Read `AGENTS.md`, the active milestone, each assigned card, and
the canonical architecture/contracts from the selected worker worktree.

Once that checks out, take card 050 first. Ship the installed Claude Agent
ACP addable descriptor. When 050 is green, continue into 051, then 052.
When refresh, update observation, and subject are green, open the PR and
stop.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Then run one
   quick read-only safety probe before
   broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch and do not compare them with the placeholders above
   or create another worktree merely because they differ.
3. Only if the current context is `main`, dirty, unregistered, or otherwise
   unusable should you inspect the named worktree. If that also cannot be used,
   read `.agents.local.env` and require `AGENTS_WORKTREE_CONTAINER_DIR`; if it is
   absent, ask the operator before creating the file or worktree. Then create a
   unique worktree and branch under that container from pushed `origin/main`,
   record the actual path and branch, and run all subsequent commands there.
   Never use `/tmp`, `TMPDIR`, or a guessed path; never clean, reset, stash-over,
   or discard the original checkout's dirty state. If the launcher supplied a
   dirty or `main` worktree, stop and report it instead of silently creating a
   second worktree.
4. From the selected worktree, confirm `git rev-parse HEAD` equals
   `git rev-parse origin/main`, confirm
   `git merge-base --is-ancestor fa33de150339ee203eff676b29f855e26f3fcfa6 HEAD`
   succeeds, and confirm this handoff file exists in the selected `HEAD`.
5. Read the active milestone, assigned cards, `AGENTS.md`, and canonical refs.
6. Run the repo's cheap orientation checks and record what you actually ran.

### While you work

- Execute the ready cards in order and keep commits aligned with meaningful
  chunks, not arbitrary model turns.
- After each meaningful chunk, report through the operator with changed files,
  validation actually run, remaining cards, new risks, and blockers.
- Stop and say so if a contract is missing, intent is ambiguous, scope expands,
  authority/access is missing, or validation changes the plan.
- Do not quietly turn an open question into a new architecture.
- Do not start llama.cpp or hosted OAuth work.

### When the assigned runway is complete

1. Run the required final validation for card 052, plus `effigy package:api`
   if public types were added.
2. Update the card/log evidence required by the runway, including the actual
   worktree and branch if the temporary fallback was used.
3. Push the selected worker branch (the fallback branch if one was created).
4. Open a reviewable PR against the current pushed `main` tip. The handoff's
   planning base `fa33de150339ee203eff676b29f855e26f3fcfa6` is the planning
   commit before the handoff was created, not a self-referential hash for the
   commit that contains this file.
5. In the PR body, link the milestone, cards 050-052, Contract 057, Research
   170, changed surfaces, evidence, validation, and unresolved items.
6. Report the PR URL and the evidence to the operator. Do not merge.

### Review and merge path

The orchestrator will review the PR against the canonical refs, diff, and checks.
Current review state: awaiting worker PR.

The orchestrator records an evidence-backed verdict in the provider's review
surface. When the orchestrator and worker share a GitHub identity, formal
self-approval is unavailable, so the orchestrator posts the verdict as a PR
comment; that comment is the canonical review record. If changes are requested,
make only those changes on this branch, push again, and report back through the
operator. Requested changes are: none yet. The PR should
link the card, milestone, spec, changed surfaces, evidence, validation, and
unresolved items. The operator must explicitly authorise any merge.

- **Closeout refs:** cards 050-052, g04.018, `docs/roadmaps/README.md`,
  `docs/logs/README.md`

### Handoff closeout

Before calling the runway complete, leave the card, roadmap, log, and next-task
state honest. If the work is blocked, record the blocker and stop rather than
making the handoff look more complete than it is. After the PR lands, the
orchestrator will return to the operator for merge. Hosted OAuth stays a
remaining gate. llama.cpp attached stays planned.
