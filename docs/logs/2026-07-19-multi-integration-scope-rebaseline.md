# Multi-Integration Scope Rebaseline

Date: 2026-07-19
Status: recorded

## Change

Operator intent clarifies that Swallowtail must connect to as many useful agent
harness and model-provider surfaces as practical. The project is not a Codex
connector generalized from two consumers.

Named initial candidates include Codex, Claude Code, OpenCode, Cursor, Pi,
Kimi, xAI/Grok direct routes, and local model runtimes. CLI, app-server/RPC,
ACP, SDK, API, WebSocket, local, remote, and provider-specific surfaces may
require separate drivers.

## Contract Delta

Contract 005 now separates integration family, adapter driver, transport
family, configured instance, and model route. Capabilities bind to concrete
instances and routes rather than provider names.

## Roadmap Effect

The prior host-port and async decision card was superseded before execution.
G01 now inventories current official integration surfaces before making runtime
API decisions. The two-consumer ownership work remains valid but is no longer
sufficient design evidence.

No runtime or adapter code changed.
