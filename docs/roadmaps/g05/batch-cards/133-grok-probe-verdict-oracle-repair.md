# 133 Grok Probe Verdict Oracle Repair

Status: complete; PR 280 merged at `0aeefa15`; oracle splits admission from invocation
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 128 merged (`63e34641`); the two 2026-09-07 live capsules (`1a1f263d…`, `a0eaafb6…`), both `inconclusive`

## Defect

Both live probes returned `inconclusive`, but the frames answer more than the
verdict admits. On each segment the outbound `session/new` carried a non-empty
`mcpServers` list, and the disposable echo server observed `initialize` **and**
`tools/list`. Grok Build therefore accepted the client-declared server,
spawned it, connected, and enumerated its tools. What did not happen is
`tools/call`.

The oracle conflates two different questions:

1. **Protocol admission** — does the route accept a client-supplied ACP MCP
   server and reach it? Answered by `initialize` on the echo transcript, and
   answered *affirmatively* on both `1.0.4` and `1.0.5`.
2. **Model invocation** — did the model choose to call the tool on this
   prompt? Answered by `tools/call`, and unanswered.

Because the enum requires `tools/call` for `accepts_client_mcp` and a
completed prompt for `ignores_client_mcp`, a run that proves admission but
elicits no call falls through to `inconclusive`. That is a harness defect, not
a provider finding: a model declining to call a tool is not evidence that
client MCP is unsupported. The card 128 decision tree's "second inconclusive
is treated as `ignores_client_mcp`" branch must not fire on this oracle, since
the frames already refute `ignores`.

## Scope

1. Split the verdict so admission and invocation are separately reported:
   retain the four existing values, and record `client_mcp_admitted`
   (echo-server `initialize` observed) and `client_mcp_tools_listed`
   (`tools/list` observed) as explicit capsule fields that a verdict cannot
   silently discard. `accepts_client_mcp` continues to require `tools/call`.
2. Make the prompt directive rather than permissive: instruct the model to
   call the echo tool by name with a fixed argument, so a compliant model has
   no reason not to invoke it. Record the exact prompt text in the capsule.
3. Capture the prompt turn's completion state and stop reason, so "no turn
   result" is distinguishable from "turn completed without a tool call".
   `inconclusive` must name which of those occurred.
4. Extend the offline fixtures to cover the new shape: admitted-and-called,
   admitted-listed-not-called-with-completed-turn,
   admitted-not-listed, and no-admission. All four verdicts keep passing.
5. Update the hand-off packet to match, including what each new field means
   for the decision tree.

## Out Of Scope

Any adapter, claim, matrix, or contract change; a third live attempt beyond
the single authorized rerun per segment; substituting another Grok version.

## Acceptance Criteria

- [ ] admission and invocation are separately reported and cannot be collapsed
- [ ] the prompt is directive and recorded verbatim in the capsule
- [ ] `inconclusive` names its cause
- [ ] offline fixtures cover the extended shape and pass
- [ ] packet updated

## Validation

- `effigy validate:focused swallowtail-testkit`
- `effigy package:verify-affected swallowtail-testkit`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the capsule distinguishes what the route did from what the model
chose. Smallest counterexample: a verdict that reports `ignores_client_mcp`
on a run whose echo transcript contains `initialize`.

## Auto-Continuation

No. Stop for exact-head review. Desktop runs the single authorized rerun per
segment after this merges.
