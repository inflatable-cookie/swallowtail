# ZCode App-Server Prepared Integration

Use `swallowtail-adapter-zcode` for the installed ZCode app-server runtime.
The route is `zcode.app-server`; the driver ID is
`swallowtail.zcode.app-server`. It owns one bounded structured run over
line-delimited JSON on a host-approved `node` + `zcode.cjs` child.

This is a separate family from OpenCode HTTP and from hosted GLM / Z.AI
HTTP. It does not wrap the TUI, desktop GUI, `--print` / `--prompt`, or
community ACP.

The package is additive unreleased source after `v0.3.2`. Consumers must pin an
explicitly reviewed commit containing it. No version bump, tag, registry
publication, or harness installation is part of this route.

## Selected Boundary

Preparation requires all of the following:

- exact runtime axis `zcode.runtime`
- exact `0.16.3` packaged payload basename `zcode.cjs`
- payload digest
  `3e3433d90fa502e5d02498dfde6c2090df898331359bcfe5f3dbc9a1d00b685f`
- host-approved Node interpreter (`node` or `node.exe`)
- host-approved `EnvironmentRef` for the settings file
- explicit provider, model, host-supplied `plan` or `build` mode, working
  resource, and deadline
- host services for task, process, and time ownership

Desktop About `3.7.7` and npm `zcode-app-cli@3.7.7-13` are packaging
provenance, not the compatibility axis. The launcher digest is recorded and
not an admission check. Preparation stream-hashes the host-approved payload
before version qualification, and execution re-checks the same digest. The
runtime claim is qualified-only for `0.16.3`; newer points do not inherit this
route.

Call `prepare_zcode_app_server` with `ZcodePreparationInput` and
`ZcodePreparationProbe`. The probe stream-hashes the host-approved payload
and classifies exact `0.16.3`. Discovery does not spawn `app-server`, send a
prompt, acquire credentials, choose a model, or inspect the settings file.

The prepared integration binds the execution host, Node interpreter, exact
target, observation, host configuration reference, access profile, and
preflight evidence. Validate that binding before reusing it. The access
profile is local and unauthenticated: Swallowtail receives no credential
reference and opens no credential lease.

## Structured Run

Create `ZcodeModelSelection` with an application-owned route ID and revision
plus explicit provider and model IDs. Create `ZcodeRunProfileInput` with a
request ID, host-supplied `ZcodeAppServerMode` (`plan` or `build`), prompt,
working `WorkingResourceRef`, and deadline. Swallowtail does not default
`yolo`. Call `prepare_run`, inspect `ZcodePreparedRun::evidence`, `plan`, and
`request`, then call `start_run` with host services for the same execution
host.

The driver owns one joined interpreted process and performs this sequence:

1. spawn `node zcode.cjs app-server`
2. send `session/create` and answer `session/requestRuntimePreferences`
   for `runtime-materialization` and later `user-execution` before treating
   create as complete. The same fail-closed defaults are reused.
3. subscribe, send the prompt, and treat send `{accepted:true}` as enqueue
4. fold flat `params.{sessionId,seq,type,payload}` events until
   `turn.completed` or `turn.failed`
5. force-stop and join process and task cleanup

Create fails closed if runtime-preferences is not answered. A `jsonrpc` field
is reject. Unknown events require a `zcode/` namespace. Kill-after-complete
does not rewrite Completed or ProviderFailed into process failure.

The stream is bounded. Live `0.16.3` create uses protocol name
`ZCode Protocol` version 1, host mode on `settings.mode.current` (and
`settings.permission.mode` when present), and session model as
`{modelId, providerId}`. Reconstructed fixtures may still use
`zcode-app-server` and a string model. Runtime `cliVersion` is checked when
present; payload digest remains the pin.

Unknown `zcode/` events remain namespaced observations. Unscoped unknown
session events such as `session.titleUpdated`, and non-session notifications
such as `state.updated` and `v4/telemetry/event`, are content-free progress.
Raw envelopes, request headers, and telemetry bodies are not projected.
Assistant text, content-free reasoning progress, harness-owned tool
lifecycle, usage, and terminal completion or failure remain separate
projections. Tool argument and result bodies, reasoning text, prompts,
credentials, private paths, session ids, and raw JSONL are not projected.

Cancellation force-stops the owned process. There is no advertised native
`session/stop`, interactive session continuity, provider-session management,
model catalogue, subagent control, consumer-tool callback, permission
exchange, or typed-question exchange.

## Host Configuration And Live Proof

The host owns the settings file. The packaged payload rejects `--settings`;
host-approved settings must already exist at `$HOME/.zcode/cli/config.json`
for the owned process. The live probe copies the host-approved file into an
isolated HOME. Swallowtail does not mint that file or default `yolo`.

A live smoke may use a custom provider under map key `zai` with an inline
`options.apiKey` and a host-local OpenAI-compatible endpoint. That proves the
installed app-server runtime and agent loop only. It does not qualify Z.AI
official.

The ignored live tests require explicit operator inputs:

```sh
export SWALLOWTAIL_ZCODE_EXECUTABLE=/absolute/path/zcode.cjs
export SWALLOWTAIL_ZCODE_NODE=/absolute/path/to/node
export SWALLOWTAIL_ZCODE_SETTINGS=/absolute/path/to/settings.json
export SWALLOWTAIL_ZCODE_CWD=/absolute/path/to/workspace
export SWALLOWTAIL_ZCODE_MODE=plan
export SWALLOWTAIL_ZCODE_PROVIDER=zai
export SWALLOWTAIL_ZCODE_MODEL=operator-selected-model
```

`SWALLOWTAIL_ZCODE_NODE` may be omitted when `node` is on `PATH`. The local
host clears ambient environment and only forwards approved bindings;
Swallowtail does not lease a provider key. Inline settings credentials stay
host-owned.

The installed probe and configured prompt smoke are separate Effigy
selectors. Neither selector logs the settings path, provider credentials,
prompt, tool bodies, session ids, or reasoning text.

## Failures And Recovery

Wrong target, wrong version, missing host configuration, rejected
interpreter, unavailable discovery, malformed or oversized frames, missing
runtime-preferences, provider errors, deadline, cancellation, process
failure, and cleanup failure remain distinct safe failure observations. A
missing credential (`MISSING_CREDENTIAL`) is a provider/configuration
failure, not evidence that Z.AI official is qualified.

There is no persistent session or provider recovery claim. A failed,
cancelled, timed-out, or mismatched run does not leave a reusable
Swallowtail continuation binding. The operation-owned process is joined or
force-stopped and its host configuration remains host-owned.

## Validation

Deterministic validation:

```sh
effigy validate:focused swallowtail-adapter-zcode
effigy package:verify-affected swallowtail-adapter-zcode
effigy qa:guides
effigy qa:routes
```

Optional operator-gated probes:

```sh
SWALLOWTAIL_LIVE_ZCODE=1 \
  effigy probe:zcode-installed

SWALLOWTAIL_LIVE_ZCODE_PROMPT=1 \
  effigy probe:zcode-live
```

The normal public shape is shown in
[`prepared_zcode_app_server`](../../crates/swallowtail-adapter-zcode/examples/prepared_zcode_app_server.rs).
