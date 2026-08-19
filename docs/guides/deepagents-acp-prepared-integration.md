# Deep Agents ACP Prepared Integration

Use `swallowtail-adapter-deepagents` for the installed LangChain Deep Agents ACP
agent. The route is `deepagents.acp`; the driver ID is
`swallowtail.deepagents.acp`. It owns initialize plus one bounded
`session/prompt` over ACP v1 stdio on a host-approved `deepagents-acp` child
with no extra argv.

This is a separate family from library embed, `npx deepagents-acp`, ACP
registry `deepagents` `0.1.7`, and CLI flags `--workspace`, `--model`,
`--name`, `--skills`, `--memory`, `--debug`, and `--log-file`. Official docs
may still show `session/prompt` params field `content`; that example is not
the selected payload. The first driver sends field `prompt`.

The package is additive unreleased source after `v0.3.2`. Consumers must pin an
explicitly reviewed commit containing it. No version bump, tag, registry
publication, or harness installation is part of this route.

New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Selected Boundary

Preparation requires all of the following:

- exact package axis `deepagents-acp.package`
- exact npm package `0.1.25`
- host-approved `deepagents-acp` executable and isolated environment
- `deepagents_provider_api_key_access_profile` with no credential reference
- working resource, plus host services for task, process, and working-resource
  ownership

The claim is qualified-only. Later releases do not inherit this route.
`UnverifiedNewer` is not a Deep Agents ACP execution posture. Bind npm
`0.1.25`, not registry `0.1.7`. CLI `agentInfo.version` `0.0.1` is the
constructor default, not the package axis. Fail closed if present
`agentInfo.name` is not `deepagents-acp`. Do not require `agentInfo.version`
to equal `0.1.25`.

Swallowtail does not install Deep Agents, search `PATH`, wrap `npx`, or bind
`ANTHROPIC_API_KEY` / `OPENAI_API_KEY` as a credential lease. Host-owned
provider keys stay in the isolated child environment. Sessions live in an
in-process map plus LangGraph `MemorySaver`; they are not durable across
restart.

Call `prepare_deepagents_acp` with `DeepAgentsPreparationInput` and
`DeepAgentsPreparationProbe`. The probe classifies the approved target only.
It does not send initialize, create a session, or prompt.

The prepared integration binds the execution host, exact target, observation,
provider-API-key access profile, and preflight evidence. Validate that binding
before reusing it. The access profile is local and unauthenticated:
Swallowtail opens no credential lease.

Wrong axis (`deepagents.python` is not this route), wrong audience, a
credential reference, or an unqualified release fails before ACP work.

## ACP Interactive Session

Create `DeepAgentsSessionProfileInput::new` with request identity and a
read-only working resource. There is no open deadline, model route, or
harness-mode option. Call `prepare_session`, inspect `evidence()`, `plan()`,
and `request()`, then `open_session`.

The driver owns one joined stdio child and performs this sequence:

1. spawn `deepagents-acp` with no extra argv; child cwd is the working resource
2. `initialize` with protocol version 1 and host `fs` and `terminal` advertised
   false
3. `session/new` with `{cwd, mcpServers: []}`
4. one bounded `session/prompt` of text blocks under field `prompt`
5. observe permission requests and cancel; never select `allow_always` /
   `allow-always`
6. join connection, process, and task cleanup

Host `fs/readTextFile` and `fs/writeTextFile` callbacks are rejected. Local
`FilesystemBackend` writes in the child cwd are not a Swallowtail
bounded-write claim. Session deadline, `session/load`, `session/set_mode`,
slash commands, `--workspace`, `--model`, `npx`, and docs field `content` are
unsupported. Missing host Anthropic or OpenAI API key maps to
`swallowtail.deepagents.acp.host_auth_required`. `authenticate` is a no-op.

Take each turn's event stream and terminal outcome immediately and poll them
concurrently. Cancellation issues `session/cancel` and joins the active turn.
Close the turn and session separately from terminal truth. Ambient-host
isolation is not filesystem or descendant-process containment.

See the compile-tested
[`prepared_deepagents_acp` example](../../crates/swallowtail-adapter-deepagents/examples/prepared_deepagents_acp.rs).

## Restart, Failure, And Promotion

ACP exposes no public load or resume binding. In-process sessions die with
the child. `prepare_working_state_restoration` opens a fresh context-losing
session after process loss; it does not recover the interrupted turn or
transcript.

Handle failures through portable classification and retain the exact
`swallowtail.deepagents.acp` diagnostic for support. Do not parse stderr, ACP
data, or LangGraph checkpoints to infer retry, auth, terminal, or cleanup
truth.

Promotion of library embed, `npx`, `--workspace` / `--model`, `session/load`,
host fs writes, `allow-always`, usage, or live qualification requires a
separate card, exact version evidence, and matrix coverage. An advertised ACP
capability or CLI flag alone is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-deepagents
effigy check:examples
```

No install, `npx`, API key, or authenticated prompt is part of deterministic
acceptance. Live evidence stays separately gated and is not claimed by this
route.
