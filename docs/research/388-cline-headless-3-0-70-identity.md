# Research 388 Cline Headless 3.0.70 Identity and Qualification

## Decision

**Status:** promoted. Qualify only `cline.headless` through official npm
stable `3.0.70`, using a compatible extension. Preserve baseline `3.0.55`,
claim `cline.headless.package-window-1`, behavior revision
`cline.headless.stdio-json-v1`, and `QualifiedOnly` posture. Qualify each
published npm stable after the baseline and exclude unpublished `3.0.59`.
Keep `cline.acp` exact at `3.0.55`.

No Swallowtail operation, route, capability, selected argument, credential
binding, or lifecycle mapping changes. The selected JSON envelope and
terminal mapping remain the same. New upstream media content remains unmapped
and fails closed.

## Official identity

On 2026-10-08 the documented channel, npm package `cline` dist-tag `latest`,
resolved to `3.0.70`, published `2026-10-08T05:56:02.947Z`. The exact root
package has registry SHA-512
`sha512-ekeGJ7YdVKL9p//KF0F5nxFXMvgSuhwPxvCUkS+SaBDJJoOUt+HMeJS+J+hkF3ezPekbSvpK1wCPGraZbAEO8A==`,
shasum `837f68cd31378bd5fcd7a38510625bdc9c33adbb`, and downloaded tarball
SHA-256
`4081fba9ec0867e32267b3875fbbe322edff8d55b7a11c1dcd0ff3066c88760b`.
Its six-file wrapper tree has 48,236 unpacked bytes and deterministic manifest
SHA-256 `e34201854e1d814d6b099fe2a57afde456d9fcce61348fb34c6d62221cc87a94`.

The npm packument contains these published stable points after the existing
ceiling: `3.0.56`, `3.0.57`, `3.0.58`, and `3.0.60` through `3.0.70`.
`3.0.59` has a GitHub source tag but no npm artifact, so it remains excluded.
`3.0.71` was absent from both the npm packument and GitHub tag refs when
observed; it is the synthetic newer point in the rejection tests. Editor
releases and npm nightly are separate channels.

The npm wrapper pins five exact `@cline/*` dependency packages at `0.0.92`
and six optional `@cline/cli-*` runtime packages at `3.0.70`. The complete
registry SHA-512 integrity, shasum, file count, unpacked size, and tarball URL
for every dependency and platform artifact are frozen in the fixture's
`identity.json`. The extracted Darwin ARM64 tree has 142 files, 95,111,856
unpacked bytes, manifest SHA-256
`e4ef11780c7ac862353e572378ab57f1265a8161d71be57b3c8e5dbd010ddd85`, and
`bin/cline` SHA-256
`3ae76234a92f4de4fe6f9bf22635ac656994b29af83ea6d5c144b7eeea5a9ae7`.
The other five platform artifacts are bound by their exact registry metadata;
they were not extracted. No runtime binary was executed.

GitHub's annotated `cli-v3.0.70` tag object is
`6c21659d8e5ef8b881dc1972e4aa8112bb9af367`, peeled to source commit
`0322bc5d510000a33ef5eadc3b4c84df7fcef285`. The selected-source inventory
freezes twelve CLI invocation, runtime, output, and agent-envelope paths at
every published tag, with SHA-256 and byte count. The full upstream compare
from `cli-v3.0.69` to `cli-v3.0.70` lists 90 changed repository files; the
selected files and unselected boundaries are classified below.

## Selected-source hop review

| npm hop | Changed selected source and classification |
| --- | --- |
| `3.0.55` → `3.0.56` | `main.ts` changes model-catalogue display; `run-agent.ts` passes selected Plan mode into prompt construction; `utils/events.ts`, shared `types.ts`, and `agent-runtime.ts` add media handling. The CLI envelope and terminal fields remain. Media is not projected and stays fail-closed. |
| `3.0.56` → `3.0.57` | `types.ts` and `agent-runtime.ts` add internal provider request identity metadata. |
| `3.0.57` → `3.0.58` | No changes in the twelve selected source paths. |
| `3.0.58` → `3.0.60` | No changes in the twelve selected source paths; npm has no `.59` package. |
| `3.0.60` → `3.0.61` | `agent-runtime.ts` changes denied-tool explanatory text only. The wrapper `bin/cline` adds Windows spawn-failure guidance while preserving exit code 1. |
| `3.0.61` → `3.0.62` | `main.ts` changes setup/migration reporting outside the selected command; `agent-runtime.ts` adds bounded internal transient-provider retries. |
| `3.0.62` → `3.0.63` | `main.ts` changes setup/config loading; `types.ts` adds internal usage and hook-origin fields; `agent-runtime.ts` adds run-start hook context. |
| `3.0.63` → `3.0.64` | `agent-runtime.ts` adds internal max-token recovery and empty-response handling. |
| `3.0.64` → `3.0.65` | `agent-runtime.ts` adds prepared-request and bounded max-token recovery internals. |
| `3.0.65` → `3.0.66` | `types.ts` adds reasoning-token usage; `agent-runtime.ts` changes content-filter and finish-reason attribution. Usage remains excluded; non-completed terminal outcomes remain generic failures. |
| `3.0.66` → `3.0.67` | No changes in the twelve selected source paths. |
| `3.0.67` → `3.0.68` | No changes in the twelve selected source paths. |
| `3.0.68` → `3.0.69` | `program.ts` makes the existing `--yolo` option visible in command help; the selected invocation passes `--auto-approve false` and never passes `--yolo`. `main.ts` changes JSON output for the config subcommand; `agent-runtime.ts` adds an internal continuation for unknown finish/no external tool. Selected permission behavior and JSON run-result shape stay unchanged. |
| `3.0.69` → `3.0.70` | `program.ts` changes unselected `--thinking`, `--key`, and `--retries` options: `--key` now saves the provider key and `--retries` requires a value. None is passed by the selected invocation; its no-argument retry default remains 3. `main.ts` hides the dashboard command from root help and replaces the retry literal with the same constant in `defaults.ts`; `defaults.ts` adds `CLI_DEFAULT_MAX_CONSECUTIVE_MISTAKES=3`. |

At `.70`, `run-agent.ts`, `session-events.ts`, `tool-policies.ts`,
`utils/events.ts`, `utils/helpers.ts`, `utils/output.ts`,
`utils/startup-settings.ts`, `agent-runtime.ts`, and shared agent types are
byte-identical to `.69`. The selected argv remains `cline --json
--auto-approve false` plus one prompt; optional Plan remains the existing
canonical `--plan` argument. The help-visible `--yolo`, credential-saving
`--key`, retry override, model and thinking options are not passed or mapped.
The adapter binds no credentials, model route, client MCP servers, catalogue,
usage, or provider session. Thus these upstream surfaces do not add a
Swallowtail operation or change its permission, wire, or lifecycle mapping.

The GitHub compare also changes CLI subcommands for dashboard, hub, MCP,
scheduling, connectors, and logging; those are other command entry points.
`@cline/core` changes MCP client/install and provider-settings migration
internals, while `@cline/llms` changes generated provider and model catalogues.
The route selects none of those operations or catalogue surfaces. Exact
`.70` dependency and runtime identities, and the full package file deltas,
remain frozen; these provider-internal changes are not projected by the
headless adapter. This qualification does not add any of those options,
commands, provider tools, or capabilities to Swallowtail.

Cline `3.0.62` and later may retry provider requests or continue internally
after selected model outcomes. Swallowtail still starts one child for one
prompt. Attempts and usage are not projected. The CLI reports a terminal
`run_result`; Swallowtail maps completed to success, aborted to cancellation,
and other finish reasons to generic failure. Denied tool requests remain
non-approved. No prompt, catalogue query, or live session was sent.

## Artifact tree deltas

The complete six-file npm wrapper inventories show `package.json` at every
published hop, plus `bin/cline` at `.60` → `.61`, and `README.md` at `.69` →
`.70`; no wrapper paths are added or removed. The `.70` README documents
updated key, thinking, and dashboard behavior; it is discovery evidence only.

The Darwin ARM64 runtime tree has 148 files through `.60` and 142 from `.61`.
Webview assets account for the recorded add/remove churn. At `.69` → `.70`,
only `bin/cline` and `package.json` change; no paths are added or removed.
The binary digest is recorded without execution. The source compare above
classifies the changed selected route sources, while webview and product UI
assets remain outside this headless interface.

Every full tree and adjacent-hop path set is frozen in
`crates/swallowtail-adapter-cline/tests/fixtures/cline-headless-3.0.70/`.
The ledger asserts the exact `.70` changed file sets for the wrapper, Darwin
ARM64 runtime, and twelve selected source paths.

## Qualification limits

- Only `cline.headless` advances. ACP stays exact at `3.0.55`.
- The supported point set is published `3.0.55` through `3.0.70`, excluding
  `3.0.59`.
- `QualifiedOnly` remains; package versions above `3.0.70` do not execute
  through this claim.
- Cline media content is not translated into portable output. The decoder
  rejects a media `content_start` with the existing safe
  `swallowtail.cline.headless.malformed_stream` diagnostic.
- Provider-internal retries, continuation, model catalogues, local MCP
  behavior, and usage are not surfaced as Swallowtail operations or records.
- npm artifacts and source files were read as data. No package or binary was
  executed; Cline was not installed or present on `PATH`; no credentials,
  provider prompt, live catalogue, host update, or host mutation occurred.

## Reproduction data

- npm latest: <https://registry.npmjs.org/cline/latest>
- npm package history: <https://registry.npmjs.org/cline>
- source tags: <https://github.com/cline/cline/tags>
- `.69` → `.70` source compare:
  <https://github.com/cline/cline/compare/cli-v3.0.69...cli-v3.0.70>
- exact identities: `identity.json`
- complete wrapper trees: `npm-wrapper-tree-inventory.json`
- complete Darwin ARM64 runtime trees: `darwin-arm64-runtime-tree-inventory.json`
- selected source hashes: `selected-source-tree-inventory.json`
- wrapper, runtime, and selected-source hop diffs: `hop-ledger.json`
