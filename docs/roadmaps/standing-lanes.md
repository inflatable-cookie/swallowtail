# Standing Lanes

Status: active
Owner: Tom
Updated: 2026-09-04

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

Last one-family claim: Codex exec and app-server `0.152.1` (g05.020, cards
048-049). The shared `codex.cli` window extends through exact npm `0.152.1`
as a compatible extension. Unpublished `0.149.2`, `0.150.2`, and `0.151.1`
stay incompatible. Unpublished `0.152.2` stays permitted `UnverifiedNewer`.
Host `0.150.1` remains inside the qualified range and stays observation-only.
Downloaded official binaries were hashed and never executed; the complete
shipped-tree and source-tree deltas are Guardian, test, and version-bump
bounded. Feature-specific exact sets remain on the `0.147.0..=0.149.1`
probed points. `AllowUnverified` remains.

Previous one-family claim: Claude Code `2.1.257` (g05.019, cards 046-047).
Headless is `2.1.220..=2.1.257` and response-only is `2.1.227..=2.1.257`.
Unpublished `2.1.244`, `2.1.249`, and hop-skipped `2.1.253` through
`2.1.256` stay incompatible. Official `2.1.258` is newer than the qualified
bound and remains an exact `UnverifiedNewer` observation until a later family
run. Host `2.1.257` matches official darwin-arm64 and stays observation-only.
Watcher stays exact `2.1.251` behind its mechanism-change gate.
Feature-specific exact sets remain on the `2.1.220..=2.1.241` probed points.
`AllowUnverified` remains.

Earlier one-family claim: Claude Agent ACP `0.73.0` (g05.018, cards 044-045).
Maintained v7 is `0.66.0..=0.73.0`. Published intermediates `0.71.0`,
`0.72.0`, and `0.73.0` are qualified. Unpublished `0.58.0` stays
incompatible. Unpublished `0.74.0` stays permitted `UnverifiedNewer`. Host
`0.63.0` stays observation-only Qualified Deprecated. `AllowUnverified`
remains.

Earlier one-family claim: Kimi Code installed harness `0.39.1` (g05.016,
cards 041-042) plus the A2 ACP cap (g05.017, card 043). Headless v1 is
`0.29.0..=0.32.0` and headless v2 is `0.33.0..=0.39.1`, above which unpublished
`0.39.2` stays permitted `UnverifiedNewer`. ACP is `QualifiedOnly` at exact
`0.28.1` plus `0.29.0..=0.38.0` with exact `0.39.0` and `0.39.1` excluded as
recorded evidence; every point above `0.38.0` fails closed. A shipped-artifact
identity run may reopen planning only if every invocation path fails closed
again for a terminal-less client, or upstream supplies a ProviderEnforced
boundary satisfying Contracts 017/023. The trigger authorizes a fresh
identity/claim decision, never automatic admission and never restoration of
AllowUnverified by itself.
`kimi-code.local-server` stays exact `0.28.1` plus `0.29.0..=0.38.0` and
remains `AllowUnverified`.

Earlier one-family claim: Pi RPC `0.84.4` (g05.015, cards 039-040).
Unpublished `0.83.1` stays incompatible. `pi.sdk-sidecar` stays exact
`0.84.2`. Claude Code headless and response-only later rose through official
`2.1.257` (g05.019, cards 046-047). g05.014 cards 037-038 remain the
`2.1.252` identity/claim. Unpublished `2.1.244`, `2.1.249`, and hop-skipped
`2.1.253` through `2.1.256` stay incompatible. Watcher stays exact `2.1.251`
behind its mechanism-change gate.
Feature-specific exact sets remain on the `2.1.220..=2.1.241` probed points.
Codex exec and app-server later rose through official `0.152.1` (g05.020,
cards 048-049). g05.013 cards 035-036 remain the `0.152.0` identity/claim.
Unpublished `0.149.2`, `0.150.2`, and `0.151.1` stay incompatible.
Feature-specific exact sets remain on the `0.147.0..=0.149.1` probed points.
Qwen headless `0.22.3` remains qualified through g05.004 cards 012-013. Kimi
Code headless agent-core-v2 stream-json first qualified as exact `0.38.0`
under `kimi.headless.stream-json.v2` (g04.064, cards 179-180); g05.016
corrected that revision's baseline to `0.33.0` and extended it to
`0.33.0..=0.39.1`. The v1 ceiling is `0.32.0`, not the `0.37.2` g04.064
recorded.

### Latest Completed Family

Codex exec and app-server `0.152.1` are compatible extensions of their
existing maintained behaviors. Research 275 and g05.020 cards 048-049 freeze
identity before the claim. Host `0.150.1` keeps its recorded identity and
stays observation-only. Downloaded official binaries were hashed and never
executed. Official npm and GitHub stable is `0.152.1` published
2026-09-01T22:36:50.784Z; the only published stable after `0.152.0` is
`0.152.1`. The complete shipped-tree delta is `package.json`,
`codex-package.json`, and the two rebuilt binaries plus a darwin-only
vendored ripgrep and zsh refresh. The complete GitHub source delta between
the two release tag commits is exactly 12 files: the workspace version bump,
Guardian auto-review/node-REPL policy sources and tests, and test files. No
file feeding a selected surface changed, and every upstream-published schema
file is byte-identical to the frozen `0.152.0` corpus values. Selected
mapped exec flags and app-server methods stay. The new optional Guardian
`AutoReviewMessages.node_repl_policy` stays unmapped. Unpublished `0.149.2`,
`0.150.2`, and `0.151.1` remain incompatible; unpublished `0.152.2` remains
permitted `UnverifiedNewer`. No provider prompt, live session, login,
install, or host update was required.

Research 274 remaining family after Claude Code `2.1.257` was this family
alone and changed no claim.

### Previous Completed Family

Claude Code `2.1.257` is a compatible extension of the separate headless and
response-only stream-JSON axes. Research 273 and g05.019 cards 046-047 freeze
identity before the claim. Host `2.1.257` matches the official darwin-arm64
package and was not installed, updated, or replaced. Official binaries were
hashed and not executed. Official npm and GitHub stable is `2.1.257`
published 2026-09-01T17:15:33.223Z. The only published stable after
`2.1.252` is `2.1.257`; `2.1.253` through `2.1.256` are unpublished.
Wrapper installer files except `package.json` and `sdk-tools.d.ts` are
byte-identical to `2.1.252`. Official `--help` is not byte-identical to
frozen `2.1.252`; the dump adds `--system-prompt-snapshot` and expands
`--bg` resume wording. Selected mapped stream-JSON flags stay. Changelog
`2.1.257` extras stay unmapped. Unpublished `2.1.244`, `2.1.249`, and
hop-skipped `2.1.253` through `2.1.256` remain incompatible; unpublished
`2.1.258` was permitted `UnverifiedNewer` at identity. Research 274 later
observed official `2.1.258` as visible newer. Watcher stays exact `2.1.251`
and is not live-ready; official `2.1.257` is rejected at both
watcher admission seams. No provider prompt, live session, login, install,
or host update was required.

Research 271 remaining family after Claude Agent ACP `0.73.0` was this
family alone and changed no claim.

### Latest Checkpoint

Research 276 revalidated all 40 production families on 2026-09-02. The
partition is 13 unchanged, 6 visible unverified-newer, 20 record-only, and 1
material candidate. Kimi Code npm and GitHub stable moved to `0.40.1`; the
separate `kimi-code.local-server` family is the first post-release candidate,
not an active implementation task, because its AllowUnverified boundary ends
at `0.38.0`, the installed host is qualified at `0.34.0`, and Research 270
recorded the local/web protocol delta. The `0.40.0` Bash tool change removes
the workspace restriction on `cwd`, the same risk class as uncontained local
process authority; the later identity run must investigate it and cannot
assume a compatible extension or qualify it. The installed harness remains
split: headless is separately bounded, while ACP stays QualifiedOnly at
`0.38.0` under the A2 gate. Claude Code `2.1.258`, Antigravity `1.1.24`,
Cursor `2026.08.31-4057e58`, Ollama `0.33.2`, OpenCode `1.18.26`, and Grok
stable `1.0.13` are visible newer observations. PR 182 is closed/unmerged;
its Antigravity branch/evidence is parked post-release and is not merged
qualification. Codex remains closed at qualified `0.152.1`; watcher stays exact
`2.1.251`; Gemini remains deferred. The checkpoint changes no claim and opens
no roadmap or batch card.

### Post-v0.4.0 Queue

g05.021 completed the bounded release-readiness runway and the operator
tagged annotated `v0.4.0` at `56f3913a` on 2026-09-04. The
feature/currentness freeze is lifted. Kimi Code local server ran through
g05.026 card 062, retargeted on 2026-09-04 from `0.40.1` to official latest
`0.41.0` after latest moved mid-run; card 062 stopped with the `0.38.0`
Bash `cwd` restriction removal as a Contract 017/023 authority question before
any segment shape, and it stopped: Research 282 froze `0.41.0` identity and
found the Bash `cwd` change uncontained, so the ceiling stays `0.38.0`.
Antigravity ran through g05.027 card 071 targeting official `1.1.26`; the
closed PR 182 branch was recomputed as evidence input. Card 071 stopped:
every hop `1.1.17..=1.1.26` is frozen, but `1.1.22` retries model-endpoint
HTTP 502 with no published bound or disable control, and Contract 023 keeps
provider-managed retry disabled unless separately accepted. The ceiling stays
`1.1.17` and card 072 is not admitted; Chatterbox recommends keeping that
ceiling until Antigravity exposes a bound. Contract 029's In-Run Latest
Movement rule (accepted 2026-09-04) applies to future lanes.

With both post-release families stopped and Research 276 two days old, the
next currentness step is the all-route checkpoint below.

### Checkpoint Manifest: Research 284

Promoted planning commit: the `main` commit that introduces this section.
This is a standing-lane checkpoint, not a numbered card; it opens no
roadmap and changes no claim.

| Field | Research 284 checkpoint |
| --- | --- |
| Readiness | ready |
| Prerequisites | Research 276, 282, and 283 on `main`; the `version-currentness` skill in checkpoint mode |
| Completion conditions | Research 284 records current official and host observations for all 40 production families, partitions them (unchanged, visible unverified-newer, record-only, material candidate), ranks the next one-family candidate with reasons, and changes no claim; research index line added; docs and Northstar gates green |
| Owned mutable paths | `docs/research/284-*.md`; `docs/research/README.md` (one index line); `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | this file's Latest Checkpoint paragraph, `docs/roadmaps/README.md`, `docs/logs/README.md`; the coordinator edits these at closeout |
| Forbidden paths | every `crates/**` path; every `selection.rs`; matrices; guides; `CHANGELOG.md`; contracts; roadmaps |
| Approved concurrent siblings | g05.009 cards 073 and 065 |
| Serial edges | the ranked candidate becomes a family lane only after Chatterbox compiles it |
| Worker capability class | evidence-first observation worker; registry and release-stream probing; no downloads executed; no provider credentials |
| Acceptance evidence | one observation row per family with source URL and timestamp; host observation without install or update; explicit rank rationale |
| Review oracle | no claim edit; no family treated as qualified from `latest`; Gemini stays deferred; Kimi ACP and local-server and Antigravity stops restated with their reopen conditions |
| Stop conditions | an official channel is unreachable for a family (record the gap, do not guess); a host binary would need execution to observe |
| Escalation owner | operator (Tom) via Chatterbox; coordinator for mechanical blockers |
Claude Code `2.1.258`, Cursor, OpenCode, Ollama, Grok, and other visible newer
observations stay recorded without implementation. Gemini stays deferred,
watcher stays exact `2.1.251`, and `kimi-code.acp` stays QualifiedOnly at the
A2 `0.38.0` cap.

### Earlier Completed Family

Claude Agent ACP `0.73.0` is a compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7`. Operator restart after
official latest moved during the unmerged `0.72.0` family. Research 272 and
g05.018 cards 044-045 freeze identity before the claim. Host `0.63.0`
matches the frozen `0.70.0` host digest and was not installed, updated,
replaced, or executed beyond `--version`. Official npm and GitHub stable at
identity was `0.73.0` published 2026-09-01T20:27:53.428Z; published stables
above the previous `0.70.0` ceiling are exactly `0.71.0`, `0.72.0`, and
`0.73.0`. Mapped `dist/index.js`, `dist/elicitation.js`, `dist/lib.js`,
`dist/settings.js`, and `dist/utils.js` are byte-identical to `0.70.0`.
Complete dist inventory `0.70.0` (33 files) → `0.71.0` (96) → `0.72.0`
(96) → `0.73.0` (96, only `package.json` changed) is frozen; every
`dist/**` file is byte-identical `0.72.0` to `0.73.0`. Remaining named
files stay unmapped with reason. `#1004` keeps mapped mode ids/categories
and `plan`/`acceptEdits`; `#1045` leaves steering unmapped and the
permission callback observable contract unchanged. `0.72.0` effort, result
attribution, PostModelSwitch, and PreModelSwitch stay classified;
Swallowtail still fails closed on explicit `set_config_option` + confirm.
The `0.73.0` Agent SDK pin `0.3.252`→`0.3.257` is unmapped. Five new
emitted update kinds stay unmapped. Unpublished `0.58.0` stays
incompatible; unpublished `0.74.0` remains permitted `UnverifiedNewer`.
Claude Code and the watcher stay untouched. No provider prompt, live ACP
initialize, login, install, host update, or execution of downloaded
official binaries was required.

Research 271 selected this family alone after the Kimi A2 cap when latest
was `0.72.0` and changed no claim.

### Earlier Completed Family

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

`kimi-code.acp` stops at `0.38.0` under `QualifiedOnly`. From `0.39.0` the
agent-core-v2 ACP terminal runner replaces two fail-closed errors with a local
host-process spawn in the leased working resource, and the route always
advertises `terminal: false`, so that branch always applies. The containment
trace found none: `HarnessIsolation::AmbientHost` makes no isolation claim,
Contract 015 denies filesystem containment from process ownership and treats a
terminal request from a terminal-less client as scope-stopping, and no adapter
or runtime control mediates the spawn. Exact `0.39.0` and `0.39.1` stay
excluded as recorded evidence; unpublished `0.38.1`, unpublished `0.39.2`, and
farther `0.40.x` fail closed with them. No new ACP behavior revision was
created.

`kimi-code.local-server` stays a separate family and is unchanged; its `kimi
web` deltas are recorded as observations only. No provider prompt, model
request, authentication, catalogue or session work, live session, install,
host update, or execution of downloaded official binaries was required.

Operator A2 is recorded. A shipped-artifact identity run may reopen planning
only if every invocation path fails closed again for a terminal-less client,
or upstream supplies a ProviderEnforced boundary satisfying Contracts 017/023.
The trigger authorizes a fresh identity/claim decision, never automatic
admission and never restoration of AllowUnverified by itself. The lane must
not adopt a growing exclusion set as a substitute for the posture.

Research 271 revalidated all 40 families after the Kimi A2 cap. Headless
`0.39.1` is on-ceiling. `kimi-code.acp` stays `QualifiedOnly` at `0.38.0` and
is not reopened. Claude Agent ACP `0.72.0` was the sole next-family
candidate. Operator restart compiled official `0.73.0` in g05.018.

### Earlier Completed Family

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

Implement the active task named by the
[roadmaps front door](README.md#next-task). Currentness implementation remains
frozen through the milestone; Kimi local server `0.40.1` stays first after the
release and PR 182 stays closed/parked evidence. This file records standing
lanes; the sole actionable pointer stays in the front door.
