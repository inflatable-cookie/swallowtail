# Research 371: Gemini CLI 0.63.0 Headless Currentness Stop

Status: official identity chain frozen; headless claim remains at `0.61.0`
pending an operator ruling on selected permission, authority, and tool-output
behavior

Owner: version-currentness lane; task swallowtail#097
Date: 2026-10-07
Base: canonical `main` `450ae5c47154b298cd3187390a88ba88213be5ae`
Authority: Contract 029; version-currentness checkpoint procedure; Queue
brief for task 097

## Result

The current official stable is npm `@google/gemini-cli` `0.63.0`, matching
GitHub stable tag `v0.63.0`. The published stable hops after the existing
`0.61.0` ceiling are `0.62.0` and `0.63.0`. Both remain `UnverifiedNewer` for
`gemini-cli.headless`. Keep the maintained `0.51.0..=0.61.0` segment and the
unpublished `0.56.1` and `0.59.1` exclusions. This evidence changes no
production claim.

The stop includes selected Plan Mode permission and authority behavior in
`0.63.0`. The frozen `--approval-mode plan` prompt changes from instructions
to consult and wait for approval to instructions that autonomously draft a
plan and use `exit_plan_mode` to begin implementation. In the same release,
noninteractive `ASK_USER` policy decisions become `DENY`; file reads resolve a
defensive path before access checks; built-in safety adds `ASK_USER` for
`.gemini` configuration files, which the noninteractive policy denies.
`tool-executor.ts` and `local-executor.ts` also cap stored tool output and
collapse older function responses under context pressure. That changes the
content available through selected Plan Mode tools. The current adapter's
selected permission and tool-output contract cannot be asserted across these
changes from source inspection alone. Tom must rule whether to retain the
`0.61.0` ceiling or authorize an adaptation and a new behavior revision with
explicit authority and tool-output semantics plus regression evidence. Do not
infer a Contract 023 exception or narrow the consumer-facing contract.

## Channels and artifacts

The npm `latest` tag and GitHub's latest non-prerelease release agreed on
`0.63.0` on 2026-10-07. npm `0.63.0` was published at
`2026-10-06T20:58:43.642Z`; GitHub released `v0.63.0` at
`2026-10-06T20:38:49Z`. The npm package's `preview` tag at `0.64.0-preview.0`
and nightly `0.65.0-nightly.20261007.gef59c532f` are not selected. The next
stable `0.63.1` is unpublished at this observation.

| Version | npm tarball SHA-256 | GitHub tag / commit | Git tree | Source archive SHA-256 | npm files / source files |
| --- | --- | --- | --- | --- | --- |
| `0.61.0` | `bc4efa5c925c4430105b552820ed3164bbeffa9dc227990fc922f954733bcd7d` | `v0.61.0` / `bb523741c7429a44d03e964bc124c7c92df59d5f` | `c4226f654bb0b35109c4f5ce34535c5addecda47` | `917e0ac08eb3ef2048910ecc84d4da672ad0c58d82bb2eaa1d6f9e18af80241d` | 449 / 3,005 |
| `0.62.0` | `2276032b1c33d2b828b1cf197e52f48e74b0a395326763ff01a80d97d0fbc0c3` | `v0.62.0` / `b460678f3db508407554afd604cc9d6635becb2a` | `65ae43a5a0bf3670294be027ffe40eb5b0c5f18c` | `18d3955d07457723e5f9b24ff2d7622081b855ea8591d00a977b89a2089626f0` | 449 / 3,015 |
| `0.63.0` | `97a6edfc10645463b517f0518d46a8c72efbdc12558a9a948607f726284a0420` | `v0.63.0` / `573846625af9e93b3b968e0e0b86bb093a4c9b16` | `a6a023123538ba75eb8412528b9942d0de8917e3` | `75903470f15719bc061df6c2792cc55274494f7c25efdb7a864c50f5913bf917` | 449 / 3,019 |

The exact npm SRI for `0.63.0` is
`sha512-mmhqMmAdsoplCzrLQQC4yJBqr4uSKXghnrLspgUU6N9drmsaq7pi7oF09skRYzEq2dYK6eMT4UpFQP6rG2o/ew==`;
the registry shasum is `0ce9cd94c6b3c490af9460fdf9f5e69bc274a84c`. Complete
sorted file inventories and per-hop hashes are frozen in
[`npm-tree-inventory.json`](../../crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-0.63.0/npm-tree-inventory.json)
and
[`source-tree-inventory.json`](../../crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-0.63.0/source-tree-inventory.json).
Identity, channel, selected protocol, and mapped-file digests are also frozen
beside them in that fixture directory.

The complete npm package tree has 449 files at each point. From `0.61.0` to
`0.62.0`, 50 files were added, 50 removed, six changed, and 393 were
identical. From `0.62.0` to `0.63.0`, 48 were added, 48 removed, five changed,
and 396 were identical. The changed files are package metadata, the bin
bundle, and bundled changelog/docs; content-hashed bundle chunk additions and
removals are retained in the tree inventory. The selected Plan prompt chunk
loaded by `bundle/gemini.js` is frozen at each point in `protocol.json`.

The complete tagged source trees contain 3,005, 3,015, and 3,019 files. The
first source hop adds 10 files and changes 78; the second adds four and
changes 87; neither removes a file. The inventory classifies every changed
path. Mapped selected-path changes are:

| Hop | Changed selected or bounded path | Classification |
| --- | --- | --- |
| `0.61.0..0.62.0` | `packages/cli/src/gemini.tsx` | Adds an uncaught-exception path that writes stderr, runs registered CLI cleanup, and exits 1. The normal selected stream and handled error sources are unchanged; stderr is not parsed and nonzero exit remains a failure. |
| `0.61.0..0.62.0` | `packages/core/src/tools/tool-registry.ts` | MCP tool presentation; selected invocation disables external MCP servers. |
| `0.61.0..0.62.0` | `packages/core/src/tools/shell.ts` | Shell behavior after leaving Plan Mode is not in the selected pre-transition mapping; remains unqualified. |
| `0.62.0..0.63.0` | `packages/core/src/prompts/snippets.ts` | Selected Plan Mode prompt changes to autonomous plan drafting and `exit_plan_mode` transition. |
| `0.62.0..0.63.0` | `packages/core/src/policy/policy-engine.ts` | Noninteractive `ASK_USER` decisions become `DENY`; shell redirection and exact rule checks also change. |
| `0.62.0..0.63.0` | `packages/core/src/tools/read-file.ts` | Defensive path resolution and access check change for an available Plan Mode tool. |
| `0.62.0..0.63.0` | `packages/core/src/safety/built-in.ts` | Adds `ASK_USER` for `.gemini` config changes; the noninteractive policy denies that decision. |
| `0.62.0..0.63.0` | `packages/core/src/scheduler/tool-executor.ts` | Caps stored tool output at `MAX_STORED_TOOL_OUTPUT_BYTES`; selected tool output supplied to the model can differ. |
| `0.62.0..0.63.0` | `packages/core/src/agents/local-executor.ts` | Collapses older function responses under context pressure and truncates response parts and display results before history recording. |
| `0.62.0..0.63.0` | `packages/core/src/tools/shell.ts` | Post-transition shell behavior remains unqualified. |
| `0.62.0..0.63.0` | `packages/core/src/core/client.ts` | Cleans up an old recording when resumed-session data points elsewhere; the fresh one-prompt path does not supply resumed-session data. |

All other changed source files are retained as exact paths and classified
`unmapped-provider-internal` in the source inventory. This bounds package
churn without transferring evidence from ACP or other routes.

## Selected route and limits

The selected invocation remains `gemini --output-format stream-json --model
<MODEL> --approval-mode plan --extensions none --allowed-mcp-server-names
"" --skip-trust --session-id <SESSION_ID>`. Selected stream event types,
terminal statuses, native exit codes, and stream formatter/types are
byte-identical across all three versions. The selected route still excludes
external MCP servers and retains caller-bound session identity and durable
transcript posture. `0.62.0`'s general uncaught-exception cleanup path does not
change normal or handled stream paths; its exact rare cleanup behavior was
not live-qualified.

The host `gemini` executable was not available on `PATH`; no install, update,
provider prompt, live session, credential, or artifact execution occurred.
No ACP claim, route capability, permission cell, output mapping, or feature
matrix truth was changed. The headless range, claim ID, behavior revision,
baseline, unpublished exclusions, and visible `UnverifiedNewer` posture stay
unchanged.

Official sources: [npm package versions](https://www.npmjs.com/package/%40google/gemini-cli?activeTab=versions),
[GitHub `v0.63.0`](https://github.com/google-gemini/gemini-cli/releases/tag/v0.63.0),
and [Plan Mode workflow change PR #29539](https://github.com/google-gemini/gemini-cli/pull/29539).
