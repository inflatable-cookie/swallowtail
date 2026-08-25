# Kimi Code Prepared Integration

Use the installed Kimi Code facade for ACP sessions or bounded headless runs.
The caller selects `Acp` or `Headless` before discovery. The facade discovers
one host-approved executable, preserves the route-specific compatibility
result, and derives the configured instance, preflight plan, policy, and
request echoes.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

Both routes live in `swallowtail-adapter-kimi`:

| Selection | Route, driver ID, and transport | Choose it for | Reject it when |
| --- | --- | --- | --- |
| `Acp` | `kimi-code.acp`; `swallowtail.kimi.acp`; ACP v1 over stdio | reusable read-write sessions, negotiated models, reasoning, bounded writes, provider load/resume, or resource-scoped catalogue/import | the application needs a one-prompt run or cannot accept durable Kimi session state |
| `Headless` | `kimi-code.headless`; `swallowtail.kimi.headless`; stream JSON over stdio | one explicit-model bounded prompt with qualified provider retry observations | the application needs callbacks, reasoning selection, usage, a reusable binding, or management |

There is no fallback between the branches. The separate local-server route is
documented in [Kimi Local Server](kimi-local-server-prepared-integration.md).

## Inputs That Stay Explicit

Preparation still requires:

- one configured-instance identity and revision
- one execution host and approved executable target
- one isolated Kimi state environment
- one opaque Kimi state-root identity when provider-session discovery or
  import is required
- one membership OAuth access profile and observed or caller-asserted evidence
- caller-owned probe cancellation and deadline

The selected operation still requires its model route, working resource,
deadline, and supported options. Swallowtail does not select a model,
workspace, account, credential, transport, or fallback route.

The host binds task, process, time, credential, and working-resource services
required by the immutable plan. ACP import also requires the opaque state-root
identity. The membership OAuth credential remains a scoped opaque lease;
Swallowtail never exposes or persists its value.

ACP exact `0.28.1` and `0.29.0..=0.38.0` are qualified. Headless exact
`0.29.0..=0.38.0` is qualified. Later stable releases remain visible
`UnverifiedNewer`; they do not inherit ACP catalogue/import support. Older,
excluded, and prerelease observations do not prepare.

## Prepare The Installation

Construct `KimiCodePreparationInput` with `KimiCodePreparedDriver::Acp` or
`Headless`, add `KimiCodePreparationProbe`, then call `prepare_kimi_code`.
The supplied host services must describe the same host that will execute the
operation and include its preflight-bound services. Only task, time, and
process services are used by the version probe.

The result is `KimiCodePreparedIntegration::Acp` or `Headless`. Each branch
exposes the exact installed observation, access provenance, configured
instance, approved target, and its low-level driver escape hatch. The older
route-specific constructors remain available.

## Browse And Import Existing Sessions

Only the ACP branch exposes provider-session discovery. Supply
`KimiPreparationInput::with_state_root` during installation preparation, then
create a `KimiSessionCatalogueInput` with an explicit catalogue id, working
resource, bounds, and optional deadline.

`prepare_session_catalogue` derives a read-only operation. `list_sessions`
negotiates ACP `session/list` and returns one bounded page. A returned cursor is
private provider state; pass it only through `next_page_request` and
`list_page`. Candidate provider ids, cwd values, and `_meta` never enter stable
diagnostics.

Import remains a separate operation:

1. select one available candidate from a catalogue page
2. call `prepare_session_import` with that catalogue, candidate, and the same
   explicit model/resource session input used for ordinary attachment
3. call `import_session`; Swallowtail re-lists and revalidates before returning
   an explicitly imported binding
4. pass that binding to an ordinary prepared session's `load_session` for
   ordered replay, or later `resume_session` without replay

The catalogue and import must retain the same configured instance, execution
host, exact qualified Kimi version, access evidence, opaque state root, working
resource, model route, and session policy. Missing, changed, active, or
cross-plan candidates issue no binding. Versions above qualified `0.38.0` may
still run other unverified-newer Kimi operations, but cannot inherit catalogue
or import support.

Claude Agent and Cursor remain unavailable on this operation. Stable ACP wire
support or a provider capability advertisement does not qualify an adapter's
list, history replay, state authority, or version range.

## Prepare One Persistent Session

Create a `KimiSessionProfileInput` with:

- request identity
- `KimiModelSelection`
- working-resource reference
- `SessionOptions`

The only portable option is reasoning `off`, `on`, `low`, `medium`, `high`,
`xhigh`, or `max`. Exact `0.29.0..=0.38.0` may prepare `xhigh` or `max` only
when the current session-open `thinking` snapshot advertises that exact value.
Catalogue declaration, caller request, one `session/set_config_option`
dispatch, and response `currentValue` confirmation stay distinct; foreign
advertised rows may coexist but never become public selections. Boolean
`0.28.1` remains `off|on`. Kimi must advertise and confirm the exact selection
before the first prompt. Load and resume cannot redeclare it. Developer
instructions, consumer tools, plan mode, attachments, and question or
permission exchanges are not mapped.

`KimiPreparedIntegration::prepare_session` derives one immutable plan and its
matching new-session request. The plan visibly binds:

- Kimi ACP and the exact executable version
- ambient harness configuration
- `AmbientHost` isolation
- ambient read-write workspace access
- provider-owned durable state prohibited by Swallowtail
- load replay, resume, streaming, active-turn interruption, bounded writes,
  and the selected reasoning mode

Ambient execution is not containment. It makes no filesystem, descendant, or
provider-tool network isolation claim. A future provider- or host-enforced
route requires a separately qualified profile; failure cannot fall back to
this ambient profile.

## Bound Operations

The prepared value exposes distinct operations:

| Operation | Prepared method | Result |
| --- | --- | --- |
| resource-scoped browse | `prepare_session_catalogue` → `list_sessions` | bounded candidate page |
| explicit import | `prepare_session_import` → `import_session` | revalidated imported binding |
| new session | `open_session` | interactive session handle |
| provider load | `load_session` | replay plus interactive session handle |
| provider resume | `resume_session` | interactive session handle, no replay |

Load and resume accept the exact `SessionResumeBinding` previously returned by
a session handle. The prepared facade derives working resource, access,
provider-state, configuration, and plan agreement. It does not reconstruct or
guess a lost binding.

Prompt content remains explicit through `InteractiveSessionHandle::start_turn`.
Active-turn interruption remains explicit through the returned turn handle's
cancellation control. Bounded filesystem writes and delegated authentication
continue through host services; the adapter does not execute consumer tools or
extract credentials.

Take turn events and the terminal outcome immediately and poll them
concurrently. Active-turn cancellation does not imply session cleanup. Close
each turn and the session to join task, connection, process, resource, and
credential work; Kimi's provider state remains preserved.

New, load, and resume handles may expose bounded
`negotiated_model_options()` from Kimi's ACP model selector. The evidence
exists only after that session is authorized. It is not the local-server model
catalogue and does not create a hidden discovery session.

`plan`, `request`, `evidence`, `low_level_driver`, derived load/resume request
helpers, and `into_parts` remain available for diagnostics and advanced use.

See the compile-tested
[`prepared_acp` example](../../crates/swallowtail-adapter-kimi/examples/prepared_acp.rs).

## Prepare One Headless Run

Select `Headless`, then create `KimiHeadlessRunInput` with an explicit model,
prompt content, working resource, and deadline. `prepare_run` derives the
immutable structured plan, mandatory `DurableAllowed` retention policy, and
exact process request.
Call `accept_managed_recovery()` to acknowledge the provider's qualified retry
behavior before preparation; this does not authorize consumer retry.
The driver executes:

```text
kimi --model <alias> --prompt <content> --output-format stream-json
```

Kimi requires prompt content in the process arguments. Swallowtail redacts the
arguments from stable diagnostics and `ProcessRequest` debug output, but the
host operating system may still expose process arguments. Consumers that
cannot accept that host boundary should use another route.

The prepared route binds the audited default v1 print engine and rejects an
environment that enables `KIMI_CODE_EXPERIMENTAL_FLAG`. It reports assistant,
tool activity, retry, and terminal events without claiming consumer tool
callbacks. Cancellation and deadline stop and join the child. Kimi may retain
provider state, so the operation requires `DurableAllowed`; no reusable
session, archive, restore, or delete authority escapes.

Take the event stream and terminal outcome immediately and poll them
concurrently, then close the run. Cancellation and deadline stop the child;
terminal, provider retry observation, and joined cleanup remain separate.

See the compile-tested
[`prepared_headless` example](../../crates/swallowtail-adapter-kimi/examples/prepared_headless.rs).

## Recovery, Failures, And Unsupported Capabilities

ACP can load or resume only from an exact opaque binding. It exposes no
interrupted-turn reconciliation. `prepare_working_state_restoration` uses the
existing binding for bounded attachment recovery without claiming replay or
turn recovery. Headless exposes no reusable binding or restoration action.

Neither installed branch exposes archive, restore, delete, native close,
structured output, attachments, permission/question response, external
search, or billed cost. Headless additionally rejects reasoning selection and
consumer tools and exposes no usage claim. Provider retry activity is
observation, not a consumer retry instruction.

Handle failures through portable classification and keep the exact
`swallowtail.kimi.*` diagnostic for support. Never parse prompt arguments,
stderr, ACP payloads, Kimi prose, or state files to infer auth, retry,
terminal, or cleanup truth.

A capability may be promoted only with exact route/version evidence, a
prepared-plan and access-policy mapping, bounded protocol projection,
deterministic fixtures, and route-matrix coverage. ACP advertisement or a
headless flag alone is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-kimi
effigy check:examples
```

No OAuth mutation, live Kimi prompt, state-file inspection, or destructive
provider work is required.
