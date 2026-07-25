# DeepSeek Prepared Integration

Use this facade for the exact DeepSeek Open Platform V4 Pro continuation route.
It binds `https://api.deepseek.com`, the `api.deepseek.com` audience, the dated
OpenAI-format facade, one public API-key profile, and the exact
`deepseek-v4-pro` route.

## Inputs That Stay Explicit

`prepare_deepseek_direct` requires:

- one configured-instance identity and revision
- one execution host
- the exact public endpoint target
- one provider-supported, pay-as-you-go Open Platform API-key profile
- observed or caller-asserted access evidence

No DeepSeek app credential, OAuth route, proxy, gateway, Anthropic facade,
`/v1` path, beta endpoint, model alias, or third-party compatible endpoint can
substitute.

Preparation performs no network or credential work. The result exposes its
safe configured instance, opaque facade compatibility assessment, access
provenance, service set, and low-level driver.

## Catalogue

`prepare_catalogue` derives a catalogue plan without selecting a model.
Catalogue presence remains source-scoped evidence. It does not prove balance,
entitlement, capacity, cache state, or invocation success.

## Direct Tool Continuation

`prepare_session` requires:

- exact route identity, revision, and `deepseek-v4-pro` model identity
- explicit `high` reasoning
- one to eight consumer-owned JSON Schema tools
- explicit
  `ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority`

That cache value records consumer acceptance of DeepSeek's provider-managed
best-effort context cache. It grants Swallowtail no cache read, deletion,
retention, or retry authority.

`open_session` acquires the endpoint and credential leases and returns the
normal `InteractiveSessionHandle`. The consumer starts each user turn through
`start_direct_continuation_turn`.

When DeepSeek returns a tool call, the turn pauses. Swallowtail neither selects
nor executes the tool. The next provider attempt starts only when the consumer
submits correlated `DirectToolResult` values through the returned
`DirectToolExchange`. Omitting, cancelling, or rejecting that submission makes
no further request.

The adapter privately retains the provider-required assistant envelope and
`reasoning_content`. It never exposes that reasoning as portable output and
cannot replay it across session, facade, model, route, access, or host
boundaries. Closing the session joins active work, clears private continuation,
then releases credentials.

The legacy `deepseek-chat` and `deepseek-reasoner` aliases are outside this
route. Compatible JSON syntax does not authorize model mapping or provider
fallback.

See the compile-tested
[`prepared_direct_continuation` example](../../crates/swallowtail-adapter-deepseek/examples/prepared_direct_continuation.rs).
