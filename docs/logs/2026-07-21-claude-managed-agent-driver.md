# 2026-07-21 Claude Managed Agent Driver

## Changed

- added a separately registered Anthropic Managed Agents remote-harness driver
  over host-approved REST/SSE blocking work
- bound the exact first-party endpoint audience, API key, API version, beta
  header, provider agent id and version, model route, deadline, durable
  retention, managed recovery, and one reattachment before effects
- added opaque provider-agent/version records to configured-instance preflight
  and explicit run-or-turn callback ownership to the shared callback exchange
- validated the operator-owned agent without mutation, then created one
  limited-network environment and one session with no files, built-in tools,
  MCP, skills, vaults, memory, multiagent, package access, or external network
- transported declared custom tool callbacks without executing them; unknown,
  mismatched, duplicated, late, failed, or oversized callback paths fail
  explicitly
- consumed authoritative persisted events, kept rescheduling as provider-
  managed recovery, performed one history reconciliation and one reattachment,
  and never created a replacement session or Swallowtail inference attempt
- projected cumulative usage plus safe request and rate evidence
- closed active input on cancellation or deadline, sent `user.interrupt`,
  deleted session before environment, preserved per-resource deletion truth,
  released the credential, and joined the scoped task

## Evidence

- nine production-driver loopback tests cover success, callbacks,
  rescheduling, reconnect, cancellation, deadline, provider failure, deletion
  ambiguity, redaction, request order, and credential release
- 181 affected-crate tests pass across Anthropic, core, runtime, testkit, and
  Codex; the callback/deadline race also passes five repeated isolated runs
- focused warnings-denied clippy and `git diff --check` pass
- `effigy doctor` is restored to the inherited 19 oversized-file findings: 12
  warnings and 7 errors, with no finding added by this batch
- fixtures use loopback HTTP only; no live credential, provider account,
  remote sandbox, external request, or paid inference was used

## Remaining Risks

- Managed Agents remains a provider beta. Endpoint, header, environment,
  session override, event, usage, rate, interruption, retention, or deletion
  drift needs a dated corpus delta before production mapping changes
- general organization rate-limit APIs still do not establish Managed Agents
  quota; only supported response headers are projected
- interruption acceptance is not terminal confirmation. Cancellation and
  deadline retain unconfirmed provider-cancellation truth while deletion still
  proceeds
- cross-topology provider-neutral conformance and full repository QA remain
  card 079

## Continuation

Card 079 is the sole ready task. Add the tenth provider-neutral conformance
profile, prove the public driver seam under local and remote-authoritative
hosts, run full QA, and close roadmap 025. No other continuation card is
active.
