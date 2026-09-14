# 307 Claude Code 2.1.270 Identity

Status: promoted
Owner: Tom
Date: 2026-09-14
Task: g05.058

## Question

Is official npm/GitHub Claude Code `2.1.270` a compatible extension of the
qualified headless `2.1.220..=2.1.257` and response-only `2.1.227..=2.1.257`
windows, a private milestone, a new facade, or a stop? Headless and
response-only stay one family. Watcher help, digest, and live authorization
stay on exact `2.1.251`.

## Remaining Rank

This run covers only Claude Code. At observation time the family was
AllowUnverified official-newer: g05.019 left the qualified ceiling at
`2.1.257` while official stable had moved to `2.1.270`.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `claude-code.headless-stream-json` | `2.1.258` on `PATH` | npm and GitHub `2.1.270` | qualified `2.1.220..=2.1.257`; AllowUnverified | official-newer |
| `claude-code.response-only-stream-json` | `2.1.258` on `PATH` | npm and GitHub `2.1.270` | qualified `2.1.227..=2.1.257`; AllowUnverified | official-newer |

Gemini remains deferred. Claude Agent ACP, the Claude Agent SDK, and the
watcher family are separate surfaces and were not reopened.

## Method

Re-probed npm `@anthropic-ai/claude-code@latest` and the GitHub latest
release, then retrieved the npm wrapper and the `darwin-arm64` and `linux-x64`
platform tarballs for the previous ceiling `2.1.257` and all eleven published
hops into `/tmp/g05-058`. Every tarball was SHA-256 hashed; the `2.1.257`
values reproduce Research 273. npm version metadata proves `2.1.262` and
`2.1.264` were never published, and every GitHub tag `v2.1.258` through
`v2.1.270` resolves to a lightweight commit.

Downloaded official binaries were never executed. Because the published
`claude` executable is a compiled artifact rather than readable shipped source,
mapped-surface evidence was recovered from the embedded `// @bun @bytecode`
source chunks the binary carries. For both platform builds and all twelve
compared versions:

- extracted every anchored source chunk and classified the selected option
  definitions for `-p`/`--print`, `--input-format`, `--output-format`,
  `--verbose`, `--no-session-persistence`, `--model`, `--effort`,
  `--permission-mode`, `--tools`, `--setting-sources`, `--mcp-config`,
  `--strict-mcp-config`, `--max-turns`, `--safe-mode`,
  `--disable-slash-commands`, `--no-chrome`, and `--prompt-suggestions` by a
  minification-insensitive normalization (ordered string literals plus method
  names);
- resolved each `.choices(...)` argument for input format, output format,
  effort, and permission mode, including the `default` → `manual` input alias
  and the separate wire enumeration;
- normalized the `init`, `stream_event`, `hook_started`, `result`, and
  `thinking_tokens` construction sites so minified identifiers collapse; and
- built hop-by-hop wrapper and platform-package file inventories with exact
  SHA-256 digests.

The host `claude` is exact `2.1.258`, and the host native SHA-256 equals the
official `2.1.258` `darwin-arm64` binary. Host `--help` therefore reproduces
official `2.1.258` help, and its digest is byte-identical to the frozen
`2.1.257` help digest. The host was observed once with `--version` and
`--help`; it was not installed, updated, or replaced. Changelog and release
bodies were read as discovery only.

No provider prompt, login, credential, install, host update, downloaded
binary execution, watcher live work, release, or consumer change occurred.

## Identity

npm and GitHub latest agree on `2.1.270`. npm published it at
2026-09-12T18:52:44.937Z with wrapper integrity
`sha512-0zMkfIWQu7/SG56VP8r780HZWvrNShzK28AbAnhKRK0ns+ToGXPT0W8UqyZmZCUKAkJDd5//TrwSOhk1+hysiw==`
and tarball SHA-256
`ee37736066e3349c977db70bdfaf4f3cf7c398a06e2a051fe8e00728e1f6e932`. GitHub
published `v2.1.270` at 2026-09-12T19:45:44Z with lightweight tag commit
`2b40e76d3f03b9070e2431e0bd05b4f3ace77982`.

| Version | npm published | GitHub tag commit | darwin-arm64 SHA-256 | linux-x64 SHA-256 |
| --- | --- | --- | --- | --- |
| `2.1.257` | 2026-09-01T17:15:33.223Z | `a1e64dc407dd57dfb4ea283b0f8049adf3eabee5` | `64590d7d9d9c189d33fb3dfa58c5408eaf2a10fe556bd84155d95efaab46b60e` | `9a64bda9d8722a1fa05bef9a5961d07e0331b99597eda9e2f6a732f3a0ff7f05` |
| `2.1.258` | 2026-09-01T22:25:07.449Z | `aef74afe01f65b602258d6102b0da9730ac6f0aa` | `b63136194160791c27cfa7b0403060d85eb0752991625fde8c09f9acacb17c78` | `704f1334ac65d3e89e1c6c1d7663293ad786a6166afdb71b5075337df630f976` |
| `2.1.259` | 2026-09-02T21:21:42.115Z | `f173a697aa6486945f1b9c4aa9ce5383d2c87db6` | `884baa38fe1a624be25c4a91568bf5a08b5cf4e7d7acf29b7760e3525d964898` | `f7dd62ae415378018cd21dd950eb3bac174ab085830304d3b8b098146bfd47b6` |
| `2.1.260` | 2026-09-03T22:32:02.087Z | `b3f0e501b79fe5cfc8c10d18cf3b0b6715c5c2fb` | `3c269f66801028823e24a63ced9fdd3988cb86cf85fccd9f03f87e463b9d3e3c` | `7a2fdc74b6836ea3d183f665b869f0ee3baebc9713cbebffe5838da4ea7bd82e` |
| `2.1.261` | 2026-09-04T17:49:34.927Z | `d7dbd9a09f59775726ed14bbea8fc9dfdff62f7b` | `5efecaff231b798be3c66def9be54183623b328b80eaef17f93c43987024e82a` | `4ae40dd1784e85753e742e09f267d29ecbb82890361ad3817d27560866d364a6` |
| `2.1.263` | 2026-09-06T02:07:58.391Z | `ab9b2cf7bb9e4f98ff264c07a22e46d83c29c558` | `ef5d2909c8af49f31ab6d5487e90316777bc2fac170adfe8160716caa8aaf4f9` | `26d020351e8112f4006790f3cfce43b4c9df0c1bb1d0e542364d64151b81d5ba` |
| `2.1.265` | 2026-09-08T19:05:16.492Z | `8e02f6ddce21f4c2585d2be501651c1d6b16246a` | `164b09eb800dedb9bb06304129fbf07743ab9db972ae07333ef4f18cde0cb8d5` | `e14738e3a58d1fc6ccc23b9c919451b4846bc27074a3fb48db976a7d595bdeeb` |
| `2.1.266` | 2026-09-08T23:32:32.880Z | `347b38e4a733d95b2f00690a4ca58ac1544f8a1c` | `553d1b9e9e7068b275c0a783c7e139ff6503096f286e674c8c919379fb0eca62` | `19842705e989393fce936804df6d2ab034860e24b8f8880357981d87ffd83fac` |
| `2.1.267` | 2026-09-09T18:25:42.820Z | `9cdc2a4d946c586a8472e504fb20b3e79106518c` | `a681f3008f0050029aeebcab3af51bb6a55ddeb625a3af3141a4416d43cd2558` | `0399c793ff571d5946ef923d80b4f330d05ac4b6842a6b0775468f5d389403c0` |
| `2.1.268` | 2026-09-10T18:41:11.770Z | `536a2e23d9e28586f81f17b3535281b5f2995a70` | `06a96d5423f83770f120859f1c58e60d7252cc4c122aa13043b7e7cd716bc76a` | `9691a2b7bd796712ca8cffb8e32e54ff7fc45b662540233171a16a94a0425653` |
| `2.1.269` | 2026-09-11T18:12:49.253Z | `df52d04a4e65195c1621fe6222e0564bcccb1804` | `c942e1228b93cb4d52183b3dfbc77f28264f35aa947acd9c0853d029164cf450` | `25e44883f54419569a3d739f38cbbdaebe83b09895da0f343e1b003710a4775b` |
| `2.1.270` | 2026-09-12T18:52:44.937Z | `2b40e76d3f03b9070e2431e0bd05b4f3ace77982` | `a506b6d970a4cf44f6abdb53a81ddcd5d3b0ce042a95c502fe9d1f946bdb8807` | `3a624a5a7cd79bbad4d32bd7db36f1197ecf458bc5bf1e2aed81834a01ad3ef0` |

Published stables after the previous ceiling are exactly the eleven hops
above. `2.1.262` and `2.1.264` are unpublished. Existing unpublished gaps
`2.1.244`, `2.1.249`, and `2.1.253` through `2.1.256` stay incompatible.
First unpublished later stable at observation: `2.1.271`.

Host `claude` is `2.1.258`, SHA-256
`b63136194160791c27cfa7b0403060d85eb0752991625fde8c09f9acacb17c78`, 199027600
bytes, codesigned `com.anthropic.claude-code` by team `Q6L2SF6YDW`. That is
observation input only; it is a qualified neighbour and is not a qualification
or install authority.

## Selected Protocol

The npm package remains an installer wrapper. Wrapper file count stays 7 and
platform package file count stays 4 at every hop. `LICENSE.md`, `README.md`,
`bin/claude.exe`, `cli-wrapper.cjs`, and `install.cjs` are byte-identical
across all twelve compared versions. `package.json` changes only its version
pin and `optionalDependencies` platform packages. `sdk-tools.d.ts` changes at
seven of the eleven hops; every delta is SDK tool declaration content
(`SkillCreate` description wording, `ArtifactPublish`/audit fields, the
browser-tab icon parameter, `project_memory_list`/`project_memory_read`,
`list_types`, `cowritten`/`staged`/`unchanged`/`seq`/`foreign`/`written`
notes, and publish metadata wording). Neither route consumes that file.

The selected mapped surface is unchanged on both platform builds across every
hop:

- Every selected flag resolves to an identical normalized definition set at
  every hop, covering flag name, value placeholder, help description, and
  parser attach methods.
- Input format choices stay `text`/`stream-json`; output format choices stay
  `text`/`json`/`stream-json`; effort choices stay `low`/`medium`/`high`/
  `xhigh`/`max`.
- Permission-mode input choices stay `acceptEdits`/`auto`/`bypassPermissions`/
  `manual`/`dontAsk`/`plan`, and the wire enumeration keeps its existing
  `default` spelling for `manual`. The selected `plan` value is unaffected.
- The normalized `init` record construction is identical at every hop once
  only the embedded version string, build timestamp, and build commit are
  blanked (`3a19423278f50a72bcbc4335a38d568f52e0f628fff7bae9d63f157838b5d187`).
  The init record still carries `type`, `subtype: "init"`, `cwd`,
  `session_id`, `tools`, `mcp_servers`, `model`, `permissionMode`,
  `slash_commands`, `api_key_source`, `betas`, and `claude_code_version`.
- The `stream_event` envelope, `hook_started`, and `result` construction
  sites are identical at every hop.
- 2.1.260 removes one duplicate `SessionsV2Client` remote-session path that
  re-emitted `thinking_tokens` from a payload. The print-route emitters and the
  `estimated_tokens`/`estimated_tokens_delta`/`session_id`/`uuid` record shape
  stay, so the response-only decoder still receives its required envelope.

The release bodies name no mapped change. The mapped-keyword bullets are
interactive, VSCode, cloud, Remote Control, gateway, hook, resume,
path-scoped permission-rule, plugin, telemetry, or sandbox surfaces, none of
which either selected route configures. `--permission-prompts none`,
`--system-prompt-snapshot`, `--forward-subagent-text`, `managedMcpServers`,
`maxEffortLevel`, `gatewayInternalNetworks`, and the background-session
commands stay unmapped. `maxEffortLevel` caps an effective session effort in a
settings file; it does not change the selected `--effort` argv shape or the
adapter's dispatch-only reasoning claim.

No new public operation, capability, or process authority is required.

## Decision

**Compatible-extension.**

- keep headless axis `claude-code.headless-stream-json`, baseline `2.1.220`,
  behavior revision `claude-code.headless.stream-json.v1`, and claim id
  `claude-code.headless.window-1`;
- keep response-only axis `claude-code.response-only-stream-json`, baseline
  `2.1.227`, behavior revision `claude-code.response-only.stream-json.v1`, and
  claim id `claude-code.response-only.window-1`;
- qualify the eleven published hops and raise both `latest_qualified` values to
  `2.1.270`;
- keep every historical gap and add the newly observed unpublished gaps
  `2.1.262` and `2.1.264`;
- keep `AllowUnverified`, watcher exact `2.1.251`, and every feature-specific
  exact-version set on the `2.1.220..=2.1.241` probed points; and
- use synthetic `2.1.271` as the later `UnverifiedNewer` point after claim.

This identity record changes no production claim. The g05.058 claim commit
applies the admitted segment.

## Sources

- npm registry: `https://registry.npmjs.org/@anthropic-ai/claude-code`
- npm tarballs: registry `dist.tarball` URLs for `2.1.257` through `2.1.270`
- GitHub releases: `https://github.com/anthropics/claude-code/releases`
- GitHub tags: `v2.1.257` through `v2.1.270`
- Frozen corpus:
  `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.270/`
