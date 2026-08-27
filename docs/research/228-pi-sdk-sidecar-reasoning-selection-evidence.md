# 228 Pi SDK Sidecar Reasoning Selection Evidence

Status: promoted; non-empty deliver-now
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Card: g04.081 / 225-227

## Question

Which exact `pi.sdk-sidecar` provider/model/value/lifecycle rows can expose
portable `ReasoningSelection` without allowing Pi 0.84.2 to clamp, substitute,
default, or restore a different thinking level?

## Decision

Yes. Research 228 admits one bounded deliver-now family on exact
`@earendil-works/pi-coding-agent@0.84.2`:

- provider `anthropic`
- model `claude-opus-4-5`
- thinking levels `off`, `minimal`, `low`, `medium`, `high`

Bootstrap resolves models through `ModelRuntime.getModel(provider, modelId)`
against the static bundled `@earendil-works/pi-ai@0.84.2` corpus. Auth filters
runtime availability through `getAvailable()`; it does not change the frozen
capability metadata used for preparation-time membership. For this row every
admitted level is identity under `clampThinkingLevel`, so
`session.thinkingLevel` agreement before readiness is sufficient.

Cards 226-227 bind only this closed table. Omission retains exact current
bootstrap bytes, Pi default/stored behavior, and no portable selection claim.

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
| [models.ts](https://github.com/earendil-works/pi/blob/v0.84.2/packages/ai/src/models.ts) | `getSupportedThinkingLevels`, `clampThinkingLevel` | 2026-08-27 | `b53c99c5fa787af57790fa1d971987faab7e7b917ab5cb985374db407a4d88dd` |
| npm `@earendil-works/pi-ai@0.84.2` tarball | bundled provider model JSON corpus | 2026-08-27 | `0262785a76b0eb2eec596cd8a7ab2ee23eef89d2ef1bb1211c4f0a1944dacf41` |
| `dist/providers/data/anthropic.json` inside pi-ai `0.84.2` | qualified row metadata (`reasoning`, supported levels) | 2026-08-27 | `e22c277e3a1ffddc3d2701b72787c9e0bd67b835de6b4cb806677b6b6a89a2f7` |
| `crates/swallowtail-adapter-pi/evidence/npm-shrinkwrap.json` | frozen npm integrity pin for `@earendil-works/pi-ai@0.84.2` | 2026-08-27 | `ec75b47c8032c9d22f9f8810840c6684cc44fa85f392cc83a3818e13e9e5208f` |
| Research 181 | SDK-sidecar route boundary | 2026-08-21 | frozen qualification |
| Contract 012, 017, 040, 044 | options, attachment, no-clamp, lifecycle events | repo | active contracts |
| `crates/swallowtail-adapter-pi/sidecar/pi-sdk-sidecar.mjs` | bootstrap factory, snapshots, event map | repo | source tag `0.3.3` |
| `crates/swallowtail-adapter-pi/src/sidecar/reasoning.rs` | closed admission table and validation | repo | cards 226-227 |

## Frozen Pi 0.84.2 Thinking-Level Semantics

Public vocabulary includes `off`, `minimal`, `low`, `medium`, `high`, `xhigh`,
and `max`. Default when absent is `medium`. `clampThinkingLevel` is silent;
Contract 040 forbids portable substitution on admitted rows.

For `anthropic/claude-opus-4-5` in tagged pi-ai `0.84.2`, supported levels are
exactly `off|minimal|low|medium|high`. Each is identity under clamp. `xhigh` and
`max` require explicit map entries on other models such as `claude-opus-4-7`
and are withheld.

Explicit bootstrap `thinkingLevel` overrides stored session state on new
attachment when provided. The sidecar runtime factory closes over bootstrap
`thinkingLevel` and re-applies it on every `createAgentSessionFromServices`
call after load/resume `session_switch`. `session.thinkingLevel` exposes the
post-clamp effective value; bootstrap and `state` snapshots stringify it.

## Static Membership Gate

| Gate | Finding |
| --- | --- |
| Preparation admission | Rust rejects foreign provider/model/mode before plan construction |
| Static metadata | `getModel(provider, id)` reads bundled pi-ai JSON; auth does not alter map membership |
| Identity clamp | admitted row levels are already in `getSupportedThinkingLevels` |
| Bootstrap dispatch | canonical lowercase `thinkingLevel` only when selected |
| Effective confirmation | bootstrap and post-switch `state` must equal requested mode before readiness |
| Omission | no bootstrap field; no effective-level comparison; no portable claim |

The full bundled corpus remains 1267 models; deliver-now does not embed it.
Only the one qualified row is admitted.

## Lifecycle Dispositions

| Lifecycle | Explicit re-declaration | Effective confirmation | Disposition |
| --- | --- | --- | --- |
| New session | bootstrap optional field | bootstrap `thinkingLevel` | **deliver now** for qualified row |
| Load | bootstrap closure + `session_switch` | bootstrap + post-switch state | **deliver now** |
| Resume | bootstrap closure + `session_switch` | bootstrap + post-switch state | **deliver now** |
| Fresh restoration | new prepared attachment/bootstrap | bootstrap + state | **deliver now** |
| Omission | no field | not compared | **current** — exact prior bytes/behavior |

## Deliver-Now Table

| Provider | Model | Requested mode | Lifecycle | Exact confirmation | Disposition |
| --- | --- | --- | --- | --- | --- |
| anthropic | claude-opus-4-5 | off | new, load, resume, fresh restoration | bootstrap/state `thinkingLevel` equals requested mode | deliver now |
| anthropic | claude-opus-4-5 | minimal | new, load, resume, fresh restoration | bootstrap/state `thinkingLevel` equals requested mode | deliver now |
| anthropic | claude-opus-4-5 | low | new, load, resume, fresh restoration | bootstrap/state `thinkingLevel` equals requested mode | deliver now |
| anthropic | claude-opus-4-5 | medium | new, load, resume, fresh restoration | bootstrap/state `thinkingLevel` equals requested mode | deliver now |
| anthropic | claude-opus-4-5 | high | new, load, resume, fresh restoration | bootstrap/state `thinkingLevel` equals requested mode | deliver now |

## Withheld Rows

| Provider | Model | Reason |
| --- | --- | --- |
| anthropic | claude-opus-4-7 | adds `xhigh`/`max`; not in closed table |
| * | * | any mode outside the five admitted levels |
| * | * | any provider/model outside the one qualified row |

## Wire / Behavior / Source-Tag Revision

No revision required. Optional bootstrap `thinkingLevel` and snapshot reporting
already exist on wire `v1`. Cards 226-227 add typed Rust admission, plan/request
agreement, bootstrap dispatch, and effective-state validation only.

## Contract Mapping

Admitted rows require `Capability::ReasoningSelection` with an exact
`ReasoningMode`, immutable plan/request agreement, canonical bootstrap
encoding, and bootstrap/state equality before readiness under Contracts 012,
017, and 040. Observed reasoning activity cannot substitute for effective-state
confirmation (Contract 044).

## Promotion

Research 228 promotes the bounded deliver-now table above. Cards 226-227
realize it. Feature matrix `reasoning_selection` becomes `Yes` for the one
closed `anthropic/claude-opus-4-5` family on exact Pi `0.84.2`.
