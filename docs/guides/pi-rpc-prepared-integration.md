# Pi RPC Prepared Integration

Use the prepared facade for Pi's maintained strict-LF RPC subprocess. It probes
one host-approved executable and derives the configured instance, exact
version binding, restrictive RPC policy, preflight plan, and open-session
request.

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

Catalogue preparation requires only a request identity and optional deadline.
Call `PiPreparedIntegration::prepare_catalogue`; its plan has no provider,
model route, prompt, session, or working resource.

Swallowtail does not choose the provider, model, account, credential,
workspace, or fallback route.

## Version Posture

Pi 0.80.10 is the qualified strict-LF RPC baseline. Discovery records the exact
installed version. A later stable release is admitted as unverified, remains
visible in evidence, and uses the latest qualified behavior mapping. Older or
prerelease versions do not prepare.

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

`plan`, `request`, `evidence`, `low_level_driver`, and `into_parts` remain
available for inspection and advanced use.

See the compile-tested
[`prepared_pi_rpc` example](../../crates/swallowtail-adapter-pi/examples/prepared_pi_rpc.rs).
