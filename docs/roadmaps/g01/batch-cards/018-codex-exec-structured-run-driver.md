# Codex Exec Structured-Run Driver

Status: completed
Owner: Tom
Roadmap: 006 Codex Proof Drivers
Updated: 2026-07-19

## Goal

Extract a provider-owned `codex exec` driver for bounded structured runs.

## Scope

- establish `swallowtail-adapter-codex` as the integration-family crate, with
  exec and later app-server drivers kept as separate role implementations
- text-only, read-only, ephemeral structured runs through the exec surface
- request-to-command translation through approved process service
- JSONL event normalization
- cancellation, terminal result, and cleanup
- applicable Contract 011 profile

## Out Of Scope

- Soundcheck tagging, validation, repair, or ranking
- Codex app-server sessions
- public Platform Responses API
- model discovery, which the exec surface does not expose
- image and schema inputs until host-authorized process-path materialization
  exists
- workspace-write and unrestricted sandbox operation
- deadline enforcement until the task/time boundary has cancellable timer
  ownership

## Acceptance Criteria

- no consumer product type or policy enters the driver
- the adapter depends on runtime ports, not `swallowtail-host-local` or
  `std::process`
- provider payloads stay behind normalization/extensions
- prompt and output content stay redacted in default formatting
- process launch uses the exact executable, model, and working resource bound
  by the request and preflight plan
- structured-run profile and claimed capability assertions pass

## Validation

- recheck installed `codex exec --help` without inspecting credentials
- deterministic fake-process JSONL fixtures
- focused adapter and Contract 011 profile tests
- `effigy qa`
- `git diff --check`

## Stop Condition

Stop and promote new evidence before implementation if the current Codex exec
surface contradicts Research 003 or requires authority outside Contracts
005-011.

## Closeout

- `swallowtail-adapter-codex` provides a separate Codex exec structured-run
  role over runtime task and process ports.
- requests use the exact preflight-bound executable and model plus an opaque
  working resource; prompt text enters the process only through bounded stdin.
- the first proof fixes `--json`, `--ephemeral`, and read-only sandbox posture.
- JSONL parsing normalizes lifecycle and agent-output events. Provider payloads,
  prompts, output, arguments, and host references remain redacted by default.
- cancellation force-stops the owned process, terminal outcome stays distinct,
  and close joins the scoped pump task.
- deadlines, attachments, structured output, model discovery, and broader
  sandbox access reject before process start or remain assigned to later
  contracts and drivers.
- deterministic fixtures cover translation, split JSONL, completion output,
  cancellation, cleanup, redaction, and unsupported-input side-effect order.
