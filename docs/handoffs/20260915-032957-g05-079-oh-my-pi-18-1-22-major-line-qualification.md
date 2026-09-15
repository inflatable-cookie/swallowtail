---
title: g05.079 Oh My Pi 18.1.22 major-line qualification
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
base_required: pushed-main
queue_dispatch: northstar-queue
roadmap: docs/roadmaps/g05/079-oh-my-pi-18-1-22-major-line-qualification.md
queue_approval: "Tom instructed Chatterbox on 2026-09-14 to action every Research 308 family, explicitly including Oh My Pi, and told it to continue serially without repeated approval."
queue:
  capability: general
  skipPRReview: false
  notifyOriginOnCloseout: true
---

## What This Thread Was Doing

Run the nineteenth and final operator-approved Research 308 family. Qualify or
stop `oh-my-pi.rpc` across every official npm stable after `17.4.0` through
current `18.1.22`, with 17.x handled independently and 18.x treated as a new
private mapping segment.

## Why It Matters

Swallowtail still qualifies `oh-my-pi.package` only through `17.4.0`, while
the installed host is `18.1.16` and official stable is `18.1.22`. Research 217
found promising wire compatibility at the 18.0 boundary but correctly stopped
when latest moved. The route now needs one complete identity-first ruling,
without confusing it with the separate Pi package.

## Current State

- Canonical task: [g05.079](../roadmaps/g05/079-oh-my-pi-18-1-22-major-line-qualification.md).
- Current claim: axis `oh-my-pi.package`, maintained
  `17.2.9..=17.4.0`, claim `oh-my-pi.rpc.package-window-1`, behavior
  `oh-my-pi.rpc-v2-v17.2.9`, posture `AllowUnverified`.
- Official npm and GitHub latest at planning: `18.1.22`, published 2026-09-14.
  npm integrity is
  `sha512-r/6rrx3PdCjcjaUvXDm8iFMIE/qHA9ryNo4RYXJuJmGXFnpxZNTHV4Ud0O8uGPMUx1YWRs0HdP6xUkG0XjZ44Q==`;
  npm shasum is `73e07a27460436b19eb0cae8dd293210b3971212`.
- Installed `/Users/tom/.local/bin/omp` reports `omp/18.1.16`, SHA-256
  `99eed6d45d984d2f13d76832f78782b9aa07862921ab4e98e2d8e31ca8129795`.
- Published successors are `17.4.1`, `17.4.2`, `18.0.0`, `18.0.1`,
  `18.0.3..=18.0.11`, `18.1.0..=18.1.6`, and `18.1.8..=18.1.22`.
  At minimum, npm gaps `18.0.2` and `18.1.7` remain explicit; rederive the
  complete ledger rather than trusting this shorthand.
- Research 327 is reserved for the new frozen identity and mapping evidence.

## Boundaries

Follow g05.079 exactly. One Oh My Pi RPC family only. Provider-free artifact
retrieval and source inspection may rerun normally. No prompt, inference,
provider catalogue call, login, credentials, host install/update,
downloaded-artifact execution, Pi adapter work, release, tag, publication, or
consumer mutation.

## Important Context

Use the repository `version-currentness` skill and `reference.md`, Contracts
017/023/029, Research 217, Research 308, and the frozen 17.x corpora. Official
release notes are the discovery authority; use one deterministic changed-file
inventory and inspect only files feeding the selected RPC route. Do not scan
every package repeatedly or perform binary archaeology.

Identity must be committed before claim changes. Classify `17.4.1` and
`17.4.2` separately. Any admitted 18.x range needs a distinct
adapter-private behavior revision and segment even when the public operation
does not change. Preserve npm-unpublished gaps. The decoder's unknown-field
tolerance is evidence only when mutation-sensitive tests prove mapped meaning
cannot change. Keep `oh-my-pi.package` separate from
`@earendil-works/pi-coding-agent` and `pi.package`.

## Suggested Next Move

Recheck npm/GitHub latest and the installed version, retrieve the baseline and
successor artifacts into `/tmp`, reproduce Research 217, then freeze Research
327 and its complete selected-source ledger before editing `selection.rs`.

## Completion Protocol

Meet every g05.079 acceptance row. Run its exact validation list, obtain an
independent exact-head review from a different provider/model identity, merge
through Northstar Queue, and synchronize `main`. Return exact npm/GitHub/host
identities, every admitted segment or stop, identity and claim commit SHAs,
tests, review comment, PR/merge, canonical closeout, and confirmation that the
nineteen-family Research 308 campaign is complete. No release or next campaign
follows automatically.
