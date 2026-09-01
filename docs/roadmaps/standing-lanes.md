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

Last one-family claim: Kimi Code installed harness `0.39.1` (g05.016, cards
041-042). Headless v1 is `0.29.0..=0.32.0` and headless v2 is
`0.33.0..=0.39.1`, above which unpublished `0.39.2` stays permitted
`UnverifiedNewer`. ACP stops at `0.28.1` plus `0.29.0..=0.38.0` with exact
`0.39.0` and `0.39.1` excluded as `Incompatible`; above its `0.38.0` ceiling
the first admissible unverified-newer point is unpublished `0.38.1`.
`kimi-code.local-server` stays exact `0.28.1` plus `0.29.0..=0.38.0`.

Previous one-family claim: Pi RPC `0.84.4` (g05.015, cards 039-040).
Unpublished `0.83.1` stays incompatible. `pi.sdk-sidecar` stays exact
`0.84.2`.

Earlier one-family claim: Claude Code headless and response-only `2.1.252`
(g05.014, cards 037-038). Unpublished `2.1.244` and `2.1.249` stay
incompatible. Watcher stays exact `2.1.251` behind its mechanism-change gate.
Feature-specific exact sets remain on the `2.1.220..=2.1.241` probed points.
Codex exec and app-server `0.152.0` (g05.013, cards 035-036) remain qualified.
Unpublished `0.149.2`, `0.150.2`, and `0.151.1` stay incompatible.
Feature-specific exact sets remain on the `0.147.0..=0.149.1` probed points.
Qwen headless `0.22.3` remains qualified through g05.004 cards 012-013. Kimi
Code headless agent-core-v2 stream-json first qualified as exact `0.38.0`
under `kimi.headless.stream-json.v2` (g04.064, cards 179-180); g05.016
corrected that revision's baseline to `0.33.0` and extended it to
`0.33.0..=0.39.1`. The v1 ceiling is `0.32.0`, not the `0.37.2` g04.064
recorded.

### Latest Completed Family

Kimi Code installed harness `0.39.1` produced a split outcome, and a
same-family claim correction. Research 270 and g05.016 cards 041-042 freeze
identity before the claim. Host `0.34.0` is byte-identical to the official
`0.34.0` darwin-arm64 extracted artifact and was not installed, updated,
replaced, or executed. Official npm and GitHub stable is `0.39.1`; published
stables above the previous `0.38.0` ceiling are exactly `0.39.0` and `0.39.1`.

Re-testing the routing premise inherited from Research 179 and 211 moved the
boundary. `experimental-v2.ts` redefines `isKimiV2Enabled()` at `0.33.0` from
`KIMI_CODE_EXPERIMENTAL_FLAG` truthy to `KIMI_CODE_LEGACY_FLAG` not truthy, so
the default `kimi -p` engine is agent-core-v2 from `0.33.0`, not `0.38.0`, and
the same release makes naked `kimi acp` run `packages/acp-server`. Production
had claimed `0.33.0..=0.37.2` as qualified `kimi.headless.stream-json.v1`
while those releases emit the `system.version` preamble the v1 decoder
rejects. `kimi.headless.stream-json.v1` corrects down to `0.29.0..=0.32.0`;
`kimi.headless.stream-json.v2` corrects down and extends to
`0.33.0..=0.39.1`; host `0.34.0` reclassifies to qualified Maintained v2.
Unpublished `0.39.2` remains permitted `UnverifiedNewer` on that axis.

`kimi-code.acp` stops at `0.38.0`. From `0.39.0` the agent-core-v2 ACP
terminal runner replaces two fail-closed errors with a local host-process
spawn in the leased working resource, and the route always advertises
`terminal: false`, so that branch always applies. The containment trace found
none: `HarnessIsolation::AmbientHost` makes no isolation claim, Contract 015
denies filesystem containment from process ownership and treats a terminal
request from a terminal-less client as scope-stopping, and no adapter or
runtime control mediates the spawn. Exact `0.39.0` and `0.39.1` are excluded
and classify `Incompatible`; unpublished `0.38.1` remains the first admissible
unverified-newer point above the `0.38.0` ACP ceiling. No new ACP behavior
revision was created.

`kimi-code.local-server` stays a separate family and is unchanged; its `kimi
web` deltas are recorded as observations only. No provider prompt, model
request, authentication, catalogue or session work, live session, install,
host update, or execution of downloaded official binaries was required.

The direction for that ACP stop is not settled by the claim. g05.017 compiles
the
[containment and mediation gate](../triage/2026-09-01-kimi-code-acp-0-39-containment-and-mediation-gate.md)
and returns exactly three mutually exclusive directions; no direction is
accepted, and this lane records none. All three move the claim's newer-version
posture to `QualifiedOnly`, so the lane must treat the present state as
incomplete: the ACP claim binds `AllowUnverified`, and a newly published point
above `0.38.0` would fall through to the unverified-newer path before a
checkpoint could react. Until the operator answers, the lane keeps
`kimi-code.acp` capped at `0.38.0`, keeps exact `0.39.0` and `0.39.1` excluded,
and adds no new exclusion, trigger, or posture change on its own authority. It
must not adopt a growing exclusion set as a substitute for the posture move.
The next all-route currentness checkpoint runs serially after that answer, not
alongside it, and does not rank the Kimi family until then.

### Previous Completed Family

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

Research 269 revalidated all 40 families after Pi RPC `0.84.4` and selected
Kimi Code installed harness `0.39.1` as the sole next-family candidate. That
checkpoint changed no claim; g05.016 then compiled the qualification.

### Earlier Completed Family

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
