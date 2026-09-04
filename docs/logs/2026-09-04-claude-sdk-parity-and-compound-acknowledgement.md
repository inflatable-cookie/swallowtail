# Claude SDK Parity Roadmap And Compound Acknowledgement Accepted

Date: 2026-09-04
Roadmaps: `../roadmaps/g05/029-claude-sdk-interactive-parity.md`,
`../roadmaps/g05/009-contract-061-consumer-projection-realization.md`

## Claude SDK parity

The Bovine Desktop requirement (Acowtancy Chatterbox handoff; its triage note
is pruned into g05.029) ruled that a read-only `claude-agent.sdk` session
does not meet the operator's stated bar: Claude must match Paseo and T3 Code.
The operator confirmed the cut on 2026-09-04. g05.029 runs nine items in the
consumer's priority order. Card 080 (read-write tools under per-call
admission on a read-write lease, permission mode at open and mid-session) is
ready and is the `v0.4.1` carrier; the items are additive under Contract 036,
so no minor is forced. Every write runs under an explicit `AmbientHost`
posture with no bounded-filesystem claim, per Contracts 017 and 023. Hosted
OAuth, API-key routes, and Bedrock stay out of scope. Bovine's interim is
Codex interactive plus one-prompt read-write Claude ACP runs.

## Compound acknowledgement

The operator accepted card 076's generic design: per-half acknowledgement
state (absent, effective with the exact token, rejected with the exact
token, terminally not dispatched), reasoning-first order preserved, no
pending state. Contract 061 is amended additively; card 079 realizes it in
runtime and testkit; card 034 reopens after card 079 merges.

## Next

Coordinator dispatches cards 079 and 080 concurrently with the active
lanes. When card 080 merges, Chatterbox compiles the `v0.4.1`
release-readiness roadmap.
