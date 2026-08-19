# Antigravity Prepared Integration

Use `swallowtail-adapter-antigravity` for Google's installed Antigravity CLI.
It is the personal Google subscription harness route. It does not replace or
fall back to Gemini CLI or Gemini Live.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Choose A Route

`prepare_antigravity` requires one explicit `AntigravityPreparedDriver`:

| Selection | Route and driver ID | Operation and transport |
| --- | --- | --- |
| `Catalogue` | `antigravity.catalogue`; `swallowtail.antigravity.catalogue` | authenticated model observation over models stdio |
| `Headless` | `antigravity.headless`; `swallowtail.antigravity.headless` | one explicit-model structured run over stream JSON stdio |
| `Continuation` | `antigravity.headless`; `swallowtail.antigravity.headless` | read-only exact-id turns over one joined stream JSON child each |

Catalogue results do not prove that a model is invocable. Headless and
continuation share the provider stream-JSON interface but keep different
operation shape, authority, and lifecycle.

## Operator Prerequisites

The host supplies:

- one approved `agy` executable target and explicit environment
- provider-owned personal Google subscription sign-in state
- an access profile from `antigravity_personal_google_access_profile`
- matching ready `PreparedAccessEvidence`
- local process, time, task, and working-resource services required by the
  selected operation

Swallowtail does not install Antigravity, start login, inspect its auth store,
or acquire a credential. The access profile is provider-supported local auth
with subscription allowance and no credential reference.

Qualified versions are `1.1.9..=1.1.15` on `antigravity-cli.release`.
Later stable versions remain visible as unverified newer. `1.1.8` is not
silently accepted from the shared documentation tag.

## Prepare The Installation

Create `AntigravityPreparationInput` with the selected driver, configured
instance identity, execution host, approved target, environment, access
profile, and access evidence. Add a bounded `AntigravityPreparationProbe`, then
call `prepare_antigravity`.

Preparation discovers only the supplied target, classifies its exact version,
checks access evidence, records available host services, and returns the
matching `AntigravityPreparedIntegration` variant. It performs no model call.

## Catalogue

On the `Catalogue` variant:

1. create `AntigravityCatalogueProfileInput` with a request ID and optional
   deadline
2. call `prepare_catalogue`
3. inspect its plan and prepared evidence if the application records admission
4. call `list_models`

The operation starts one joined `agy models` child. It observes the signed-in
account's bounded model list without selecting a model or granting invocation.

See
[`prepared_antigravity_catalogue`](../../crates/swallowtail-adapter-antigravity/examples/prepared_antigravity_catalogue.rs).

## Headless Structured Run

On the `Headless` variant, build an
`AntigravityHeadlessRunProfileInput` with:

- request ID and prompt content
- explicit `AntigravityHeadlessModelSelection`
- exact working resource and `Read` or `ReadWrite` access
- `AmbientHost` or explicitly selected `ProviderEnforced` isolation
- deadline
- optional `low`, `medium`, or `high` reasoning effort
- optional provider-native JSON Schema 2020-12 output

Read access selects provider plan mode. Read-write authority remains explicit.
Neither isolation choice enables dangerous permission bypass. The route does
not approve provider permission requests.

Call `prepare_run`, then `start_run`. Drain the run event stream and its single
terminal outcome before `close`. Closing joins the child; it does not delete
provider state.

The stream projects assistant output, provider-disclosed reasoning, correlated
tools, child-agent activity, terminal usage, and the result. Display payloads,
stderr, and raw tool data are not stable diagnostics.

## Exact-ID Continuation

On the `Continuation` variant, build
`AntigravityContinuationProfileInput` with the explicit model and working
resource. The resulting session:

- is read-only and ambient
- supports at most 24 turns
- starts one joined child per turn
- privately supplies only the conversation ID returned by the preceding clean
  turn
- never uses ambient `--continue` or a latest-session lookup

A failed, cancelled, timed-out, or mismatched turn does not advance private
continuation state. There is no public load or resume operation.

For every turn, take and poll its event stream and terminal outcome
concurrently, then close the turn. Cancellation stops only that turn's child.
Close the session after all turns to join local task and resource work.

`prepare_working_state_restoration` opens a fresh replacement session after
process loss. It does not recover the lost conversation or interrupted turn.

The headless and continuation paths are both shown in
[`prepared_antigravity_headless`](../../crates/swallowtail-adapter-antigravity/examples/prepared_antigravity_headless.rs).

## Lifecycle And Failure Handling

Cancellation is scoped to the active run or turn. Consumers still drain the
terminal outcome and call `close`; cleanup truth remains separate. Handle
ordinary failures through the portable classification, retaining the exact
Antigravity diagnostic code for support.

## Unsupported

Antigravity exposes no Swallowtail permission or question exchange, consumer
tools, attachments, external search, provider-session catalogue/import,
public load/resume, archive, restore, delete, or native close. Provider task
and subagent observations grant no direct-control authority.

Promotion requires an exact Antigravity surface and release, prepared-plan and
resource-authority mapping, bounded fixtures, lifecycle tests, and route-matrix
coverage. A CLI flag, provider display, or retained conversation file alone is
insufficient.

## Deterministic Validation

Run:

```sh
effigy validate:focused swallowtail-adapter-antigravity
effigy check:examples
```

No authenticated prompt is required.
