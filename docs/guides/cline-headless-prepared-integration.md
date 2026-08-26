# Cline Headless Prepared Integration

Use `swallowtail-adapter-cline` for the installed Cline JSON print run. The
route is `cline.headless`; the driver ID is `swallowtail.cline.headless`. It
owns one bounded `cline --json --auto-approve false` child plus one prompt
argv operand over envelope NDJSON.

This is a separate family from `cline.acp` (`cline --acp`), the Cline hub or
TUI, and `--id` resume. Swallowtail does not pass `--auto-approve true`.

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
- working resource, plus host services for task, process, time, and
  working-resource ownership
- a host process deadline on the print run

The claim is qualified-only. Later packages do not inherit this route.
`UnverifiedNewer` is not a Cline headless execution posture.

Swallowtail does not install Cline, search `PATH`, run OAuth `authenticate`,
read `CLINE_API_KEY`, or default auto-approve. Host-owned account state and
persisted settings stay outside the prepared plan. There is no model catalogue,
caller-supplied model route, ACP session, or `--id` resume on this route.

Call `prepare_cline_headless` with `ClineHeadlessPreparationInput` and
`ClineHeadlessPreparationProbe`. The probe classifies the approved target
only. It does not send a prompt.

The prepared integration binds the execution host, exact target, observation,
local-account access profile, and preflight evidence. Validate that binding
before reusing it. The access profile is local and unauthenticated: Swallowtail
opens no credential lease.

Wrong axis (`cline.acp` is not this route), wrong audience, a credential
reference, a model route, or an unqualified package fails before JSON work.
`prepare_cline_acp` stays a separate constructor.

## One Bounded Print Run

Create `ClineHeadlessRunProfileInput::new` with request identity, prompt
content, a read-only working resource, and a host deadline. There is no model
route or auto-approve option. Optional
`with_harness_mode(HarnessMode::Plan)` selects portable Plan. Omission keeps
the current argv and provider-default mode; it is not implicit Plan. Call
`prepare_run`, inspect `evidence()`, `plan()`, `request()`, and
`harness_mode()`, then `start_run`.

The driver owns one joined stdio child and performs this sequence:

1. spawn `cline --json --auto-approve false` plus optional canonical `--plan`,
   then `-c <cwd>` and one prompt operand
2. close stdin immediately; the prompt is argv, not a stdin document
3. decode envelope NDJSON (`run_start`, `agent_event`, `run_result`,
   `run_aborted`)
4. join process and task cleanup on terminal or abort

Selected Plan is a fixed process argument. Exact `3.0.55` applies it to the
one-run config, system prompt, mode-tagged user input, plan tool preset, and
`run_commands` blacklist. That is provider Plan behavior, not filesystem,
network, shell, process, sandbox, or descendant containment. The JSON child
does not register `switch_to_act_mode`. Observation of effective mode stays
withheld: the selected argv does not pass `--verbose`. `act`, `yolo`, and
`zen` are not public values.

Docs `ask`/`say` is the wrong wire. `--acp`, `--id`, `--yolo`, `--zen`, hub,
and TUI stay unselected. CLI `--timeout` is unselected; the host deadline is
required.

Take the run's event stream and terminal outcome immediately and poll them
concurrently. Cancellation kills the child and joins. Close the run separately
from terminal truth. Ambient-host isolation is not filesystem or
descendant-process containment.

See the compile-tested
[`prepared_cline_headless` example](../../crates/swallowtail-adapter-cline/examples/prepared_cline_headless.rs).

## Restart, Failure, And Promotion

Headless is one-prompt. It never auto-retries provider work and exposes no
load, resume, or working-state restoration binding.

Handle failures through portable classification and retain the exact
`swallowtail.cline.headless` diagnostic for support. Do not parse stderr,
envelope payloads, or Cline settings files to infer retry, auth, terminal, or
cleanup truth. Unknown envelopes fail closed.

Promotion of ACP `--acp`, auto-approve true, `--id`, model selection, usage,
session load, or live qualification requires a separate card, exact version
evidence, and matrix coverage. An advertised CLI flag alone is insufficient.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-cline
effigy check:examples
```

No login, install, or authenticated prompt is part of deterministic acceptance.
Live evidence stays separately gated and is not claimed by this route.
