# Kimi Code Prepared Integration

Use the installed Kimi Code facade for ACP sessions or bounded headless runs.
The caller selects `Acp` or `Headless` before discovery. The facade discovers
one host-approved executable, preserves the route-specific compatibility
result, and derives the configured instance, preflight plan, policy, and
request echoes.

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
cross-plan candidates issue no binding. Versions above qualified `0.31.1` may
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

See the compile-tested
[`prepared_headless` example](../../crates/swallowtail-adapter-kimi/examples/prepared_headless.rs).
