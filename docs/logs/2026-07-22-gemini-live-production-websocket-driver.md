# Gemini Live Production WebSocket Driver

Date: 2026-07-22

## Changed

- added the separate `swallowtail.gemini.live` realtime-media descriptor and
  production driver
- bound the exact preview facade, authorization-key profile, model, asymmetric
  PCM formats, manual activity, voice, thinking, transcript, and turn limits
- added host-leased raw WebSocket access with private query authentication
- added bounded response pumping for audio, transcript, usage, warning,
  failure, cancellation, deadline, and terminal evidence
- added one idle-boundary replacement using the latest private provider handle
- added joined two-generation cleanup before the sole credential release

## Evidence

The loopback opens two WebSocket generations under one immutable plan, sends
setup before input on each, resumes with the latest handle, closes the old
generation only after replacement confirmation, and sends each input once.
The provider warning and handle remain adapter-private and clear at handoff or
session close.

Missing handles, replacement setup loss, a second rollover warning, unexpected
disconnect, cancellation, and deadline all end the session without retry,
replay, reconnect, or fresh-session fallback. Cancellation and deadline report
provider stop as unconfirmed because the selected subset has no native cancel
frame.

## Validation

- all Gemini adapter tests pass; the installed Gemini CLI probe remains gated
  and ignored by default
- focused production loopback covers two-turn success and four failure groups
- `cargo check -p swallowtail-adapter-gemini --all-targets` passes
- `effigy doctor` remains at the inherited 19 oversized-file findings: seven
  errors and twelve warnings, with no new finding
- no credential, paid request, browser, device, or live provider was used

## Next

Card 097 is ready. Prove the unchanged realtime profile, bounded rollover, and
full failure matrix under local and remote-authoritative host identities, then
close roadmap 033 with one full repository validation round.
