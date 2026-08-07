# Oh My Pi Prepared Integration

Use `swallowtail-adapter-oh-my-pi` for the separately qualified Oh My Pi
coding-agent package. `oh-my-pi.rpc` does not alias `pi.rpc`: artifact,
executable, auth state, protocol, framing, models, and capability truth remain
independent.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

The driver ID is `swallowtail.oh-my-pi.rpc`.

## Route And Operations

`prepare_oh_my_pi_rpc` prepares one installed route with three bound operation
types:

| Prepared type | Operation |
| --- | --- |
| `OhMyPiPreparedCatalogue` | authenticated local model catalogue |
| `OhMyPiPreparedRun` | one-prompt structured run |
| `OhMyPiPreparedSession` | interactive session with typed questions |

Catalogue, run, and session all negotiate OMP RPC v2. No third-party ACP bridge
or global TypeScript dependency is involved.

## Operator Prerequisites

The host supplies an approved OMP executable target and explicit environment.
The installed launcher may require Bun; interpreter resolution belongs to the
host-approved launch recipe, not adapter PATH search.

Authentication remains in OMP's local provider configuration. Swallowtail
requires its maintainer-supported local access profile with
`CredentialState::NotRequired`; it does not read, serialize, or lease provider
credentials. Provider selection, login, billing, and fallback stay inside OMP
and operator configuration.

Prepared operations bind task, process, time, read-only working-resource, and
optional attachment services. They use no credential service. Provider-
suppressed tools and ambient execution are not host sandboxing.

Exact package `17.2.9` is qualified on `oh-my-pi.package`. Later stable
versions remain visible unverified newer.

## Prepare The Installation

Create `OhMyPiPreparationInput` with the configured instance, execution host,
approved target, environment, local access profile, and matching evidence. Add
a bounded `OhMyPiPreparationProbe`, then call `prepare_oh_my_pi_rpc`.

Preparation performs exact installed discovery and returns one
`OhMyPiPreparedIntegration`. It sends no prompt and does not inspect auth state.
Validate host/target equality again if a persisted preparation input is reused.

## Catalogue And Model Binding

Call `prepare_catalogue` with `OhMyPiCatalogueProfileInput`, then
`list_models`. The RPC response supplies provider, model, and reasoning-support
observations. The consumer still selects one exact
`OhMyPiModelSelection`; Swallowtail does not choose a default or fallback.

The catalogue is evidence for this OMP installation. It does not authorize a
different route or provider account.

## Structured Run

Create `OhMyPiRunProfileInput` with request ID, exact model, prompt content,
working resource, and deadline. Optional controls are:

- reasoning: `off`, `minimal`, `low`, `medium`, `high`, `xhigh`, or `max`
- at most one PNG attachment with known size no greater than one MiB

Call `prepare_run`, then `start_run`. The provider-suppressed configuration
does not expose write-capable tools. Drain assistant, reasoning, provider-tool,
question, usage, and terminal events before `close`. Cancellation stops the
active run; cleanup joins pump and callback tasks.

Structured output, output-token limits, consumer tools, permission exchange,
and external search are not qualified.

## Interactive Session And Questions

Create `OhMyPiSessionProfileInput` with request ID, exact model, working
resource, and `SessionOptions`. The only supported portable option is an
optional qualified reasoning mode. Call `with_image_attachments` before
preparation if later turns may carry the bounded PNG input.

Call `prepare_session`, then `open_session`. Each turn uses the normal
`TurnRequest`. OMP dialogs are projected through the typed harness-user-input
callback contract, preserving ordered choice options and free-text answers.
The consumer must answer through the provided responder exactly once or
abandon the callback. A question is not a permission request.

Take turn events, the callback exchange, and terminal outcome immediately and
poll them concurrently, then close the turn. Cancellation stops the active
operation. Session close joins process, pump, resource, and attachment work.

Developer instructions, harness plan mode, consumer tools, permission
exchange, and write authority are rejected during preparation.

The catalogue, session, and prompt boundary is shown in
[`prepared_oh_my_pi_rpc`](../../crates/swallowtail-adapter-oh-my-pi/examples/prepared_oh_my_pi_rpc.rs).

## Activity, Usage, And Framing

OMP activity includes assistant messages, reasoning, provider-owned tools,
questions, and usage where the negotiated protocol supplies them. Empty
provider display updates clear ephemeral UI state and do not fail the turn.
Model and thinking-level changes are session lifecycle observations.

Physical RPC frames are bounded at one MiB. Chunked logical frames are bounded
at 64 MiB. Consumers receive portable events and do not parse OMP wire payloads.

## Recovery And Cleanup

OMP exposes no durable Swallowtail session binding, load, resume, import, or
provider-session management. `prepare_working_state_restoration` opens a fresh
context-losing session after process loss. It does not recover private OMP
continuation, callbacks, or the interrupted turn.

Keep terminal status and cleanup outcome separately. Handle classified
failures through the portable interface and retain exact OMP codes for support.

## Unsupported

The route does not expose write tools, host-tool injection, permission
exchange, provider-session switching/import, archive/restore/delete, durable
provider state, external search, or subagent observation/control.

Promotion requires an exact OMP protocol/package milestone, prepared-plan and
authority binding, bounded framing fixtures, lifecycle tests, and route-matrix
coverage. A configured OMP provider, plugin, or RPC advertisement alone is
insufficient.

## Validation And Optional Probes

Deterministic validation:

```sh
effigy validate:focused swallowtail-adapter-oh-my-pi
effigy check:examples
```

Optional operator-gated evidence:

```sh
SWALLOWTAIL_LIVE_OMP=1 effigy probe:omp-installed
SWALLOWTAIL_LIVE_OMP_PROMPT=1 effigy probe:omp-luna-low
```

The second command spends provider allowance and must use an already configured
OMP account. Neither command exposes credential values to Swallowtail.
