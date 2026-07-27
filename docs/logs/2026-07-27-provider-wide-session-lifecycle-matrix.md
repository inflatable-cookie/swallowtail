# Provider-Wide Session Lifecycle Matrix

Date: 2026-07-27
Card: `../roadmaps/g02/batch-cards/058-provider-wide-session-lifecycle-matrix.md`

## Classification

The production route guide now classifies provider-session lifecycle across
the exact 22-route inventory:

- supported: Codex app-server, Claude Agent ACP, OpenCode HTTP/SSE
- unsupported on the selected transport: Kimi Code ACP, Gemini CLI ACP
- not applicable: the other seventeen current operation shapes

Unsupported and not applicable are intentionally different. Kimi and Gemini
have persistent provider sessions, but their selected ACP routes do not
advertise qualified archive, restore, or delete actions. The other routes have
no user-managed persistent provider session under their current contracts.

## Boundaries

Each row records management-binding support, individual action posture,
deletion strength, version evidence, and driver-owned cleanup. An alternate
CLI, SDK, REST route, filesystem path, UI, or private provider surface cannot
change the selected driver's row.

Alibaba conversation cleanup and Anthropic Managed Agent cleanup remain
operation-owned resource deletion. Native close, transport teardown,
background-run cancellation, realtime disconnect, and owned-server stop also
remain cleanup. None creates management authority or satisfies a
user-directed provider-session action.

## Machine Evidence

The route check now validates the facade and lifecycle matrices separately.
Both must contain the same 22 canonical identities exactly once. The lifecycle
check also fixes every route's applicability, binding, action, and deletion
strength posture and requires version and cleanup evidence.

Focused production claims passed:

- Codex lifecycle corpus: 5
- Claude Agent prepared management: 5
- OpenCode deletion range: 4
- OpenCode prepared deletion: 5

## Continuation

Card 059 is ready. Build one clean local 23-package candidate and prove the
three supported mappings, both unsupported ACP routes, all not-applicable
surfaces, and existing consumer integrations without publication or live
provider effects.
