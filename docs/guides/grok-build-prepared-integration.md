# Grok Build Prepared Integration

Use `swallowtail-adapter-grok` for the installed Grok Build ACP harness. It is
separate from xAI's hosted Responses WebSocket route.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

The route is `grok-build.acp`, driver ID `swallowtail.grok-build.acp`, over
ACP v1 stdio. Choose it for the installed subscription harness and reject it
when the application needs hosted xAI inference, answerable permissions,
usage, or public provider-session management.

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
`grok-build.executable`. Exact `1.0.4` is the maintained milestone and binds
`grok-4.6`. Exact `0.2.117` keeps its private task-control behavior revision.
Later stable versions above `1.0.4` remain visible unverified newer. Mid-gap
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
exact `1.0.4` and permitted unverified-newer points that inherit that
milestone. No model fallback is performed. Interactive initialization may expose
authorized session model options on the returned handle; that observation
does not become a standalone provider catalogue.

Reasoning selection, output limits, and structured output are not qualified.

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

Provider permission requests are observable but not answerable. They stop the
turn. Do not treat observation as approval, ambient permission, or sandbox
evidence.

An exact existing binding may use `prepare_working_state_restoration` for
bounded attachment recovery after process loss. This reattaches the durable
provider session without claiming transcript replay or interrupted-turn
reconciliation. There is no public load or resume operation.

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
output, attachments, consumer tools, permission/question response, external
search, provider-session management, or provider-managed retry.

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
