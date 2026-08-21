# Pi SDK Sidecar Prepared Integration

Use the prepared facade for the Pi SDK sidecar route: Pi's official
TypeScript SDK running inside a host-owned Node sidecar over the private
strict LF-JSON wire `swallowtail-pi-sdk-jsonl-v1`. The application provisions
the exact approved Node runtime, the source-tagged sidecar entry point, and
the exact SDK package through a host-approved interpreted-script launch
recipe; preparation binds the configured instance, exact version bindings,
restrictive policy, preflight plan, and session request.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

The route is `pi.sdk-sidecar` in `swallowtail-adapter-pi`, with driver ID
`swallowtail.pi.sdk-sidecar`. Choose it for Pi's configured model catalogue
or an interactive session with durable provider-session continuation:
persistent new, load with bounded typed replay, and replay-free resume under
the exact host-leased working directory. Reject it when the application
cannot provision the Node runtime, sidecar asset, and SDK package, or needs
structured runs, typed UI questions, reasoning selection, writes, permission
exchange, or provider-session lifecycle management.

Both Pi routes remain production. `pi.rpc` needs only one installed upstream
executable and speaks an upstream-owned wire, but stays fresh-only: Research
180's attachment gate still blocks RPC load and resume. `pi.sdk-sidecar`
realizes continuation through the public SDK but requires the provisioned
Node/SDK boundary and a Swallowtail-owned private wire. Neither route
substitutes for the other; pick by operational posture.

## Explicit Inputs

Admission requires an admitted instance record for the `pi.sdk-sidecar`
addable route carrying opaque host-owned references only:

- a launch-recipe reference binding the approved Node runtime and sidecar
  entry point (`LocalExecutableLaunch::interpreted_script`)
- an environment reference whose approved body carries
  `PI_SDK_SIDECAR_SDK_MODULE` (exact SDK entry path),
  `PI_SDK_SIDECAR_AGENT_DIR`, and `PI_SDK_SIDECAR_SESSION_DIR`
- a delegated harness credential reference

`PiSdkSidecarSessionPreparation::from_admitted` lifts those references into
the explicit preparation input without exposing paths, environment values,
or credential bytes. Direct construction takes the same pieces explicitly:
configured-instance identity and revision, execution host, launch target,
environment, credential, access profile, provider, model route, model,
working-resource reference, and request identity.

Swallowtail does not choose the provider, model, account, credential,
workspace, Node runtime, SDK package, or fallback route, and never runs
`npm install` or ambient discovery.

The host binds task, process, time, credential, and working-resource
services. The delegated harness credential remains an opaque scoped lease;
the sidecar's environment is fully cleared except the approved entries. The
working resource is read-only and `ProviderSuppressed` configuration is not
a sandbox.

## Version Posture

Four separate axes carry qualified-only one-point claims; none admits an
unverified-newer point:

- `pi.sdk-sidecar.package`: exact `@earendil-works/pi-coding-agent@0.84.2`
- `pi.sdk-sidecar.node`: exact Node `22.23.2` (satisfying the upstream
  `>=22.19.0` requirement)
- `pi.sdk-sidecar.wire`: exact `swallowtail-pi-sdk-jsonl-v1` (opaque)
- `pi.sdk-sidecar.sidecar`: the exact source-tagged sidecar revision
  (opaque)

Older, newer, and prerelease points on any axis do not prepare. The claim
does not inherit the RPC package window or its unverified-newer posture even
though both routes qualify the same upstream package release.

## Execution Boundary

The prepared plan binds:

- the private `swallowtail-pi-sdk-jsonl-v1` wire over one owned sidecar
  process per attachment
- ambient-host execution with provider configuration sources suppressed by
  construction (in-memory settings, no extensions, skills, prompts, context
  files, themes, aliases, update checks, retry, or catalogue network)
- read-only workspace access and exactly the `read`, `grep`, `find`, and
  `ls` tools
- durable provider-session state preserved on close inside the
  application-provisioned session directory
- one caller-selected provider and model route
- the restrictive prompt, steering, follow-up, configuration, and background
  action bounds from the harness RPC policy

`AmbientHost` is an explicit posture, not containment. The adapter does not
add permission prompts or infer filesystem or process isolation.

Before any provider work the driver verifies the bootstrap response: wire,
behavior revision, SDK package and version, Node version, the exact leased
working directory, provider, model, the read-only tool set, and a state
re-check. Every mismatch fails startup before a prompt exists.

`PiSdkSidecarPreparedSession::open_session` returns the interactive session
handle with the opaque provider-session reference and the exact Contract 017
restart binding. Export the binding only through
`SessionResumeBinding::export_persisted`; restore requires the current exact
plan, resource, and policy, and failure never falls back to a new session.

`load_session` switches the fresh sidecar to the bound session with the
expected-cwd gate, rejects a substituted session reference, transports the
bounded ordered typed replay (1,024 messages, 4 MiB of content), and becomes
ready only after the replay response and state re-check. `resume_session`
attaches to the same bound session with no replay phase; replay evidence
during resume fails the transport. Close and disconnect preserve the durable
provider state while joining process, resource, and credential work.

`take_callbacks` returns no exchange: the sidecar surface has no UI,
question, or permission callbacks. Cancellation aborts the active turn;
session close issues the sidecar `close` command, joins the process, then
releases the resource and credential leases in contract order.

`PiSdkSidecarDriver::list_models` spawns the sidecar in catalogue-only mode,
reads the bounded configured catalogue from the explicitly constructed
offline runtime, then closes and joins the process. It does not select or
invoke a model.

See the compile-tested
[`prepared_pi_sdk_sidecar` example](../../crates/swallowtail-adapter-pi/examples/prepared_pi_sdk_sidecar.rs).

## Restart, Failures, And Promotion

Load and resume re-derive the provider working directory only from the host
lease resolved for the binding; provider-stored roots and ambient state
cannot replace it. A stale, substituted, malformed, or drifted binding fails
before any provider work. Disconnect invalidates the runtime attachment but
never deletes the durable provider session.

Handle failures through portable classification and retain the exact
`swallowtail.pi.sdk-sidecar.*` diagnostic for support. Do not parse sidecar
records, stderr, SDK state, or provider prose to infer authentication,
retry, terminal, or cleanup truth.

Unsupported capabilities include structured runs, typed question and
permission exchange, reasoning control, structured output, consumer tools,
writes, external search, provider-session catalogue/import, archive,
restore, delete, recovery attachment, and billed cost. Promotion requires
exact SDK, Node, sidecar, and wire evidence, immutable prepared-plan
binding, bounded fixtures, and route-matrix coverage.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-pi
effigy check:examples
```

No Node install, package resolution, configured provider call, credential
use, prompt, or account mutation is required.
