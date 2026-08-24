# Qoder Headless Prepared Integration

Use `swallowtail-adapter-qoder` for the installed Qoder stream-json print
run. The route is `qoder.headless`; the driver ID is
`swallowtail.qoder.headless`. It owns one
`qodercli --print --output-format stream-json --permission-mode dont_ask
--max-turns 8 --no-session-persistence --cwd` child over stream-json NDJSON.
Exact `1.1.25` retains `--max-turns 8` as historical inert argv; the selected
CLI headless factory AgentLoop ceiling is `1000`, not `8`.

This is a separate family from Qoder ACP, SDK stdio, the TUI, and the `qoder`
IDE dispatcher. Swallowtail does not pass `--yolo`, `bypass_permissions`, or
`accept_edits`. `--permission-mode dont_ask` is this-invocation-only and
already in driver argv. Bind `qodercli`, not `qoder`.

The package is additive unreleased source after `v0.3.2`. Consumers must pin an
explicitly reviewed commit containing it. No version bump, tag, registry
publication, or harness installation is part of this route.

New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Selected Boundary

Preparation requires all of the following:

- exact package axis `qoder.package`
- exact npm `@qoder-ai/qodercli@1.1.25`
- host-approved `qodercli` executable and isolated environment
- `qoder_local_config_access_profile` with no credential reference
- working resource, plus host services for task, process, time, and
  working-resource ownership
- a host process deadline on the print run

The claim is qualified-only. Later releases do not inherit this route.
`UnverifiedNewer` is not a Qoder headless execution posture.

Swallowtail does not install Qoder, search `PATH`, read provider config, or
select a model. Host-owned local config stays outside the prepared plan. There
is no model catalogue, caller-supplied model route, ACP session, or reusable
provider-session identity on this route.

Call `prepare_qoder_headless` with `QoderHeadlessPreparationInput` and
`QoderHeadlessPreparationProbe`. The probe classifies the approved target
only. It does not send a prompt.

The prepared integration binds the execution host, exact target, observation,
local-config access profile, and preflight evidence. Validate that binding
before reusing it. The access profile is local and unauthenticated: Swallowtail
opens no credential lease. Entitlement metering stays `Unknown`.

Wrong axis (`qoder.acp` is not this route), wrong audience, a credential
reference, a model route, or an unqualified release fails before stream work.

Do not pass `--yolo`, `bypass_permissions`, or `accept_edits`. Must pass
`dont_ask` and `--no-session-persistence`. Route argv also always includes
historical inert `--max-turns 8` (does not set AgentLoop `maxTurns`).
Pretty-printed JSON dump is not this streaming decoder.

## One Bounded Print Run

Create `QoderHeadlessRunProfileInput::new` with request identity, prompt
content, a read-only working resource, and a host deadline. There is no model
route or auto-approve option. Call `prepare_run`, inspect `evidence()`,
`plan()`, and `request()`, then `start_run`.

The driver owns one joined stdio child and performs this sequence:

1. spawn `qodercli --print --output-format stream-json --permission-mode
   dont_ask --max-turns 8 --no-session-persistence --cwd <cwd> <prompt>`
2. close stdin immediately; the prompt is argv, not a stdin document
3. decode stream-json NDJSON (`assistant` text; skip `system` /
   `stream_event`; terminal `result`)
4. join process and task cleanup on terminal or abort

ACP JSON-RPC, SDK stdio, TUI, `--yolo` / `bypass_permissions` /
`accept_edits`, and the `qoder` IDE dispatcher stay unselected. Synthetic
`error_max_turns` / `Maximum turns exceeded` fails closed as provider-failed
(decoder mapping only; not proof that argv `8` stops at turn 8). Host abort
during streaming maps to Cancelled. Unknown stream types and post-result
extras fail closed. The host deadline is required.

Take the run's event stream and terminal outcome immediately and poll them
concurrently. Cancellation kills the child and joins. Close the run separately
from terminal truth. Ambient-host isolation is not filesystem or
descendant-process containment.

See the compile-tested
[`prepared_qoder_headless` example](../../crates/swallowtail-adapter-qoder/examples/prepared_qoder_headless.rs).

## Restart, Failure, And Promotion

Headless is one-prompt. It never auto-retries provider work and exposes no
load, resume, or working-state restoration binding.

Handle failures through portable classification and retain the exact
`swallowtail.qoder.headless` diagnostic for support. Do not parse stderr, raw
NDJSON, or Qoder config files to infer retry, auth, terminal, or cleanup
truth. Unknown stream types fail closed.

Promotion of ACP, SDK stdio, yolo/bypass/accept_edits, continuation, model
selection, usage, or live qualification requires a separate card, exact version
evidence, and matrix coverage. An advertised CLI flag alone is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-qoder
effigy check:examples
```

No login, install, or authenticated prompt is part of deterministic acceptance.
Live evidence stays separately gated and is not claimed by this route.
