# Cline ACP Prepared Integration

Use `swallowtail-adapter-cline` for the installed Cline ACP agent. The route is
`cline.acp`; the driver ID is `swallowtail.cline.acp`. It owns initialize plus
one bounded `session/prompt` over ACP v1 stdio on a host-approved `cline --acp`
child.

This is a separate family from `cline.headless` (`cline --json`), the Cline hub
or TUI, and `--id` resume. Swallowtail does not pass `--auto-approve true`.

The package is additive unreleased source after `v0.3.2`. Consumers must pin an
explicitly reviewed commit containing it. No version bump, tag, registry
publication, or harness installation is part of this route.

New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Selected Boundary

Preparation requires all of the following:

- exact package axis `cline.package`
- exact npm wrapper `3.0.55`
- host-approved `cline` executable and isolated environment
- `cline_local_account_access_profile` with no credential reference
- working resource, plus host services for task, process, and working-resource
  ownership

The claim is qualified-only. Later packages do not inherit this route.
`UnverifiedNewer` is not a Cline ACP execution posture.

Swallowtail does not install Cline, search `PATH`, run OAuth `authenticate`,
read `CLINE_API_KEY`, or default auto-approve. Host-owned account state and
persisted settings stay outside the prepared plan. Root `--plan` is discarded
by the ACP early-return and is not this row; portable Plan uses negotiated
`session/set_config_option` only.

Call `prepare_cline_acp` with `ClinePreparationInput` and
`ClinePreparationProbe`. The probe classifies the approved target only. It
does not send initialize, create a session, or prompt.

The prepared integration binds the execution host, exact target, observation,
local-account access profile, and preflight evidence. Validate that binding
before reusing it. The access profile is local and unauthenticated: Swallowtail
opens no credential lease.

Wrong axis (`cline.headless` is not this route), wrong audience, a credential
reference, or an unqualified package fails before ACP work.

## ACP Interactive Session

Create `ClineSessionProfileInput::new` with request identity and a read-only
working resource. Optional `with_harness_mode(HarnessMode::Plan)` selects Plan
on exact `3.0.55`. Omission sends no mode request and claims neither selected
Plan nor provider-default Act. There is no open deadline or model route. Call
`prepare_session`, inspect `evidence()`, `plan()`, and `request()`, then
`open_session`.

The driver owns one joined stdio child and performs this sequence:

1. spawn `cline --acp` with no extra argv
2. `initialize` with host `fs` and `terminal` advertised false
3. `session/new` with `{cwd, mcpServers: []}`
4. when Plan is selected: require unique `plan` advertisement, send one
   `session/set_config_option` `{configId: mode, value: plan}`, and require
   response `mode.currentValue = plan` before readiness
5. one bounded `session/prompt` of text blocks
6. observe permission requests and cancel; never select `allow_always`
7. join connection, process, and task cleanup

Host `fs/readTextFile` and `fs/writeTextFile` callbacks are rejected. Session
deadline and `session/load` are unsupported. Plan is provider behavior only: it
does not widen permission, auto-approve, resource, isolation, tool, filesystem,
network, shell, process, model, or account authority. Mode updates stay
metadata.

Take each turn's event stream and terminal outcome immediately and poll them
concurrently. Cancellation issues `session/cancel` and joins the active turn.
Close the turn and session separately from terminal truth. Ambient-host
isolation is not filesystem or descendant-process containment.
Fresh working-state replacement opens a new provider session and renegotiates
the same immutable Plan selection; it remains context-losing `SessionReplaced`,
not load or resume.

See the compile-tested
[`prepared_cline_acp` example](../../crates/swallowtail-adapter-cline/examples/prepared_cline_acp.rs).

## Restart, Failure, And Promotion

ACP exposes no public load or resume binding.
`prepare_working_state_restoration` opens a fresh context-losing session after
process loss; it does not recover the interrupted turn or transcript.

Handle failures through portable classification and retain the exact
`swallowtail.cline.acp` diagnostic for support. Do not parse stderr, ACP data,
or Cline settings files to infer retry, auth, terminal, or cleanup truth.

Promotion of headless `--json`, auto-approve, host fs writes, model selection,
usage, session load, or live qualification requires a separate card, exact
version evidence, and matrix coverage. An advertised ACP capability or CLI flag
alone is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-cline
effigy check:examples
```

No login, install, or authenticated prompt is part of deterministic acceptance.
Live evidence stays separately gated and is not claimed by this route.
