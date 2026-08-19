# GitHub Copilot CLI ACP Prepared Integration

Use `swallowtail-adapter-copilot-cli` for the installed GitHub Copilot CLI ACP
agent. The route is `copilot-cli.acp`; the driver ID is
`swallowtail.copilot-cli.acp`. It owns initialize plus one bounded
`session/prompt` over ACP v1 stdio on a host-approved `copilot --acp --stdio`
child.

This is a separate family from Copilot CLI TCP `--port`, the Copilot IDE, the
GitHub Copilot API, and interactive-only slash commands. Public preview stays
visible. Swallowtail does not pass `--yolo`, `--allow-all`, or server-start
tool/effort flags.

The package is additive unreleased source after `v0.3.2`. Consumers must pin an
explicitly reviewed commit containing it. No version bump, tag, registry
publication, or harness installation is part of this route.

New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Selected Boundary

Preparation requires all of the following:

- exact package axis `copilot-cli.package`
- exact npm wrapper `1.0.80`
- host-approved `copilot` executable and isolated environment
- `copilot_cli_host_account_access_profile` with no credential reference
- working resource, plus host services for task, process, and working-resource
  ownership

The claim is qualified-only. Later packages do not inherit this route.
`UnverifiedNewer` is not a Copilot CLI ACP execution posture. Public preview is
visible as `ExperimentalObserved` and `COPILOT_CLI_ACP_MATURITY`.

Swallowtail does not install Copilot CLI, search `PATH`, run GitHub login, bind
`GH_TOKEN` / `GITHUB_TOKEN` as a Swallowtail lease, or default `--yolo`.
Host-owned GitHub Copilot login or BYOK stays outside the prepared plan.

Call `prepare_copilot_cli_acp` with `CopilotCliPreparationInput` and
`CopilotCliPreparationProbe`. The probe classifies the approved target only. It
does not send initialize, create a session, or prompt.

The prepared integration binds the execution host, exact target, observation,
host-account access profile, and preflight evidence. Validate that binding
before reusing it. The access profile is local and unauthenticated: Swallowtail
opens no credential lease.

Wrong axis (`copilot-cli.tcp-port` is not this route), wrong audience, a
credential reference, or an unqualified package fails before ACP work.

## ACP Interactive Session

Create `CopilotCliSessionProfileInput::new` with request identity and a
read-only working resource. There is no open deadline, model route, or
harness-mode option. Call `prepare_session`, inspect `evidence()`, `plan()`,
and `request()`, then `open_session`.

The driver owns one joined stdio child and performs this sequence:

1. spawn `copilot --acp --stdio` with no extra argv
2. `initialize` with host `fs` and `terminal` advertised false
3. `session/new` with `{cwd, mcpServers: []}`
4. one bounded `session/prompt` of text blocks
5. observe permission requests and cancel; never select `allow_always`
6. join connection, process, and task cleanup

Host `fs/readTextFile` and `fs/writeTextFile` callbacks are rejected. Session
deadline, `session/load`, and `session/close` are unsupported. Unexpected
initialize `agentInfo.version` fails closed against selected `1.0.80`.

Take each turn's event stream and terminal outcome immediately and poll them
concurrently. Cancellation issues `session/cancel` and joins the active turn.
Close the turn and session separately from terminal truth. Ambient-host
isolation is not filesystem or descendant-process containment.

See the compile-tested
[`prepared_copilot_cli_acp` example](../../crates/swallowtail-adapter-copilot-cli/examples/prepared_copilot_cli_acp.rs).

## Restart, Failure, And Promotion

ACP exposes no public load or resume binding.
`prepare_working_state_restoration` opens a fresh context-losing session after
process loss; it does not recover the interrupted turn or transcript.

Handle failures through portable classification and retain the exact
`swallowtail.copilot-cli.acp` diagnostic for support. Do not parse stderr, ACP
data, or GitHub account state to infer retry, auth, terminal, or cleanup truth.
Unauthorized initialize maps to `swallowtail.copilot-cli.acp.host_auth_required`
without GitHub policy.

Promotion of TCP `--port`, `--yolo` / `allow_all`, server-start tool or effort
flags, GitHub login as Swallowtail action, treating preview as stable, Copilot
IDE/API coverage, host fs writes, model selection, usage, session load, or live
qualification requires a separate card, exact version evidence, and matrix
coverage. An advertised ACP capability or CLI flag alone is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-copilot-cli
effigy check:examples
```

No login, install, or authenticated prompt is part of deterministic acceptance.
Live evidence stays separately gated and is not claimed by this route.
