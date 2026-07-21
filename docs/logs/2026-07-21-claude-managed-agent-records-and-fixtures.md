# 2026-07-21 Claude Managed Agent Records And Fixtures

## Outcome

Completed card 077. The common records and dated offline corpus now settle the
Claude Managed Agents production boundary. Card 078 is ready.

## Changed

- added explicit durable provider retention and provider-managed recovery as
  independent policy and capability opt-ins
- added owned environment/session deletion authority and per-resource
  confirmed versus unconfirmed terminal truth
- reused bounded tool declarations and callback exchange for structured runs
  instead of creating Managed-Agents-specific common records
- added provider-neutral preflight fixtures that reject missing retention,
  recovery, or ownership before provider effects
- made every existing structured-run driver reject managed recovery or tools
  it does not implement
- froze the first-party API-key, endpoint, version, beta, pinned agent,
  session override, empty-host limited environment, message, custom callback,
  interrupt, event, history, failure, and deletion shapes
- added a fail-closed event parser and bounded exact-ID history reconciliation;
  preview deltas cannot establish output or terminal truth
- added a loopback transcript proving one session, one stream attachment,
  callback and interruption input, bounded history, and session-before-
  environment deletion

## Authority And Bounds

- `managed-agents-2026-04-01` at `api.anthropic.com`
- public Claude API key, API billing, and provider beta support only
- one operator-owned agent at one exact version and model
- one driver-owned cloud environment and one driver-owned session
- explicit empty `allowed_hosts`, MCP allowance false, package-manager
  allowance false, and no local container
- custom client tools only; no built-in tools, MCP, skills, files, vaults,
  memory, multiagent, external network, repository, retry, or fallback
- current `user.interrupt` API union; stale `interrupt` curl shape excluded
- no live account, credential, remote resource, external request, or paid work

## Validation

- focused core, runtime, testkit, Anthropic, Codex, Bedrock, llama.cpp, and
  OpenAI tests pass
- focused warnings-denied clippy passes
- full repository QA passes; Nextest reports 328 passed and three separately
  gated installed/live probes skipped
- one first Nextest run observed an existing Codex callback-deadline ordering
  race; the isolated test and the complete 328-test rerun passed
- `git diff --check` passes
- doctor remains at the inherited 19 oversized-file findings: 12 warnings and
  7 errors; the three new warning findings were split before closeout

## Remaining Risks

- the production HTTP/SSE lifecycle, callback wait, cancellation/deadline
  races, usage/rate projection, and joined remote cleanup remain card 078
- Managed Agents is beta; header, session override, environment, event, usage,
  rate, retention, or deletion drift requires a dated fixture delta
- a failed or ambiguous delete must remain degraded cleanup; local close cannot
  claim provider resource removal
- provider-managed rescheduling is not a Swallowtail retry and remains visible
- live authentication remains separately gated

## Continuation Record

Card 078 is the sole ready task. Promote the frozen protocol module into a
separately registered driver, preserve the exact subset, and prove every
terminal path deletes session then environment before lease release. Card 079
remains in bounds for cross-topology conformance and roadmap closeout.
