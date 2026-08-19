# Goose ACP Prepared Integration

Use `swallowtail-adapter-goose` for the installed Goose ACP agent. The route is
`goose.acp`; the driver ID is `swallowtail.goose.acp`. It owns initialize plus
one bounded `session/prompt` over ACP v1 stdio on a host-approved `goose acp`
child.

This is a separate family from `goose serve` (HTTP/WebSocket ACP), desktop, TUI,
recipes as routing, and Goose ACP-providers. Swallowtail does not pass
`--with-builtin` or `--enable-scheduler`.

The package is additive unreleased source after `v0.3.2`. Consumers must pin an
explicitly reviewed commit containing it. No version bump, tag, registry
publication, or harness installation is part of this route.

New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Selected Boundary

Preparation requires all of the following:

- exact release axis `goose.release`
- exact GitHub release `1.46.0`
- host-approved `goose` executable and isolated environment
- `goose_local_config_access_profile` with no credential reference
- working resource, plus host services for task, process, and working-resource
  ownership

The claim is qualified-only. Later releases do not inherit this route.
`UnverifiedNewer` is not a Goose ACP execution posture.

Swallowtail does not install Goose, search `PATH`, run `goose configure`, bind
`GOOSE_PROVIDER` / `GOOSE_MODEL` as Swallowtail selection, or default
`GooseMode` `auto`. Host-owned `~/.config/goose/` stays outside the prepared
plan.

Call `prepare_goose_acp` with `GoosePreparationInput` and
`GoosePreparationProbe`. The probe classifies the approved target only. It
does not send initialize, create a session, or prompt.

The prepared integration binds the execution host, exact target, observation,
local-config access profile, and preflight evidence. Validate that binding
before reusing it. The access profile is local and unauthenticated: Swallowtail
opens no credential lease.

Wrong axis (`goose.serve` is not this route), wrong audience, a credential
reference, or an unqualified release fails before ACP work.

## ACP Interactive Session

Create `GooseSessionProfileInput::new` with request identity and a read-only
working resource. There is no open deadline, model route, or harness-mode
option. Call `prepare_session`, inspect `evidence()`, `plan()`, and `request()`,
then `open_session`.

The driver owns one joined stdio child and performs this sequence:

1. spawn `goose acp` with no extra argv
2. `initialize` with host `fs` and `terminal` advertised false
3. `session/new` with `{cwd, mcpServers: []}`
4. one bounded `session/prompt` of text blocks
5. observe permission requests and cancel; never select `allow_always`
6. join connection, process, and task cleanup

Host `fs/readTextFile` and `fs/writeTextFile` callbacks are rejected. Session
deadline, `session/load`, `session/list`, and `session/close` are unsupported.
Missing host provider or model maps to
`swallowtail.goose.acp.host_provider_unconfigured`.

Take each turn's event stream and terminal outcome immediately and poll them
concurrently. Cancellation issues `session/cancel` and joins the active turn.
Close the turn and session separately from terminal truth. Ambient-host
isolation is not filesystem or descendant-process containment.

See the compile-tested
[`prepared_goose_acp` example](../../crates/swallowtail-adapter-goose/examples/prepared_goose_acp.rs).

## Restart, Failure, And Promotion

ACP exposes no public load or resume binding.
`prepare_working_state_restoration` opens a fresh context-losing session after
process loss; it does not recover the interrupted turn or transcript.

Handle failures through portable classification and retain the exact
`swallowtail.goose.acp` diagnostic for support. Do not parse stderr, ACP data,
or Goose config files to infer retry, auth, terminal, or cleanup truth.

Promotion of `goose serve`, `--with-builtin`, `GooseMode` `auto`, host fs
writes, model selection, usage, session load, or live qualification requires a
separate card, exact version evidence, and matrix coverage. An advertised ACP
capability or CLI flag alone is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-goose
effigy check:examples
```

No login, install, configure, or authenticated prompt is part of deterministic
acceptance. Live evidence stays separately gated and is not claimed by this
route.
