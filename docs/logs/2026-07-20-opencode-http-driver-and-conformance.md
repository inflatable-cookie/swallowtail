# OpenCode HTTP Driver And Conformance

Date: 2026-07-20
Roadmap: g01 012
Cards: 040-041

## Outcome

- implemented the first non-Codex production adapter
- kept OpenCode provider authentication delegated through a scoped credential
  lease; no provider secret enters the driver
- bound catalogue and sessions to the exact host-approved endpoint, audience,
  execution host, provider, model, and `1.14.48` server version
- mapped deny-first read-only sessions without config, auth, share, delete,
  server-dispose, resume, write, tool, or fallback behavior
- ran libcurl only in host `BlockingWorkService` jobs
- placed every bounded SSE reader under one joined host task
- added a sixth synthetic conformance profile for attached network harnesses
- added an ignored, explicit-endpoint installed probe task

## Evidence

- 19 OpenCode tests cover request shape, catalogue translation, ordered output,
  duplicate idle, foreign sessions, cancellation, timeout, provider failure,
  disconnect, unknown events, redaction, rejection before network work, and
  attached close
- local and remote-authoritative host identities use the same driver surface
- dependency scan contains libcurl, futures channels, serde, URL parsing,
  core, and runtime; it contains no Tokio, global executor, or process client
- full `effigy qa` passes with 150 tests
- `effigy doctor` retains only the known structural baseline: 19 findings,
  including 7 errors

## Current Limits

- server Basic auth, resume, write access, provider requests, tools, and
  non-default session options remain unsupported
- the production driver accepts only the frozen six-route `1.14.48` subset
- default QA performs no live authentication or provider inference
- the installed probe requires an operator-started unauthenticated endpoint

## Next

Execute card 042. Freeze the public Anthropic Models and Messages surface from
current provider documentation. Keep Claude Code subscription authentication
outside the public API lane and make unknown SSE behavior explicit in fixtures.
