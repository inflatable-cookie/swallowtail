# Standing Lanes

Status: active
Owner: Tom
Updated: 2026-08-25

These lanes outlive a generation. They are not generation runway goals
and they do not keep a generation open.

## Contract 029 Currentness

Authority: Contract 029 Recurring Currentness Checkpoint and Upgrade
Workflow. Runbook:
[Version Currentness Checkpoint](../guides/version-currentness-checkpoint.md).
Execute through the repo skill `version-currentness`.

Cadence is operator-triggered: after a consumer defect on an
unverified-newer point, after a cluster of stables move, or when the
operator asks. It is not CI and not a calendar job.

A checkpoint writes research. It does not change claims and does not
need an active generation.

One-family qualification that changes a claim compiles into the
then-active generation. If none is active, stop and ask. Do not invent a
generation to house currentness.

Last one-family claim: Kimi Code headless exact `0.38.0` default
agent-core-v2 stream-json under `kimi.headless.stream-json.v2` (g04.064, cards
179-180). The v1 ceiling `0.37.2` remains on `kimi.headless.stream-json.v1`.

Previous one-family claim: Claude Code headless and response-only `2.1.241`
(g04.055, cards 153-154). Codex CLI `0.149.1` across exec and app-server
(g04.054, cards 151-152). Neither changes the generation pointer.

### Completed Family

Claude Code `2.1.241` is a compatible extension of the separate headless and
response-only stream-JSON axes. Published intermediates `2.1.239` and
`2.1.240` are qualified; the first unpublished stable, `2.1.242`, remains
permitted `UnverifiedNewer`. g04.055 and cards 153-154 compile the work. No
provider prompt, live session, authentication, install, or host update was
required.

### Do Not

- treat currentness as a reason to keep a generation active
- bulk-bump from registry `latest`
- leave the current host or official stable `UnverifiedNewer` without a
  named incompatible reason

## Next Task

Implement the active roadmap card named by the
[roadmaps front door](README.md#next-task). This file records standing
lanes; the batch pointer stays in the front door.
