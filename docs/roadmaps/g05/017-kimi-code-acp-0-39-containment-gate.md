# g05.017 Kimi Code ACP 0.39.x Containment Gate

Status: planned; strict-paused on one operator decision; no direction accepted
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Depends on: Contracts 010, 015, 017, 023, 029; Research 011, 259, 270;
completed g05.016 cards 041-042
Vision tags: process authority, route currentness, Kimi Code, compatibility

## Purpose

Hold `kimi-code.acp` above `0.38.0` on exactly one operator decision, with the
authority failure path re-derived and the choice space closed.

g05.016 recorded the claim outcome: exact `0.39.0` and `0.39.1` are excluded
and classify `Incompatible` because the agent-core-v2 ACP terminal runner
spawns local host processes under the `terminal: false` capabilities the route
advertises, and no adapter or runtime control contains that spawn. It did not
record a direction. This milestone compiles the direction question and stops.

The
[containment and mediation gate](../../triage/2026-09-01-kimi-code-acp-0-39-containment-and-mediation-gate.md)
holds the re-derived failure path, the actor ledger, the mutually exclusive
choices, the per-choice analysis, the proved uncertainties, the review oracle,
and the single question. It states a recommendation as analysis only. That
recommendation is deliberately absent from this file, from the front doors,
from the standing lane, and from every contract.

## Runway

1. The gate is compiled. The lane is paused.
2. The operator answers the single question the gate returns.
3. The answer selects one follow-on card shape. The gate names one per
   direction; none is compiled here, because compiling one would choose the
   direction. Every shape includes the `QualifiedOnly` posture move, because
   the cap is not real without it.
4. The fresh all-route Contract 029 currentness checkpoint runs serially after
   the answer is recorded, not before and not alongside.

## Boundary

Planning only. No Rust, no manifest, no fixture, no matrix, no guide, no
claim change, no contract amendment, no behavior revision, no public API
change, and no implementation card.

No provider or model call, authentication, install, host mutation, live probe,
or execution of a downloaded binary.

`kimi-code.local-server`, `kimi-code.headless`, `kimi-platform.chat`, every
second family, the Gemini deferral, g05.009, card 034, and the 249 proved /
518 remaining projection counts stay untouched.

## Choices Compiled, Not Chosen

Exactly one of these governs. There is no sub-choice.

| Id | Direction |
| --- | --- |
| A1 | Cap `kimi-code.acp` at `0.38.0` permanently under `QualifiedOnly`, with no re-open trigger. |
| A2 | Cap at `0.38.0` indefinitely under `QualifiedOnly`, with one artifact-level upstream re-open trigger recorded in the currentness lane. |
| B | Fund `HostEnforced` execution-host containment, starting with an artifact-feasibility experiment against the exact `0.39.1` binary, while the `QualifiedOnly` cap holds throughout. |

All three move the claim's newer-version posture to
`InterfaceNewerVersionPosture::QualifiedOnly`. That is not a separate question.
The ACP claim binds `AllowUnverified` today, so current `main` is safe only for
the exact known exclusions: any other published point above `0.38.0` falls
through to the unverified-newer path and would be admissible before a
checkpoint could react. The chosen direction's follow-on claim card closes that
future-release race. A2's trigger authorizes a fresh identity run and a fresh
claim decision; it never authorizes automatic admission and does not restore
`AllowUnverified`.

Not choices. Adapter or runtime mediation while the route still advertises
`terminal: false` is impossible, not deferred: under that capability the agent
issues no terminal request, so no message exists to mediate. Requalification
from wire stability, process ownership, capability omission, `AmbientHost`, or
a test-only wrapper is rejected outright. A cap that keeps `AllowUnverified`
and adds exclusions release by release is internally inconsistent and rejected.
Negotiated terminal execution cannot close `0.39.1` alone, because the runner
also falls back for any invocation that is not the interactive Bash tool, and
it cannot be selected without B or an upstream change; the gate records B+C and
C-plus-upstream as later designs only. The gate's review oracle holds every one
of these rejections.

## Batch Cards

None. Each direction implies a different first card, so compiling one would
silently accept that direction. The gate names the smallest follow-on shape
for each.

## Acceptance

- [x] the authority failure path is re-derived from current source, not
      inherited from the card 041 summary
- [x] every current actor is classified for observe, prevent, mediate, cancel,
      clean up, and attest
- [x] the governing choice set is exactly three and genuinely mutually
      exclusive, and choices hiding materially different authority, claim, or
      API consequences are split
- [x] a shape that cannot be selected without another shape is not a member of
      the set
- [x] impossible, dishonest, internally inconsistent, and incomplete shapes are
      marked as such and kept out of the set
- [x] the recommendation is analysis and appears in no status, contract, lane,
      or pointer
- [x] whether existing seams can express containment without a shared public
      type is decided, with the residual uncertainties proved and returned as
      operator scope
- [x] a review oracle blocks containment claimed from process ownership, wire
      stability, `terminal: false` omission, `AmbientHost`, or a test-only
      wrapper
- [x] production claims, code, manifests, matrices, fixtures, contracts,
      architecture, PAPERCUTS, and existing logs are unchanged
- [ ] the operator answers the single question

## Stop Conditions

- any direction recorded without a recorded operator answer
- an implementation card compiled before the answer
- `0.39.0` or `0.39.1` requalified, or
  `KIMI_CODE_LATEST_QUALIFIED_VERSION` raised above `0.38.0`
- a cap recorded that leaves the claim on `AllowUnverified`
- negotiated terminal execution reintroduced as a governing choice
- the all-route currentness checkpoint started before the answer

## Auto-Continuation

No. This lane is strict-paused on the operator decision.
