# 031 Realtime Provider Facades

Status: complete
Owner: Tom
Created: 2026-07-25
Milestone: `../011-specialized-runtime-facades.md`

## Objective

Add typed prepared connection facades for xAI WebSocket, OpenAI Realtime, and
Gemini Live.

## Governing Refs

- Contracts 014, 016, 020, 026-027, 029, and 037
- current realtime corpora
- card 023

## Scope

1. Prepare exact endpoint audience, credential source, route/model, formats,
   limits, and connection policy per adapter.
2. Bind connection open, response start/cancel, and close through each existing
   realtime role.
3. Preserve xAI billed ticks, OpenAI media/event semantics, and Gemini planned
   rollover behavior separately.
4. Keep device capture, playback, and consumer continuity downstream.
5. Expose connection invalidation and cleanup truth.

## Acceptance Criteria

- [x] no common method hides provider-specific realtime semantics
- [x] media formats and bounded chunks remain exact
- [x] rollover remains distinct from retry and consumer resume
- [x] parallel-turn and cancellation rules remain adapter-specific
- [x] duplex work joins before credential release

## Validation

- all three deterministic realtime corpora
- realtime-media and rollover conformance
- supported topology fixtures
- low-level regression suites

## Evidence

- xAI, OpenAI, and Gemini expose three adapter-local preparation functions and
  three different typed session values
- the prepared values retain exact access, endpoint, model, format, turn,
  cancellation, invalidation, rollover, and cleanup evidence
- 85 adapter tests pass; the existing Gemini live authentication probe remains
  separately ignored
- all three adapter packages pass warnings-denied lint, formatting, and public
  API declaration checks
- local and remote-authoritative fixtures exercise prepared open, native
  operation, invalidation, joined cleanup, and credential release
- current official provider evidence was rechecked on 2026-07-25

## Auto-Continuation

Yes. Continue to card 032.
