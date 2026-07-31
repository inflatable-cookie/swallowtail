# 077 Cursor Headless Installed-Source Qualification

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

Does installed Cursor Agent `2026.07.01-41b2de7` provide enough exact evidence
for a bounded headless structured-run driver, and does print mode expose any
reasoning activity?

## Method

This pass combined the exact installed help surface, read-only inspection of
the installed bundle, current official Cursor CLI documentation, and
deterministic source-derived fixtures. It sent no provider prompt, created no
run, mutated no workspace, and read no credential or account identity.

The relevant installed chunk is `3479.index.js`, SHA-256
`ac4050a1cd5c798979f890d21c4abc2faf074f6ac3586036090ad87f36191811`.
The inspected source modules are `run-agent.ts`, `output-format.ts`, and
`build-prompt.ts`.

## Correction To Research 075

Research 075 said print mode suppresses thinking. Exact installed source is
more authoritative for this qualified artifact and disproves that statement.
Plain `stream-json` emits `thinking` events with `delta` and `completed`
subtypes. Swallowtail projects those exact provider-disclosed text deltas as
reasoning summaries. It does not infer thinking from assistant output.

## Selected Invocation

The driver launches one host-approved executable with:

```text
--print --output-format stream-json --model <explicit-model> --trust
```

Read-only authority adds `--mode plan`. Explicit read-write authority omits
that suffix and uses Cursor's default mode. The prompt is written to stdin and
stdin is closed. It is never placed in process arguments.

The first route does not select `--force`, `--yolo`, `--sandbox`, or
`--stream-partial-output`. Cursor sandboxing remains optional rather than a
baseline requirement. Omitting partial output avoids duplicate buffered and
delta assistant projections in this artifact.

## Qualified Stream

The exact source proves:

- `system/init` with session, model, cwd, permission, and access-source fields
- user records
- assistant text blocks
- thinking `delta` and `completed`
- tool-call `started` and `completed`, correlated by `call_id`
- successful terminal result with durations, request and session ids
- optional camel-case usage fields: `inputTokens`, `outputTokens`,
  `cacheReadTokens`, and `cacheWriteTokens`

Successful output must end in one result. A non-zero process exit remains a
provider failure even if stderr contains private provider details. A zero exit
without a result is a distinct incomplete-stream runtime failure.

Tool arguments and results, raw stderr, cwd, prompts, and provider payloads do
not enter stable diagnostics. Tool identity and lifecycle retain only bounded
safe labels and opaque correlation.

## Contract Result

No shared contract change is required. Existing structured-run, activity,
usage, installed-version, ambient-configuration, host-service, and optional
sandbox contracts represent the route without flattening it into ACP.

The normalized source record and stream live under
`swallowtail-protocol-acp/tests/fixtures/acp-v1-cursor-agent-2026.07.01-41b2de7`.
They are source-derived fixtures, not a live provider transcript.

## Risks

- Cursor calendar releases and opaque build revisions remain separate evidence
  points. Later releases are visible as unverified newer.
- Failed runs can end without a JSON terminal record. Stable diagnostics cannot
  expose raw stderr.
- The qualified write profile relies on explicit consumer authority and
  Cursor's ambient host controls. It does not claim containment.
- Optional sandbox behavior and partial-output event duplication remain
  unqualified.

## Primary Sources

- [Cursor headless CLI](https://cursor.com/docs/cli/headless)
- [Cursor output formats](https://cursor.com/docs/cli/reference/output-format)
- exact installed Cursor Agent `2026.07.01-41b2de7` bundle
