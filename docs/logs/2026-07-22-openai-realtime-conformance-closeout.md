# 2026-07-22 OpenAI Realtime Conformance Closeout

## Change

Card 093 proves the production OpenAI Realtime driver against the eleventh
provider-neutral profile and closes roadmap 031.

The common profile now records its existing separation of usage, rate, quota,
and request-correlation observations. Adapter fixtures add local and remote-
authoritative exact-plan assertions plus parallel rejection, provider failure,
format drift, disconnect, cancellation uncertainty, timer lifecycle, and
cleanup-failure coverage. No provider branch enters the shared profile.

The disconnect case exposed one production defect: a WebSocket reset without a
close frame was classified as a provider failure. Transport read loss now
becomes disconnected runtime truth. Protocol parsing and provider `error`
events remain separate failures.

## Boundaries Preserved

- one exact public OpenAI API-key audience and `gpt-realtime-2.1` route
- fixed PCM16 24 kHz mono media and 32 KiB chunks
- two serial manual turns and one active response
- no device, playback, truncation, WebRTC, reconnect, resume, or quota
  inference claim
- provider cancellation acknowledgement remains confirmed, raced, or
  unconfirmed rather than inferred from local intent
- tasks, timers, and blocking connection work join before credential release

No credential, live provider, paid request, audio device, or external network
inference was used.

## Evidence

- all 28 OpenAI adapter tests pass
- focused warnings-denied clippy passes
- full `effigy qa` passes with 430 tests
- three existing installed or live probes remain ignored by default
- Effigy doctor remains at 19 inherited findings: 12 warnings and 7 errors

## Continuation

Roadmap 032 and card 094 form the post-realtime coverage checkpoint. They
revalidate Gemini Live and the remaining harness, direct, protocol, and runtime
candidates before another provider is selected.
