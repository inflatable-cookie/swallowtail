# 2026-07-22 OpenAI Realtime WebSocket Driver

## Change

Card 092 adds `OpenAiRealtimeDriver` as a separate production role inside
`swallowtail-adapter-openai`. It does not widen or alter the existing
background Responses driver.

The driver binds one host-approved `api.openai.com` endpoint and public API-key
lease to the exact `gpt-realtime-2.1` route. One blocking worker owns the
server-to-server WebSocket, all client writes, all provider reads, and the safe
handshake request reference. Session configuration fixes manual PCM16 24 kHz
mono input and output, 32 KiB chunks, no tools, and maximum two serial
responses.

Input append and commit remain separate. Response tasks emit ordered start,
request correlation, rate, audio, transcript, usage, and terminal records
through bounded channels. Known GA structural lifecycle events are recognized;
unknown semantics still fail closed. Raw endpoint, secret, provider ids,
frames, audio, and transcript content remain outside stable diagnostics.

## Currentness Delta

The current official client-event schema makes `response_id` optional on
`response.cancel` and says omission cancels the active response in the default
conversation. The frozen client corpus and production driver now use that
form. It preserves one native cancel even when cancellation races
`response.created`.

- https://developers.openai.com/api/reference/resources/realtime/client-events#response.cancel
- https://developers.openai.com/api/docs/guides/realtime-websocket
- https://developers.openai.com/api/docs/guides/realtime-conversations

No credential, paid request, live model, browser, WebRTC client, device, or
audio hardware was used.

## Evidence

- two serial responses pass under local and remote-authoritative execution-host
  identities
- native cancellation and deadline each send at most one cancel frame, retain
  confirmed provider acknowledgement, close the session, and prohibit reuse
- unknown provider semantics and preflight media drift fail without leaking
  provider payloads or reaching unauthorized access effects
- response tasks and blocking connection work join before the sole credential
  release
- all 23 `swallowtail-adapter-openai` tests pass
- focused warnings-denied clippy passes
- `git diff --check` passes
- Effigy doctor remains at the inherited 19 findings: 12 warnings and 7 errors

## Continuation

Card 093 remains in bounds. It runs the eleventh provider-neutral profile
against the production role, closes remaining provider-failure and cleanup
cases, performs full repository QA, and closes roadmap 031. It is the sole next
task and does not require live authentication.
