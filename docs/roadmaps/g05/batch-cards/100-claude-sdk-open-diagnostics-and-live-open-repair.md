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

## Scope

1. **Surface the sidecar code.** `open_rejected` and every other
   command-level rejection carry the sidecar's failure code in the
   diagnostic (code set is the fixed sidecar enumeration; no message text,
   path, or account value crosses). Consumers see, for example,
   `open_rejected: model_mismatch`.
2. **Effective model as evidence, not equality.** The sidecar returns the
   SDK-reported `system.model` in the open response; `readiness()` records
   requested and effective model separately and publishes the effective
   value. Fail open only when the SDK reports no model or a model outside
   `supportedModels` when that list is available. Do not fail on a
   canonical id differing from a requested alias.
3. **Node newer-allowed at open.** Keep the pinned Node `22.23.2` as the
   qualified point but treat a newer Node that passes the sidecar floor as
   `UnverifiedNewer` on that axis rather than an `open_mismatch`, recording
   the observed version in readiness. Card 087 owns the full qualified-range
   redesign; this is the minimal patch-compatible relief.
4. **Live proof.** One operator-authorized live open against the real
   `@anthropic-ai/claude-agent-sdk` with a first-party subscription login,
   on Node 22.23.2 and on the Homebrew Node the consumer uses, recording
   the real `system.model` for the requested alias and the account
   projection. Provider-free fixtures stay the regression proof; the live
   run is the first-time evidence this route lacked.
5. Guide, matrix cells, `CHANGELOG.md` `[Unreleased]`, additive API
   baseline. One PR.

## Out Of Scope

Card 082's model change and effort surfaces (paused behind this card and
rebased after it); card 087's ranges; Bash, resume, MCP.

## Acceptance Criteria

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
