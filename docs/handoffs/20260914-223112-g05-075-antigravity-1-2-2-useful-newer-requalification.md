---
title: g05.075 Antigravity 1.2.2 useful-newer requalification
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
updated: 2026-09-14
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom directed Swallowtail on 2026-09-14 to action every Research 308 family, explicitly including Antigravity, and retained the rule that Antigravity must gain a bounded mechanism or fail closed. This authorizes provider-free exact-artifact requalification through current official 1.2.2 and the smallest honest per-claim segment decision. It does not authorize unknown provider-managed retry, provider calls, release, tag, publication, installation, host update, or downloaded-binary execution."
roadmap: docs/roadmaps/g05/075-antigravity-1-2-2-useful-newer-requalification.md
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
tags: [coordination, handoff, worker, currentness, antigravity, containment]
---

# g05.075 Antigravity 1.2.2 Useful Newer Requalification

## What This Thread Was Doing

Execute g05.075 from fresh canonical `main`. Requalify Antigravity catalogue
and headless independently through official `1.2.2`, or fail closed at each
claim's smallest exact uncontained boundary.

## Why It Matters

Antigravity is family fifteen in the authorized Research 308 campaign. The
route remains qualified through `1.1.17` because `1.1.22` introduced
provider-managed HTTP 502 retry without a published bound or disable control.
Five newer releases now reach `1.2.2` and broaden that retry behavior, so the
old stop must be tested rather than assumed away or applied blindly to the
independent catalogue claim.

## Current State

- Production axis: `antigravity-cli.release`; baseline `1.1.9`, latest
  qualified `1.1.17`, `AllowUnverified`.
- Existing evidence: Research 283 and fixtures freeze `1.1.17..=1.1.26` and
  the exact `1.1.22` retry stop.
- New stable hops: `1.1.27`, `1.1.28`, `1.2.0`, `1.2.1`, `1.2.2`.
- Latest tag commit: `ba985e6b5de2ac8aa09860a154a102831eb7722b`;
  linux-x64 asset digest `sha256:2cfa5c9a4a1edd96db6d4058f34970be60d3bcacda866e2bdce6aefb2451b48e`;
  mac-arm64 digest `sha256:f90ff6094a196f1be3854ac45d999a542d47ec66a1513aa882a8505b947b9a0f`.
- Host `agy 1.1.19` remains installed at the exact official digest recorded in
  Research 283.

## Boundaries

Provider-free only. Download, extract, hash, and statically inspect official
assets in `/tmp`; do not execute them. Host `agy --version` and digest checks
are allowed; no catalogue, print, continuation, login, credential, prompt, or
provider operation. No host update, release, tag, publication, ACP/Gemini
flattening, or consumer work.

Ordinary provider-free retries are allowed. Do not impose a one-shot limit on
downloads, extraction, scanning, parsing, fixtures, or validation. Stop only
on semantic conditions or an actual external blocker.

## Important Context

Follow
`docs/roadmaps/g05/075-antigravity-1-2-2-useful-newer-requalification.md`.
Read Contracts 017/023/029, Research 177/283/308/322, the prepared guide,
selection, commands, catalogue/headless/session drivers, and all Antigravity
fixtures. Freeze Research 323 before changing claims. `1.1.28`'s “much
longer” retry and changed `--print-timeout` result, `1.2.0`'s content-filter
stop, and `1.2.1`'s broader automatic retries are discovery leads only.

Assess catalogue and headless separately. A host deadline cannot satisfy the
provider-retry rule. Keep an explicit incompatible gap. Do not reject a safe
catalogue advance merely because headless stops, and do not admit headless
because catalogue is stable.

Oracle gate: not required. Tom and Contracts 017/023/029 already settle the
decision: exact finite containment or fail closed. No unknown retry policy or
new operator policy may be accepted by this task.

## Suggested Next Move

Freeze the five new releases and both official platform assets, extend the
exact binary ledger from `1.1.26`, then classify catalogue and headless deltas
plus retry/timeout semantics. Commit identity first. Land only the maximal
honest segments, validate, push, open a PR, and hand the exact head to
independent review.

## Completion Protocol

The Queue owns worker launch, independent review, merge, lifecycle closeout,
and handoff deletion. Return exact release/tag/asset identities, Research 323,
the retry/timeout ruling, per-claim segment and gap result, validation, review
comment, merge SHA, and closeout SHA. Gemini follows. No provider, release, or
tag action follows automatically.
