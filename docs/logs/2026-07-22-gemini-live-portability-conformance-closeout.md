# Gemini Live Portability Conformance Closeout

Date: 2026-07-22

## Changed

- ran the unchanged eleventh realtime-media profile against the Gemini lane
- kept planned rollover in its separate provider-neutral assertion pack
- extended production loopback across local and remote-authoritative host ids
- added latest-handle replacement, provider failure, unknown event, format
  drift, uncertain-session nonreuse, timer ordering, and cleanup-failure proof
- closed roadmap 033 and opened roadmap 034's provider-coverage checkpoint

## Evidence

Both topologies preserve the same configured instance, approved endpoint,
audience, authorization-key profile, preview facade, exact route and model,
asymmetric formats, maximum-one rollover, and credential lease. Two setup
handshakes carry one initial setup and one latest-handle replacement setup.
Each input audio chunk appears once.

The first response can receive `GoAway` while active and still complete before
handoff. Missing handle, replacement failure, a second warning, provider
failure, unknown event, format drift, disconnect, cancellation, deadline, and
credential cleanup failure remain separate. Interrupted or uncertain sessions
cannot accept later input. Cancellation and deadline remain provider-
unconfirmed.

## Validation

- focused Gemini production conformance: 10 tests pass
- `effigy qa --json` passes docs QA, formatting, workspace checks,
  warnings-denied clippy, all 443 runnable repository tests, and doc tests
- three installed or live probes remain gated and ignored by default
- doctor remains at the inherited 19 oversized-file findings: seven errors and
  twelve warnings, with no new finding
- `git diff --check` passes
- no credential, paid inference, browser, device, or live provider was used

## Next

Card 098 is ready. Refresh the remaining provider and transport evidence, rank
new runtime pressure, and compile the next exact proof or operator gate inside
g01.
