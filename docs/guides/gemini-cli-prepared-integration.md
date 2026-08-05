# Gemini CLI Prepared Integration

Use `swallowtail-adapter-gemini` for the installed Gemini CLI. Select one
branch before discovery; neither falls back to the other.

## Choose A Route

| Route | Driver ID and transport | Choose it for | Reject it when |
| --- | --- | --- | --- |
| `gemini-cli.acp` | `swallowtail.gemini.acp`; ACP v1 over stdio | an interactive session, authorized session-negotiated model options, optional read-only plan mode, or bounded host text writes | the application needs a caller-selected model before session creation, usage, or a one-prompt run |
| `gemini-cli.headless` | `swallowtail.gemini.headless`; Gemini stream JSON over stdio | one explicit-model prompt with streaming and usage | the application needs a reusable session, callbacks, or non-durable transcript posture without accepting best-effort cleanup |

Both branches reject reasoning selection, output-token limits, structured
output, attachments, consumer tools, external search, and provider-session
import or management.

## Operator Prerequisites

The host supplies one approved Gemini executable target, explicit process
environment, configured-instance and execution-host identity, a
provider-supported Gemini Developer API-key profile with matching access
evidence, and the task, process, time, credential, and working-resource
services required by the selected profile. The credential service leases the
opaque API-key reference for the qualified Google audience; no secret enters
the plan or diagnostics.

Swallowtail does not install Gemini CLI, search `PATH`, choose an account,
credential, model, workspace, sandbox, endpoint, billing route, or fallback.
ACP exact `0.51.0` is qualified. Headless `0.51.0..=0.52.0` is qualified.
Later stable releases may prepare as visible `UnverifiedNewer`; older and
excluded releases do not prepare and newer releases gain no capability.

## Prepare The Installation

Build `GeminiCliPreparationInput` with `GeminiCliPreparedDriver::Acp` or
`Headless`, add `GeminiCliPreparationProbe`, and call `prepare_gemini_cli`.
The older branch-specific preparation functions remain public escape hatches.
Preparation probes only the approved target and performs no prompt or model
invocation.

Inspect `evidence()`, `plan()`, and `request()` before effects. The prepared
value retains the exact version, access provenance, target, host, required
services, resource authority, and route-specific low-level driver.

## ACP Interactive Session

Create `GeminiSessionProfileInput::new` for read-only access, or
`GeminiSessionProfileInput::bounded_write` for explicit bounded text-write
authority. Supply request identity, working resource, and `SessionOptions`.
The only portable session option is `HarnessMode::Plan`, and it is accepted
only on the read-only profile.

Call `prepare_session`, then `open_session`. After ACP authorization, inspect
`negotiated_model_options()` on the returned handle. These are bounded
observations from that exact session, not a pre-session catalogue or authority
for another route. The facade does not invent a model selection.

The read-write profile serves exact ACP `fs/read_text_file` and
`fs/write_text_file` requests through the bound working-resource service. It
does not expose a consumer tool callback. Provider permission requests are
observed, rejected, and terminate the turn as `ProviderRequestObserved`; they
cannot be answered through this route.

Take each turn's event stream and terminal outcome immediately and poll them
concurrently. Cancellation interrupts the active turn. Close the turn and
session to join connection, process, resource, credential, and task work.
Terminal and cleanup truth remain separate. Ambient execution and bounded host
writes are not filesystem or descendant-process containment.

See the compile-tested
[`prepared_gemini_acp` example](../../crates/swallowtail-adapter-gemini/examples/prepared_gemini_acp.rs).

## Headless Structured Run

Prepare one run with explicit provider/model route, model, prompt content,
working resource, deadline, and durable-retention acceptance. The route sends
the prompt over stdin, selects plan approval mode, disables extensions and MCP
servers, trusts only the explicit working resource, and does not force
Gemini's separate sandbox.

Call `prepare_run`, then `start_run`. Drain assistant, tool, usage, and
terminal events while awaiting the single terminal outcome, then close the
run. Cancellation or deadline stops and joins the owned process.

Gemini retains its local transcript by default. Opting into
`with_owned_transcript_cleanup()` issues one exact joined delete attempt for
the operation-owned transcript after the run. Because listing can invoke
summary inference and mutate history, Swallowtail performs no list-based
confirmation and reports removal unconfirmed. Close never stops an external
service and grants no reusable session or management binding.

See the compile-tested
[`prepared_gemini_headless` example](../../crates/swallowtail-adapter-gemini/examples/prepared_gemini_headless.rs).

## Restart, Failure, And Promotion

ACP exposes no public load or resume binding. Its
`prepare_working_state_restoration` opens a fresh context-losing session after
process loss; it does not recover the interrupted turn or transcript.
Headless exposes no working-state restoration.

Handle failures through portable classification and retain the exact
`swallowtail.gemini.*` diagnostic for support. Do not parse stderr, ACP data,
provider prose, or transcript files to infer retry, auth, terminal, or cleanup
truth.

Promotion of reasoning, callbacks, attachments, public continuation,
management, reconciliation, or sandbox claims requires an exact Gemini
surface, qualified version evidence, prepared-plan binding, bounded protocol
projection, deterministic fixtures, and route-matrix coverage. An advertised
ACP capability or CLI flag alone is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-gemini
effigy check:examples
```

No API call, login mutation, transcript deletion, or authenticated prompt is
part of deterministic acceptance. Any live probe must be separately enabled
by the operator.
