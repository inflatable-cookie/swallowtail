# Muse Code Prepared Integration

Use `swallowtail-adapter-muse` for Meta's installed Muse Code harness. The
route is `muse-code.headless`; the driver ID is
`swallowtail.muse-code.headless`. It owns one exact read-only structured run
over Muse event JSONL stdio.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

This package is additive unreleased source after `v0.1.1`. Consumers must pin
an explicitly reviewed commit containing it. Neither `v0.1.0` nor `v0.1.1`
contains the package or route.

## Selected Boundary

The route selects:

- exact signed payload `muse-bin-0.1.0-R708.1`
- opaque release axis `muse-code.signed-payload`
- exact provider `meta`
- exact model `muse-spark-1.2`
- one explicit reasoning effort
- one read-only filesystem working resource
- ambient local-account configuration and Muse's provider-enforced sandbox
- one attached child process with prohibited retention, recovery, and stream
  reattachment

The mutable `muse` launcher is not an execution target. It may update before
delegating. The host resolves and approves the versioned payload directly.
Swallowtail does not search PATH, install Muse, run login, choose a model, or
follow the launcher's update channel.

## Local Meta Account Access

Call `muse_local_meta_account_access_profile` with an application-owned
`AccessProfileId`. Supply matching `PreparedAccessEvidence` with:

- `CredentialState::NotRequired`
- `EntitlementState::Available`
- `EndpointAuthorization::Allowed`
- `RuntimeReadiness::Ready`
- `SupportAuthority::ProviderSupported`

This means Muse owns its local Meta account state. It does not mean anonymous
access. Swallowtail receives no credential reference, opens no credential
lease, and does not inspect an auth file. The approved process environment may
carry `HOME` or another operator-approved configuration root needed by Muse.
Do not copy tokens into an environment value or portable evidence.

## Prepare The Installed Payload

Construct `MusePreparationInput` with:

- configured instance ID and revision
- execution host ID
- `InstalledExecutableTarget` for the exact versioned payload
- approved `EnvironmentRef`
- local Meta account profile and matching evidence

Construct `MusePreparationProbe` with a request ID, scope, deadline, and
`DiscoveryCancellation`. Then call `prepare_muse_headless`.

Preparation runs only the approved payload with `--version`. It requires the
exact line `Muse Code 0.1.0 (0.1.0-R708.1)`, binds the resulting compatibility
observation, and returns `MusePreparedIntegration`. It sends no prompt and does
not test account access. A launcher target, wrong axis, wrong version, wrong
host, unavailable discovery result, or mismatched access profile fails closed.

Use `validate_execution_binding` before reusing persisted host/target input.
`environment`, `target`, `observation`, `access_profile`, `access_evidence`,
`instance`, and admitted host-service kinds remain inspectable as immutable
evidence.

## Select Model And Effort

Create `MuseHeadlessModelSelection` with an application-owned route ID and
revision plus exact provider `meta` and model `muse-spark-1.2`.

Create `MuseRunProfileInput` with:

- request ID
- model selection
- prompt `OperationContent`
- one `ReasoningMode`
- read-only `WorkingResourceRef`
- deadline

The accepted efforts are exactly:

- `none`
- `minimal`
- `low`
- `medium`
- `high`
- `xhigh`
- `ultra`

There is no default. Every prepared request and model route carry the selected
effort. A missing or different effort, provider, or model is rejected before
the model process starts.

## Execute And Observe

Call `prepare_run`, inspect `MusePreparedRun::evidence`, `plan`, and `request`,
then call `start_run` with host services for the same execution host. Drain the
event stream and terminal outcome before `close`.

The command fixes JSON output, provider, model, effort, one model-step budget,
bounded tool output, automatic resolution of otherwise unattended user input,
and the following prohibitions:

- web tools
- foreign personal context
- session log
- writes
- shell
- parallel tool calls

Muse's provider sandbox stays enabled. Ambient configuration describes the
local account/config source; it does not weaken the provider-enforced sandbox.
The host working resource remains read-only in the portable plan.

Run output appears through ordinary ordered runtime events and the terminal
outcome. Muse task events project as identity-and-lifecycle-only `Task`
activity. This is not a task-list snapshot. Unknown bounded events are kept
under `muse-code.headless.event.*` without gaining terminal, callback, or
semantic authority.

Keep terminal status separate from cleanup. Cancellation force-stops the child;
deadline and cancellation remain distinct; close joins the pump task. A
provider terminal failure, nonzero harness exit, malformed event stream, host
failure, and cleanup failure retain separate safe diagnostics and portable
failure classification.

## Unsupported

This route exposes no:

- model catalogue or fallback model selection
- interactive session, continuation, load, resume, import, or management
- plan mode
- permission or typed-question callback
- attachment or consumer-tool exchange
- structured output or output-token ceiling
- workspace write, shell, or external-search authority
- usage or billed-cost evidence
- retained background work, stream reattachment, or recovery
- task-list snapshot
- subagent topology, attribution, messaging, or control

### Working-State Restoration Disposition

Muse has no `prepare_working_state_restoration` on its prepared facade,
unlike the interactive headless peers. The disposition is recorded rather
than migrated: Muse exposes no interactive session, continuation, load, or
resume route, so there is no interrupted working state to restore; every run
is one exact-model structured run that disables the session log and admits no
reusable provider-session identity. The route stays replacement-only, and a
working-state restoration surface would be a new route qualification, not a
facade gap.

Help text for Muse's broader session, transcript, skill, trace, login, and
cross-session features does not promote them into this route. The direct Meta
Model API is a separate possible provider route.

The normal public shape is shown in
[`prepared_muse_headless`](../../crates/swallowtail-adapter-muse/examples/prepared_muse_headless.rs).

## Validation And Optional Probes

Deterministic validation:

```sh
effigy validate:focused swallowtail-adapter-muse
effigy package:verify-affected swallowtail-adapter-muse
```

Optional operator-gated evidence:

```sh
SWALLOWTAIL_LIVE_MUSE=1 effigy probe:muse-installed
SWALLOWTAIL_LIVE_MUSE_PROMPT=1 effigy probe:muse-spark-low
```

The second command spends provider allowance through an already configured
local Meta account. It requests low effort, forbids tools in the prompt and
command, and must not mutate the workspace. Neither probe exposes credential
values to Swallowtail.
