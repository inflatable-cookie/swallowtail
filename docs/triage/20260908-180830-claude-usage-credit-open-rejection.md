# Claude Usage Credit Open Rejection

Status: open intake
Owner: Chatterbox
Created: 2026-09-08

## Issue

The operator confirmed that the Claude account is currently out of usage
credit. This may explain Card 132's live
`open.failed.swallowtail.claude-agent.sdk.open_rejected`, but the immutable
capsule did not expose an account- or quota-class cause. The relationship is a
hypothesis, not a classification of that past result.

The harness should surface a bounded account or usage rejection when the
SDK/sidecar provides one. It must not collapse that observation into an opaque
construction failure, expose account identity or billing details, or infer
quota from an otherwise generic rejection.

## Current Evidence

- Operator-confirmed: the Claude account has no usage credit at present.
- Observed: Card 132 stopped at open with generic `open_rejected`; no turn,
  registered-tool dispatch, control attempt, or retry ran.
- Shipped in Card 144: the prepared open receipt carries failure stage, bounded
  sidecar subcode, provider-readiness truth, and cleanup disposition.
- Provider-free proof: an account rejection emitted by the frozen fake sidecar
  becomes `SidecarRejected` with subcode `account_unavailable`.
- Unknown: whether the exact native SDK path emits a quota/account subcode for
  exhausted usage, or collapses that state before the harness can classify it.

## Proposed Disposition

Do not authorize or run another Claude live gate while usage credit is known
exhausted. After credit is restored, revise the gate packet to consume the Card
144 receipt and require any account/quota rejection exposed by the sidecar to
remain visible as its exact bounded safe subcode. One fresh primary attempt may
then distinguish a passing route from a live-only provider/SDK rejection. No
automatic retry follows either result.

If the credited run still returns only a construction-class rejection, retain
both Contract 061 cells as unqualified and return the receipt to Chatterbox. Do
not invent a provider limitation or release claim.

## Next Check

Operator confirms restored Claude usage credit. Chatterbox then promotes the
fresh-gate card and packet, including the Card 144 receipt oracle, for separate
execution authorization.
