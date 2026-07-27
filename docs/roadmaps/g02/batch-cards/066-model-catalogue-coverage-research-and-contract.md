# 066 Model Catalogue Coverage Research And Contract

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../021-model-catalogue-coverage.md`

## Objective

Reclassify every catalogue `No` using current authoritative evidence before
adding another driver role.

## Governing Refs

- Research 042
- Contracts 014, 020, 029, 037
- provider solution feature matrix

## Scope

1. Verify Pi RPC catalogue behavior at the qualified baseline and current
   maintained release.
2. Distinguish dedicated, attached, hosted, control-plane, session-negotiated,
   caller-supplied, and not-applicable model evidence.
3. Promote the source and timing rules into Contract 020 and architecture.
4. Sequence implementation without changing realized CSV claims.

## Acceptance Criteria

- [x] all thirteen `No` rows have an evidence-backed disposition
- [x] Pi `0.80.10` is proven to expose `get_available_models`
- [x] ACP model selectors do not become hidden session creation
- [x] serving-only lifecycle does not become a catalogue
- [x] current provider sources and compatibility posture are recorded
- [x] no runtime behavior, provider effect, credential, or publication occurs

## Evidence

- Research 042 identifies six definite solution-row conversions after
  implementation.
- Contract 020 records catalogue source and timing.
- Architecture records the route-free Pi catalogue and ACP separation.
- Roadmap g02.021 sequences the implementation.

## Auto-Continuation

Yes. Continue to card 067.

