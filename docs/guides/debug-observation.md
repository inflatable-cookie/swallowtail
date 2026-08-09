# Debug Observation

Opt into a host-owned debug sink when you need restricted wire, lifecycle, or
process context that must not enter safe diagnostics or public events.
Ordinary integrations leave the observer unregistered.

Authority: [Contract 053](../contracts/053-opt-in-debug-observation.md),
[Contract 010](../contracts/010-execution-host-services-and-inputs.md).
Related: [portable failure handling](portable-failure-handling.md).

## When To Use It

Register a `DiagnosticObserver` while chasing harness drift, protocol parse
failures, or version surprises. Turn it off for normal product runs. Swallowtail
does not persist observations or own log retention.

Do not use debug observation as application control flow. Keep retries,
fallback, and user messaging on `SafeDiagnostic` and portable classification.

## Register On The Host

```rust
use std::sync::Arc;
use swallowtail_core::{Diagnostic, ExecutionHostId};
use swallowtail_runtime::{
    DebugObservation, DiagnosticObserver, HostServices,
};

struct TracingObserver;

impl DiagnosticObserver for TracingObserver {
    fn observe(&self, diagnostic: &Diagnostic) {
        // Optional: restricted Diagnostic.internal_detail.
        let _ = diagnostic;
    }

    fn observe_debug(&self, observation: &DebugObservation) {
        // Host-owned sink: file, ring buffer, or UI panel.
        // observation.detail() is restricted; Display/Debug stay redacted.
        let _ = (
            observation.kind(),
            observation.stage(),
            observation.correlated_code(),
            observation.detail(),
        );
    }
}

fn services_with_debug() -> HostServices {
    HostServices::new(ExecutionHostId::new("host.local").expect("host id"))
        .with_diagnostic_observer(Arc::new(TracingObserver))
}
```

Missing registration is a no-op. Observer panics cannot change terminal status,
classification, cleanup, or route selection.

## What Arrives

Each `DebugObservation` may carry:

- optional request, scope, run, turn, or session correlation
- optional route and stage labels
- one kind (`Lifecycle`, `WireInbound`, `ProtocolParse`, `StderrRing`, …)
- optional correlated exact safe diagnostic code
- one bounded restricted detail body (runtime truncates at 4096 characters)

Public events and activity stay on the ordinary operation stream. Debug
observations are not sequenced delivery and carry no completeness guarantee.

## Safe Path Stays Separate

| Surface | Use |
| --- | --- |
| `SafeDiagnostic` | stable public code and message |
| Portable classification | ordinary app branching |
| Bounded safe excerpts | short operator-visible context on selected failures |
| `DebugObservation` | opt-in restricted timeline for hosts that asked |

## Current Emitters

Failure-path emissions only. Happy-path wire spam is not enabled.

### Shared runtime

| Surface | When |
| --- | --- |
| Installed discovery probe | `HostProcess` / `InterfaceVersion` / `Cleanup` on probe outcomes; route label is the adapter solution string |
| Plan-family host-service readiness | `Lifecycle` when required host services are missing (`prepared.plan`) |

### Installed harness and RPC

| Route | When |
| --- | --- |
| `codex.app-server` | Malformed-inbound: `WireInbound`, `ProtocolParse`, `StderrRing` |
| `codex.exec` | Stream/process decode and process failures |
| `claude-agent.acp` | Pump decode/dispatch/read failures; turn-local `fail` |
| `cursor-agent.acp` | Pump transport/parse failures |
| `gemini-cli.acp` | Pump transport/parse failures |
| `grok-build.acp` | Pump transport/parse failures |
| `kimi-code.acp` | Pump transport/parse failures |
| `pi.rpc` | Pump transport/parse failures |
| `oh-my-pi.rpc` | Pump transport/parse failures |

### Headless stream-json

| Route | When |
| --- | --- |
| `qwen.headless` | Decode/process failures |
| `muse-code.headless` | Decode/process failures |
| `command-code.headless` | Decode/process failures |
| `antigravity.headless` | Decode/process failures |
| `cursor-agent.headless` | Decode/process failures |
| `claude-code.headless` | Decode/process failures |
| `gemini-cli.headless` | Decode/process failures |
| `kimi-code.headless` | Decode/process failures |

### Hosted, attached, realtime, remote

| Route | When |
| --- | --- |
| `opencode.http` | HTTP transport/decode/map failures |
| `anthropic.messages` | HTTP/SSE transport/decode/map failures |
| `anthropic.managed-agent` | Managed HTTPS/SSE transport/decode/map failures |
| `openai.background` | HTTP transport/decode failures |
| `openai.realtime` | WebSocket transport/decode failures |
| `deepseek.continuation` | HTTP transport/decode/map failures |
| `alibaba.conversations` | HTTP transport/decode failures |
| `kimi-platform.chat` | HTTP transport/decode/map failures |
| `kimi-code.local-server` | WebSocket transport/decode failures |
| `bedrock.runtime` | HTTP transport/map failures |
| `bedrock.catalogue` | SDK catalogue transport/projection failures |
| `ollama.attached` | Native HTTP/NDJSON transport/map failures |
| `llama-cpp.attached` | HTTP/SSE transport/decode/map failures |
| `llama-cpp.owned` | Owned process startup failures; run pump uses the owned route label |
| `gemini.live` | WebSocket transport/decode failures |
| `xai.responses-websocket` | WebSocket transport/map failures |
| `acp.remote` (transport) | Connect and worker failures |

Catalogue-only installed harnesses that only fail in discovery/prep
(`antigravity.catalogue`, `cursor-agent.catalogue`, and similar) are covered by
the shared installed-discovery and plan-family emitters above.

## Example

See
[`debug_observation_host.rs`](../../crates/swallowtail-runtime/examples/debug_observation_host.rs)
for a compiling host registration pattern.
