# Questions

Questions that block or shape work. Reference them by ID from the plan and from
briefs. An answered question keeps only its pointer to where the answer lives.

An open question can own feature-matrix `evidence_pending` cells. It must name
the gate owner, state the decision tree converting each outcome into
`producer_gap` or `provider_limitation`, and list its cells, one
`- <route> <feature>` line each, after an `Evidence gate scope:` line. The
route-matrix check enforces this.

## Q-001 — Run a live HTTP MCP honouring gate on `claude-agent.acp`?

Status: answered 2026-09-26
Answer: yes, as recommended (Tom). One attempt, harness proof first, pinned
`claude-agent-acp` 0.79.0. Plan item `claude-agent-acp-http-mcp-live`.
Asked: 2026-09-25

Research 351 wired five ACP routes to emit the Contract 063 consumer HTTP MCP
entry; only `opencode.acp` is live-proven (Research 349). Recommendation: yes,
first, one attempt, harness proof against the fake SDK before the live turn,
pinned `claude-agent-acp` 0.79.0 over the host's 0.63.0. Gemini second.
Copilot, Goose and Kiro held until a consumer needs them.

Live result: Research 352. The one attempt was not accepted (typed stop
`cleanup_failed`). The cell stays No.

## Q-002 — Which command validates a fresh checkout before merge?

Status: answered 2026-09-26
Answer: the candidate below (Tom). Recorded in `AGENTS.md` "Validate".
Asked: 2026-09-26

Queue does not yet run plain pre-merge validation. The command must validate a
fresh disposable checkout, install its own dependencies and exit 0 on pass.
Candidate: `effigy qa:docs && effigy qa:routes && effigy format:check`, with
package-scoped `effigy validate:focused` named per brief. Full `effigy qa`
runs the whole workspace test suite and is likely too slow per task.

## Q-003 — How should `antigravity.headless` qualify past the `1.1.22` retry stop?

Status: answered 2026-09-26
Answer: the recommendation below (Tom): option 2 evidence first, option 1 as
the fallback. Plan item `version-currentness`.
Asked: 2026-09-26

From `1.1.22` through official `1.2.11`, Antigravity retries failed model
requests itself with no published attempt bound or off switch (Research 353,
"Ruling request"). The claim stays `1.1.9..=1.1.17`. Options:

1. Accept provider-managed retry as a Contract 023 exception on a new
   milestone. Consumer-visible: only the host deadline bounds a turn.
2. Pin a finite or disabling retry control. `1.2.11` contains an
   undocumented `AGY_CLI_MODEL_API_MAX_RETRIES`; its semantics need evidence
   first, and pinning it changes the route's approved environment.
3. Keep the ceiling with a successor task.

Recommendation: option 2 first, as a bounded evidence task on the frozen
artifact against a fake model endpoint (no provider); fall back to option 1
if the control doesn't hold.

