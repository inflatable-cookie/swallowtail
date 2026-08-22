# DeepSeek Prepared Integration

Use `swallowtail-adapter-deepseek` route `deepseek.continuation`, driver ID
`swallowtail.deepseek.direct`, for the exact DeepSeek Open Platform V4 Pro
surface over OpenAI-compatible HTTPS/SSE. Choose it for catalogue observation,
one bounded reasoning request, or consumer-owned direct tool continuation.
Reject it when the application needs a durable provider thread, working
resource, attachments, structured output, or managed recovery.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Add The Connection

`deepseek.continuation` currently exports an addable descriptor. Topology is
**hosted**. It is not `ExecutionLayer`. Follow
[connection lifecycle](connection-lifecycle.md) before the prepared facade.

1. Assemble `AddableRouteCatalog` from
   `deepseek_continuation_addable_route_descriptor`. The row is `Available`
   when the host exposes the Credential service; otherwise
   `Unavailable(HostService)`.
2. `admit_instance` writes the configured instance. Admission does not
   prepare.
3. Collect the secret API-key field `api_key` as `CredentialRef`. There is
   no advertised environment name. Host stores the bytes. The library-owned
   API-key loop does not use URL-open, loopback, or device-code ports.
4. Config field `endpoint` is an opaque `ApiEndpoint` `ConfigFieldRef`.
5. `refresh_readiness` writes host-supplied `AccessStatus`. Enablement stays
   independent of 047 `Ready` / `NotReady`.
6. `observe_authenticated_subject` is `Absent`. Do not probe Open Platform
   for identity.
7. Overlay may mark catalogue rows whose `provider_id` is `deepseek`.
   Overlay copies readiness and cannot change it.
8. Build `DeepSeekPreparationInput::from_admitted` from the admitted record,
   then call `prepare_deepseek_direct`. The constructor selects the stored
   `endpoint` and `api_key` refs; the host resolves them, and preparation still
   requires the exact `https://api.deepseek.com` endpoint.

The compile-tested
[`connection_lifecycle` example](../../crates/swallowtail-adapter-deepseek/examples/connection_lifecycle.rs)
shows catalog through prepare. The canonical route-map example remains
[`prepared_direct_continuation`](../../crates/swallowtail-adapter-deepseek/examples/prepared_direct_continuation.rs).
Hosted interactive OAuth is not a realized path on this route.

## Operator Prerequisites

`DeepSeekPreparationInput::from_admitted` requires the admitted continuation
route's endpoint and API-key refs. Preparation then requires the exact
`https://api.deepseek.com` target, the Open Platform API-key pay-as-you-go
profile, and matching access evidence. The host resolves the refs and supplies
credential, HTTP, task, and time services.

App credentials, OAuth, proxies, gateways, Anthropic facades, `/v1`, beta
endpoints, model aliases, and third-party compatible APIs cannot substitute.
Swallowtail does not discover an account, read an environment variable, choose
a model or endpoint, select billing, or fall back.

The route binds one exact opaque `deepseek.openai-chat-facade` revision. It has
no ordered or unverified-newer range. Compatible JSON does not widen the
contract.

## Preparation And Catalogue

Preparation performs no network or credential work. The result exposes its
safe configured instance, exact facade assessment, access provenance, service
set, and `DeepSeekDirectDriver` escape hatch.

`prepare_catalogue` and `list_models` observe the exact account source without
selecting a model. Catalogue presence proves neither balance, entitlement,
capacity, cache state, billing, nor invocation success.

## Generation Controls

The exact V4 Pro route accepts portable `ReasoningMode` values `low`, `high`,
and `max`. Preparation copies the selected value into the immutable plan,
prepared evidence, configured driver, and request policy. Every request sends
the same value as `reasoning_effort`; the adapter never clamps, aliases, or
reports an effective reasoning depth. `medium`, `xhigh`, provider aliases, and
unknown values fail before endpoint or credential work.

Every admitted profile sends `thinking: {"type":"enabled"}`. DeepSeek
documents `disabled` upstream, but this route has no qualified typed control
for that independent field. It remains withheld for structured runs, and
continuation cannot admit it because the existing tool-bearing proof requires
private `reasoning_content` replay.

The route distinguishes dispatched request fields from provider acceptance,
effective reasoning depth, and observed private continuation. Deterministic
tests prove only exact local dispatch and selection agreement. Provider
acceptance and effective depth remain unclaimed. `reasoning_content` is
adapter-held bounded state used only for same-route continuation; it is never
consumer output or durable session material.

## One-Request Structured Run

`prepare_run` requires the exact `deepseek-v4-pro` route, `low`, `high`, or
`max` reasoning, text content, positive output-token limit, optional host
deadline, and explicit
`ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority`.
`start_run` sends one tool-free streamed request.

Take and drain events and terminal concurrently, then close the run. Assistant
output, reasoning, usage, rate and request correlation, provider failure,
cancellation, deadline, and cleanup remain distinct. Provider-required
reasoning validation state is discarded at terminal completion. No result or
error authorizes retry.

## Direct Tool Continuation

`prepare_session` requires the exact model, `low`, `high`, or `max` reasoning,
one to eight consumer-owned JSON Schema tools, and the same unmanaged-cache
acceptance. The cache policy grants no cache read, deletion, retention, or
retry authority.

The prepared selection stays fixed on the initial request, every authorized
tool-result continuation, each later user turn, and fresh local restoration.

Call `open_session`, then `start_direct_continuation_turn`. When DeepSeek emits
a tool call, the turn pauses. Swallowtail never selects or executes the tool;
the next provider attempt starts only after the consumer submits exact
correlated `DirectToolResult` values through `DirectToolExchange`. Missing,
cancelled, or rejected results send no further request.

Drain events, exchange, and terminal concurrently, then close turns and the
session. Closing joins work, clears private assistant and
`reasoning_content` envelopes, and releases credentials. Those envelopes are
bounded adapter state, not portable output or durable provider identity.

`prepare_working_state_restoration` opens a fresh session and returns
`SessionReplaced`, retaining only the interrupted consumer turn ID. It cannot
recover transcript, reasoning, tool exchange, cache, or terminal truth.

The route exposes no attachments, structured output, working resource,
provider session, public load/resume, reconciliation, archive/restore/delete,
background execution, reattachment, external search, or billed cost.

## Failures, Promotion, And Validation

Handle failures through portable classification and keep the exact
`swallowtail.deepseek.*` diagnostic for support. Never parse HTTP bodies, SSE
frames, provider prose, cache state, credentials, or endpoint values.

Promotion requires exact DeepSeek route/facade evidence, immutable plan and
access binding, bounded continuation fixtures, lifecycle tests, and
route-matrix coverage. Legacy `deepseek-chat` and `deepseek-reasoner` aliases
remain outside this route.

The compile-tested
[`prepared_direct_continuation` example](../../crates/swallowtail-adapter-deepseek/examples/prepared_direct_continuation.rs)
covers run and session preparation. Validate without provider work:

```sh
effigy validate:focused swallowtail-adapter-deepseek
effigy check:examples
```

Live API calls and cache-affecting inference remain operator-gated.
