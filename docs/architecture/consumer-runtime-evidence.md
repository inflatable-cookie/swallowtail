# Consumer Runtime Evidence

Status: active
Owner: Tom
Updated: 2026-07-20

## Purpose

Record the realized runtime shapes in the first two consumers. This is not a
final Swallowtail trait design.

## Nucleus: Interactive Session

Nucleus currently wraps a Codex app-server child in a blocking live-session
trait. One process remains alive across turns. The adapter starts or resumes a
provider thread, sends turns, parses message deltas and tool calls, waits for a
terminal turn event, and kills the child when the session drops.

The server layer resolves project resources, supplies developer instructions
and tool declarations, executes tool calls under Nucleus policy, stores product
session mappings and receipts, and decides whether a provider thread may be
resumed.

Current gaps include active-turn interruption, host-visible progressive event
delivery, typed diagnostics, and topology-neutral host ports.

The current interactive UI path is narrower and gives adoption a clean seam:

- `nucleus-agent-adapters/src/codex_runtime.rs` owns Codex process launch,
  JSONL-RPC, model catalog parsing, session/turn transport, and dynamic-tool
  callback envelopes behind Nucleus's `AgentSessionRuntime` facade.
- `nucleus-server/src/local_codex_chat/runtime.rs` owns developer instructions,
  the two Nucleus task portals, tool execution, receipts, stored-session
  mapping, and reply projection. Those remain Nucleus behavior.
- `nucleus-server/src/project_resource_target.rs` resolves product resource ids
  under server authority. Adoption maps that authority to opaque Swallowtail
  host references; it does not send raw client paths into Swallowtail.
- the broader `codex_supervision` persistence and admission surfaces are not a
  wholesale extraction target. They remain Nucleus control-plane evidence and
  may consume normalized Swallowtail events and provider references later.

Swallowtail now has provider-neutral records for session instructions,
reasoning selection, dynamic-tool declarations, callback request/response
correlation, callback wait state, deadline, abandonment, and model reasoning
metadata. The Codex app-server driver now translates those records into the
current provider protocol. Its local and remote-authoritative topology proof
now fixes host identity, resume binding, callback lifecycle, and joined cleanup.
The bounded downstream handoff is now recorded. It targets the existing live
registry and keeps Nucleus records unchanged. The first slice is explicitly
embedded-host only; remote-authoritative resources wait for a real Nucleus
host-invocation route. Stored tool-enabled chats open a fresh provider session
with transcript context because current provider schema evidence cannot safely
redeclare tools on resume.

## Soundcheck: Structured Run

Soundcheck now routes each bounded Codex operation through Swallowtail. It
supplies a host-approved executable, environment, temporary working directory,
schema, optional screenshot, model, reasoning effort, search policy, and
deadline. Swallowtail owns app-server and exec process/protocol mechanics,
normalized progress, cancellation, timeout, and cleanup.

Soundcheck then decodes and validates the result, performs one bounded repair,
runs optional product-specific comparison calls, and applies the proposal only
through separate product commands.

The connector mechanics are reusable. Evidence collection, taxonomy schemas,
validation, repair, ranking, and application are not.

The realized adoption seam is:

- `src-tauri/src/swallowtail_codex.rs` maps Soundcheck operations onto public
  Swallowtail model-catalogue and structured-run roles.
- focused host and preflight modules construct capability-scoped local services
  and immutable plans without importing product types into Swallowtail.
- `src-tauri/src/assistant_tagging.rs` still owns product evidence, prompts, taxonomy validation,
  one repair, peer ranking, companion selection, and proposal application.
  Those remain Soundcheck behavior.
- `src-tauri/src/app_settings.rs` keeps selected model and reasoning settings
  while projecting Swallowtail catalogue records into existing DTOs.

Swallowtail now covers that shared transport seam: schema and image leases,
exact reasoning and search policy, host deadlines, ordered progress, terminal
output, cancellation, cleanup, and catalog display/default/reasoning metadata.
The direct model RPC and `codex exec` implementations are removed. Automated
consumer tests, authenticated local catalogue discovery, and native operator
acceptance pass.

## Adoption Order

| Consumer | First replaceable mechanism | Shared gaps before handoff | Consumer-owned remainder |
| --- | --- | --- | --- |
| Soundcheck | Codex model discovery and bounded exec transport | adopted and natively accepted | evidence, prompts, validation, repair, ranking, proposal review/application |
| Nucleus | `AgentSessionRuntime` Codex implementation behind the existing facade | adopted; automated parity and authenticated catalogue pass, native callback acceptance remains | projects/resources, tools and authority, receipts, persistence, tasks/goals/memory, UI |

Soundcheck went first because its one-shot boundary does not require callback
exchange or durable multi-turn continuity. Swallowtail covers the shared
Nucleus callback, session-option, and host-topology boundary. Nucleus now uses
the sibling Swallowtail workspace for local development; version pinning waits
for versioned distribution.

## Common Boundary

```text
consumer intent and authority
  -> execution request + model route + capabilities
  -> Swallowtail runtime mechanism on authorized host
  -> provider adapter / owned or external runtime
  -> ordered safe events + terminal outcome + opaque refs
  -> consumer persistence, validation, receipts, and consequences
```

Both consumers need discovery, readiness, model catalogs, execution context,
process or connection lifecycle, deadlines, cancellation, correlated events,
safe diagnostics, and cleanup. Only interactive sessions require durable
multi-turn continuity and callback exchange. Only structured runs require a
bounded result contract with optional schema and attachments.

## Topology Evidence

Nucleus explicitly supports embedded, sidecar, remote authoritative, and remote
worker host forms. Soundcheck currently runs blocking connector work in the
Tauri host rather than the renderer. The shared conclusion is host placement,
not server placement: provider work executes where binary, credential,
filesystem, and network authority exists.

Clients identify product resources and operation intent. They do not establish
authority by sending arbitrary local paths or secret material.
