# Research 388 Cline Headless 3.0.69 Identity and Qualification

## Decision

Qualify only `cline.headless` through official npm stable `3.0.69`. Preserve
baseline `3.0.55`, claim `cline.headless.package-window-1`, behavior revision
`cline.headless.stdio-json-v1`, and `QualifiedOnly` posture. Qualify each
published npm stable hop and exclude unpublished `3.0.59`. Keep
`cline.acp` exact at `3.0.55`.

No public operation, route, capability, selected argument, credential binding,
or lifecycle mapping changes. The selected JSON envelope and terminal mapping
remain the same. New upstream media content stays unmapped and fails closed.

## Official Identity

Observed 2026-10-08. The selected channel is npm package `cline`, dist-tag
`latest`, at `https://registry.npmjs.org/cline/latest`. It resolves to
`3.0.69`, published `2026-10-07T06:13:57.143Z`. The official package
packument lists published stables after the existing `3.0.55` ceiling:
`3.0.56`, `3.0.57`, `3.0.58`, and `3.0.60`–`3.0.69`. It does not list
`3.0.59`; the upstream `cli-v3.0.59` tag does not provide an npm artifact and
does not fill that hole. Editor releases and nightly tags are separate
channels.

Each published root wrapper has an exact npm tarball SHA-512 integrity and
shasum, a locally computed tarball SHA-256, and a six-file extracted tree.
Each root package pins five exact `@cline/*` dependency package versions and
all six `@cline/cli-{darwin-arm64,darwin-x64,linux-arm64,linux-x64,
windows-arm64,windows-x64}` runtime package versions. Their registry SHA-512,
shasum, file count, and unpacked size are frozen for every hop. A complete
extracted file tree is included for Darwin ARM64; its runtime binary was
hashed but not executed. The other five runtime packages are identified by
their exact registry digests and metadata and were not extracted.

The `cli-v<version>` annotated tag object and peeled source commit are frozen
for every published point. Ten selected source files used for the headless
wire, Plan argument, lifecycle, tool, permission, failure, and usage mapping
have per-version SHA-256 and byte counts. Those paths and every changed-file
classification appear in the machine-checked ledger below.

## Hop Review

| npm hop | Selected source changes and classification |
| --- | --- |
| `3.0.55` → `3.0.56` | `main.ts` changes model-catalogue display; `run-agent.ts` passes Plan mode into prompt construction; `utils/events.ts`, `types.ts`, and `agent-runtime.ts` add media handling. The CLI envelope and terminal fields remain. Media is not projected and stays fail-closed. |
| `3.0.56` → `3.0.57` | `types.ts` and `agent-runtime.ts` add internal request identity metadata. |
| `3.0.57` → `3.0.58` | No changes in the ten selected source files. |
| `3.0.58` → `3.0.60` | No changes in the ten selected source files; no npm artifact exists for `.59`. |
| `3.0.60` → `3.0.61` | `agent-runtime.ts` changes denied-tool explanatory text only. The wrapper `bin/cline` adds Windows spawn-failure guidance while preserving exit code 1. |
| `3.0.61` → `3.0.62` | `main.ts` changes setup/migration reporting outside the selected command; `agent-runtime.ts` adds bounded internal transient-provider retries. |
| `3.0.62` → `3.0.63` | `main.ts` changes setup/config loading; `types.ts` adds internal usage and hook-origin fields; `agent-runtime.ts` adds run-start hook context. |
| `3.0.63` → `3.0.64` | `agent-runtime.ts` adds internal max-token recovery and changes empty-response handling. |
| `3.0.64` → `3.0.65` | `agent-runtime.ts` adds prepared-request and bounded max-token recovery internals. |
| `3.0.65` → `3.0.66` | `types.ts` adds reasoning-token usage; `agent-runtime.ts` changes content-filter/finish-reason failure attribution. Usage remains excluded; non-completed terminal outcomes remain generic failures. |
| `3.0.66` → `3.0.67` | No changes in the ten selected source files. |
| `3.0.67` → `3.0.68` | No changes in the ten selected source files. |
| `3.0.68` → `3.0.69` | `main.ts` changes JSON output for the config subcommand; `agent-runtime.ts` adds one internal continuation for unknown finish/no external tool. The JSON run-result shape is unchanged. |

Swallowtail still starts one child for one prompt. Cline `3.0.62` and later
may retry provider requests or continue internally after selected model
outcomes; individual attempts and usage are not projected. The CLI still
reports a terminal `run_result`; Swallowtail maps completed to success,
aborted to cancellation, and other finish reasons to generic failure. The
selected command keeps `--auto-approve false`, and denied tool requests remain
non-approved. No prompt, catalogue query, or live session was sent.

The complete six-file wrapper trees show `package.json` changing at each
release and `bin/cline` additionally changing at `.61`; the other four wrapper
files remain byte-identical. The Darwin ARM64 runtime trees contain 148 files
through `.60` and 142 from `.61`; webview assets account for broad add/remove
churn. Every runtime package artifact is still pinned by its npm digest. Full
wrapper/runtime inventories and exact path-level hop sets are frozen in
`crates/swallowtail-adapter-cline/tests/fixtures/cline-headless-3.0.69/`.

## Qualification Limits

- Only `cline.headless` advances. ACP stays exact `3.0.55`.
- The supported point set is `3.0.55`–`3.0.69` minus `3.0.59`.
- `QualifiedOnly` remains; versions above `3.0.69` do not execute through this
  claim.
- Cline media content is not translated into portable output. The decoder
  rejects a media `content_start` with the existing safe
  `swallowtail.cline.headless.malformed_stream` diagnostic.
- Provider-internal retry/continuation and usage details are not surfaced as
  Swallowtail operations or records.
- npm artifacts and source files were read as data. No package or binary was
  executed; Cline was not installed or present on `PATH`; no credentials,
  provider prompt, live catalogue, host update, or host mutation occurred.

## Reproduction Data

- npm latest: <https://registry.npmjs.org/cline/latest>
- npm package history: <https://registry.npmjs.org/cline>
- source tags: <https://github.com/cline/cline/tags>
- exact identities: `identity.json`
- complete wrapper trees: `npm-wrapper-tree-inventory.json`
- complete Darwin ARM64 runtime trees: `darwin-arm64-runtime-tree-inventory.json`
- selected source hashes: `selected-source-tree-inventory.json`
- wrapper, runtime, and selected-source hop diffs: `hop-ledger.json`
