# 2026-09-09 g05.029 Claude SDK Interactive Parity Acceptance Audit

Date: 2026-09-09
Task: `../roadmaps/g05/029-claude-sdk-interactive-parity.md`
Sources frozen: Swallowtail `HEAD` / `origin/main`
`3a4894e7d18d733ddc3036400a2e3fed9964eac1`; tags `v0.4.0..v0.4.4`;
GitHub PRs and reviews on `inflatable-cookie/swallowtail`; immutable
Desktop capsules on `acowtancy/bovine-accelerator-desktop` whose SHA-256
values match Research 301 and g05.036. No provider call, consumer mutation,
or runtime edit.

## Result

Honest evidence stop. The runway delivered permission modes, mediated
writes and Bash, model/effort, resume, MCP, diagnostics, and one live
registered-tool qualification. That is not the parent Acceptance target.
A consumer never drove a multi-turn editing session on `claude-agent.sdk`.

| Clause | Verdict |
| --- | --- |
| A consumer drives a multi-turn editing session on the SDK route with every tool call mediated before it runs | **fail**. Provider-free tests prove write mediation. Live capsules prove one-turn registered-tool Allow/Deny/cancel/stale on `desktop/reconcile`, not `Edit`/`Write`/`MultiEdit`, and not two user turns in one session. `v0.4.1` had no pre-tag application drive; the first Desktop open failed; the `v0.4.2` smoke never reached Chat |
| Permission mode is selectable at open and changeable mid-session | **pass**. PR 221 provider-free tests at accepted head `16ad903b` / merge `80d69dc6`: open sends the selected mode; `set_permission_mode` round-trips confirmed `plan` then `default`; `bypassPermissions`, `auto`, and `dontAsk` fail closed. Live capsules used frozen `permissionMode: default` only and do not add consumer proof |
| The read-only default and `v0.4.0` behaviour are unchanged | **pass**. Default constructor stays `read_only()`: `Read`/`Glob`/`Grep`, `default` mode, read lease. Named axes match across `v0.4.0`, `v0.4.1`, and current main. Later additive fields stay omitted unless opted in. Card 100's later open-path repair is outside this default-profile claim |
| Later scope lands in priority order without reopening the card-080 seam | **pass**. Merge ancestry respects the promoted serial edges. Concurrent 085/086/088 PR numbers do not follow the numbered list and are not scored as landing order. Later profile/session edits are additive bindings; `read_write()` still excludes Bash |
| `v0.4.1` carried card 080's items on the Contract 036 gates | **pass**. PRs 221 and 224 are ancestors of tagged `c3cce750`. Research 286 classified the tree patch-compatible. Candidate PR 229 merged after local gates, exact-head review, and workflow-dispatch `33969131592` (six jobs). Source consumer passed. Operator compression dropped the Bovine editing smoke; the release note states no application drove the candidate before the tag |

Coordinator closeout sets `Status: stopped` and reconciles reserved
indexes. Card 087's stopped no-range gate is preserved. This audit invents
no repair, live probe, range widening, release, or tag.

## Frozen Identities

### Tags

| Tag | Peel | Tag object |
| --- | --- | --- |
| `v0.4.0` | `56f3913ac99af44b6ff45384cfc53a0adea587ba` | `6f398b9f0fedae4215ea7f58fdf04f888871e540` |
| `v0.4.1` | `c3cce7504ffd5eae138a0190f1cd81332db68c3c` | `c888b2dc1a968d8dda66a99da1bb5fd51067df58` |
| `v0.4.2` | `f94dd16f2e4db79c5b7c4440cc1eb2d20f8b6af8` | `927d14eccc16b5f24fc427913e087cd47fcfa499` |
| `v0.4.3` | `cbd4ddc8f9d6aa55bd947b92a55ea3a779582b79` | `d83004302801222258c3496791de1e3305571860` |
| `v0.4.4` | `49c9e3b291609c9ebf5b35a284c08302f3b8d5e3` | `41da6c1afe60380d248275bd0780c2e8f79e5ab9` |

`git merge-base --is-ancestor 80d69dc6 v0.4.1` and
`23d3cd8d` likewise: both Card 080 merges are in the `v0.4.1` peel.
`97f37e4d` (Card 081) is not.

### Runway merges

Times are GitHub `mergedAt` (UTC). Squash merges keep the squash SHA as
the merge identity.

| Card | PR | Merged UTC | Merge SHA | Role |
| --- | --- | --- | --- | --- |
| 089 | 223 | 2026-09-04T21:59:21Z | `e4399790` | bounded-profile exclusion; unblocks 080 PR 2 |
| 080 | 221 | 2026-09-04T22:07:16Z | `80d69dc6` | permission policy; write admission still refused |
| 080 | 224 | 2026-09-04T23:31:01Z | `23d3cd8d` | ambient read-write editing admitted |
| 081 | 233 | 2026-09-05T14:53:57Z | `97f37e4d` | Bash under mediation |
| 082 | 239 | 2026-09-06T15:38:39Z | `c8512290` | model and effort |
| 086 | 241 | 2026-09-06T16:19:02Z | `d127837f` | identity research; no range |
| 085 | 243 | 2026-09-06T16:52:59Z | `8bbeceb4` | Grok ACP permissions; serial: none |
| 105 | 245 | 2026-09-06T18:50:15Z | `217de072` | termination cause; later g05.029 fold-in |
| 083 | 244 | 2026-09-06T22:00:37Z | `7cb08b1f` | resume and listing |
| 088 | 242 | 2026-09-06T22:38:07Z | `80e004b4` | install guidance; serial: none |
| 084 | 255 | 2026-09-06T23:27:22Z | `8a377c1b` | client MCP |
| 087 | 256 | 2026-09-06T23:41:47Z | `54153c68` | stopped; no admitted range |

Ancestry (merge-base is-ancestor):
`80d69dc6` → `23d3cd8d` → `97f37e4d` → `c8512290` → `7cb08b1f` →
`8a377c1b`. `d127837f`, `8bbeceb4`, and `80e004b4` are ancestors of
`8a377c1b` and not descendants of `7cb08b1f`. That matches the 2026-09-06
promotion `0acba239`: 084 follows 083; 087 follows 084 and 086; 085 and
088 have serial edges none.

### Live consumer capsules

| Gate | Swallowtail source | Capsule path (Desktop) | SHA-256 | Shape |
| --- | --- | --- | --- | --- |
| Desktop Card 318 / Research 301 | `24f88fb8` | `docs/proofs/claude-credited-registered-tool-qualification.capsule.json` | `c4de15a8a8b4a47996dc94c9cf7610e56a311a1b52266d156309cf944231c1f6` | four fresh opens, four one-turn attempts |
| Desktop Card 323 / g05.036 | `49c9e3b2` (`v0.4.4` peel) | `docs/proofs/claude-registered-tool-live-card323.capsule.json` | `bf40f96107fc35b3ceea1b29da779230f031de2c487e3534a79ba23b59316343` | same four-control shape on the tagged tree |

Both capsules: route `claude-agent.sdk`; schema
`bovine.claude-agent-sdk-registered-tool-live.v1`;
`permissionMode: default`; `persistSession: false`; `selected_tools` is
only `mcp__swallowtail-registered-tools__desktop_reconcile`;
`allowedTools` omitted. Attempt kinds: `primary_allow`, `deny`,
`cancellation`, `stale_or_foreign_callback`. Allow records
`before_dispatch: true` and one dispatcher invocation. Distinct session
identities per attempt. Prompt SHA-256 values are the registered-tool
prompts, not an editing prompt. No `Edit`, `Write`, `MultiEdit`,
`setPermissionMode`, or `acceptEdits` field is present.

Card 318 Desktop chain: task `4356b461-cea3-4079-85cf-d9e63a1bb178`;
PR 181 accepted head `18b70c91`; review `5599408741`; merge `807f7a3f`;
closeout `03e71e90`. Card 323: queue task
`e8bca18b-5689-4974-b5a2-503e5aaad73a`; PR 186 accepted head `977a4f29`;
review `5602628180`; merge `32344306`; closeout `da9edb8d`.

## Clause 1 — Consumer multi-turn editing session

Required together in one admissible chain: a consumer operation, at least
two correlated user turns in one session, an editing-tool effect
(`Edit`/`Write`/`MultiEdit`), and a recorded pre-effect consumer decision
on every tool call.

| Candidate | What it is | Why it does not satisfy |
| --- | --- | --- |
| PR 221/224 fake-SDK tests | two-turn admitted/denied writes, `acceptEdits` narrowing, intact callback input | not a consumer |
| `v0.4.1` tag | Card 080 in tree; gates green | tag record: no application drove the candidate |
| First Desktop open on `v0.4.1` | `open_rejected`; sidecar failure code dropped | no session, no turn, no edit |
| Card 100 live turn | typed unresolved `ProviderFailed` | not a completed editing session |
| Card 102 `v0.4.2` smoke | Desktop PR head `d48a63bf`; window never visible | no Chat, open, sidecar record, or prompt |
| Card 318 capsule | four independent one-turn registered-tool controls at `24f88fb8` | MCP `desktop/reconcile`; not editing; not multi-turn in one session |
| Card 323 capsule | same four-control suite at `49c9e3b2` | same limits; 18-cell release ledger, not an editing session |

Pre-effect mediation is proved twice, in different places: fake-SDK write
callbacks, and live Allow `before_dispatch: true` on the registered tool.
Those proofs do not combine into one consumer editing session.

Fail. A new live editing session would need operator authority; this
audit does not open that work.

## Clause 2 — Permission mode at open and mid-session

PR 221 exact-head review
https://github.com/inflatable-cookie/swallowtail/pull/221#issuecomment-5546986309
accepted `16ad903b` (merge `80d69dc6`). Provider-free proofs still present
on current main:

- `a_session_sends_its_admitted_set_and_selected_mode_on_open` — read-only
  profile with `Plan` sends `tools: ["Read","Glob","Grep"]` and
  `permissionMode: plan`
- `a_mid_session_mode_change_round_trips_the_confirmed_mode` — confirmed
  `plan` then `default`; admitted tool set does not move with the mode
- sidecar asset loop refuses `bypassPermissions`, `auto`, and `dontAsk`
  with `permission_mode_rejected` before SDK construction
- Rust type and sidecar name-refusal keep `bypassPermissions`
  unrepresentable

PR 224 (`23d3cd8d`) lifted the temporary write-admission refusal after
Card 089 (`e4399790`) without changing those mode tests.

Live capsules freeze `permissionMode: default` and record no
`set_permission_mode`. That is not a counterexample; it is absence of
consumer-mode evidence. The clause does not require a consumer drive.

Pass on the provider-free seam.

## Clause 3 — Read-only default and `v0.4.0` behaviour

Compared constructor/profile selection, advertised tools, access, and
permission mode — not API additivity alone.

| Axis | `v0.4.0` `56f3913a` | `v0.4.1` `c3cce750` | current main `3a4894e7` |
| --- | --- | --- | --- |
| Default constructor | hardcoded `EXPECTED_TOOLS = ["Read","Glob","Grep"]`; open body `{cwd, model}` | `ClaudeAgentSdkSessionProfile::read_only()` / `Default`; `permissionMode: default` | same `read_only()` / `Default` |
| Advertised default tools | Read, Glob, Grep | same, now sent on open | same unless the consumer opts into writes, Bash, MCP, or registered tools |
| Access | read-only lease | `ResourceAccess::Read`; policy `claude-agent-sdk-ambient-read` | unchanged on the default profile |
| Permission mode | sidecar restrictive default (not a profile field) | `ClaudeAgentSdkPermissionMode::Default` | unchanged |
| Write tools | outside the route | opt-in `read_write()` | `read_write()` still Edit/Write/MultiEdit; Bash only via `new` |

`v0.4.1` started sending `tools` and `permissionMode` on open. The
default values are the `v0.4.0` set. Research 286 (audited
`2187bbec`, merged PR 225 as `3dcf4f12`) classified that as additive and
recorded the Claude SDK default profile unchanged. PR 221's accepted
review used the same preservation claim.

HEAD `read_write()` enumerates Edit/Write/MultiEdit explicitly so Card
081's Bash bit in `WRITE_ADMITTED` cannot widen an existing editing
profile. `persist_session` defaults false; effort, MCP, resume, and
selected-skill fields are omitted unless opted in.

Card 100 later changed open diagnostics, initialize-first readiness, and
newer-Node posture. That is a g05.032 repair of the `v0.4.1` live-open
defect, not a default-tool or permission-mode widening. This clause
scores the named default-profile axes, not every later open-path repair.

Pass.

## Clause 4 — Later scope landing and the Card 080 seam

### Planned order (task runway)

1. 080 read-write session (`v0.4.1` carrier)
2. 081 Bash
3. 082 model and effort
4. 083 resume
5. 084 client MCP
6. 085 Grok ACP permissions
7. 086 identity, then 087 ranges
8. 088 install guidance

### Promoted serial edges (`0acba239`, after 082)

- 083 ready on 082 and tagged `v0.4.2`; 084 follows 083
- 087 follows 084 and 086
- 085 serial: none; 088 serial: none
- 086 ready on Research 280; does not wait for 084

### Actual landing

Claude SDK chain landed in order: 080, 081, 082, 083, 084. 086, 085, and
088 prepared concurrently after 082 and merged before 084. PR numbers
241/242/243/244/255 therefore do not match the numbered list. Concurrent
preparation is not landing; merge ancestry matches the serial edges.
087 (`54153c68`) landed after 084 and 086 and stopped.

### Card 080 seam after `23d3cd8d`

Later commits touch `profile.rs`, `driver/session.rs`, `prepared/build.rs`,
`turn.rs`, and `permission.rs`. Classification:

| Later card | Seam edit | Kind |
| --- | --- | --- |
| 081 `f68d5c92` | Bash opt-in via `new`; `read_write()` unchanged | additive use |
| 082 `921ca127` | effort on the Copy profile | additive |
| 083 `0ce4124d` | `persist_session` default false | additive |
| 084 `f52c48c6` | MCP binding beside the profile | additive |
| 100 | open diagnostics / initialize order | later defect repair, not permission/editing semantics |
| 105 | termination-cause projection | additive |
| registered tools / skills | bindings beside native admission | additive |

No later delta reopens bypass admission, the read-only default, or
auto-allow of write tools. `allowedTools` stays omitted on the Card 080
open path.

Pass.

## Clause 5 — `v0.4.1` carried Card 080 on Contract 036 gates

Card 080 content in the tagged peel: permission modes, mid-session change,
bypass rejection, ambient read-write editing (PR 224 after 089). Card
081+ is explicitly a later card in the `v0.4.1` release note.

Contract 036 lane evidence:

- Research 286: patch-compatible; default profile unchanged
- PR 229 merged as `c3cce750` (tree of the tag peel)
- g05.030 result: all local gates on the frozen candidate; exact-head
  review; workflow-dispatch
  https://github.com/inflatable-cookie/swallowtail/actions/runs/33969131592
  six jobs green at the merged SHA
- clean detached `effigy package:source-consumer` at that SHA
- operator compression 2026-09-05: Card 092 reduced to the external
  source consumer; the recorded `v0.4.0` Nucleus smoke stands in for
  Contract 036's working-application line; Bovine editing becomes
  post-tag adoption. The release note and tag record state that no
  application drove the candidate

The compression is a documented gate substitution, not missing Card 080
content. This clause scores carrier content plus the authorized gates, not
the later failed first Desktop open.

Pass. The missing pre-tag editing smoke is why clause 1 fails, not this
one.

## Held Gate 087

PR 256 merged `54153c68` as a stop: Research 287 admits no range. Five
exact `QualifiedOnly` pins remain on current main:

- `claude-agent.sdk.package` `@anthropic-ai/claude-agent-sdk` `0.3.259`
- `claude-agent.sdk.native` `2.1.259`
- `claude-agent.sdk.node` `22.23.2`
- `claude-agent.sdk.wire` `swallowtail-claude-agent-sdk-jsonl-v1`
- `claude-agent.sdk.sidecar` source-tagged sidecar revision

This audit does not reopen the gate or treat the stop as acceptance.

## Out of scope

No runtime, test, sidecar, baseline, guide, matrix, contract, research,
release, tag, or Desktop edit. No provider contact. No follow-up task is
opened here; Chatterbox owns any later live editing session, range
reopening, or default-profile reinterpretation.
