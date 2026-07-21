# 054 xAI WebSocket Conformance

Status: complete
Owner: Tom
Updated: 2026-07-20
Milestone: `../017-xai-responses-websocket-proof.md`

## Objective

Prove the first provider-neutral connection-scoped direct-session profile and
close the xAI WebSocket proof.

## Governing References

- `../../../contracts/011-runtime-conformance-profiles.md`
- `../../../contracts/014-hosted-transport-credential-and-evidence-boundary.md`
- `../../../contracts/016-connection-scoped-direct-sessions-and-billed-cost.md`
- `../017-xai-responses-websocket-proof.md`

## Scope

- reusable direct-session conformance profile
- local and remote-authoritative host identity
- first/chained turns, cost, provider failure, invalid continuation,
  connection limit, cancellation, deadline, disconnect, redaction, and cleanup
- roadmap, front-door, and closeout evidence

## Out Of Scope

- live credential as a default test
- Kimi ACP or owned serving implementation

## Ordered Steps

1. Add a provider-neutral connection-scoped direct-session profile.
2. Run it against the xAI driver under local and remote-authoritative hosts.
3. Prove terminal-event closure and joined endpoint/credential cleanup.
4. Add a separately gated live probe only if an explicit credential reference
   is available without ambient scanning.
5. Run full QA once for the completed three-card batch.
6. Close roadmap 017 and re-rank Kimi ACP versus owned llama.cpp.

## Acceptance Criteria

- [x] common direct-session profile passes without provider identity
- [x] no retry, reconnect, storage, or resume occurs
- [x] exact billed cost remains turn-scoped evidence
- [x] full QA passes or failures are recorded honestly
- [x] one next provider-coverage task is explicit

## Validation

- focused conformance tests
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Evidence Required

- focused and full test counts
- local/remote topology results
- credential cleanup, redaction, and no-detached-work results
- remaining provider-auth, versioning, and protocol risks

## Stop Conditions

- conformance requires provider-specific behavior in core or testkit
- cancellation or connection-limit cleanup cannot be joined deterministically

## Auto-Continuation

No. Return to the roadmap 017 planning checkpoint.

## Evidence

- an eighth provider-neutral profile distinguishes connection-scoped direct
  sessions from one-shot hosted APIs and harness sessions
- the profile proves resource absence, active-turn interruption, no resume,
  scoped endpoint and credential acquisition, joined task and connection close
  before credential release, exact turn/route/access/attempt cost, and local or
  remote-authoritative host identity
- the xAI adapter claims the exact active-turn interruption constraint and runs
  two chained turns under both topology identities
- adapter fixtures close the event stream at terminal, retain the credential
  through both turns, release it once after connection cleanup, and expose no
  prompt, output, endpoint, secret, response id, or provider payload through
  stable formatting
- 46 focused testkit and xAI tests pass; focused strict clippy passes
- full `effigy qa` passes with 227 tests; the Gemini and OpenCode installed
  probes remain separately gated and ignored
- doctor remains at the pre-existing 19 findings: 12 warnings and 7 errors
- no explicit live xAI credential reference was supplied, so no live probe or
  ambient credential scan was added

## Continuation

Roadmap 017 is complete. Research 005 keeps Kimi Code ACP ahead of owned
llama.cpp because a second agent can test ACP portability, load/resume, replay,
write callbacks, and delegated-login authority. Roadmap 018 and cards 055-058
are in bounds. Card 055 is ready; implementation remains paused behind its
evidence and contract gate.
