# 331 Claude Code 2.1.278 Identity

Status: promoted
Owner: Tom
Date: 2026-09-21
Task: g06.009

## Question

Is official npm/GitHub Claude Code `2.1.278` a compatible extension of the
qualified headless `2.1.220..=2.1.270` and response-only `2.1.227..=2.1.270`
windows, a private milestone, a new facade, or a stop? Headless and
response-only stay one family. Watcher help, digest, and live authorization
stay on exact `2.1.251`.

## Remaining Rank

This run covers only Claude Code. At observation time the family was
AllowUnverified official-newer: Research 307 / g05.058 left the qualified
ceiling at `2.1.270` while official stable had moved to `2.1.278`.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `claude-code.headless-stream-json` | not installed | npm and GitHub `2.1.278` | qualified `2.1.220..=2.1.270`; AllowUnverified | official-newer |
| `claude-code.response-only-stream-json` | not installed | npm and GitHub `2.1.278` | qualified `2.1.227..=2.1.270`; AllowUnverified | official-newer |

Gemini remains deferred. Claude Agent ACP, the Claude Agent SDK native
`2.1.270` pin, and the watcher family are separate surfaces and were not
reopened.

## Method

Re-probed npm `@anthropic-ai/claude-code@latest` and the GitHub latest
release, then retrieved the npm wrapper and the `darwin-arm64` and `linux-x64`
platform tarballs for the previous ceiling `2.1.270` and all eight published
hops into `/tmp/g06-009`. Every tarball was SHA-256 hashed; the `2.1.270`
values reproduce Research 307. npm version metadata proves `2.1.271` through
`2.1.278` were published and `2.1.279` was unpublished. Every GitHub tag
`v2.1.271` through `v2.1.278` resolves to a lightweight commit.

Downloaded official binaries were never executed. Because the published
`claude` executable is a compiled artifact rather than readable shipped source,
mapped-surface evidence was recovered from the embedded `// @bun @bytecode`
source chunks the binary carries. For both platform builds and all nine
compared versions:

- extracted every `.option(` / `addOption` constructor site and classified
  the selected option definitions for `-p`/`--print`, `--input-format`,
  `--output-format`, `--verbose`, `--no-session-persistence`, `--model`,
  `--effort`, `--permission-mode`, `--tools`, `--setting-sources`,
  `--mcp-config`, `--strict-mcp-config`, `--max-turns`, `--safe-mode`,
  `--disable-slash-commands`, `--no-chrome`, and `--prompt-suggestions` by
  constructor-site string-literal fingerprints;
- resolved each `.choices(...)` argument for input format, output format,
  effort, and permission mode, including the `default` → `manual` input alias
  and the separate wire enumeration;
- recovered the `init` required keys plus `stream_event`, `hook_started`,
  `result`, and `thinking_tokens` construction presence; and
- built hop-by-hop wrapper and platform-package file inventories with exact
  SHA-256 digests.

Host `claude` was not on `PATH`. Missing host install is not a gap and was
not installed, updated, or replaced. Changelog and release bodies were read
as discovery only.

No provider prompt, login, credential, install, host update, downloaded
binary execution, watcher live work, release, or consumer change occurred.

## Identity

npm and GitHub latest agree on `2.1.278`. npm published it at
2026-09-19T01:48:59.758Z with wrapper integrity
`sha512-mfNRqC0GaEXqmP97NiwJBeYBmRuqe2VzgLUreUUaEhyJxJWx2Z6ClW1tBOncGNNXdhj6EY4LPvUIWP+oq311CA==`
and tarball SHA-256
`08c6dfcf3dafcfd30e09b2926c596e274f0fa20844a5801ada7f1c8e6227157e`. GitHub
published `v2.1.278` at 2026-09-19T03:10:40Z with lightweight tag commit
`bf7d404e26a5fb6167d21b46c93a2bf6c22ab274`.

| Version | npm published | GitHub tag commit | darwin-arm64 SHA-256 | linux-x64 SHA-256 |
| --- | --- | --- | --- | --- |
| `2.1.270` | 2026-09-12T18:52:44.937Z | `2b40e76d3f03b9070e2431e0bd05b4f3ace77982` | `a506b6d970a4cf44f6abdb53a81ddcd5d3b0ce042a95c502fe9d1f946bdb8807` | `3a624a5a7cd79bbad4d32bd7db36f1197ecf458bc5bf1e2aed81834a01ad3ef0` |
| `2.1.271` | 2026-09-14T19:45:19.456Z | `f2ccbe279409fc6357c2fb1bfd7e195e0e1bcfca` | `87d119eb46782a1369d79d6e8cc00557f1b51bc9db7a51bd01fa2f909d8fe3dc` | `5e7b6fc24d0e124f68e99a0641c47b662945c6d197d768d7ad7a72cbf1897f58` |
| `2.1.272` | 2026-09-14T23:34:13.565Z | `f96c3b49c4c8721685206aaab23609b2d399df4e` | `195e24e8e1f9bf46f1eaee72d434a33e18f9f5796f29a6348a00d16c5f8aee75` | `d81396a668eb76fbddb49a2a5841f1b5d7af96b4c1f6500ced92f2c988f5bcd4` |
| `2.1.273` | 2026-09-15T18:06:34.098Z | `aad35ba32864f3f7cdaf1a08bb2e400c6e93358b` | `953e9880dbcb0b70f31c1f508de6a3fd389753d131688557fd992da9184693fb` | `6c752e2cc7c110c9df15f26d8d134d438c5ae95dbd610efc1a308bf7f9c5f6c1` |
| `2.1.274` | 2026-09-16T22:36:09.883Z | `68ac8bbf0245b615b41517bf8f2b2f35af1ae31d` | `3509913f9d1576316c8845b88837f8fd3bbbcf26625833ac82cfb6b8985da94a` | `15e2d05148f801b5774032faad87e624ecd172e9903288bda448b892eb58fa07` |
| `2.1.275` | 2026-09-17T20:20:31.805Z | `38035964a79edd8995a556e2027f891a826e533c` | `1b8177fe49f2be5bacc75e89b5f88fa7454791283113ace16a453fe9171d179b` | `13586f3150a7ca1655f36e1dba759fb404e0f7cf7021d4a3dcd5e6f604e56156` |
| `2.1.276` | 2026-09-18T01:39:31.986Z | `31a3b00bef145a0393d9dbf840a98674fec07712` | `9de364db11a410d53cbbb0f6b1f18c66c90053efc9a63370072856d10db66329` | `8a56c8a14bd3cb246e2bdb7e60aefe0f609bff78c8bbcc5ea6b1817c111c6145` |
| `2.1.277` | 2026-09-18T16:22:26.548Z | `ca02e7deeb0707f558b0afd7e9e5d67a382e12b3` | `73d6a2a55c46907e49bd8bb7608e134333bd71173351ee16ddce7d7db9914b9c` | `722210f05ba494d8f6df69423c4d4f2960900f7a007d0532851c7a36e375cab7` |
| `2.1.278` | 2026-09-19T01:48:59.758Z | `bf7d404e26a5fb6167d21b46c93a2bf6c22ab274` | `bd245662fb8a0e321b3bf133e930371d6563c387527885f30b2613aef3ba14d6` | `5c4735937844e84f8a93306e841a5b0e12252909b07870f789b190468da147ab` |

The `2.1.270` wrapper tarball SHA-256
`ee37736066e3349c977db70bdfaf4f3cf7c398a06e2a051fe8e00728e1f6e932` and both
platform binaries reproduce Research 307 exactly.

Published stables after the previous ceiling are exactly the eight hops
above. Existing unpublished gaps `2.1.244`, `2.1.249`, `2.1.253` through
`2.1.256`, `2.1.262`, and `2.1.264` stay incompatible. First unpublished
later stable at observation: `2.1.279`.

Host `claude` is not installed. That is observation input only; it is not a
qualification or install authority.

## Selected Protocol

The npm package remains an installer wrapper. Wrapper file count stays 7 and
platform package file count stays 4 at every hop. `LICENSE.md`, `README.md`,
`bin/claude.exe`, `cli-wrapper.cjs`, and `install.cjs` are byte-identical
across all nine compared versions. `package.json` changes only its version
pin and `optionalDependencies` platform packages. `sdk-tools.d.ts` changes at
five of the eight hops; every delta is SDK tool declaration content
(Monitor deadline wording and removal of `persistent`, ArtifactPublish
list/read_asset wording, additive `outside_writer`/`public_read`,
icon/favicon/external/role/written/asset_uploads metadata, and removal of
deprecated TaskOutput and REPL). Neither route consumes that file.

The selected mapped surface is unchanged on both platform builds across every
hop:

- Every selected flag keeps an identical constructor-site fingerprint
  intersection at every hop, covering flag name, value placeholder, help
  description, and choices.
- `--verbose`, `--model`, and `--effort` each carry one additional hop-local
  fingerprint from bunfs chunk remangle or minified template ids
  (`${Mc.join` → `${Zc.join` → `${cu.join`). Those remangles do not change
  selected help or choices.
- Input format choices stay `text`/`stream-json`; output format choices stay
  `text`/`json`/`stream-json`; effort choices stay `low`/`medium`/`high`/
  `xhigh`/`max`.
- Permission-mode input choices stay `acceptEdits`/`auto`/`bypassPermissions`/
  `manual`/`dontAsk`/`plan`, and the wire enumeration keeps its existing
  `default` spelling for `manual`. The selected `plan` value is unaffected.
- Init required keys stay `type`, `subtype`, `cwd`, `session_id`, `tools`,
  `mcp_servers`, `model`, `permissionMode`, `slash_commands`, and
  `claude_code_version`. From `2.1.274` an optional `mcp_servers[].source`
  field is additive. The response-only decoder requires `mcp_servers` to be
  an array and does not fail on extra keys of an empty array; headless init
  does not parse `mcp_servers`.
- `stream_event`, `hook_started`, `result`, and `thinking_tokens`
  construction sites remain present at every hop.

Changelog mapped-keyword bullets that touch print/stream-JSON are discovery
only and stay compatible or unmapped: `2.1.273` stream-json subagent/
background drop fix (already-mapped envelope reliability); `2.1.274` MCP
startup wait and empty `--strict-mcp-config`+`--mcp-config` wait (timing;
selected routes already use that combo and selected input is text);
`2.1.275` `--forward-subagent-text` (unmapped); `2.1.277` `claude -p`
hang→error+exit 1 (failure-path reliability, still provider failure);
`2.1.278` auto-mode classifier (selected routes use `plan` / tools `""`).

`--permission-prompts none`, `--system-prompt-snapshot`,
`--forward-subagent-text`, `managedMcpServers`, `maxEffortLevel`,
`gatewayInternalNetworks`, and the background-session commands stay
unmapped.

No new public operation, capability, or process authority is required.

## Decision

**Compatible-extension.**

- keep headless axis `claude-code.headless-stream-json`, baseline `2.1.220`,
  behavior revision `claude-code.headless.stream-json.v1`, and claim id
  `claude-code.headless.window-1`;
- keep response-only axis `claude-code.response-only-stream-json`, baseline
  `2.1.227`, behavior revision `claude-code.response-only.stream-json.v1`, and
  claim id `claude-code.response-only.window-1`;
- qualify the eight published hops and raise both `latest_qualified` values to
  `2.1.278`;
- keep every historical unpublished gap;
- keep `AllowUnverified`, watcher exact `2.1.251`, and every feature-specific
  exact-version set on the `2.1.220..=2.1.241` probed points; and
- use synthetic `2.1.279` as the later `UnverifiedNewer` point after claim.

This identity record changes no production claim. The g06.009 claim commit
applies the admitted segment.

## Sources

- npm registry: `https://registry.npmjs.org/@anthropic-ai/claude-code`
- npm tarballs: registry `dist.tarball` URLs for `2.1.270` through `2.1.278`
- GitHub releases: `https://github.com/anthropics/claude-code/releases`
- GitHub tags: `v2.1.270` through `v2.1.278`
- Frozen corpus:
  `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.278/`
