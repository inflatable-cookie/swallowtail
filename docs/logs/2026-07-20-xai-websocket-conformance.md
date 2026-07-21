# xAI WebSocket Conformance

Date: 2026-07-20
Roadmap: 017
Card: 054

## Changed

- added the eighth provider-neutral conformance profile for connection-scoped
  direct sessions
- separated this shape from hosted one-shot direct APIs and harness-managed
  interactive sessions
- bound resource absence, active-turn interruption, endpoint and credential
  leases, exact turn cost, no resume, topology, and cleanup order
- ran the profile against xAI under local and remote-authoritative host ids
- closed roadmap 017 and re-ranked Kimi ACP ahead of owned llama.cpp

## Evidence

- 46 focused testkit and xAI tests pass
- full `effigy qa` passes with 227 tests
- Gemini and OpenCode installed probes remain gated and ignored by default
- each topology completes two private chained turns and closes its event stream
- billed cost remains scoped to turn, route, access profile, and attempt
- one endpoint and credential acquisition spans both turns; task join and
  connection close precede one awaited credential release
- prompt, output, endpoint, secret, response id, and provider payload remain
  absent from stable formatting and diagnostics
- doctor remains at the pre-existing 19 findings: 12 warnings and 7 errors
- no live credential, installed xAI client, or external inference request was
  used

## Risks

- xAI WebSocket evidence remains a dated provider-guide snapshot rather than a
  versioned protocol release
- live bearer acceptance, provider lifetime timing, and service-side close
  behavior remain separately gated
- the first driver has no model catalogue, recovery, tools, storage, or resume
- Kimi's persistent-session, replay, write, and delegated-login boundaries must
  be contracted before its fixtures or driver begin

## Continuation

Roadmap 018 is active. Card 055 is ready for current Kimi Code ACP evidence and
persistent-session authority. Cards 056-058 remain in bounds behind that gate.
Owned llama.cpp remains the next ranked coverage proof after Kimi.
