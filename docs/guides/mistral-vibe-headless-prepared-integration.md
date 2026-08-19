# Mistral Vibe Headless Prepared Integration

Use `swallowtail-adapter-mistral-vibe` for the installed Vibe streaming print
run. The route is `mistral-vibe.headless`; the driver ID is
`swallowtail.mistral-vibe.headless`. It owns one bounded
`vibe --prompt --output streaming --max-turns 8 --trust --agent plan --workdir`
child over streaming NDJSON.

This is a separate family from `vibe-acp`, the Vibe TUI, `--continue` /
`--resume`, and teleport. Swallowtail does not pass `--auto-approve` or
`--yolo`. `--trust` is this-invocation-only and already in driver argv.

The package is additive unreleased source after `v0.3.2`. Consumers must pin an
explicitly reviewed commit containing it. No version bump, tag, registry
publication, or harness installation is part of this route.

New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

## Selected Boundary

Preparation requires all of the following:

- exact release axis `mistral-vibe.release`
- exact GitHub/PyPI `2.24.2`
- host-approved `vibe` executable and isolated environment
- `mistral_vibe_local_config_access_profile` with no credential reference
- working resource, plus host services for task, process, time, and
  working-resource ownership
- a host process deadline on the print run

The claim is qualified-only. Later releases do not inherit this route.
`UnverifiedNewer` is not a Vibe headless execution posture.

Swallowtail does not install Vibe, search `PATH`, read provider config, or
select a model. Host-owned local config stays outside the prepared plan. There
is no model catalogue, caller-supplied model route, ACP session, or
`--continue` / `--resume` on this route.

Call `prepare_mistral_vibe_headless` with
`MistralVibeHeadlessPreparationInput` and
`MistralVibeHeadlessPreparationProbe`. The probe classifies the approved
target only. It does not send a prompt.

The prepared integration binds the execution host, exact target, observation,
local-config access profile, and preflight evidence. Validate that binding
before reusing it. The access profile is local and unauthenticated: Swallowtail
opens no credential lease. Entitlement metering stays `Unknown`.

Wrong axis (`mistral-vibe.acp` is not this route), wrong audience, a credential
reference, a model route, or an unqualified release fails before stream work.

Official docs that programmatic mode defaults to auto-approve are stale.
Swallowtail must pass `--agent plan`. Do not pass `--auto-approve` or `--yolo`.
`--output json` is the dump-at-end sibling, not this streaming decoder.

## One Bounded Print Run

Create `MistralVibeHeadlessRunProfileInput::new` with request identity, prompt
content, a read-only working resource, and a host deadline. There is no model
route or auto-approve option. Call `prepare_run`, inspect `evidence()`,
`plan()`, and `request()`, then `start_run`.

The driver owns one joined stdio child and performs this sequence:

1. spawn `vibe --prompt <text> --output streaming --max-turns 8 --trust
   --agent plan --workdir <cwd>`
2. close stdin immediately; the prompt is argv, not a stdin document
3. decode streaming NDJSON (`message`, `reasoning`, `effect`; skip
   `callback` / `checkpoint` / `notice` and `generationStatus == in_progress`)
4. join process and task cleanup on terminal or abort

ACP JSON-RPC, dump-at-end JSON arrays, TUI, `--continue` / `--resume`,
`--teleport`, `--auto-approve`, and `--yolo` stay unselected. CLI conversation
limit appears on stderr and fails closed. The host deadline is required.

Take the run's event stream and terminal outcome immediately and poll them
concurrently. Cancellation kills the child and joins. Close the run separately
from terminal truth. Ambient-host isolation is not filesystem or
descendant-process containment.

See the compile-tested
[`prepared_mistral_vibe_headless` example](../../crates/swallowtail-adapter-mistral-vibe/examples/prepared_mistral_vibe_headless.rs).

## Restart, Failure, And Promotion

Headless is one-prompt. It never auto-retries provider work and exposes no
load, resume, or working-state restoration binding.

Handle failures through portable classification and retain the exact
`swallowtail.mistral-vibe.headless` diagnostic for support. Do not parse
stderr, raw NDJSON, or Vibe config files to infer retry, auth, terminal, or
cleanup truth. Unknown stream types fail closed.

Promotion of ACP `vibe-acp`, auto-approve, continuation, teleport, model
selection, usage, or live qualification requires a separate card, exact version
evidence, and matrix coverage. An advertised CLI flag alone is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-mistral-vibe
effigy check:examples
```

No login, install, or authenticated prompt is part of deterministic acceptance.
Live evidence stays separately gated and is not claimed by this route.
