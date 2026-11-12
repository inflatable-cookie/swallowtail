# claude-agent-sdk-v2 corpus

The contract for the distinct SDK behavior revision that preserves an inherited
managed `permissions.denyInsteadOfPrompting` denial. Tom's ruling in decision
`d5a14f90` (v2) chooses preservation over refusing an affected pair; the planner
relayed it as `task232` brief v2.

- `managed-denial-contract.json` is the reviewable contract: the ruling
  reference, the selected option surface it applies to, the four consumer
  guarantees, what the bytes still cannot prove, and the explicit statement that
  no claim moves.
- The guarantees are pinned by fake regressions in
  `tests/claude_agent_sdk_native_proof_harness.rs`
  (`validate:claude-agent-sdk-native-proof-harness`): the `managed-deny` peer
  proves no fabricated permission request, no admission claim, and a completed
  turn; the `permission` peer answered `deny` proves a mediated denial is
  distinguishable by its callback and recorded decision.

## Why a distinct revision

The setting first appears at SDK `0.3.319`. The administrator tier is not among
the tiers `settingSources` selects, so the sidecar's `[]` does not exclude it;
the SDK's own parent tier stays empty because `Options.managedSettings` is never
set. A managed denial therefore answers a prompt-requiring call without a
`can_use_tool` request. A consumer that reads "no callback" as permission
granted would be wrong, so the route needs a revision that names the correct
reading instead of silently inheriting the v1 expectation.

`claude-agent.sdk-v1` remains the revision every maintained claim uses, and the
package and native segments stay `0.3.284..=0.3.317` and `2.1.284..=2.1.317`.
The v2 revision is declared for the probe admission only; it becomes a claim
revision when the native proof supports an extension.

## What this corpus is not

It is not evidence about a vendor binary. Whether a real native honours the
administrator tier, and whether that honouring bypasses the callback on every
tool class, needs the original-artifact run the probe ledger describes, which is
still unauthorised.
