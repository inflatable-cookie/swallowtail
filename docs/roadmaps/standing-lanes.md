# Standing Lanes

Status: active
Owner: Tom
Updated: 2026-09-01

These lanes outlive a generation. They are not generation runway goals
and they do not keep a generation open.

## Contract 029 Currentness

Authority: Contract 029 Recurring Currentness Checkpoint and Upgrade
Workflow. Runbook:
[Version Currentness Checkpoint](../guides/version-currentness-checkpoint.md).
Execute through the repo skill `version-currentness`.

Currentness is a persistent serial queue. Continue it after a consumer defect
on an unverified-newer point, after official stables move, or when the operator
asks. It is not CI, a calendar job, or authority to qualify several families
at once.

A checkpoint writes research. It does not change claims and does not
need an active generation.

One-family qualification that changes a claim compiles into the
then-active generation. If none is active, stop and ask. Do not invent a
generation to house currentness.

Last one-family claim: Pi RPC `0.84.4` (g05.015, cards 039-040). Unpublished
`0.83.1` stays incompatible. `pi.sdk-sidecar` stays exact `0.84.2`.

Previous one-family claim: Claude Code headless and response-only `2.1.252`
(g05.014, cards 037-038). Unpublished `2.1.244` and `2.1.249` stay
incompatible. Watcher stays exact `2.1.251` behind its mechanism-change gate.
Feature-specific exact sets remain on the `2.1.220..=2.1.241` probed points.
Codex exec and app-server `0.152.0` (g05.013, cards 035-036) remain qualified.
Unpublished `0.149.2`, `0.150.2`, and `0.151.1` stay incompatible.
Feature-specific exact sets remain on the `0.147.0..=0.149.1` probed points.
Qwen headless `0.22.3` remains qualified through g05.004 cards 012-013. Kimi
Code headless exact `0.38.0` default agent-core-v2 stream-json under
`kimi.headless.stream-json.v2` (g04.064, cards 179-180). The v1 ceiling
`0.37.2` remains on `kimi.headless.stream-json.v1`.

### Latest Completed Family

Pi RPC `0.84.4` is a compatible extension of
`pi.rpc.strict-lf-v0.84.0-message-update-delta`. Research 268 and g05.015
cards 039-040 freeze identity before the claim. Current host `0.83.0`
matches the previously frozen `0.84.2` host digest. Official npm and GitHub
stable is `0.84.4`. Mapped `jsonl.ts`, `session-cwd.ts`, `json-event.ts`, and
`args.ts` are byte-identical to `0.84.3`. `clear_queue` stays unmapped.
Unpublished `0.83.1` remains incompatible; unpublished `0.84.5` remains
permitted `UnverifiedNewer`. `pi.sdk-sidecar` stays exact `0.84.2`. No
provider prompt, live session, login, install, host update, or execution of
downloaded official binaries was required.

Research 269 revalidated all 40 families after Pi RPC `0.84.4`. Pi is now
unchanged/on-ceiling. Kimi Code installed harness `0.39.1` is the sole
next-family candidate. Host `0.34.0` already sits on a qualified bound. This
checkpoint changes no claim. One-family qualification waits for a later
Upgrade Workflow PR and must not infer the family from registry `latest`
alone.

### Previous Completed Family

Claude Code `2.1.252` is a compatible extension of the separate headless and
response-only stream-JSON axes. Research 266 and g05.014 cards 037-038 freeze
identity before the claim. Current host `2.1.251` matches the previously frozen
official darwin-arm64 package. Official npm and GitHub stable is `2.1.252`.
Official extracted help is byte-identical to frozen `2.1.251`. Unpublished
`2.1.244` and `2.1.249` remain incompatible; unpublished `2.1.253` remains
permitted `UnverifiedNewer`. Watcher stays exact `2.1.251` and is not
live-ready. No provider prompt, live session, login, install, or host update
was required.

Research 267 revalidated all 40 families after Claude Code `2.1.252` and
selected Pi `0.84.4` as the sole next-family candidate. That checkpoint
changed no claim.

### Do Not

- treat currentness as a reason to keep a generation active
- bulk-bump from registry `latest`
- leave the current host or official stable `UnverifiedNewer` without a
  named incompatible reason

## Next Task

Implement the active roadmap card named by the
[roadmaps front door](README.md#next-task). This file records standing
lanes; the batch pointer stays in the front door.
