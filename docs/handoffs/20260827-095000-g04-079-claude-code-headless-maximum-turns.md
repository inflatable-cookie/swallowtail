---
title: g04.079 Claude Code headless maximum turns worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-27
updated: 2026-08-27
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260827-095000-g04-079-claude-code-headless-maximum-turns.md
base_required: pushed-main-planning-base
tags: [coordination, handoff, worker, pr]
---

## What This Thread Was Doing

The orchestrator fast-forward merged PR 77 at exact head
`d7edcb34ed186df7738437d466d4ecaf6611077a`, kept g04 open, and resumed the
sole roadmap continuation. It reassessed the advanced per-route feature tail
and selected Claude Code headless maximum agentic turns as the next bounded
evidence candidate.

The orchestrator compiled g04.079, cards 219-221, Research 226, programme and
front-door updates, triage disposition, and the compilation log. The planning
base was validated and pushed to `main` at
`a7b7865e2d306232ee55735cbe7d5bc28923214e`.

This is one bounded manual implementation thread. Start from this file without
a copied transcript or a second prompt. Do not spawn internal agents; the
operator owns parallelism in their harness.

## Why It Matters

`claude-code.headless` already owns one read-only Plan-mode Claude Code child,
one structured-run lifecycle, fixed `Read,Glob,Grep`, selected model and
reasoning, strict empty MCP, no session persistence, stream decoding, deadline,
terminal mapping, and joined cleanup. It does not expose the native
`--max-turns` limit.

Current official documentation describes positive print-mode `--max-turns` as
a maximum over agentic tool-use turns, `error_max_turns` when reached, and
`CLAUDE_CODE_MAX_TURNS` as an equivalent environment value that explicit argv
overrides. Those mutable claims do not backport to the qualified
`2.1.220..=2.1.241` artifacts. The frozen `2.1.241` help specimen does not
advertise the flag, while current docs say help is incomplete. Exact artifact
support, parser bounds, native enforcement, terminal truth, and precedence are
the gate.

## Current State

- **Repository:** `inflatable-cookie/swallowtail`
- **Planning base:** `main`
- **Planning commit before this handoff:**
  `a7b7865e2d306232ee55735cbe7d5bc28923214e`
- **Planning publication:** planning commit is exact `origin/main` before this
  handoff commit
- **Planning checkout:** shared main checkout; do not use it for worker edits
- **Worker mode:** implementation worker dispatched by the orchestrator; this
  handoff activates worker-only worktree preflight
- **Planning artifacts:** g04.079, cards 219-221, Research 226 reservation,
  compilation log, programme/triage/index updates, and sole Next Task
- **Worker branch:** `worker/g04-079-claude-code-headless-maximum-turns`
- **Worker worktree:**
  `/Users/tom/Dev/worktrees/swallowtail-g04-079-claude-code-headless-maximum-turns`
- **Worktree creation command:** `git worktree add -b
  worker/g04-079-claude-code-headless-maximum-turns
  /Users/tom/Dev/worktrees/swallowtail-g04-079-claude-code-headless-maximum-turns
  origin/main`
- **Worker worktree policy:** first use the clean, dedicated, non-`main`
  registered worktree supplied by the launcher, even if its generated path or
  branch differs from these placeholders. Record the actual path/branch and do
  not create a second worktree for that reason. If the current context is
  unusable, use the named worktree when it matches; only then read
  `.agents.local.env`, require `AGENTS_WORKTREE_CONTAINER_DIR`, and create a
  unique manual worktree/branch under that container from `origin/main`. Ask
  the operator first if the file or key is absent; never use `/tmp`, `TMPDIR`,
  or a guessed path for a worktree.
- **Active spec lane:** per-route feature completion programme
- **Roadmap milestone:**
  `docs/roadmaps/g04/079-claude-code-headless-maximum-turns.md`
- **Ready cards, in order:**
  `219-claude-code-headless-maximum-turns-evidence.md`, then conditional
  `220-claude-code-headless-maximum-turns-binding.md`, then conditional
  `221-claude-code-headless-maximum-turns-acceptance.md`
- **Allowed runway:** execute card 219 and promote Research 226; continue to
  cards 220-221 only for a non-empty exact row with proved native loop
  enforcement and explicit-argv precedence over the ambient env equivalent
- **Remaining card budget:** three cards; stop after card 219 when evidence is
  empty or any decision gate fires
- **Dispatch topology:** one serial worker lane; one reviewable PR; no internal
  agents or subagents
- **Parallel safety check:** serial because evidence decides whether binding
  and acceptance exist and every card touches the same Claude headless route
- **Canonical refs:** `docs/architecture/system-architecture.md`; Contracts
  010, 011, 029, 037, 039, 040, 041, and 052
- **Model capability profile:** exact official package/native artifact research
  plus route-local Rust implementation and deterministic conformance
- **Tool/runtime restrictions:** secret-free exact-artifact/source work only;
  no install, update, provider prompt, account/login inspection, paid work,
  ambient configuration mutation, or sibling-route work
- **Required validation:** card 219 checks first; if delivery proceeds,
  `cargo fmt -p swallowtail-adapter-claude-agent`,
  `effigy validate:focused swallowtail-adapter-claude-agent`,
  `effigy package:verify-affected swallowtail-adapter-claude-agent`,
  `effigy check:examples`, `effigy package:api`, `effigy qa:northstar`, named
  research/log/roadmap/card/next-action checks, `effigy doctor`, and
  `git diff --check`
- **Inherited doctor baseline:** `scan.god-files` reports 380 findings (334
  warnings, 46 errors); `scan.generated-in-src` reports one warning; graph
  index is stale. Existing papercut records cover the structural baseline;
  record drift and do not add duplicates or repair unrelated findings.
- **PR base:** `main`
- **PR head:** worker branch
- **PR URL:** pending
- **Review state:** awaiting worker evidence and PR
- **Merge authorisation:** not authorised; operator must explicitly request it

## Boundaries

Keep this run inside the named runway:

- **In scope:** exact qualified Claude Code `--max-turns` support boundary,
  parser, numeric domain, repetition, environment precedence, counted-turn
  definition, loop enforcement, result subtype, `num_turns`, stop reason,
  usage, stream ordering, stderr/exit behavior, current decoder mapping,
  conditional closed adapter-local binding, canonical argv, deterministic
  acceptance, route-local docs/matrices/API truth, Research 226, closeout, and
  sole Next Task
- **Out of scope:** portable budgets, output-token/tool/cost/wall-time limits,
  `--max-budget-usd`, autocompact, Fast, Ultracode, structured output, advisor,
  agents, teams, fallback, writable permission modes, response-only, ACP, live
  provider work, currentness, release, generation rollover, g04 closure, or
  merge
- Existing construction must remain exact: omission sends no `--max-turns`
  argument and passes the approved environment unchanged. Do not claim omission
  means unlimited execution because `CLAUDE_CODE_MAX_TURNS` may be ambient.
- A selected value is eligible only if exact evidence proves explicit argv
  overrides the environment equivalent and the native loop enforces it.
- Keep agentic turns distinct from output tokens, tool calls, provider requests,
  wall time, cost, context, retries, and portable generation controls.
- Requested, prepared, dispatched, parser-accepted, enforced, reached, and
  observed truth remain separate. Claim only the strongest exact evidence
  level Research 226 admits.
- Do not invent architecture, change contracts, widen the roadmap, or choose an
  unresolved product/API/persistence/security decision.
- This handoff represents one worker lane. Do not edit another lane's scope or
  spawn subagents. If shared mutable scope or a hidden dependency appears,
  stop and report it through the operator.
- Work only in the selected clean worker worktree. Never edit the orchestrator
  planning checkout or an unrelated dirty checkout.
- Do not merge the PR. Merge remains a separate operator-authorised action.

## Important Context

- **Exact route point:** route `claude-code.headless`, driver
  `swallowtail.claude-code.headless`, axis
  `claude-code.headless-stream-json`, qualified window
  `2.1.220..=2.1.241`, behavior `claude-code.headless.stream-json.v1`.
- **Official leads:** `https://code.claude.com/docs/en/cli-reference` names
  positive print-mode `--max-turns`, error on reaching the limit, and no limit
  by default; `https://code.claude.com/docs/en/agent-sdk/agent-loop` says turns
  are tool-use round trips and names `error_max_turns`; and
  `https://code.claude.com/docs/en/env-vars` says
  `CLAUDE_CODE_MAX_TURNS` is equivalent when argv is absent and explicit argv
  has precedence. Freeze retrieved bodies and exact artifacts; do not cite
  mutable docs as qualified-version proof.
- **Exact evidence tension:** the frozen `2.1.241` identity/help corpus does not
  advertise `--max-turns`, and current docs explicitly say help omits some
  flags. Absence from help is not a stop by itself; extracted parser and loop
  evidence must settle support.
- **Related evidence:** Research 202 freezes the `2.1.241` artifact and qualified
  window. Research 212 demonstrates the exact-package/native-binary method and
  why parser acceptance alone is insufficient. Research 121 and Contract 039
  govern the bounded headless structured-run projection.
- **Current command:** `claude -p --input-format text --output-format
  stream-json --verbose --no-session-persistence --model <selected>
  [--effort <selected>] --permission-mode plan --tools Read,Glob,Grep
  --setting-sources user,project,local --mcp-config {"mcpServers":{}}
  --strict-mcp-config`. Omission must preserve it exactly.
- **Environment tension:** the host supplies an opaque approved environment to
  preserve local subscription access. Production preparation cannot inspect or
  scrub `CLAUDE_CODE_MAX_TURNS`. Explicit selection may still qualify if exact
  argv precedence is unconditional; omission preserves current ambient truth.
- **Terminal tension:** determine whether `error_max_turns` is a nonzero process
  exit, a result subtype on an otherwise successful process, or both. Preserve
  current failure and cleanup semantics; do not turn native bound reached into
  completion.
- **Decisions and preferences:** closed Claude Code-local positive type only;
  no raw number/string escape hatch; no live proof; an empty Research 226 set
  is valid when enforcement or terminal truth cannot be frozen without provider
  work.
- **Report after:** Research 226 and card 219 are complete, or earlier when a
  stop condition fires. If evidence is non-empty, continue through cards
  220-221 before reporting the complete review-ready lane unless a real blocker
  appears.
- **Report to:** the operator, who will relay progress to the orchestrator.

## Suggested Next Move

This handoff explicitly activates worker mode. Read it from the top, then run
the quick worktree-safety preflight in `## Completion Protocol` before broad
repository reads. Accept a clean launcher-provided non-`main` worktree even if
its generated path or branch differs from the placeholders. Do not create a
second worktree or spawn internal agents.

Execute card 219 as one coherent evidence chunk. Begin with exact package and
native-artifact identity, then trace `--max-turns` through parsing, options,
environment precedence, the loop counter, limit reached, stream/result fields,
exit, and current driver mapping. Promote an exact empty or non-empty Research
226 set before touching production binding.

## Completion Protocol

### Before you start

1. Read this handoff path. Its `worker_mode: implementation` and
   `dispatch_authority: orchestrator` metadata activate worker mode. Run one
   quick read-only safety probe before broad repository reads:
   `git rev-parse --show-toplevel`, `git branch --show-current`,
   `git status --porcelain`, and `git worktree list --porcelain`.
2. If the current root is a registered worktree, its status is empty, and its
   branch is not `main`, accept it as the launcher-provided worktree. Record its
   actual root/branch; do not compare it with the placeholders or create
   another worktree merely because they differ.
3. Otherwise stop before edits. Use the named registered worktree if it is
   clean and correctly based. If no usable worker worktree exists, follow the
   `.agents.local.env` fallback policy in `## Current State`. Never edit
   `main`, a dirty checkout, or another worker's branch.
4. Fetch `origin/main`. Require the worker base to contain planning commit
   `a7b7865e2d306232ee55735cbe7d5bc28923214e`. Fast-forward or recreate the
   clean worker branch if needed; do not merge main into it.
5. Read `AGENTS.md`, the `northstar` and `effigy` skills, the g04.079 roadmap,
   cards 219-221, Research 226, Research 202, Research 212, the Claude prepared
   guide, relevant contracts, and the advanced-feature triage tail before
   edits.

### Execute and stop correctly

6. Run `effigy tasks`, `effigy doctor`, and `effigy test --plan`. Treat the
   inherited doctor baseline above as known; report only drift.
7. Execute card 219. Use exact official package/native artifacts and
   deterministic, prompt-free local evidence. Do not install, update, prompt a
   provider, inspect account state, or mutate ambient configuration.
8. Promote Research 226 and update card/milestone state. If the deliver-now set
   is empty, mark cards 220-221 blocked, complete the honest evidence-stop
   closeout, update indexes and the sole Next Task, validate, and stop.
9. If and only if Research 226 admits a non-empty exact set, execute cards 220
   and 221 serially. Bind only admitted version/value rows. Do not use a fixture
   to invent enforcement or precedence the production route cannot guarantee.
10. Work in meaningful batches. Run focused validation after the evidence chunk
    and the complete named acceptance round once after implementation. Do not
    repair inherited doctor findings or unrelated papercuts.

### Prepare the review handoff

11. Update Research 226, the roadmap/cards, guide, matrices, programme, triage,
    logs, indexes, changelog, API baseline when changed, and sole Next Task so
    they agree on complete delivery or honest stop.
12. Run every applicable card command. At minimum run the exact package-focused
    selectors and all named docs/index checks. Run `git diff --check` and
    `effigy doctor`; record exact failures or baseline drift.
13. Review `git diff --stat`, `git diff --check`, `git status --short`, and the
    full changed-file list. Ensure the branch contains no credentials,
    downloaded packages/binaries, runtime caches, ambient config, generated
    probe debris, or unrelated changes.
14. Commit coherent worker changes, push the worker branch, and open one PR to
    `main`. Do not merge it. Confirm the PR head SHA equals the pushed branch
    head and report required CI state.
15. Return a compact operator report containing: outcome and evidence tier;
    exact Research 226 deliver-now or empty set; cards executed/blocked; files
    and public API changed; validation and doctor drift; PR URL, number, base,
    head SHA, mergeability, and CI; unresolved risks; and the precise next move.
    Keep g04 open.
