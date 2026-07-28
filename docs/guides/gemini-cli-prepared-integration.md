# Gemini CLI Prepared Integration

Use the prepared Gemini CLI facade with one explicit route:

- `Acp` for an interactive ACP v1 session
- `Headless` for one bounded `stream-json` structured run

Both discover one host-approved executable. They keep separate drivers,
version axes, transports, operation shapes, plans, and prepared operation
types.

## Explicit Inputs

Preparation requires:

- configured-instance identity and revision
- execution host and approved executable target
- selected process environment
- provider-supported Gemini Developer API-key profile and access evidence
- probe deadline and cancellation

ACP session preparation requires a request identity, working-resource
reference, and empty `SessionOptions`.

Headless run preparation requires an explicit provider and model route,
private input content, working-resource reference, and deadline. It derives
durable transcript retention and ambient harness authority from the qualified
route. It rejects attachments, consumer tools, structured output, output-token
limits, external search, and reasoning selection before starting the process.

Gemini CLI's current ACP surface reports its current model as provider
observation. The prepared facade therefore does not invent a selectable model
route. Swallowtail also does not choose an account, credential, workspace,
endpoint, or fallback route.

## Version Posture

Gemini CLI ACP is qualified at `0.51.0`. Headless `stream-json` is qualified
across `0.51.0..=0.52.0`. Newer stable releases are admitted as visible
unverified-newer observations rather than hard-denied. The exact discovered
version stays visible in evidence. Older versions do not prepare.

## Execution Boundary

The ACP plan binds:

- `acp-v1-stdio`
- ambient configuration inside the selected environment
- `AmbientHost` isolation
- ambient read-only workspace access
- provider-owned durable state prohibited
- no consumer-selected model route

Ambient execution is not sandbox containment. The facade does not silently
select remote ACP, HTTP, or another transport. Remote ACP composition is a
separate explicit route.

The headless plan binds:

- `gemini-stream-json-stdio`
- prompt bytes over stdin, never argv
- exact caller-selected model and derived provider session identity
- `plan` approval mode
- extensions and MCP servers disabled
- trust already granted for the explicit working resource
- attached execution with a host deadline and joined process cleanup
- durable local Gemini transcript retention

The route does not force `--sandbox`. Sandbox configuration remains an
explicit ambient harness choice. Closing a run joins Swallowtail-owned work;
it does not delete the Gemini transcript.

`GeminiPreparedSession::open_session` executes the bound operation. `plan`,
`request`, `evidence`, `low_level_driver`, and `into_parts` remain available
for inspection and advanced use.

After authorization and session creation, the returned handle may expose
bounded `negotiated_model_options()`. These are the current and available
models advertised by that exact ACP session. They are not a pre-session
catalogue, cannot be caller-synthesized, and do not authorize another route.

See the compile-tested
[`prepared_gemini_acp` example](../../crates/swallowtail-adapter-gemini/examples/prepared_gemini_acp.rs)
and
[`prepared_gemini_headless` example](../../crates/swallowtail-adapter-gemini/examples/prepared_gemini_headless.rs).
