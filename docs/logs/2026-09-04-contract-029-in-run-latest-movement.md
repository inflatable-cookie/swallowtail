# Contract 029 In-Run Latest Movement

Date: 2026-09-04
Contract: `../contracts/029-interface-version-qualification-and-compatibility.md`

## Decision

Kimi Code and Antigravity both published new stables during one-family runs
on 2026-09-04, and the strict "latest moved, stop and ask" rule turned each
into an operator escalation. The operator accepted the Chatterbox
recommendation: when official latest moves before the identity commit lands,
the worker adds the new stable as a further hop, recomputes its identity from
official artifacts, and extends the hop-by-hop ledger. It stops only when an
added hop changes a selected surface, capability, or process authority, is a
major-line reset, or disagrees across channels. A move after the identity
commit stays `UnverifiedNewer` until the next checkpoint.

## Surfaces

Contract 029 gains the In-Run Latest Movement subsection. The
`version-currentness` skill hard rule and reference, the checkpoint guide,
g05.027, card 071, and the standing-lane queue paragraph now carry the rule.
Identity-before-claim and the mandatory pre-push recheck are unchanged.

## Next

Card 071 runs under the new rule. No other lane changes.
