# Pi RPC Prepared Integration

Use the prepared facade for Pi's maintained strict-LF RPC subprocess. It probes
one host-approved executable and derives the configured instance, exact
version binding, restrictive RPC policy, preflight plan, and open-session
request.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

The route is `pi.rpc` in `swallowtail-adapter-pi`, with driver ID
`swallowtail.pi.rpc` and strict-LF JSONL RPC over stdio. Choose it for Pi's
configured model catalogue, a one-prompt no-session run, or an interactive
session with typed UI questions and optional bounded PNG input. Reject it when
the application needs reasoning selection, writes, permission exchange,
provider-session continuation, or lifecycle management.

Both Pi routes remain production. `pi.rpc` needs only one installed upstream
executable and speaks Pi's own RPC wire; it stays fresh-only because the RPC
surface cannot prove a stored session's cwd against the host lease.
[Pi SDK sidecar](pi-sdk-sidecar-prepared-integration.md) realizes persistent
new, load-with-replay, and replay-free resume through the public SDK, but
requires the application to provision an exact Node runtime, the source-tagged
sidecar, and the exact SDK package over a Swallowtail-owned private wire.
Neither route substitutes for the other.

## Explicit Inputs

Preparation requires:

- configured-instance identity and revision
- execution host and approved executable target
- selected process environment
- maintainer-supported delegated harness access profile, credential reference,
  and access evidence
- probe deadline and cancellation

Session preparation requires a request identity, provider, model route, model,
working-resource reference, optional session deadline, and empty
`SessionOptions`.

Structured-run preparation requires a request identity, provider, model route,
model, content, working-resource reference, and mandatory deadline.

Catalogue preparation requires only a request identity and optional deadline.
Call `PiPreparedIntegration::prepare_catalogue`; its plan has no provider,
model route, prompt, session, or working resource.

Swallowtail does not choose the provider, model, account, credential,
workspace, or fallback route.

The host binds task, process, time, credential, working-resource, and optional
attachment services required by the chosen plan. The delegated harness
credential remains an opaque scoped lease; Swallowtail does not inspect Pi's
provider configuration, login state, billing, or endpoint selection. The
working resource is read-only and `ProviderSuppressed` configuration is not a
sandbox.

## Version Posture

Pi `0.80.10` is the qualified strict-LF RPC baseline. Exact published
`0.81.0`, `0.81.1`, `0.82.0`, `0.82.1`, `0.83.0`, `0.84.0`, `0.84.1`, and
`0.84.2` are also qualified with their own behavior milestones. Unpublished
`0.83.1` does not prepare. Discovery records the exact installed version.
A later stable release is admitted as unverified, remains visible in evidence,
and uses the latest qualified behavior mapping. Unpublished gaps, older points,
and prereleases do not prepare.

The installed npm form may use an interpreted launcher such as
`#!/usr/bin/env node`. Prefer `LocalExecutableLaunch::interpreted_script` (exact
interpreter plus script prefix) before `approve_installed_executable_launch`.
Approving the script as a native executable fails with
`interpreted_launcher_requires_host_recipe` because ambient `PATH` is cleared.
The Pi adapter does not search PATH, inspect npm layouts, or substitute a
launcher. Exact installed `0.83.0` and later qualified `0.84.2` discovery use
this path.

## Execution Boundary

The prepared plan binds:

- `strict-lf-jsonl-stdio`
- ambient-host execution with provider configuration sources suppressed by
  exact Pi flags
- read-only workspace access
- provider-owned durable session state prohibited
- one caller-selected provider and model route
- the restrictive prompt, steering, follow-up, configuration, and background
  action bounds from the Pi RPC contract

`ProviderSuppressed` describes Pi configuration flags. It is not a sandbox or
containment claim. The adapter does not add permission prompts or infer
filesystem or process isolation.

`PiPreparedCatalogue::list_models` starts one ephemeral provider-suppressed
RPC child, calls `get_available_models`, projects bounded configured
provider/model evidence, then closes and joins the child. It does not select or
invoke a model.

`PiPreparedSession::open_session` returns the unchanged interactive session
handle. Prompt turns, steering, follow-up scheduling, UI callback relay,
interruption, and joined cleanup remain on the existing session and turn
interfaces. The facade does not copy those lifecycle rules into shared facade
records.

Take each turn's events, typed question callbacks, and terminal outcome
immediately and poll them concurrently. Answer each callback at most once with
the correlated portable question and option IDs. Cancellation interrupts the
active turn; session close joins the child, task, resource, credential, and
attachment work. A question is not a permission request.

Pi `confirm`, `select`, `input`, and `editor` requests project into the common
typed harness user-input callback. Stable runtime question and option ids
correlate the response; Pi display-only UI stays a separate observation.
These prompts ask for user input. They do not grant provider permissions.

`PiPreparedRun::start_run` starts a separate `StructuredRun` operation. It
launches one `--no-session` RPC child, sends one prompt, exposes Pi's bounded UI
callback exchange, awaits `agent_end`, then joins the turn, process,
working-resource, and credential work. Its policy prohibits provider
retention. It exposes no provider run, reusable session, resume binding, or
management binding.

Drain run events, callbacks, usage, and terminal concurrently before `close`.
Terminal status and cleanup remain separate. The run has no provider session
to archive, restore, delete, load, or resume.

`PiRunProfileInput::with_attachments` accepts at most one `image/png`
descriptor with a declared size no greater than one MiB.
`PiSessionProfileInput::with_image_attachments` binds the same capability for
later `TurnRequest` values. The host materializes the opaque reference; the
driver reads the bounded file through host-approved blocking work, sends Pi's
inline base64 image record, and releases the lease after provider work.

The configured restrictive RPC policy remains visible in the immutable plan.
Interactive steering and follow-up scheduling are not reclassified as
structured-run operations.

`plan`, `request`, `evidence`, `low_level_driver`, and `into_parts` remain
available for inspection and advanced use.

See the compile-tested
[`prepared_pi_rpc` example](../../crates/swallowtail-adapter-pi/examples/prepared_pi_rpc.rs).

## Restart, Failures, And Promotion

Pi exposes no durable Swallowtail resume or management binding. A prepared
interactive profile can use `prepare_working_state_restoration` only to open a
fresh context-losing session after process loss. It does not recover the
interrupted turn, pending callbacks, or transcript.

Handle failures through portable classification and retain the exact
`swallowtail.pi.*` diagnostic for support. Do not parse RPC records, UI text,
stderr, Pi state, or provider prose to infer authentication, retry, terminal,
or cleanup truth.

Unsupported capabilities include reasoning control, structured output,
consumer tools, permission exchange, writes, external search, public
continuation, provider-session catalogue/import, archive/restore/delete,
subagent control, and billed cost. Promotion requires exact Pi protocol and
published-version evidence, immutable prepared-plan binding, bounded fixtures,
and route-matrix coverage; a provider plugin or UI behavior alone is not
sufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-pi
effigy check:examples
```

No configured provider call, credential use, prompt, or account mutation is
required.
