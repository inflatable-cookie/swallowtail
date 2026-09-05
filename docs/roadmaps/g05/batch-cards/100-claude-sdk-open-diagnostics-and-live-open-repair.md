# 100 Claude SDK Open Diagnostics And Live-Open Repair

Status: ready
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Milestone: `../032-v0-4-2-release-readiness.md`
Depends on: `v0.4.1`; the 2026-09-05 Bovine Desktop failure report (PR 97 head `17321cc9`); Contract 036 patch rules

## Defect

Bovine Desktop's first live open of `claude-agent.sdk` on `v0.4.1` failed
with `swallowtail.claude-agent.sdk.open_rejected` and nothing else. Two
facts from the `v0.4.1` source:

1. `sdk/driver/startup.rs` `open()` maps every unsuccessful sidecar `open`
   response to `open_rejected` and discards the sidecar's failure code, even
   though the sidecar sends it (`respondFailure`) and `wire/decode.rs`
   decodes it. Every sidecar rejection is therefore indistinguishable to a
   consumer. This is a route defect regardless of the root cause.
2. The sidecar requires the SDK init message's `system.model` to equal the
   requested model string exactly, and the plan pins Node to exact
   `22.23.2` and checks it in `readiness()`. Neither check had ever met a
   real SDK init before Bovine's run; card 080's proofs use a fake SDK that
   echoes the request. A canonicalised model id or a newer Node fails open.

## Root Cause (Bovine probe, 2026-09-05)

The Acowtancy Chatterbox ran the `v0.4.1` sidecar directly on this host
(Node 22.23.2, SDK 0.3.259, manifest 2.1.259). The open response was
`failure.code = construction_failed`: `sdk.query()` throws inside the spawn
hook. SDK 0.3.259 calls `spawnClaudeCodeProcess(options)` with ONE object
`{command, args, cwd, env, signal}` (`sdk.d.ts` `SpawnOptions`), while the
sidecar's `spawnNative(command, args, options)` is positional, so
`spawn(<object>, undefined, ...)` throws a `TypeError`. Node floor,
manifest, and module import had all passed. With the hook corrected the
native process spawns. The probe also saw `query.accountInfo()` return
`apiProvider: "firstParty"` with `apiKeySource: undefined`, so the
`account_not_subscription` check would fail next on a real subscription
login unless the evidence key is verified against 0.3.259. The fake-SDK
fixture never caught either, because it mirrored the sidecar's own
assumptions about the hook signature and the account shape.

## Scope

0. **Fix the spawn hook and the account projection.** `spawnNative` takes
   the SDK's single `SpawnOptions` object `{command, args, cwd, env,
   signal}` and forwards `signal` to `spawn`. Verify the subscription
   evidence field names against the frozen 0.3.259 `sdk.d.ts` (Research
   280 corpus) and project readiness from the fields the SDK actually
   returns. `apiProvider` is the only account gate; subscription fields are
   labelled observations, and `account_not_subscription` is retained only as
   a retired code. Update the fake SDK to call the hook with the real
   signature and to return `accountInfo()` in the real 0.3.259 shape, so the
   fixture can never again agree with a wrong assumption; add a test that
   fails if the hook is called positionally.
1. **Surface the sidecar code.** `open_rejected` and every other
   command-level rejection carry the sidecar's failure code in the
   diagnostic (code set is the fixed sidecar enumeration; no message text,
   path, or account value crosses). Consumers see, for example,
   `open_rejected: model_mismatch`.
2. **Effective model as evidence, not equality.** The sidecar reports the
   requested model and initialize-served supported list at open, then requires
   the first query's first yielded message to be `system/init`. That evidence
   supplies the effective model and capabilities; `readiness()` records
   requested and effective model separately and confirms only after the first
   turn. Fail with `init_missing` when init is absent or not first, keep
   `initialization_failed` distinct for an SDK init throw, and reject an
   effective model outside a non-empty `supportedModels` list when it is
   available. An empty list is unavailable and imposes no model constraint. Do
   not fail on a canonical id differing from a requested alias.
3. **Node newer-allowed at open.** Keep the pinned Node `22.23.2` as the
   qualified point but treat a newer Node that passes the sidecar floor as
   `UnverifiedNewer` on that axis rather than an `open_mismatch`, recording
   the observed version in readiness. Card 087 owns the full qualified-range
   redesign; this is the minimal patch-compatible relief.
4. **Live proof** (operator authorized 2026-09-05, "Both"; Chatterbox extended the
   grant the same day, with the operator informed, to further open-only probes under
   the same constraints: open, observe the handshake, close; no prompt, tool call, or
   write). One live open against the real
   `@anthropic-ai/claude-agent-sdk` with a first-party subscription login,
   on Node 22.23.2 and on the Homebrew Node the consumer uses, recording
   the real `system.model` for the requested alias and the account
   projection. Provider-free fixtures stay the regression proof; the live
   run is the first-time evidence this route lacked.
5. Guide, matrix cells, `CHANGELOG.md` `[Unreleased]`, additive API
   baseline. One PR.

## Second Layer (2026-09-05, after the first live probes)

With the spawn hook fixed, both authorized probes reached the native child
(first native record `control_response`, empty stderr) and then timed out
waiting for `system/init`. The frozen 0.3.259 declarations refute the spawn
shape as a cause. Ruling: the sidecar's open protocol is wrong for a
streaming-input query. The SDK serves the needed readiness (`supportedModels`,
`accountInfo`) from the `initialize` control exchange; `supportedCommands` is
unevidenced and not needed by this route,
and `system/init` arrives only after the first user message. Open must
take readiness from the initialize exchange and treat `system/init` as
first-turn evidence (cwd and effective-model confirmation, capabilities),
failing typed (`init_missing`) if it is not the first message of the first
turn. The fake SDK must reproduce that ordering. The 094 follow-up note the
worker added is ratified.

## Out Of Scope

Card 082's model change and effort surfaces (paused behind this card and
rebased after it); card 087's ranges; Bash, resume, MCP.

## Acceptance Criteria

- [ ] `sdk.query()` constructs against 0.3.259 with the object-form spawn hook, proved by a fixture that calls the hook the way the real SDK does
- [ ] account readiness projects from the real 0.3.259 `accountInfo()` shape
- [ ] every sidecar rejection reaches the consumer with its code
- [ ] a canonical model id no longer fails open; effective model is published
- [ ] Node newer than the pin passes open with an `UnverifiedNewer` record
- [ ] the live open succeeded once with the recorded evidence
- [ ] API diff additive; default profile unchanged

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:guides`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- the operator-authorized live open (item 4)

## Review Oracle

Invariant: a consumer can act on every open failure, and the route reports
what the runtime said rather than what was asked. Smallest counterexample: a
rejection without its sidecar code, or an effective model asserted from the
request.

## Auto-Continuation

No. Stop for exact-head review; the `v0.4.2` prepare follows.

## Result

- Provider-free repair complete: the frozen 0.3.259 object-form spawn hook and
  `signal` forwarding are covered by the fake SDK regression fixture; the
  real `AccountInfo` declaration was checked in the 0.3.259 `sdk.d.ts`, where
  the subscription-evidence fields are `subscriptionType`, `tokenSource`,
  and `apiKeySource`, with `apiProvider` kept as the first-party gate and the
  subscription fields projected as labelled presence observations. Every
  command rejection preserves its fixed sidecar code without
  forwarding message, path, or account data. Effective model evidence and
  requested/effective model separation are covered, as is newer-Node
  `UnverifiedNewer` readiness. Reproducible `sdk.d.ts` excerpts for
  `AccountInfo`, `SpawnedProcess`, and `SpawnOptions` are frozen under the
  owned 0.3.259 fixture corpus, and a unit drift test compares
  `COMMAND_FAILURE_CODES` with the Rust command-code enumeration.
- The protocol-order repair is provider-free and complete: open now consumes
  the SDK initialize exchange plus bounded `supportedModels()` and
  `accountInfo()` controls without awaiting `query.next()`. Open reports
  `requested-with-supported-list`; the first query requires
  `system/init` as its first yielded message and then returns `confirmed`
  readiness with cwd, effective model, and capabilities. Missing or reordered
  init is `init_missing`; an SDK throw remains `initialization_failed`.
  Fixtures cover both paths and preserve the canonical/effective model and
  account-check proofs.
- Provider-free validation passed: `cargo fmt -p
  swallowtail-adapter-claude-agent -- --check`; `effigy validate:focused
  swallowtail-adapter-claude-agent`; `effigy package:verify-affected
  swallowtail-adapter-claude-agent`; `effigy package:api`; `effigy qa:routes`;
  `effigy qa:guides`; `effigy qa:docs`; `effigy qa:northstar`; and `git diff
  --check`.
- The prior operator-authorized live item-4 pair used exactly two real 0.3.259
  opens, with no prompt, tool call, or write. Node 22.23.2 reached the real
  2.1.259 native process but produced no init response in the bounded window;
  the Homebrew Node 26.7.0 outcome was explicitly
  `initialization_failed`. No `system.model`, account field-presence, or
  rejection code was exposed by that pair, and no account values were
  recorded.
- Under the resumed extended authorization, one bounded open-only diagnostic
  was run on each runtime in order: Node 22.23.2, then Node 26.7.0. Both
  reached the real native child and timed out without an SDK `system/init`.
  Sanitized native-child evidence for each was empty stderr and one first
  native protocol record of type `control_response`; the SDK first-message
  list and account projection were empty/not reached. No prompt, tool call, or
  write was sent. No rejection requiring design judgment was exposed. Live
  acceptance remains unresolved and the card stays open pending a successful
  live init capture.
- After the provider-free validation, the one further extended-authorization
  live probe was run exactly once on Node `22.23.2` with the real SDK and native
  binary. The open-only control exchange returned the sanitized sidecar result
  `success: false`, `failure.code: account_not_subscription`; stderr was empty.
  No prompt, tool call, write, turn, `system/init`, model value, or account
  field presence was captured. This is the live design-review stop point: do
  not infer subscription evidence or run another live probe until the code is
  reviewed. The card remains open pending a successful live init capture.
- After retiring `account_not_subscription`, the one authorized Node
  `22.23.2` open-only probe ran exactly once with the real SDK and native
  binary. Open reported `requested-with-supported-list`, advanced through the
  first-party account gate and the initialize controls, then the sidecar
  exited during close before the normal `opened_and_closed` marker. Sanitized
  control evidence recorded
  `apiProvider` present (first-party gate passed), `subscriptionType` absent,
  `tokenSource` present, and `apiKeySource` absent. The initialize-served
  supported-model list contained 5 rows; only row field labels were retained,
  with no model or account values. No live turn, prompt, tool call, write, or
  `system/init` was run or captured. The card remains open pending successful
  live init evidence.
- The unrelated Stable process-spawning nextest job was rerun exactly once:
  `cargo nextest run --workspace --all-features --locked --profile
  ci-process` — 195 passed, 0 skipped, 0 failed.
- The additive API baseline and default read-only profile remain unchanged.
