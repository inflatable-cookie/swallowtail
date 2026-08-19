# Kiro ACP Prepared Integration

Use `swallowtail-adapter-kiro` for the installed Kiro CLI ACP agent. The route
is `kiro.acp`; the driver ID is `swallowtail.kiro.acp`. It owns initialize plus
one bounded `session/prompt` over ACP v1 stdio on a host-approved `kiro-cli acp`
child.

This is a separate family from `kiro-cli chat --no-interactive`, `--cloud`,
`--agent`, TUI `kiro-cli-chat`, and npm/PyPI names that are not the official
CLI. Swallowtail does not pass `--trust-all-tools`. Official docs still show
`session/prompt` params field `content`; that example is not the selected
payload. The first driver sends field `prompt`.

The package is additive unreleased source after `v0.3.2`. Consumers must pin an
explicitly reviewed commit containing it. No version bump, tag, registry
publication, or harness installation is part of this route.

New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Selected Boundary

Preparation requires all of the following:

- exact release axis `kiro-cli.release`
- exact installer-manifest release `2.18.1`
- host-approved `kiro-cli` executable and isolated environment
- `kiro_local_account_access_profile` with no credential reference
- working resource, plus host services for task, process, and working-resource
  ownership

The claim is qualified-only. Later releases do not inherit this route.
`UnverifiedNewer` is not a Kiro ACP execution posture.

Swallowtail does not install Kiro, search `PATH`, run `kiro-cli login`, bind
`KIRO_API_KEY` as a credential lease, or default `--trust-all-tools`. Host-owned
`~/.kiro/` session files stay outside the prepared plan.

Call `prepare_kiro_acp` with `KiroPreparationInput` and `KiroPreparationProbe`.
The probe classifies the approved target only. It does not send initialize,
create a session, or prompt.

The prepared integration binds the execution host, exact target, observation,
local-account access profile, and preflight evidence. Validate that binding
before reusing it. The access profile is local and unauthenticated: Swallowtail
opens no credential lease.

Wrong axis (`kiro.headless` is not this route), wrong audience, a credential
reference, or an unqualified release fails before ACP work.

## ACP Interactive Session

Create `KiroSessionProfileInput::new` with request identity and a read-only
working resource. There is no open deadline, model route, or harness-mode
option. Call `prepare_session`, inspect `evidence()`, `plan()`, and `request()`,
then `open_session`.

The driver owns one joined stdio child and performs this sequence:

1. spawn `kiro-cli acp` with no extra argv
2. `initialize` with host `fs` and `terminal` advertised false
3. `session/new` with `{cwd, mcpServers: []}`
4. one bounded `session/prompt` of text blocks under field `prompt`
5. observe permission requests and cancel; never select `allow_always`
6. join connection, process, and task cleanup

Initialize result fields are unrecovered from public Kiro source. A present
`agentInfo.name` must be `kiro-cli`; a present version must match `2.18.1`.
Missing fields are not invented.

Host `fs/readTextFile` and `fs/writeTextFile` callbacks are rejected. Session
deadline, `session/load`, `session/set_mode`, `session/set_model`,
`_kiro.dev/*`, and docs field `content` are unsupported. Missing host login or
API key maps to `swallowtail.kiro.acp.host_auth_required`.

Take each turn's event stream and terminal outcome immediately and poll them
concurrently. Cancellation issues `session/cancel` and joins the active turn.
Close the turn and session separately from terminal truth. Ambient-host
isolation is not filesystem or descendant-process containment.

See the compile-tested
[`prepared_kiro_acp` example](../../crates/swallowtail-adapter-kiro/examples/prepared_kiro_acp.rs).

## Restart, Failure, And Promotion

ACP exposes no public load or resume binding.
`prepare_working_state_restoration` opens a fresh context-losing session after
process loss; it does not recover the interrupted turn or transcript.

Handle failures through portable classification and retain the exact
`swallowtail.kiro.acp` diagnostic for support. Do not parse stderr, ACP data,
or `~/.kiro/` files to infer retry, auth, terminal, or cleanup truth.

Promotion of `kiro.headless`, `--cloud`, `--agent`, `--trust-all-tools`,
`session/load`, host fs writes, model selection, usage, or live qualification
requires a separate card, exact version evidence, and matrix coverage. An
advertised ACP capability or CLI flag alone is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-kiro
effigy check:examples
```

No login, install, or authenticated prompt is part of deterministic
acceptance. Live evidence stays separately gated and is not claimed by this
route.
