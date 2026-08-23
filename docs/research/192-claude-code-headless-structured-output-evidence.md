# 192 Claude Code Headless Structured Output Evidence

Status: promoted; evidence stop
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Card: g04.045 / 124

## Question

Can exact `@anthropic-ai/claude-code@2.1.238` bind one bounded JSON Schema
structured-output subset on the qualified `claude-code.headless` stream-JSON
Plan-mode route while satisfying Contracts 039 and 040?

## Method And Boundary

Evidence was collected on 2026-08-23 with no Claude installation, login,
credential, account inspection, provider request, prompt, or paid operation.
The exact npm package and its Darwin arm64 platform package were downloaded to
temporary paths only. The native executable was inspected and run only through
local help, version, schema-parse, and no-authentication-gate cases. The host
`claude` executable was not installed or replaced.

The route is `claude-code.headless`, driver
`swallowtail.claude-code.headless`, axis
`claude-code.headless-stream-json`, exact evidence point `2.1.238`, and
existing private behavior `claude-code.headless.stream-json.v1`. This record
does not amend the separate response-only or Claude Agent ACP claims.

## Frozen Sources

| Source | Use | Retrieved | SHA-256 or identity |
| --- | --- | --- | --- |
| [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference) | `--json-schema`, output formats, `--max-turns`, validation and annotation descriptions | 2026-08-23 | `fabc0534dff2e2ca4e31ac095c8c774b0e51900990f1c9ab972eda5ab5c825b0` |
| [Claude Code headless mode](https://code.claude.com/docs/en/headless) | print-mode schema examples, stream result, invalid-schema and exit behavior | 2026-08-23 | `a076cf2eb884f8dbdeff287016de71c169cc00b39d7755474e7f5318d6d07def` |
| [Claude Agent SDK structured outputs](https://code.claude.com/docs/en/agent-sdk/structured-outputs) | schema validation, re-prompt/retry and missing structured-result failure semantics | 2026-08-23 | `1a7676b2e60bff8cb35bcff52993c29cbed3518c76d6f90f3e51911f2e432c30` |
| [`@anthropic-ai/claude-code@2.1.238` registry record](https://registry.npmjs.org/@anthropic-ai/claude-code/2.1.238) | wrapper package identity and published package metadata | 2026-08-23 | integrity `sha512-8AgGrM8qxsA5B8KU/MvVND/fMUsF3vZQxeYjz+1Z/rGx/ZmNr0iqjfmUVKVASKN7P9OzkAUHoXgKEpyvgRfUkA==`; shasum `a8ba2539a61441b7a268a07dc2bf5623534fd127`; tarball `6a7b0ef9b12feea02d7c166b16d2674edca7658daeb137efb4c85d9e5371b6ea` |
| [`@anthropic-ai/claude-code-darwin-arm64@2.1.238` registry record](https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-arm64/2.1.238) | native platform package selected by the wrapper | 2026-08-23 | integrity `sha512-/v6LuTgudzxPrpPOb54+Sg7m/O5NVOYE5YDPxWhxjS7IweNjbQIaY+eKFhnooE199YogqrEqrml0jibWdbVnOw==`; shasum `d658798e7455ac0db9baf43b3461234b2466cf2a`; tarball `0769cc2f4173a652c8c61c292010270c2036b0fd7400b5aa0487661c927c7c8e` |
| Native `claude` binary, Darwin arm64 | exact executable version and implementation strings | 2026-08-23 | binary `1c196c456373b57818ae87df84aecee96cb659448c0d6a6bbb401ac5758431b2`; extracted strings `bcd71d117806ac85d72dfbaf10c281d30ba59c58b60f2f572a11b35faa3c7b61` |
| `headless-structured-output.json` | sanitized deterministic specimen corpus | 2026-08-23 | `24f18d83b20cea17dfee49eb4fdf0709b9bccef3b0d2003a2b9a39f6a4630a3c` |

The wrapper package was published at `2026-08-20T18:01:54.712Z`. The exact
point is held for this lane; later published versions are not inherited by
this evidence.

## Exact Package And Help

The npm package is a wrapper. Its platform dependency supplies the native
Darwin arm64 executable, which reports `2.1.238 (Claude Code)`. Exact help
declares:

- `--json-schema <schema>` for JSON Schema structured-output validation
- input `text|stream-json` and output `text|json|stream-json`
- `--permission-mode` including `plan`
- the selected model, effort, tool, setting-source, MCP, strict-MCP, verbose,
  and no-session-persistence controls

The exact `2.1.238` help does not list `--max-turns`. The current CLI reference
describes `--max-turns` as an agentic-turn limit; it does not make that flag a
structured-output retry budget. No exact retry count or preflight-bindable
attempt bound was exposed by the selected package/help surface.

The package accepts a standard object specimen locally. It rejects malformed
JSON, a non-array `required`, and an unknown `x-unsupported` keyword with the
exact diagnostics frozen in the fixture. A `format: date-time` annotation and
a deterministic 200000-byte description passed local schema validation and
reached the same no-authentication gate. The observed size behavior does not
establish an upper limit.

The exact draft or dialect is not named. The observed strict keyword behavior
and documented `format` annotation are not enough to infer a draft, a portable
keyword subset, or compatibility with an arbitrary JSON Schema descriptor.

## Selected Command And Secret-Free Specimen

The selected no-provider command was:

```text
claude -p --input-format text --output-format stream-json --verbose \
  --no-session-persistence --model claude-opus-5 --effort high \
  --permission-mode plan --tools Read,Glob,Grep \
  --setting-sources user,project,local --mcp-config '{"mcpServers":{}}' \
  --strict-mcp-config --json-schema '<valid-object-schema>'
```

The valid schema passed local validation. With an empty isolated configuration
and no authentication, the process emitted the following sanitized shapes and
made no provider request:

| Event | Observed fields |
| --- | --- |
| `system/init` | `tools: ["Glob", "Grep", "Read", "StructuredOutput"]`; `mcp_servers: []`; model `claude-opus-5`; `permissionMode: "plan"`; version `2.1.238`; `apiKeySource: "none"` |
| assistant | local authentication failure text; `error: "authentication_failed"` |
| result | `is_error: true`; `num_turns: 1`; `subtype: "success"`; no `structured_output` field; process exit `1` |

The result subtype is not sufficient success truth. A strict adapter would
have to inspect the error and structured-output field independently. This
specimen proves command composition and the model-visible tool boundary; it
does not prove a valid provider-produced structured result.

The existing schema-absent command, Plan posture, fixed `Read,Glob,Grep`
tools, working-resource authority, model/reasoning controls, activity, usage,
deadline, cancellation, process stop, and joined cleanup remain the existing
qualified behavior. Their composition with an effective structured result was
not promoted from the no-authentication specimen.

## Enforcement, Retry, And Result Truth

The exact native binary contains the model-visible `StructuredOutput` tool and
implementation signals for:

- `StructuredOutput enforcement failed`
- `Output does not match required schema:`
- `structured_output_retry_exhausted`
- `error_max_structured_output_retries`
- `Error: Failed to provide valid structured output after maximum retries`
- structured-output retraction and surviving-valid-output messages

The current Agent SDK documentation independently describes validation,
re-prompting, a retry limit, and failure when no structured output survives.
Together with the exact CLI binary's model-visible tool and retry signals,
Contract 040 classifies this implementation as `HarnessValidated`, not
`ProviderNative`. This is an evidence classification, not an admitted
capability.

The exact retry maximum remains unknown and is not bindable from the selected
CLI surface. A hidden positive retry count without an immutable preflight
bound fails Contract 040. The CLI's agentic-turn control cannot be substituted
for a schema-attempt bound.

The no-auth result has no `structured_output` field despite a `success`
subtype, while also carrying `is_error: true` and exit `1`. The evidence does
not qualify the valid-output terminal shape, null-versus-missing behavior
after a provider attempt, duplicate or post-terminal handling, cumulative
usage, or cancellation during schema retries. No live prompt was authorized
to fill those gaps.

## Route And Schema Disposition

| Row | Exact evidence | Disposition |
| --- | --- | --- |
| schema absent | Existing route fixtures and prepared tests preserve current command and ordinary result behavior | unchanged; remains the only accepted headless schema row |
| valid object schema | Local parse succeeds; selected command composes; `StructuredOutput` appears beside fixed tools | withheld; exact dialect, retry bound, valid terminal result, and full route composition are not qualified |
| `format` annotation | Local parse succeeds and official CLI describes it as an annotation | withheld; annotation acceptance is not validation semantics |
| malformed, invalid, unknown-keyword schema | Exact local rejection diagnostics are frozen | rejection evidence only; not a capability row |
| large schema | 200000-byte description reaches authentication gate | no size limit or portable bound admitted |
| enforcement | Exact binary and SDK evidence classify `HarnessValidated` | withheld until attempt and terminal contracts are exact |
| `2.1.238` version | exact package/native binary identity is frozen | evidence-only; no new behavior revision |
| later versions | later releases exist outside this exact package probe | `UnverifiedNewer`; no compatibility inference |

Deliver-now rows: **none**.

No new Contract 029 facade point or private behavior revision is needed. The
existing `claude-code.headless.stream-json.v1` behavior and schema-absent
command remain unchanged. No production binding, guide capability claim,
matrix row, or compatibility-range change follows from this record.

## Decision

Card 124 is complete as an evidence stop. Cards 125 and 126 are blocked and
were not executed. The exact evidence is sufficient to retain the current
route and to reject a future schema request until a named package surface
exposes an exact dialect, immutable attempt bound, valid terminal result, and
full lifecycle composition.
