# 033 Gemini Live Realtime Portability Proof

Status: completed
Owner: Tom
Updated: 2026-07-22

## Purpose

Prove the shared realtime-media role against Gemini Live while adding one
explicit provider-planned connection rollover and no general reconnect or
consumer-resume behavior.

## Generation Runway

Keep g01 active. It now contains 33 numbered roadmaps and remains inside the
normal 30-50 roadmap range. This proof does not create g02.

## Contracts

- Contract 005: Integration Identity And Transport Diversity
- Contract 006: Execution Layer And Access Boundary
- Contract 009: Async Operation Lifecycle
- Contract 011: Runtime Conformance Profiles
- Contract 014: Hosted Transport, Credential, And Evidence Boundary
- Contract 016: Connection-Scoped Direct Sessions And Billed Cost
- Contract 020: Model Catalogue Observation And Availability Boundary
- Contract 026: Realtime Media Direct Session Boundary
- Contract 027: Planned Connection Rollover And Realtime Continuity

Research 021 selects the exact route. Gemini CLI ACP, Gemini Live, the
Generative Language API, Vertex AI, standard keys, authorization keys, and
ephemeral client tokens remain separate driver and access surfaces.

## Goals

- [x] Add the minimum provider-neutral bounded-rollover capability and request
      policy without changing disabled existing routes.
- [x] Freeze the current Gemini Live preview raw-WebSocket corpus for one exact
      server-to-server manual-audio subset.
- [x] Add a separately registered Gemini Live realtime-media driver without
      changing the Gemini CLI ACP driver.
- [x] Prove two serial turns across one planned connection replacement under
      local and remote-authoritative host identities.
- [x] Preserve private handle, query-secret, cancellation, failure, and joined
      two-connection cleanup truth.

## Execution Plan

- [x] Rollover records and Gemini Live corpus: card 095.
- [x] Gemini Live WebSocket driver: card 096.
- [x] Gemini Live portability conformance and closeout: card 097.

## Cards

- `batch-cards/095-realtime-rollover-records-and-gemini-corpus.md` — completed
- `batch-cards/096-gemini-live-websocket-driver.md` — completed
- `batch-cards/097-gemini-live-portability-conformance-and-closeout.md` — completed

## Bounded First Proof

One resource-free direct realtime session binds Gemini Live preview,
`v1beta` raw WebSocket, exact model `gemini-3.1-flash-live-preview`, one
project authorization-key lease, project billing, PCM16 mono 16 kHz input,
PCM16 mono 24 kHz output, `Kore` voice, minimal thinking, manual activity, and
output transcription.

The session permits two successful serial turns. After the first completed
turn, a deterministic provider warning and latest resumable handle trigger one
replacement connection. Setup must confirm the same provider session before
the second turn. There is no replay, retry, unexpected reconnect, consumer
resume, durable handle, or fresh-session fallback.

Cancellation and deadline close local transport and report provider stop as
unconfirmed. Old and replacement connection work joins before one credential
release.

## Completion Evidence

The production driver passes the unchanged realtime-media profile and the
separate bounded-rollover assertion pack. Local and remote-authoritative
fixtures preserve exact plan, access, preview, route, format, handle, event,
and cleanup identity across both connection generations.

The complete offline matrix distinguishes provider warning, missing handle,
replacement failure, rollover exhaustion, provider failure, unknown event,
format drift, unexpected disconnect, cancellation, deadline, and cleanup
failure. No path retries, reconnects, replays, exposes a handle, or claims
confirmed provider stop.

Full repository QA passes 443 tests. Three installed or live probes remain
separately gated and ignored. Doctor remains at the inherited 19 oversized-file
findings with no new finding.

## Stop Condition

Stop if the preview route or exact model disappears, authorization-key use is
not supported by the raw WebSocket, rollover requires public durable handles or
input replay, asymmetric formats cannot remain exact, an unexpected disconnect
must be treated as continuity, or deterministic fixtures require credentials,
paid inference, a browser, or an audio device.
