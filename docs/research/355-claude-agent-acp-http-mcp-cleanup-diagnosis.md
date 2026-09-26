# 355 Claude Agent ACP HTTP MCP Cleanup Diagnosis

Status: diagnosis without a provider; gate keeps the typed cleanup diagnostic;
leading cause named but not proven; no live attempt
Owner: Swallowtail worker
Created: 2026-09-26
Task: swallowtail#067; planning `41989748fc2ab5f00e2dafac9ca4efe1279c2722`
Contracts: 063

## Question

Why was session close not `Clean` in the swallowtail#064 live gate (Research
352), as far as that can be shown without a provider, and what must change so
a later attempt cannot lose that information again?

## Answer

The spent attempt ran no rerun and no provider work, so its exact cleanup
stage is not recoverable: Research 352 kept the gate's own stop name
`cleanup_failed`, and that name is assigned by the gate whenever
`CleanupOutcome` is not `Clean`. The gate's record and stderr kept no
diagnostic code, and no surviving artifact carries one (PR 384 body and
review). The record now keeps it.

What was fixed (all Swallowtail-side, test-visible only):

- `HttpMcpLiveRecord` keeps the typed cleanup evidence for every stop: class
  (`clean`, `degraded`, `failed`, `not_applicable`, `not_attempted`), the
  exact diagnostic code, and the adapter's stage tag when the adapter appends
  one (`... (task_join_failed)`). A message body is never kept: a code and a
  stage are Swallowtail-owned identifiers, a message may carry host text or
  provider text.
- The live gate prints class, code, and stage on its one stderr record line,
  so the next attempt's run log carries the diagnostic even if the process
  dies afterwards.
- The gate's turn-start failure path closed the session but discarded the
  close outcome and recorded the *turn* error as cleanup. It now records the
  cleanup outcome the close actually produced. The session-open failure path
  reports `not_attempted` instead of inventing a cleanup failure.
- The disposable loopback listener no longer treats an idle read as a close.
  It used a five-second read timeout that ended the connection on silence and
  answered HTTP/1.1 clients without an explicit `Connection: keep-alive`
  header with `Connection: close`. Both are unlike a streamable-HTTP MCP
  server, where a client holds a silent GET SSE stream open and connections
  persist by default. The listener now polls only to observe its stop flag,
  keeps partial request bytes across polls, defaults to persistent
  connections for HTTP/1.1, and still ends the connection on an explicit
  `Connection: close`. The harness proves an idle SSE stream survives, that a
  later request on the same connection is served, and that dropping the
  listener joins a handler waiting on that stream: a handler that blocked
  inside the read would hang the drop, and the gate would never print the
  record it had already built.

The harness fails if the record drops the diagnostic: two fake shapes drive a
full honouring tuple (connect, list, call, `Completed`) and then a non-`Clean`
close. A held `session/close` response with an expiring caller deadline must
surface as class `failed` with code
`swallowtail.session_cleanup.deadline_expired`; a fixture provider rejection
must surface as class `degraded` with code
`swallowtail.claude_agent.acp.request_rejected` and no stage, and the record
must not contain the provider's message text.

The production claim, the `claude-agent.acp` `client_mcp_servers` cell, and
every adapter cleanup path are unchanged. No live session, prompt, login, or
auth change occurred; the spent attempt was not rerun. The frozen `0.79.0`
package was downloaded and read, never executed. The repo-local sidecar pin,
the host Homebrew install, and the harness proof remain as Research 352 left
them.

## Frozen 0.79.0 trace

Artifact: npm `@agentclientprotocol/claude-agent-acp@0.79.0`, tarball SHA-256
`8c7a692b0266389eb7d81d4cb836c1f21293fb6a0ed11bff2e15db42c36177cb`,
reproducing Research 335. File citations are `dist/acp-agent.js`.

`session/close` on this point is not a stream teardown:

- The agent advertises the `close` session capability (L897) and `0.79.0` is
  qualified, so the gate's negotiated `native_close` is on and `session/close`
  is sent. Research 352's stop name also proves cleanup was applicable: the
  gate assigns a cleanup stop only when `CleanupOutcome` is not `Clean`.
- `closeSession` requires the session to still be in the map and otherwise
  throws `Session not found` (L4792-L4798). An evicted session turns into an
  ACP error response, not a hang.
- `teardownSession` awaits `this.cancel({ sessionId })` inside a try/catch
  that only logs a rejection (L4756-L4769), then disarms the force-cancel
  timer, aborts the consumer controller, calls `closeQueryStream`, aborts the
  SDK abort signal, clears the subagent, eager-tool-call and async-task
  runtimes, and deletes the session.
- `cancel` awaits `session.query.interrupt()` before returning (L4666). Its
  force-cancel backstop arms a timer that aborts the *consumer* controller and
  settles the active turn; it does not bound the `interrupt()` await, and the
  surrounding source comment names a wedged SDK consumption (issue #680) as
  the case it exists for (L4646-L4668).
- `closeQueryStream` is where the query stream is released:
  `settingsManager.dispose()`, `session.input.end()`, `session.query.close()`
  (L4743-L4752). So the sidecar's own pre-release step is `cancel()`: a
  rejected `interrupt()` is swallowed, but an `interrupt()` that never settles
  keeps `session/close` pending before any stream release.

The HTTP MCP client is not in the sidecar. ACP `session/new` `mcpServers`
entries become Agent SDK `options.mcpServers` entries
(`{type: 'http', url, headers}`, L5883-L5905) and are handed to the SDK
`query()` options (L6056). The Agent SDK pin on `0.79.0` is unmapped (Research
335), so the process that opens and holds the consumer-declared HTTP
connection is the Claude Code CLI the SDK spawns, below the frozen artifact
this family owns. The sidecar's only own MCP interaction is an OAuth status
path (`startMcpAuthentication` -> `authenticateMcpServers`, L608-L645) gated
on a client `elicitation.url` capability Swallowtail does not advertise; it
never holds the declared server's session.

Consequence for the split: an open streamable-HTTP MCP client can reach this
close path only through the CLI subprocess the sidecar spawns. The route's
close is provider-side by default: Swallowtail sends `session/close` and
bounds its own wait; everything between that request and the sidecar's `{}` is
the sidecar's `cancel`/`closeQueryStream` sequence.

## Most likely cause

Leading hypothesis: the sidecar did not answer `session/close` within the
gate's thirty-second cleanup boundary, and the observable was
`swallowtail.session_cleanup.deadline_expired` (class `failed`). Evidence: the
attempt had already completed connect, `tools/list`, `tools/call` and a
`Completed` turn, so the ACP connection was healthy immediately before close;
`0.79.0`'s close path awaits `query.interrupt()` before it releases the query
stream and exposes no bound of its own around that await; and a single
close request that never receives a response is exactly what the caller
deadline reports. The hypothesis explains the stop without invoking any
Swallowtail stage failure, but it cannot be confirmed from the spent record,
so it is a hypothesis, not a finding.

Alternative provider-side shape with the same spent stop name: the sidecar
answered with an ACP error, which Research 352's record name cannot
distinguish. `Session not found` is reachable whenever the session was
already evicted, for example after the spawned CLI or its query stream died;
that surfaces as `swallowtail.claude_agent.acp.request_rejected` (class
`degraded`).

Swallowtail-side candidates, and their disposition after this task:

- The record and the gate dropped the diagnostic: fixed, harness-proven.
- The listener dropped idle connections and closed persistent HTTP/1.1
  clients: fixed, harness-proven. This is a harness fidelity defect that could
  perturb the provider under test; it is a candidate contributor to the spent
  attempt, not a proven cause of it.
- Local process-tree teardown (`swallowtail.claude_agent.acp.process_cleanup_failed`,
  or the adapter's reader-join bound): unchanged.
  Reachable only if a group descendant outlived the host's SIGTERM-then-SIGKILL
  escalation, which is not MCP-specific.
- The thirty-second cleanup deadline: unchanged. It is the caller's hard
  boundary, not a defect. When it fires the inner stage is unrecoverable, so a
  later `deadline_expired` must be read as "provider-side stall, stage
  unknown", never as a stage finding.

## Next move

A second live attempt needs fresh operator authority and spends nothing until
it has it. When it runs, the harness proof must pass first and the attempt
must show the typed cleanup diagnostic. The code decides the split:

| Observed code | Class | Reads as |
| --- | --- | --- |
| `swallowtail.session_cleanup.deadline_expired` | failed | sidecar did not answer `session/close` inside the boundary: provider-side stall in `cancel`/`closeQueryStream` |
| `swallowtail.claude_agent.acp.request_rejected` | degraded | sidecar answered with an ACP error, e.g. `Session not found` after eviction |
| `swallowtail.claude_agent.acp.close_malformed` | degraded | sidecar answered non-empty |
| `swallowtail.claude_agent.acp.process_cleanup_failed` | failed | Swallowtail's local host observed process or reader teardown as failed |
| `swallowtail.claude_agent.acp.cleanup_failed` | failed | a Swallowtail join or release stage failed; the stage tag names it (`task_join_failed`, `turn_join_failed`, `resource_release_failed`, `credential_release_failed`) |
| `swallowtail.claude_agent.acp.cleanup_missing` | failed | the protocol task ended without recording an outcome; should be unreachable |

`Clean` with the full honouring tuple is the only outcome that would move the
`client_mcp_servers` cell to `Yes`; anything else leaves Research 352's
disposition and the matrix cell exactly as they stand.

## Limits

The diagnosis is source-and-fake evidence only. It names no provider defect
as a finding, proves no honouring, and does not claim that an open HTTP MCP
client stalls `session/close`. The sidecar artifact is read from the frozen
`0.79.0` package; the Agent SDK pin and the CLI below it stay unmapped, so the
HTTP client's own teardown behaviour is not frozen here. The spent attempt
stays spent; its record cannot be reconstructed.

No raw provider stream, bearer, account identifier, session id, or private
path is retained. Nothing downloaded for this diagnosis was executed.

## Sources

- npm tarball `@agentclientprotocol/claude-agent-acp@0.79.0` (SHA-256 above),
  `dist/acp-agent.js` L608-L645, L897, L4457-L4668, L4743-L4798, L5883-L5905,
  L6056
- `package.json` of the same package: `@agentclientprotocol/sdk` `1.4.0`,
  `@anthropic-ai/claude-agent-sdk` `0.3.274`
- Research 335 `0.79.0` identity (tarball digest reproduced)
- Research 352 spent stop; PR 384 body and review (no cleanup code retained)
- `crates/swallowtail-adapter-claude-agent/tests/http_mcp_live/`,
  `http_mcp_live_harness.rs`, `http_mcp_live_gate.rs`
