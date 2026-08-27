# 228 Pi SDK Sidecar Reasoning Selection Evidence

Status: promoted; empty deliver-now
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.081 / 225

## Question

Which exact `pi.sdk-sidecar` provider/model/value/lifecycle rows can expose
portable `ReasoningSelection` without allowing Pi 0.84.2 to clamp, substitute,
default, or restore a different thinking level?

## Decision

No. Research 228 admits an empty deliver-now set. No typed
`ReasoningSelection` binding is admitted on `pi.sdk-sidecar` at exact
`@earendil-works/pi-coding-agent@0.84.2`.

Exact Pi construction always clamps the requested thinking level to model
capability before `AgentSession` exists. Contract 040 forbids portable
substitution. A deliver-now row therefore requires a closed static
provider/model/value gate before process, environment, credential, or provider
work, plus `session.thinkingLevel` agreement before readiness. The sidecar
seam can report the effective level, but the route has no preparation admission
surface, the selectable model set depends on configured auth, and the bundled
0.84.2 catalog is too large to embed as a durable closed table.

Cards 226-227 stay blocked. Omission retains exact current bootstrap bytes,
Pi default/stored behavior, and no portable selection claim.

## Method And Boundary

Exact `@earendil-works/pi-coding-agent@0.84.2` and `@earendil-works/pi-ai@0.84.2`
tagged source, the npm `0.84.2` tarballs, the source-tagged sidecar entry
point, private wire fixtures, and production Rust startup/validation seams were
inspected on 2026-08-27. No provider prompt, API call, credential, account
inspection, package install into the worker tree, or ambient configuration
mutation was used.

The selected route remains only `pi.sdk-sidecar` with exact Node `22.23.2`,
driver `swallowtail.pi.sdk-sidecar`, wire `swallowtail-pi-sdk-jsonl-v1`,
behavior `pi.sdk-sidecar-v1`, source tag `swallowtail-pi-sdk-sidecar@0.3.3`,
maintainer-supported delegated harness auth, explicit provider/model,
host-leased working resource, and durable provider-session preservation.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| [Pi 0.84.2 SDK guide](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/docs/sdk.md) | public `ThinkingLevel` vocabulary and construction examples | 2026-08-27 | `aa8e11de93a04f17a35681840f4d8d78d7049f40e25253a6a3aab86ed43e78b7` |
| [sdk.ts](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/sdk.ts) | `createAgentSession` precedence, restore, clamp, persistence | 2026-08-27 | `48ea062da677d6e52c270f4ec767726b48208ebfd0ee61986baacfd707c76e30` |
| [agent-session-services.ts](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/agent-session-services.ts) | `createAgentSessionFromServices` forwarding | 2026-08-27 | fetched same tag; service-only wrapper over `createAgentSession` |
| [agent-session-runtime.ts](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/agent-session-runtime.ts) | runtime replacement via stored factory | 2026-08-27 | `2df7d32d9697a22d1e9b70b08480dfe45c4c8e3a2e6719fcf130d927e34edb87` |
| [defaults.ts](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/defaults.ts) | default thinking level `medium` | 2026-08-27 | `030018b104890188457f5f9d28b4821837f3f0fbe1d09eef58f9d794e73f7554` |
| [models.ts](https://github.com/earendil-works/pi/blob/v0.84.2/packages/ai/src/models.ts) | `getSupportedThinkingLevels`, `clampThinkingLevel` | 2026-08-27 | `b53c99c5fa787af57790fa1d971987faab7e7b917ab5cb985374db407a4d88dd` |
| npm `@earendil-works/pi-ai@0.84.2` tarball | bundled provider model JSON corpus | 2026-08-27 | `0262785a76b0eb2eec596cd8a7ab2ee23eef89d2ef1bb1211c4f0a1944dacf41` |
| `dist/providers/data/anthropic.json` inside pi-ai `0.84.2` | specimen model metadata (`reasoning`, `thinkingLevelMap`) | 2026-08-27 | `e22c277e3a1ffddc3d2701b72787c9e0bd67b835de6b4cb806677b6b6a89a2f7` |
| Research 181 | SDK-sidecar route boundary | 2026-08-21 | frozen qualification |
| Contract 012, 017, 040, 044 | options, attachment, no-clamp, lifecycle events | repo | active contracts |
| `crates/swallowtail-adapter-pi/sidecar/pi-sdk-sidecar.mjs` | bootstrap factory, snapshots, event map | repo | source tag `0.3.3` |
| `crates/swallowtail-adapter-pi/src/sidecar/driver/startup.rs` | bootstrap/state validation | repo | omits/ignores `thinkingLevel` |
| `crates/swallowtail-adapter-pi/tests/fixtures/pi-sdk-sidecar-v1/*` | wire shape only | repo | not membership proof |

Agent-session `setThinkingLevel` and `thinking_level_changed` emission were
verified from tagged `agent-session.ts` source referenced by the SDK guide.

## Frozen Pi 0.84.2 Thinking-Level Semantics

### Vocabulary

Public SDK guide and construction options use thinking levels
`off`, `minimal`, `low`, `medium`, `high`, `xhigh`, and `max`. Default when
absent is `medium` (`DEFAULT_THINKING_LEVEL`).

Portable `ReasoningMode` would map one-to-one to these lowercase Pi ids when
admitted. They are adapter-private unless a deliver-now row exists.

### Capability representation

`getSupportedThinkingLevels(model)` in pi-ai `models.ts`:

- when `model.reasoning` is false, only `off` is supported;
- otherwise each extended level is supported unless
  `model.thinkingLevelMap[level] === null`;
- `xhigh` and `max` require an explicit non-null map entry.

`clampThinkingLevel(model, level)` returns the requested level when it is
supported; otherwise it walks upward then downward through the extended order
and returns the nearest supported level. Unknown vocabulary (`index === -1`)
returns `availableLevels[0] ?? "off"`. Clamping is silent; the SDK does not
surface requested-vs-effective mismatch.

### Construction precedence (`createAgentSession`)

1. start from `options.thinkingLevel` when provided;
2. else restore from stored session when messages exist and a thinking entry
   exists, otherwise settings default or `medium`;
3. else settings default or `medium`;
4. always clamp to model capability before `Agent` construction;
5. persist the clamped level on new sessions and append a thinking-level change
   when restoring without an existing thinking entry.

Explicit bootstrap `thinkingLevel` therefore overrides stored session state on
new attachment when provided. Omission preserves stored/default Pi behavior.

### Runtime replacement

`AgentSessionRuntime` stores the sidecar's `createRuntime` factory. Every
`/new`, `/resume` (`switchSession`), fork, and import path tears down the
current session and re-invokes that factory. The sidecar factory closes over
bootstrap `thinkingLevel` and passes it on every `createAgentSessionFromServices`
call, including after `session_switch`.

Each Swallowtail attachment is a fresh sidecar process. Load and resume send
bootstrap then `session_switch`; the closed-over bootstrap level is the only
explicit re-declaration path today because `session_switch` accepts only
`sessionRef` and `expectedCwd`.

### Effective-state confirmation

`session.thinkingLevel` exposes the post-clamp agent state. Bootstrap and
`state` snapshots in the sidecar stringify that value. Agreement with the
caller-selected mode is sufficient only when the selected value is already in
`getSupportedThinkingLevels(model)` so clamp is identity. There is no separate
"requested level" field on the wire.

### Events

`thinking_level_changed` is emitted only from `setThinkingLevel` when the
effective level changes. Initial construction sets `agent.state.thinkingLevel`
directly and does not emit this event. The sidecar subscribes after runtime
construction, classifies `thinking_level_changed` as `progress`, and does not
terminate on it.

## Bundled Model Corpus Size

Offline inspection of `@earendil-works/pi-ai@0.84.2`
`dist/providers/data/*.json` (39 provider files, excluding manifest):

| Metric | Count |
| --- | ---: |
| bundled models | 1267 |
| `reasoning: true` models | 1004 |
| non-`off` supported level rows across corpus | 4050 |

Specimen: `anthropic/claude-opus-4-5` supports
`off|minimal|low|medium|high`; `anthropic/claude-opus-4-7` adds `xhigh|max`
via explicit `thinkingLevelMap`. These metadata facts are static in the tagged
package, but the full corpus is not a small closed admission table.

## Current Route Seams

| Surface | Posture |
| --- | --- |
| Sidecar bootstrap params | optional string `thinkingLevel`; permissive |
| Sidecar bootstrap dispatch | forwards to `createAgentSessionFromServices` when set |
| Sidecar snapshots | report `session.thinkingLevel` |
| Sidecar catalogue | `provider` and `id` only; no `reasoning` or map |
| Rust bootstrap command | sends `cwd`, `provider`, `model` only |
| Rust bootstrap/state validation | ignores returned `thinkingLevel` |
| Rust `SessionOptions` / attachment | must be empty |
| Capability profile | no `ReasoningSelection` |
| Fixture bootstrap | contains `thinkingLevel: "medium"` as wire-shape evidence only |

Omission today emits no bootstrap `thinkingLevel`, leaves Pi default/stored
behavior authoritative, and must not be labeled caller-selected.

## Lifecycle Dispositions

| Lifecycle | Explicit re-declaration | Stored override | Effective confirmation | Disposition |
| --- | --- | --- | --- | --- |
| New session | bootstrap optional field | n/a when explicit | bootstrap `thinkingLevel` when membership pre-proved | **withheld** — no static admission table |
| Load | bootstrap closure only | explicit wins in SDK when provided | bootstrap + post-switch state | **withheld** — same gate |
| Resume | bootstrap closure only | explicit wins in SDK when provided | bootstrap + post-switch state | **withheld** — same gate |
| In-process `session_new` | factory closure | explicit wins when provided | state snapshot | **withheld** — same gate |
| Fresh restoration | new prepared attachment/bootstrap | explicit wins when provided | bootstrap + state | **withheld** — same gate |
| Omission | no field | Pi restore/default + clamp | not compared today | **current** — exact prior bytes/behavior |

## Why No Deliver-Now Row Survives

| Gate | Finding |
| --- | --- |
| No-substitution (Contract 040) | SDK always clamps unsupported or unknown vocabulary before readiness |
| Closed selectable membership | Sidecar `getAvailable()` depends on configured `auth.json`; catalogue returns ids only |
| Small/durable table | Bundled 0.84.2 corpus is 1267 models / 4050+ level rows — not route-local admissible size |
| Pre-effect rejection | Rust has no frozen membership artifact, no `ReasoningSelection` capability, and rejects non-empty options |
| Permissive wire | Sidecar accepts any string and forwards to SDK before capability check |
| Effective confirmation | `session.thinkingLevel` is sufficient only after exact membership is proved; no requested/effective split |
| Catalogue reasoning boolean | Pi RPC reads a `reasoning` observation; sidecar catalogue omits it — insufficient alone even as a lead |

Per-model metadata in the tagged pi-ai JSON is static once provider/model ids
are fixed, but "models already selectable through `pi.sdk-sidecar`" remains
auth-filtered. Card 225 therefore stops with an honest empty set rather than a
partial or fixture-inferred table.

## Deliver-Now Table

| Provider | Model | Requested mode | Lifecycle | Exact confirmation | Disposition |
| --- | --- | --- | --- | --- | --- |
| — | — | — | — | — | empty set |

## Wire / Behavior / Source-Tag Revision

No revision is required to record this evidence stop. Optional bootstrap
`thinkingLevel` and snapshot reporting already exist on wire `v1`. A future
binding would still need typed admission, Rust validation, and likely
catalogue or embedded membership — none are authorized by this empty set.

## Contract Mapping

Had a row survived, it would require `Capability::ReasoningSelection` with an
exact `ReasoningMode`, immutable plan/request agreement, canonical bootstrap
encoding, and bootstrap/state equality before readiness under Contracts 012,
017, and 040. Observed reasoning activity and token shape cannot substitute
for effective-state confirmation (Contract 044).

## Promotion

Research 228 promotes an empty deliver-now set. Card 225 is complete. Cards
226-227 remain blocked. Feature matrix `reasoning_selection` stays `No`.
