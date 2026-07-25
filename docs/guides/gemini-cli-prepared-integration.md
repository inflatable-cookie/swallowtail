# Gemini CLI Prepared Integration

Use the prepared facade for Gemini CLI ACP over stdio. It discovers one
host-approved executable and derives the configured instance, exact version
binding, preflight plan, ambient read-only access agreement, and open-session
request.

## Explicit Inputs

Preparation requires:

- configured-instance identity and revision
- execution host and approved executable target
- selected process environment
- provider-supported Gemini Developer API-key profile and access evidence
- probe deadline and cancellation

Session preparation requires a request identity, working-resource reference,
and empty `SessionOptions`.

Gemini CLI's current ACP surface reports its current model as provider
observation. The prepared facade therefore does not invent a selectable model
route. Swallowtail also does not choose an account, credential, workspace,
endpoint, or fallback route.

## Version Posture

Gemini CLI ACP 0.51.0 is the current qualified baseline. Newer stable releases
are admitted as unverified rather than hard-denied. The exact discovered
version stays visible in evidence and must match ACP initialization. Older
versions do not prepare.

## Execution Boundary

The prepared plan binds:

- `acp-v1-stdio`
- ambient configuration inside the selected environment
- `AmbientHost` isolation
- ambient read-only workspace access
- provider-owned durable state prohibited
- no consumer-selected model route

Ambient execution is not sandbox containment. The facade does not silently
select remote ACP, HTTP, or another transport. Remote ACP composition is a
separate explicit route.

`GeminiPreparedSession::open_session` executes the bound operation. `plan`,
`request`, `evidence`, `low_level_driver`, and `into_parts` remain available
for inspection and advanced use.

See the compile-tested
[`prepared_gemini_acp` example](../../crates/swallowtail-adapter-gemini/examples/prepared_gemini_acp.rs).
