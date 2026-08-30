# Standing Lanes

Status: active
Owner: Tom
Updated: 2026-08-30

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

Last one-family claim: Claude Code headless and response-only `2.1.251`
(g05.005, cards 017-018). Unpublished `2.1.244` and `2.1.249` stay
incompatible. Feature-specific exact version sets stay on the
`2.1.220..=2.1.241` probed points.

Previous one-family claim: Qwen headless `0.22.3` (g05.004, cards 012-013).
Kimi Code headless exact `0.38.0` default agent-core-v2 stream-json under
`kimi.headless.stream-json.v2` (g04.064, cards 179-180). The v1 ceiling
`0.37.2` remains on `kimi.headless.stream-json.v1`. Claude Code
`2.1.241` (g04.055, cards 153-154) is superseded by g05.005.

### Completed Family

Claude Code `2.1.251` is a compatible extension of the separate headless and
response-only stream-JSON axes. Published intermediates `2.1.242`, `2.1.243`,
`2.1.245`, `2.1.246`, `2.1.247`, `2.1.248`, and `2.1.250` are qualified;
unpublished `2.1.244` and `2.1.249` stay incompatible; the first unpublished
later stable, `2.1.252`, remains permitted `UnverifiedNewer`. g05.005 and
cards 017-018 compile the work. No provider prompt, live session,
authentication, install, or host update was required.

### Do Not

- treat currentness as a reason to keep a generation active
- bulk-bump from registry `latest`
- leave the current host or official stable `UnverifiedNewer` without a
  named incompatible reason

## Next Task

Implement the active roadmap card named by the
[roadmaps front door](README.md#next-task). This file records standing
lanes; the batch pointer stays in the front door.
