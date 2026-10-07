# Research 373: Claude Code 2.1.293 headless currentness stop

Status: promoted.

Question: can `claude-code.headless` extend its `2.1.281` ceiling through the
current official stable, `2.1.293`, as a compatible extension?

## Method

Re-probed npm package metadata and GitHub's latest non-prerelease release on
2026-10-07. The channels agree on `2.1.293`. npm's `stable` dist tag remains
`2.1.285`; it is a separate delayed channel, as documented in Research 369.
The host CLI was observed read-only at `2.1.286`; its digest and signing
identity are in the frozen identity fixture.

Downloaded the npm wrapper, Darwin arm64, and Linux x64 packages for every
published point from the previous ceiling, `2.1.281`, through `2.1.293`.
Verified each tarball's SHA-512 against registry metadata, extracted the
packages in a fresh temporary directory, and recorded a complete file and
SHA-256 inventory with every adjacent hop's added, removed, changed, and
identical paths. No downloaded executable was run. Static string inspection of
all 26 Darwin and Linux executables found the selected print, input/output,
permission, tool, model, effort, session, settings, and strict MCP options plus
the stream-JSON input and output help text. This confirms published CLI
surface markers; it does not substitute for executable behavior proof.

GitHub tags identify source commits for each hop, but the public repository does
not establish source-to-npm-runtime provenance. No source-parity claim is made.
The compiled published executables and their digests are the runtime identity.

## Identity

| Surface | Identity |
| --- | --- |
| npm `latest` | `@anthropic-ai/claude-code` `2.1.293`, published `2026-10-07T17:18:04.464Z` |
| GitHub latest stable | `v2.1.293`, published `2026-10-07T18:10:20Z`, commit `79babc372d64101f981bd2b52c3dbe588596dc56` |
| npm `stable` | `2.1.285`, separate delayed channel |
| Host | Claude Code `2.1.286`; SHA-256 `75e3016e9d2570767b08e43a7467d4817a4f149232c169ca295f2c95fef21433`; signed by Anthropic PBC |
| Published hops | `2.1.282` through `2.1.293`, with no unpublished point between the qualified ceiling and latest |
| First unpublished later stable | `2.1.294`; npm returned 404 and the GitHub tag was absent |

The exact npm publication times, source tag commits, runtime digests, file
inventories, and file deltas are frozen under
[`claude-code-2.1.293`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.293/).

## Hop review

The wrapper package changes only version metadata on most hops. Its
`install.cjs` changes at `2.1.288` and `sdk-tools.d.ts` changes at `2.1.284`,
`2.1.285`, `2.1.290`, and `2.1.292`; these are installation and SDK
declaration surfaces, not the selected headless runtime. Each Darwin and Linux
package changes `claude` and `package.json` at every hop. The package manifests
carry the matching runtime version. The binary is provider runtime code and is
classified per hop in `protocol.json`; the inventory gives its exact digest.

| Hop | Selected candidate and classification |
| --- | --- |
| `2.1.281 → 2.1.282` | No selected stream or permission change found in release notes. Settings and tool-permission changes concern managed policy and tool paths outside the fixed selected tool set. |
| `2.1.282 → 2.1.283` | `system/init.plugin_errors[].path` is an additive field; the headless parser requires `session_id`, `model`, and `permissionMode` and ignores other init fields. The non-streaming `result.usage` fallback is outside this stream-JSON route. |
| `2.1.283 → 2.1.284` | No selected stream, permission, failure, usage, or lifecycle candidate found in release notes. SDK declaration change is outside this route. |
| `2.1.284 → 2.1.285` | The CLI now keeps explicit `--permission-mode` over an unspecified default; this route already sends `--permission-mode plan`. The `--max-turns` resume fix is outside the newer headless points because that optional feature keeps its separate exact-version gate and this route disables session persistence. |
| `2.1.285 → 2.1.286` | No selected stream or permission candidate found in release notes. |
| `2.1.286 → 2.1.287` | **Stop.** [The release notes](https://github.com/anthropics/claude-code/releases/tag/v2.1.287) change `--output-format stream-json` turn streaming for a `context: fork` skill invoked as the prompt. The headless command does not disable slash commands, so the mapped prompt/stream shape is not established as unchanged. |
| `2.1.287 → 2.1.288` | No additional selected stream or permission candidate found in release notes. The wrapper installer delta is outside the runtime path. |
| `2.1.288 → 2.1.289` | No selected stream, permission, failure, usage, or lifecycle candidate found in release notes. |
| `2.1.289 → 2.1.290` | **Stop.** [The release notes](https://github.com/anthropics/claude-code/releases/tag/v2.1.290) change permission and safety checks after a `PreToolUse` hook rewrites tool input. Ambient user, project, and local settings remain selected, so this affects a permission boundary that needs a ruling. Connector and resume fixes are otherwise excluded by strict empty MCP configuration and disabled session persistence. |
| `2.1.290 → 2.1.291` | The permission-prompt fix is for cloud sessions and is outside the local headless route. |
| `2.1.291 → 2.1.292` | Resume behavior is outside the route because session persistence is disabled; MCP startup changes are outside the strict empty MCP configuration. Explicit model and tool selections remain fixed. |
| `2.1.292 → 2.1.293` | Default Haiku selection cannot affect the explicit model argument. `$.tool.register.isDeferred` is a mod extension outside the fixed `Read,Glob,Grep` tool path. |

The detailed exact-file classification is in the fixture's `protocol.json` and
`dist-inventory.json`. Historical version claims and Research 369 remain
unchanged.

## Decision

The current family task stops before the claim step. Research 369 qualified
`claude-code.headless` through `2.1.281` with the unpublished gaps
`2.1.244`, `2.1.249`, `2.1.253`–`2.1.256`, `2.1.262`, `2.1.264`, and
`2.1.279`. This record does not change that claim, its behavior revision, or its
exclusions.

Contract 029 and the task brief require a ruling or adaptation for the
`2.1.287` stream behavior and the `2.1.290` permission-security behavior.
The planner should continue this family through an adaptation task that
re-probes the current official stable, qualifies each selected change, and
preserves the released consumer contract. No other route family is in scope.

## Sources

- [npm registry metadata](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code)
- [GitHub `v2.1.293` release](https://github.com/anthropics/claude-code/releases/tag/v2.1.293)
- [Claude Code release channels](https://code.claude.com/docs/en/setup#configure-release-channel)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
- [`claude_code_events.rs`](../../crates/swallowtail-adapter-claude-agent/src/claude_code_events.rs)
- [`claude_code_command.rs`](../../crates/swallowtail-adapter-claude-agent/src/claude_code_command.rs)
