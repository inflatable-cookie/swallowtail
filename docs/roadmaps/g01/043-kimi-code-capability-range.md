# 043 Kimi Code Capability Range

Status: completed
Owner: Tom
Updated: 2026-07-24

## Purpose

Prove that one harness driver can support exact release milestones whose
negotiated capabilities differ while preserving one public interactive-session
shape.

## Authority

- Research 028: Post-Forward-Compatibility Provider Coverage Selection
- Contract 012: Interactive Session Options And Callback Exchange
- Contract 015: ACP v1 Negotiation And Client Callbacks
- Contract 017: Provider-Owned Session Load, Replay, And Host Containment
- Contract 029: Interface Version Qualification And Compatibility
- Contract 032: Installed Executable Observation And Discovery
- Contract 034: Negotiated Harness Session Options

## Generation Runway

Keep g01 active. It contains 43 numbered roadmaps and remains inside the normal
30-50 roadmap range.

## Goals

- [x] Freeze exact Kimi Code `0.28.1` and `0.29.0` executable, ACP adapter,
      SDK, wire, option, and behavior evidence.
- [x] Add provider-neutral negotiated reasoning setup without a generic
      provider option map.
- [x] Observe one host-approved Kimi executable and corroborate the exact
      release during ACP initialization.
- [x] Publish two exact qualified segments plus unverified-newer execution.
- [x] Preserve the existing ambient persistent-session lifecycle when options
      are empty.
- [x] Prove both authoritative host topologies without live authentication.

## Execution Plan

- [x] Negotiated option records and Kimi range corpus: card 129.
- [x] Kimi version discovery, range dispatch, and reasoning setup: card 130.
- [x] Kimi range conformance and closeout: card 131.

## Cards

- `batch-cards/129-negotiated-session-option-records-and-kimi-range-corpus.md`
  — completed
- `batch-cards/130-kimi-version-discovery-range-and-reasoning-dispatch.md` —
  completed
- `batch-cards/131-kimi-capability-range-conformance-and-closeout.md` —
  completed

## Qualified Shape

- interface axis: `kimi-code.executable`
- baseline: exact `0.28.1`
- latest-qualified: exact `0.29.0`
- segment 1: `0.28.1`, legacy `off`/`on` reasoning option
- segment 2: `0.29.0`, declared effort levels plus legacy aliases
- no inferred releases between the exact segments
- stable exact releases above `0.29.0`: unverified newer through the latest
  qualified private behavior
- prereleases, malformed versions, `0.28.0`, older points, and runtime drift:
  incompatible

## Boundaries

- no installation, update, downgrade, executable search, or ambient fallback
- no container, sandbox, or containment requirement
- existing delegated access remains separate from installed discovery
- no generic ACP or provider configuration surface
- no model, mode, tool-policy, agent, provider, endpoint, or credential switch
- reasoning setup applies only to new sessions in the first proof
- empty options preserve existing load, resume, replay, write, and cancellation
  behavior
- no Nucleus or Soundcheck edit
- default QA uses deterministic fixtures only

## Planning Checkpoint

The baseline preserves the existing production proof. The latest boundary is
current stable. No support-floor, authentication, isolation, or topology policy
changes.

Cross-topology conformance preserves the existing persistent ACP lifecycle and
adds no generic provider configuration surface. Roadmap 044 returns to
provider-coverage evidence.
