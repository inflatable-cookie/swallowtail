# 104 Working-State Restoration Facade

Status: promoted
Owner: Tom
Updated: 2026-08-05

## Trigger

Consumers need one restart path across routes with read-only reconciliation and
ACP routes that can only restore a resumable session. Requiring each consumer
to interpret those capabilities independently would duplicate provider
qualification and encourage continuation to be reported as terminal truth.

## Finding

The common operation is consumer recovery, not a common provider primitive.
Three truthful outcomes exist:

- bounded read-only provider-session reconciliation
- bounded read-only exact provider-run reconciliation
- stateful provider-session continuation recovery with retained replay

Continuation recovery cannot simulate reconciliation. ACP `session/load`
restores resumable context, reconnects requested MCP servers, replays history,
and returns a live session. It cannot prove whether the interrupted turn is
active, completed, failed, or cancelled.

A portable facade can hide execution selection without hiding evidence
strength. Route-specific preparation selects the strongest qualified method.
The prepared operation is then consumed exactly once and returns one explicit
outcome variant. Consumers need no second dispatch branch, but still receive
the distinction required for transcript and lifecycle truth.

Selection is static. Failure of a prepared reconciliation operation must not
trigger stateful load. Such a fallback would widen authority after an error and
make provider failure indistinguishable from route qualification.

## Route Selection

| Route | Facade method |
| --- | --- |
| `codex.app-server` | provider-session reconciliation |
| `opencode.http` | provider-session reconciliation |
| `kimi-code.local-server` | exact-turn provider-session reconciliation |
| `openai.background` | exact provider-run reconciliation |
| `anthropic.managed-agent` | exact provider-run reconciliation |
| `claude-agent.acp` | provider-session continuation recovery |
| `kimi-code.acp` | provider-session continuation recovery |

Other routes remain unsupported. Capability does not inherit by provider
family or transport.

## Promoted Decisions

- Contract 050 owns working-state restoration orchestration.
- The facade wraps existing prepared operations; it is not a provider router.
- Adapter-specific preparation inputs remain exact and route-bound.
- Prepared execution is consuming and exact-once at the facade boundary.
- Reconciliation is always preferred where the exact route supports it.
- Continuation recovery returns a live loaded session and no interrupted-turn
  state claim.
- No failure-triggered fallback, prompt, retry, callback answer, cancellation,
  import, management, cleanup, or cross-route selection is added.
- Claude Agent ACP and Kimi ACP are the first continuation-recovery mappings.

## Sources

- Contracts 017, 037, 046, and 048
- Research 099 ACP retained-history qualification
- qualified Claude Agent ACP and Kimi ACP load/replay corpora
- current prepared reconciliation mappings for Codex, OpenCode, Kimi local
  server, OpenAI background, and Anthropic Managed Agents
