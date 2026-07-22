# 031 OpenAI Realtime Media Direct Session Proof

Status: active
Owner: Tom
Updated: 2026-07-22

## Purpose

Realize bounded duplex audio transport through the provider-supported OpenAI
Realtime GA WebSocket without widening existing text sessions or taking device
and playback policy from consumers.

## Generation Runway

Keep g01 active. It now contains 31 numbered roadmaps and remains inside the
normal 30-50 roadmap range. Realtime media extends the current generation; it
does not create g02.

## Contracts

- Contract 005: Integration Identity And Transport Diversity
- Contract 006: Execution Layer And Access Boundary
- Contract 009: Async Operation Lifecycle
- Contract 011: Runtime Conformance Profiles
- Contract 014: Hosted Transport, Credential, And Evidence Boundary
- Contract 016: Connection-Scoped Direct Sessions And Billed Cost
- Contract 020: Model Catalogue Observation And Availability Boundary
- Contract 026: Realtime Media Direct Session Boundary

Research 020 selects the exact route. OpenAI public API, ChatGPT, Codex,
Realtime WebSocket, WebRTC, SIP, and ephemeral client credentials remain
separate surfaces and access boundaries.

## Goals

- [x] Realize provider-neutral realtime media identity, format, chunk, input,
      event, role, lifecycle, preflight, and conformance records.
- [x] Freeze the current OpenAI Realtime GA WebSocket corpus for one exact
      audio-only manual-turn subset.
- [ ] Implement a separately registered OpenAI Realtime media driver without
      changing the background Responses driver.
- [ ] Prove two serial audio turns, native cancellation, connection failure,
      redaction, and joined cleanup under local and remote-authoritative hosts.

## Execution Plan

- [x] Realtime media records and OpenAI corpus: card 091.
- [ ] OpenAI Realtime WebSocket driver: card 092.
- [ ] Realtime media conformance and closeout: card 093.

## Cards

- `batch-cards/091-realtime-media-records-and-openai-corpus.md` — completed
- `batch-cards/092-openai-realtime-websocket-driver.md` — ready
- `batch-cards/093-realtime-media-conformance-and-closeout.md` — planned

## Bounded First Proof

One resource-free direct interactive session binds the public OpenAI Realtime
WebSocket, API-key access, API usage billing, exact model
`gpt-realtime-2.1`, and fixed mono PCM input and output formats. It permits two
successful serial manual audio turns and one active response.

The consumer supplies already encoded bounded chunks. Swallowtail appends and
commits one logical input, starts one response, and emits ordered audio,
transcript, usage, rate, request, and terminal evidence. It does not capture,
play, convert, resample, or infer played position.

Cancellation and deadline use one native response-cancel request when
possible, then end the session after bounded acknowledgement and joined
connection cleanup. There is no reconnect, resume, replay, retry, storage, or
playback-truncation claim.

## Later Runway

Gemini Live is the first later realtime portability candidate because it adds
preview session resumption and constrained ephemeral credentials. Grok Build,
DeepSeek, and Z.AI remain provider-breadth candidates. Cursor and remote ACP
remain behind policy and protocol gates.

## Stop Condition

Stop if media needs a generic byte tunnel, existing text session traits must
silently widen, played position is inferred, provider cancellation cannot be
distinguished from local close, the exact format or model cannot be bound, or
deterministic fixtures require a credential, audio device, or paid request.
