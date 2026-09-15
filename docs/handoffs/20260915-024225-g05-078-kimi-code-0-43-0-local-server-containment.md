---
title: g05.078 Kimi Code 0.43.0 local-server containment
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
updated: 2026-09-15
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom directed Swallowtail on 2026-09-14 to action every Research 308 family, explicitly including renewed Kimi containment, and instructed the campaign to continue without repeated approval. Research 308 requires containment or fail-closed behavior. This authorizes provider-free requalification of the separate Kimi local-server claim through official 0.43.0, landing the maximal safe prefix and changing claim posture/identity as Contract 029 requires so uncontained 0.40.0+ points cannot be admitted. It does not authorize provider calls, prompts, login, local-server execution, installation, host update, installed ACP/headless mutation, release, tag, publication, or consumer mutation."
roadmap: docs/roadmaps/g05/078-kimi-code-0-43-0-local-server-containment.md
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, currentness, kimi, local-server, containment]
---

# g05.078 Kimi Code 0.43.0 Local-Server Containment

## What This Thread Was Doing

Execute g05.078 from fresh canonical `main`. Requalify the separate Kimi Code
local-server claim through official `0.43.0`, maximize the safe prefix, and
make every uncontained later point fail closed.

## Why It Matters

Kimi local server is family eighteen in Tom's authorized Research 308
campaign. Research 282 proved `0.39.0..=0.39.1` retains the workspace assertion
and `0.40.0` removes it, but the production claim still stops at `0.38.0` with
`AllowUnverified`. That neither lands the safe prefix nor reliably rejects the
known unsafe releases. Research 308 explicitly requires containment or a
fail-closed result.

## Current State

- Claim `kimi.local-server.executable-window-2`: baseline `0.28.1`, maintained
  heartbeat-ping segment `0.35.0..=0.38.0`, `AllowUnverified`.
- Research 282: selected REST/WS sources through `0.41.0` are stable;
  `0.40.0` removes the Bash `cwd` workspace assertion; `0.41.0` removes the
  auto-mode dangerous-command guard; AmbientHost contains neither.
- Research 325 freezes exact npm/GitHub identity through `0.43.0` and records
  the local-server observations without changing this family.
- npm latest remains `0.43.0`; installed `kimi 0.34.0` remains qualified.

## Boundaries

Provider-free only. Reuse frozen identities and retained artifacts when their
digests reproduce; official source/tarballs may be retrieved into `/tmp` and
must not be executed. Do not run the local server. No prompt, catalogue, login,
credential, session, install, update, release, tag, publication, installed
ACP/headless mutation, or consumer work.

Ordinary provider-free retrieval, parsing, comparison, correction, and
validation may rerun. Use release notes for discovery and one deterministic
selected-file inventory for proof. No binary archaeology or repeated broad
scans.

## Important Context

Follow `docs/roadmaps/g05/078-kimi-code-0-43-0-local-server-containment.md`.
Read Contracts 017/023/029; Research 270/282/308/325; g05.017, g05.026, and
g05.077; local-server selection, prepared guide, drivers/protocol/activity,
and frozen fixtures. Freeze Research 326 before changing the claim.

The settled result is not “stop and leave it permissive.” Reproduce the safe
`0.39.x` prefix, trace containment at every later hop, and use
`QualifiedOnly` or the Contract 029-equivalent claim shape when the uncontained
boundary persists. A later segment may reopen only on exact restored
containment. Ambient cwd, loopback, process ownership, allowlists, leases, and
prompts are not containment. Keep installed ACP/headless and all sibling Kimi
surfaces separate.

## Suggested Next Move

Reproduce Research 282/325 identities, freeze the local-server selected-file
ledger, and classify the two newly relevant hops through `0.43.0`. Then land
the maximal safe segment and fail-closed claim posture as one coherent batch,
validate, push the queue branch, open the canonical PR, and report its exact
head for independent review.

## Completion Protocol

The Queue owns worker launch, independent review, merge, lifecycle closeout,
and handoff deletion. Return Research 326, the exact safe ceiling and rejected
gap, claim posture/identity, containment trace, validation, review comment,
merge SHA, and closeout SHA. Oh My Pi RPC follows. No provider, release, or tag
action follows automatically.
