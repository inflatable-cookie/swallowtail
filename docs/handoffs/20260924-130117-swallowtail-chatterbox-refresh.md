---
kind: northstar-handoff
title: "Swallowtail Chatterbox continuation"
handoff_mode: chatterbox-continuation
chatterbox_mode: conversational-planning
dispatch_authority: chatterbox
status: ready-to-launch
base_required: pushed-main
created: 2026-09-24
successor_launch: "provider claude; model claude-opus-5-5 (label Opus 5.5); thinking medium; full-access mode bypassPermissions; title Chatterbox; label Chatterbox=true; notifyOnFinish false; same workspace wks_f4a6ebd6fbb93734"
queue_transfer_plan: "9290c6c241fdd5c9297d0c0dd611ad38fb9304908de137c7935ded536c23426b"
queue_transfer_tasks:
  - "e8d3efa3-6d54-4d07-a932-b935276a698d (g06.017, version 14 at preflight)"
---

## What This Thread Was Doing

Act as Swallowtail's long-running Northstar Chatterbox in workspace
`wks_f4a6ebd6fbb93734` (`/Users/tom/Dev/projects/swallowtail`). This thread
succeeded the previous Chatterbox (`8b7bf4df-3098-448d-9939-3299a5a60ce7`) and
has been the sole planning authority since 2026-09-17.

The work since then, in order:

- **Two recorded currentness rulings.** Tom delegated the rule choice, so the
  Goose failure-binding and Qoder turn-binding rulings were recorded in
  `docs/roadmaps/standing-lanes.md`; both families then reopened and landed
  real claims (Goose `1.46.0` → `1.50.1` on
  `goose.acp.stdio-v2.auth-required`; Qoder `1.1.25` → `1.1.54` with a
  deliberate adapter-owned eight-turn bound).
- **Docs drift and triage doctrine.** Repaired stale Contract 061 coverage
  figures, reduced `docs/triage/` to unresolved intake only, re-homed fourteen
  promoted records into `docs/logs/`, and dropped a triage index policy that
  contradicted the doctrine that the triage README is a static anchor.
- **The external PR suite.** Grok Bot produced currentness PRs outside the
  queue. The pilot (#349 Claude Code) and then #351 OpenCode, #353 Codex, #352
  Qwen and #350 Claude Agent ACP were admitted through the new Queue
  **review-entry** route — reviewer first, no worker, changes-requested
  spawning a worker in the same workspace — after that capability was requested
  from the Queue Oracle and delivered.
- **Command Code live gates.** Two typed stops (an account-level credit refusal
  that refused even free models, then a free-backend `malformed_stream`) before
  a paid-model gate on `deepseek/deepseek-v4-flash` accepted both a structured
  turn and the Contract 043 two-turn private continuation.
- **The production MCP boundary.** Longhorn withdrew Contract 023's production
  MCP role in favour of the Contract 022 `agent-control` server. Swallowtail's
  Contracts 063 and Spec 014 were corrected — they had asserted the *opposite*
  stance — and a stdio carrier for stdio-only harnesses was requested from and
  accepted by Longhorn as generic client-side infrastructure.
- **A fourth harness route.** `opencode.acp` was identified, frozen (Research
  337: ACP `session/new` accepts `mcpServers` in stdio, **http** and sse form)
  and implemented, landing as Swallowtail's 51st production route.
- **A process rule.** After a closeout was refused for a worker editing its own
  task card, the contradiction was traced to our own template and manifest
  wording and fixed in Contract 001.

## Why It Matters

Canonical docs hold the durable state, but not the live conversational state a
successor needs: which decisions the operator has already made, which questions
are still open, what is in flight in the Queue, and the process rules learned
this session. This thread is context-heavy; the transfer must preserve the
checkout, Queue provenance and every task identity while making one fresh
thread the sole active planning authority.

## Current State

**Canonical `main`** is `72849202` (`docs: admit the Claude Code 2.1.280
identity stop through review entry (g06.017)`), pushed and synchronized.

**Revision head** is best read from `main`; the milestones a successor should
know are `b369cbd9` (route-native mediation stays production-capable),
`6176c851` (registered-tool production MCP role withdrawn), `2fc05d60` (task
prose pinned against worker writes), `d58ceb00` (Goose/Qoder rulings) and
`e1de5059` (triage reduced to intake).

**Generation g06** holds seventeen tasks: `001`–`005` planned and dormant
(`001` awaits the producer proposal's independent review and canonical contract
promotion; `002`/`003` sit behind the empty Research 256 disposition; `004`
and `005` have no operator promotion or consumer requirement); `006`–`016`
complete; `017` in review.

**Route inventory**: 51 production routes. `opencode.acp` is live and properly
registered, unflattened from `opencode.http` and counted separately by
`qa:docs`.

**Triage** holds five live notes: the Claude watcher instruction-isolation
mechanism (open; planning required), the Grok native MCP mediation lead
(conditional on an ACP probe), provider-session binding persistence (deferred),
deferred route surfaces (parked) and the new-route candidate assessment.

**Standing lane**: Contract 029 currentness. Command Code is the largest known
gap — our qualified point is `1.54.0` while official stable is `1.62.1`.

**Queue, live**: exactly one unfinished task targets this Chatterbox —
`e8d3efa3-6d54-4d07-a932-b935276a698d` (`g06.017 — Review external Claude Code
2.1.280 identity stop`), at version 14 at preflight, with an independent
reviewer running on workspace `wks_1690b3319ad37b43`. Transfer plan
`9290c6c241fdd5c9297d0c0dd611ad38fb9304908de137c7935ded536c23426b`.

## Boundaries

Remain the Swallowtail Chatterbox in this existing workspace. Planning and
confirmed promotion belong here; implementation, independent review, merge and
closeout belong to Northstar Queue. Do not create a Chatterbox worktree,
archive or rename the source workspace, alter task provenance, or resubmit
completed work.

No release, tag, publication, provider call, live probe, host update, consumer
mutation, or new task is authorized by this handoff. Currentness is a standing
lane and must not keep g06 open by itself.

Respect the rules this thread learned the hard way: worker evidence belongs in
the run result and designated evidence files, never in a task card's prose; a
task file's generated lifecycle block is hook-owned; a handoff must not grant
write access to pinned prose or "result lines". Do not mass-edit existing task
pins. Do not reuse the `#349` author attestation for another PR without
evidence.

## Important Context

**What Tom has already decided**, so it is not asked twice:

- He delegates rule choices when he says he will take the Chatterbox's
  recommendation; record the ruling with that attribution.
- Prefer a **bounded pilot** first for any suite of colliding external PRs, then
  sequence the rest after fresh base reconciliation.
- Live gates are budgeted: one attempt, cheapest adequate model, no rerun or
  model substitution without new authority.
- The production MCP is **Longhorn's Contract 022 `agent-control` server**.
  Swallowtail's production role is harness-side MCP client configuration, plus
  route-native host-mediated mediation where a protocol carries tool calls
  natively. Never interpose a Swallowtail-owned production MCP listener — that
  is exactly what the 2026-09-22 direction withdrew — and do not reuse the
  Contract 060 watcher bridge or the `claude-agent.sdk` / `grok-build.acp`
  couriers as the production path.
- **Bovine's harness set**: Codex works through host-mediated `dynamicTools`
  with no MCP at all; Claude (`claude-agent.sdk`) and Grok (`grok-build.acp`)
  reach the production MCP through Longhorn's generic stdio carrier; and
  `opencode.acp` is the first route whose provider can take a consumer-supplied
  streamable-HTTP entry directly.

**Open questions still with the operator:**

1. **The contract admission that matters most.** Contract 063 is the only
   contract naming MCP placements and all three it admits are stdio-shaped, so
   a consumer-supplied **URL-plus-header** placement is a new shape.
   `opencode.acp` models it but does not wire it to production. Admitting it is
   what lets a capable harness point straight at the published MCP with no
   carrier. A recommendation was given (admit, because it is the shape the
   boundary already describes emitting) and no ruling has come back.
2. **`#354`–`#359`** — six further Grok Bot PRs (Ollama, Cursor Agent, Pi RPC,
   Oh My Pi RPC, Grok Build ACP, Antigravity catalogue) remain unintegrated.
   They were deliberately out of scope for the original suite.
3. **`g06.005`** needs a concrete consumer requirement plus operator direction
   before any ACP producer-gap route can be promoted; this is also the lever if
   Bovine needs a harness beyond its current four.
4. **g06 shape** — the generation is well past the 30–50 task range, so a
   rollover is available whenever the operator wants one.

**Process knowledge worth carrying:**

- External PRs are integrated through **review-entry**, not `queue.adopt`. The
  skill and `references/review-entry.md` document it; the helper is
  `node bin/northstar-review-entry.mjs --preview|--key`.
- The Queue Oracle thread is `b24645db-c610-442f-8b96-a76a205338f5` in the
  Northstar Queue workspace; it answers planning and defect questions and has
  twice moved quickly on real issues.
- Tom's harness crashed once mid-session, which produced host-level transport
  errors that looked like provider faults; recovery is an operator restart.
- A closeout hook cannot survive an integration advance between merge and
  closeout until the Oracle's narrow fix lands.

## Suggested Next Move

Read canonical `main`, then present Tom the smallest meaningful choices rather
than reopening settled ground:

1. **Rule on the contract admission** for a consumer-supplied URL-plus-header
   MCP placement — the one decision that makes `opencode.acp` a direct-HTTP
   harness route and removes Longhorn's carrier from its path.
2. **Reconcile g06.017** when its reviewer reports; a stop keeps both ceilings
   at `2.1.278`, so no claim work follows.
3. **Decide on `#354`–`#359`** — batch them through review-entry with a pilot
   first, or leave them open deliberately.
4. **Choose the next lane**: a Command Code currentness run (`1.54.0` →
   `1.62.1`), one of the dormant `g06.001`–`005`, or a g06 rollover.

## Completion Protocol

After receiving `Ownership transfer complete`, become the sole active Swallowtail
Chatterbox in workspace `wks_f4a6ebd6fbb93734`. Continue from this handoff and
current canonical docs, not from stale chat recollection. Promote confirmed
planning in coherent docs-only commits, dispatch only explicitly authorized
ready handoffs through Northstar Queue, and inspect full task detail before any
blocker ruling or recovery control.

The source remains visible as history and must not compete. A later refresh
repeats this exact sequence: push one seven-section continuation handoff,
preflight Queue attention, create one same-workspace successor with the exact
requested settings, transfer the unchanged preflight set, verify it, then send
`Ownership transfer complete`.
