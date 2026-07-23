# 104 DeepSeek V4 Driver Conformance And Closeout

Status: completed
Owner: Tom
Updated: 2026-07-22
Milestone: `../036-deepseek-direct-continuation-proof.md`

## Objective

Implement and prove the exact DeepSeek V4 direct-continuation driver, then
close roadmap 036.

## Readiness Gate

Card 103 must complete with an active shared contract, pure preflight, an
unchanged or explicitly extended conformance shape, and a frozen exact corpus.

## Scope

- separately registered production driver for the exact selected facade and
  V4 model
- host-approved endpoint and credential leases
- bounded HTTP/SSE attempts around consumer-executed tool calls
- provider-private continuation capture, replay, zeroization, and redaction
- exact usage, cache, rate, quota, finish, error, cancellation, deadline,
  disconnect, and cleanup evidence
- local and remote-authoritative topology conformance
- full repository QA, exact inventory, roadmap closeout, and next selection

## Acceptance Criteria

- [x] no provider, model, facade, endpoint, auth, retry, or tool fallback occurs
- [x] consumer tool authority and execution stay outside Swallowtail
- [x] provider continuation is replayed only into its bound route and attempt
- [x] raw continuation never appears in stable events or diagnostics
- [x] all stream, timer, endpoint, credential, and continuation work joins
- [x] production passes the selected common profile under both host identities
- [x] full QA passes or failures are recorded honestly

## Completion Evidence

- `swallowtail.deepseek.direct` registers separate catalogue and direct-session
  roles for the exact dated OpenAI-format facade and V4 Pro route.
- Host-approved `/models` and `/chat/completions` work holds one audience-bound
  API-key lease across the session. No `/v1`, alias, alternate facade, retry,
  provider tool, or automatic continuation path exists.
- One buffered tool attempt pauses on a bounded direct-tool exchange. Only the
  consumer's exact correlated result authorizes the SSE continuation attempt.
  A later user turn replays bounded private history into the same route.
- Deterministic production tests cover exact three-request replay, per-attempt
  usage and cache hit/miss evidence, finish and request correlation, account-
  concurrency failure, disconnect, active-stream cancellation, tool-wait
  deadline, third-turn rejection, redaction, and credential-last cleanup.
- The unchanged locally continued direct-session profile passes under local
  and remote-authoritative execution hosts.
- Full repository QA passes with a 489-test inventory: 486 run and pass; three
  installed/live probes remain ignored. Doctor retains the inherited 19
  findings: seven errors and twelve warnings.

## Auto-Continuation

No. Close the exact direct proof before selecting later provider breadth.
