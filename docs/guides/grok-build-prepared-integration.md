# Grok Build Prepared Integration

Use `swallowtail-adapter-grok` for the installed Grok Build ACP harness. It is
separate from xAI's hosted Responses WebSocket route.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

The route is `grok-build.acp`, driver ID `swallowtail.grok-build.acp`, over
ACP v1 stdio. Choose it for the installed subscription harness. It can expose
answerable one-shot permissions on an explicitly opted-in interactive session;
structured runs and the default session profile reject permission requests.
On the maintained `1.0.4..=1.0.5` segments it also carries consumer registered
tools through the Swallowtail-owned courier.
Reject it when the application needs hosted xAI inference, usage, or public
provider-session management.
The separate `grok-build.catalogue` route (`GrokCatalogueDriver`,
`swallowtail.grok-build.catalogue`) lists installed models on exact `1.0.25`
only; see Model Catalogue below. It never opens ACP or sends a prompt.

## Route And Operation Shapes

`prepare_grok_build` prepares `grok-build.acp`. One prepared installation may
derive either:

- `GrokPreparedRun` for an operation-private one-prompt structured run
- `GrokPreparedSession` for a durable interactive ACP session

Both use ACP v1 over one Swallowtail-owned child. They share delegated
subscription access, exact model selection, activity, and ambient working
resource authority; their operation lifecycles remain distinct.

## Operator Prerequisites

The host supplies:

- one approved Grok Build executable target and environment
- provider-owned delegated Grok subscription OAuth state
- one opaque `CredentialRef` admitted by the credential service
- `grok_build_subscription_access_profile` and matching ready access evidence
- task, process, working-resource, and credential services; runs also require
  time service

Run and session plans bind one exact read-write working resource. Ambient
execution is not a sandbox or filesystem/descendant-process containment claim.

Swallowtail does not perform browser login, extract tokens, switch accounts,
or fall back to an API key. ACP activation uses only the advertised
`cached_token` mechanism after initialization.

Versions `0.2.114..=0.2.117` remain permitted as deprecated segments on
`grok-build.executable`. `1.0.4` through `1.0.5` are the maintained milestone
and bind `grok-4.6`. Exact `0.2.117` keeps its private task-control behavior
revision. Later stable versions above `1.0.5` remain visible unverified newer,
including published alpha `1.0.6` which is not official latest. Mid-gap
`0.2.118..=0.2.121` and unprobed `1.0.0..=1.0.3` are incompatible.

## Prepare The Installation

Create `GrokPreparationInput` with configured instance, revision, execution
host, target, environment, delegated access profile, and access evidence. Add
a bounded `GrokPreparationProbe`, then call `prepare_grok_build`.

Preparation discovers only the supplied target and validates the access
profile before provider work. Keep the returned `GrokPreparedIntegration`
bound to that exact target and host.

## Model Selection

Both operations require `GrokModelSelection` for the model qualified to the
admitted executable behavior: `grok-4.5` on the `0.2` segments, `grok-4.6` on
`1.0.4` through `1.0.5` and permitted unverified-newer points that inherit that
milestone. No model fallback is performed. Interactive initialization may expose
authorized session model options on the returned handle; that observation
does not become a standalone provider catalogue. That negotiated evidence stays
separate from the prepared `grok-build.catalogue` operation below, which is
the only pre-session model inventory.

Reasoning selection, output limits, and structured output are not qualified.

## Model Catalogue

Create `GrokCatalogueProfileInput` with request ID and optional deadline, then
call `prepare_catalogue` on the admitted integration and `list_models`. The
operation runs exactly `models` with stdin closed unwritten, bounds combined
output, requires successful exit, and joins cleanup on success, failure,
timeout, and cleanup failure. It never retries, updates the CLI, opens ACP,
or falls back to an execution default.

Preparation admits only exact installed `1.0.25` under the `QualifiedOnly`
claim `grok-build.catalogue.executable-1-0-25`
(`grok-build.catalogue.models-text-v1`); every older or newer point fails
closed before any process starts. Preparation also requires the same delegated
subscription access readiness as the ACP operations.

Rows preserve provider order and exact opaque ids. The top-level default from
the `models` document marks `is_default`; a missing, repeated, or disagreeing
marker fails the operation instead of guessing. Display name, description,
input token limit, and reasoning modes come only from the frozen exact-`1.0.25`
default-model document by exact id equality (`grok-4.5` has no description,
so it stays unknown there); unknown future ids pass through with empty
metadata and no provider id is ever attached. Malformed, duplicate, empty,
and over-limit documents fail closed, exit status is reported without quoting
stderr, and there is no consumer cancellation surface: enforce the deadline
instead. Research 305 freezes the static command/output evidence.

## Structured Run

Create `GrokRunProfileInput` with request ID, explicit model, prompt content,
read-write working resource, and optional deadline. Call `prepare_run`, then
`start_run`.

The route creates one operation-private provider session, prompts once, emits
assistant, reasoning-summary, plan/task-list, tool, and terminal activity, and
closes the Swallowtail attachment. Provider-owned local session state remains.
There is no usage claim or automatic transcript cleanup.

Drain the event stream and single terminal outcome before `close`. Cancellation
is scoped to the structured run; cleanup joins the owned child.

## Interactive Session

Create `GrokSessionProfileInput` with request ID, explicit model, read-write
working resource, and empty `SessionOptions`. Portable session instructions,
reasoning, tools, and plan-mode options are not mapped.

Call `prepare_session`, then `open_session`. The session exposes streaming
assistant, reasoning-summary, plan/task-list, and tool activity plus active-turn
interruption.

For every prompt, take and poll events and terminal concurrently, then close
the turn. Cancellation stops the active turn. Session close joins local ACP,
process, credential, and working-resource work while preserving Grok state.

Provider permission requests are rejected and cancel the turn by default. An
interactive session may opt in with
`GrokSessionProfileInput::with_consumer_mediated_permissions()`. The prepared
plan then binds the exact `acp/session/request-permission` namespace and the
session turn exposes a `CallbackExchange`. Each callback carries the bounded
tool-call view and only the offered `allow_once` and `reject_once` choices;
persistent choices are withheld. A successful consumer response selects one
offered option, while consumer failure maps to the offered one-shot rejection.
The callback uses the turn deadline, permits only one outstanding request, and
abandonment on cancellation, timeout, failure, or close rejects the pending
exchange. Silence never grants access. Structured runs retain the default
reject-and-cancel behavior.

An exact existing binding may use `prepare_working_state_restoration` for
bounded attachment recovery after process loss. This reattaches the durable
provider session without claiming transcript replay or interrupted-turn
reconciliation. There is no public load or resume operation.

## Consumer Registered Tools

On the maintained `1.0.4..=1.0.5` segments the route carries consumer
registered tools. `GrokRegisteredToolBinding::qualify` binds one Contract 063
`RegisteredToolPreparation`, and an interactive session opens with
`GrokPreparedSession::open_registered_session`. The Swallowtail-owned
mediated-stdio courier is then declared as one reserved entry in the ACP
`session/new` `mcpServers` list; Grok spawns that child, and every admitted
call settles through the Contract 063 kernel. The lease is bound to one exact
turn and settles at its terminal or on cancellation.

Registered-tool qualification is version-scoped: a session opened with a
registered binding on any executable version outside exact maintained
`1.0.4..=1.0.5` fails typed with `version_not_admitted` before any host,
lease, or provider work, and the projection publishes the unqualified truth
for those versions. On an admitted version Grok Build admits the
client-declared server and calls its tools, but it cannot represent a consumer
Deny (the provider's one-shot permission exchange is a separate channel), and
the route delivers no consumer tool progress. A session that opens without a
registered binding is byte-identical to the plain route: `session/new` still
carries `mcpServers: []`.

Selected-skill bundles are not carried. ACP v1 session setup has no
session-scoped, distinctly labelled skill input, the frozen Grok artifacts name
none, and the accepted live capsules carried none; Grok-native skills are
ambient configuration, which is not an opted-in bundle. Appending skill text
to the user prompt is not an admissible substitute.

Both operation shapes are shown in
[`prepared_grok_build_acp`](../../crates/swallowtail-adapter-grok/examples/prepared_grok_build_acp.rs).

## Persistence And Control

Grok owns durable local session state. Ordinary handle close preserves it.
Swallowtail exposes no provider-session catalogue, import, archive, restore,
delete, native close, or cleanup operation.

The `0.2.117` task-control delta is private compatibility evidence. It does not
grant provider task control, targeted child cancellation, or subagent control.

## Failure Handling

Keep preparation stage, terminal status, and cleanup outcome separate. Use
portable failure classification when present and retain the exact Grok
diagnostic. Never parse stderr, provider text, or permission display content
to infer retry or authentication policy.

## Unsupported

The route has no usage or billed-cost evidence, reasoning control, structured
output, attachments, selected-skill bundles (see above), question response,
external search, provider-session management, or provider-managed retry.
Consumer registered tools are qualified on the maintained `1.0.4..=1.0.5`
segments only, with the exact Deny and progress bounds stated above; no
registered-tool claim extends to deprecated `0.2.x` segments or unverified
newer points.

Promotion requires an exact Grok Build surface and release, prepared-plan and
access binding, bounded ACP fixtures, lifecycle tests, and route-matrix
coverage. Provider task-control internals or ACP advertisement alone are
insufficient.

## Deterministic Validation

Run:

```sh
effigy validate:focused swallowtail-adapter-grok
effigy check:examples
```

The optional installed-version probe is identity-only. Do not run it or an
authenticated prompt as part of deterministic integration acceptance.
