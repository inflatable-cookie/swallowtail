# Qwen Headless Prepared Integration

Use the prepared facade for one bounded Qwen Code headless run. It probes one
host-approved executable and derives the configured instance, exact version
binding, preflight plan, fixed native budget invocation, and structured-run
request.

## Explicit Inputs

Preparation requires:

- configured-instance identity and revision
- execution host and approved executable target
- selected process environment
- maintainer-supported delegated harness access profile and access evidence
- probe deadline and cancellation

Run preparation requires a request identity, provider, model route, model,
prompt content, working-resource reference, and host deadline.

Catalogue preparation requires the same approved executable, environment,
access, and exact version evidence, plus a request identity and optional
deadline. Call `prepare_qwen_catalogue`; the result is independent from
`prepare_qwen_headless` and has no provider, model route, prompt, or working
resource.

Swallowtail does not choose the provider, model, prompt, account, workspace, or
fallback route.

## Version Posture

Qwen Code 0.19.11 is the qualified headless baseline. Discovery records the
exact installed version from `qwen --version`. A later stable release is
admitted as unverified, remains visible in evidence, and uses the latest
qualified behavior mapping. Older or prerelease versions do not prepare.

## Execution Boundary

The prepared plan and request bind:

- text prompt input over stdin
- line-delimited `stream-json` output with partial messages
- explicit provider and model route
- read-only core tools and the frozen excluded-tool set
- 60-second native wall time, 16 tool calls, and 24 session turns
- host deadline and structured-run cancellation
- ambient harness configuration and `AmbientHost` isolation
- provider retention allowed, with recovery and stream reattachment prohibited

Qwen's `--safe-mode` and tool registry are provider behavior. They do not prove
host sandboxing, filesystem containment, descendant-process isolation, or
transcript deletion. The prepared path does not enable Qwen's separate
`--sandbox` route.

`QwenPreparedCatalogue::list_models` starts one ephemeral safe-mode
stream-JSON process, initializes the control protocol, verifies
`can_get_available_models`, calls `get_available_models`, projects bounded
model identity, label, and context-window evidence, then closes and joins the
child. It does not open a model session or claim OS sandboxing.

`QwenPreparedRun::start_run` executes exactly one run. `plan`, `request`,
`evidence`, `low_level_driver`, and `into_parts` remain available for
inspection and advanced use.

See the compile-tested
[`prepared_qwen_headless` example](../../crates/swallowtail-adapter-qwen/examples/prepared_qwen_headless.rs).
