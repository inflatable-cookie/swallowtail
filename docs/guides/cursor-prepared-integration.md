# Cursor Prepared Integration

Use `swallowtail-adapter-cursor` for the installed `cursor-agent` harness. Do
not substitute the colliding `agent` executable: that command may identify as
Grok Build.

## Choose A Route

`prepare_cursor` requires one explicit `CursorPreparedDriver`:

| Selection | Route and driver ID | Operation and transport |
| --- | --- | --- |
| `Catalogue` | `cursor-agent.catalogue`; `swallowtail.cursor-agent.catalogue` | authenticated model observation over models stdio |
| `Acp` | `cursor-agent.acp`; `swallowtail.cursor-agent.acp` | durable interactive session over ACP v1 stdio |
| `Headless` | `cursor-agent.headless`; `swallowtail.cursor-agent.headless` | explicit-model run over stream JSON stdio |

The branches share installation and subscription evidence only. They do not
share transport, model selection, authority, lifecycle, or continuation
semantics.

## Operator Prerequisites

The host supplies one approved `cursor-agent` target, an explicit environment,
provider-owned delegated Cursor subscription login, and ready evidence for
`cursor_subscription_access_profile`.

Swallowtail does not install Cursor, perform login, read its credential store,
or search PATH. Local subscription state crosses the boundary only as safe
readiness evidence; no credential reference enters the prepared plan.

Two exact calendar/build pairs are qualified:

- `2026.07.01-41b2de7`
- `2026.07.23-e383d2b`

The gap is not inferred. A qualified date with a different opaque build is
rejected. Later dates remain visibly unverified newer.

## Prepare The Installation

Create `CursorPreparationInput` with one selected driver and exact instance,
host, target, environment, access profile, and access evidence. Add a bounded
`CursorPreparationProbe`, then call `prepare_cursor`.

Preparation discovers only the supplied target and returns the matching
`CursorPreparedIntegration` variant. Reusing evidence from another branch
does not bypass that explicit selection.

## Catalogue

On the `Catalogue` variant, call `prepare_catalogue` with
`CursorCatalogueProfileInput`, then `list_models`. The joined `models` command
observes the authenticated model set. It does not claim model invocability or
choose the model for another branch.

See
[`prepared_cursor_catalogue`](../../crates/swallowtail-adapter-cursor/examples/prepared_cursor_catalogue.rs).

## ACP Interactive Session

On the `Acp` variant, create `CursorAcpSessionProfileInput` with a request ID
and exact working resource, then call `prepare_session` and `open_session`.

The profile is ambient read-write ACP. It exposes assistant, provider-disclosed
thought, plan, and tool activity plus active-turn interruption. It does not
accept portable model, reasoning, tool, permission, question, or plan-mode
options. Provider requests remain observational and cannot be approved through
this route.

Start each prompt with the common `TurnRequest`. Take and poll the event stream
and terminal outcome concurrently, then close the turn. Cancellation stops the
active turn; session close joins the connection, child, resource, and task
work without deleting Cursor state.

Cursor preserves provider session state, but Swallowtail exposes no catalogue,
import, public load, public resume, archive, restore, delete, or native close.
After process loss, an exact existing resume binding may use
`prepare_working_state_restoration` for bounded attachment recovery. That
reattaches the live session while discarding non-authoritative replay; it does
not reconcile the interrupted turn or restore a transcript.

See
[`prepared_cursor_acp`](../../crates/swallowtail-adapter-cursor/examples/prepared_cursor_acp.rs).

## Headless Structured Run

On the `Headless` variant, build `CursorHeadlessRunProfileInput` with:

- request ID and prompt content
- explicit `CursorHeadlessModelSelection`
- exact working resource
- explicit `Read` or `ReadWrite` authority
- deadline

Read authority selects Cursor plan mode; read-write remains ambient and
explicit. The prepared path does not request dangerous force flags or optional
sandboxing.

Call `prepare_run`, then `start_run`. Drain streaming assistant, thinking,
tool, result, usage, and terminal events before `close`. Cancellation and
deadline stop the owned child; cleanup joins it. Durable provider state may
remain, but no management authority escapes.

Reasoning selection, JSON Schema output, attachments, callbacks, and external
search are not qualified on this route.

See
[`prepared_cursor_headless`](../../crates/swallowtail-adapter-cursor/examples/prepared_cursor_headless.rs).

## Lifecycle And Failure Handling

Operation IDs remain consumer-unique even when Cursor reuses provider-local
tool IDs. Persist the portable `ActivityKey`, not a raw activity or provider
reference. Treat terminal status and cleanup separately. Use portable failure
classification for normal handling and retain exact Cursor codes for support.

## Unsupported

Cursor exposes no provider-session import or management, consumer tools,
permission/question response, attachments, external search, public reasoning
control, or structured output. Activity does not grant tool or child-control
authority.

Promotion requires an exact Cursor surface and calendar/build pair,
prepared-plan and authority binding, bounded corpus evidence, lifecycle tests,
and route-matrix coverage. An ACP advertisement, CLI flag, or provider-local
state file alone is insufficient.

## Deterministic Validation

Run:

```sh
effigy validate:focused swallowtail-adapter-cursor
effigy check:examples
```

No Cursor account mutation or provider prompt is required.
