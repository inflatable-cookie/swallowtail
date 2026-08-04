# 100 Controlled Shutdown Active Operation Detachment

Status: promoted
Owner: Tom
Updated: 2026-08-04

## Trigger

Crash reconciliation now prevents a lost runtime handle from leaving consumer
state permanently active. Controlled application shutdown still calls ordinary
handle close, which many routes define as active provider cancellation. A
provider may be able to retain the work safely, but dropping local ownership is
not evidence of a clean detach.

## Finding

Detachment is a third lifecycle disposition, separate from completion and
cancellation.

- cancellation asks the provider to stop work
- detachment stops the local observation attachment without asking the
  provider to stop
- reconciliation later observes the bound provider state

The runtime cannot implement this by dropping a handle. Local streams, tasks,
callbacks, credentials, and resources must still join and release. The local
operation ends as `Detached`; that status is not provider terminal truth.

OpenCode is the first complete route. Its attached HTTP server owns the
session and `prompt_async` work independently of one SSE client. The read-only
session already returns a durable resume binding and now has session-scoped
reconciliation. Closing the SSE attachment without `/abort` preserves the
only honest restart path.

Provider callbacks are excluded. A permission or question cannot survive
local callback-exchange teardown, and reconciliation grants no callback-answer
authority.

## Route Classification

| Class | Routes | Promotion gate |
| --- | --- | --- |
| supported attached-turn detachment | `opencode.http` read-only interactive session; attached `kimi-code.local-server` interactive session | realized in g03.028-g03.029 with durable binding/checkpoint plus reconciliation |
| continuation unproven | `codex.app-server` | prove provider turn survival after app-server connection/process teardown without callback or lifecycle loss |
| retained-operation record required | `openai.background` | persist exact response and cursor authority before temporary-retention cleanup |
| incompatible owned-resource cleanup | `anthropic.managed-agent` | define a retained-resource profile that does not inherit delete-on-close |
| process-bound or non-retained | ACP, headless child-process, direct, realtime, attached inference, and owned serving routes | prove provider work can outlive local ownership before claiming detachment |

## Promoted Decisions

- Contract 049 owns controlled detachment.
- Detachment is explicit and opt-in per prepared profile.
- A supported run or turn exposes an optional `OperationDetachmentControl`.
- Request acknowledgement is idempotent and local; it does not assert provider
  completion, continued activity, or cancellation.
- The local terminal class is `Detached` and carries no provider terminal
  truth.
- Cancellation wins any concurrent cancellation/detachment race.
- Successful detachment still requires ordinary awaited handle close to join
  local work and release leases.
- Default close, unsupported routes, and callback-bearing OpenCode turns remain
  unchanged.

## Sources

- Contracts 009, 017, 042, and 048
- OpenCode qualified `prompt_async`, SSE, status, session lookup, retained
  message, and abort fixtures
- current route handle cleanup implementations in this repository
