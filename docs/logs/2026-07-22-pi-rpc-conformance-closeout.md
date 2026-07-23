# Pi RPC Conformance Closeout

Date: 2026-07-22

## Changed

- added deterministic callback deadlines owned by the Pi connection, one
  cancellation response on expiry, exact late-response rejection, and joined
  callback timer cleanup
- added production conformance under local and remote-authoritative host
  identities with exact provider, model, access, resource, target, and ambient
  posture agreement
- completed prompt, steering, follow-up, acknowledgement, callback, provider
  failure, retry drift, disconnect, malformed frame, mismatch, concurrency,
  cancellation, deadline, redaction, and cleanup-failure coverage
- closed roadmap 035 and compiled roadmap 036 plus cards 102-104 for the
  DeepSeek V4 direct-continuation lane

## Evidence

The Pi driver passes the unchanged long-lived RPC profile and the separate
Contract 028 assertion pack. Both host identities preserve the same public
driver seam. Prompt, steering, and follow-up retain their distinct scheduling
classes. A callback expiry sends one cancelled response without ending the
model turn; a later response fails with `swallowtail.pi.rpc.callback_expired`.

Provider success remains visible when process cleanup fails. Provider failure,
retry drift, disconnect, malformed input, and correlation mismatch keep
distinct safe diagnostics. No provider, model, authentication, retry, sandbox,
or fallback default enters the route.

Official DeepSeek material checked on 2026-07-22 still exposes exact V4 Flash
and Pro routes and retires the legacy aliases on 2026-07-24. Thinking-mode tool
calls require full assistant reasoning continuation on later requests. Card 102
must freeze that provider-private continuation boundary before a contract,
corpus, or driver proceeds.

## Validation

- 15 Pi adapter tests, 47 runtime tests, and 46 testkit tests pass in the
  focused batch: 108 total
- focused runtime, testkit, and Pi warnings-denied clippy passes
- full repository QA passes with a 469-test inventory: 466 pass and 3 gated
  installed/live probes remain ignored
- the first full test attempt exposed a pre-existing Gemini Live mock-server
  broken-pipe race; the exact failed case passed alone and the complete QA
  rerun passed without code changes
- `effigy doctor` retains the inherited 19 findings: 7 errors and 12 warnings

## Next

Card 102 is ready. Revalidate and promote the exact DeepSeek V4 direct-
continuation boundary. Do not implement a provider route before its shared
contract is active.
