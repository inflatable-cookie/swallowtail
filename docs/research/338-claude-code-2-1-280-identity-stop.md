# 338 Claude Code 2.1.280 Identity Stop

Status: promoted; identity evidence only. This record lands a stop and does
not change the production claim.

Date: 2026-09-23
Task: extracurricular Contract 029. No generation card, Next Task edit, or
handoff.

## Question

Is official npm `latest` / GitHub Claude Code `2.1.280` a compatible
extension of the qualified headless `2.1.220..=2.1.278` and response-only
`2.1.227..=2.1.278` windows, a private milestone, a new facade, or a stop?
Headless and response-only stay one family. Watcher help, digest, and live
authorization stay on exact `2.1.251`.

## Remaining Rank

This run covers only Claude Code. At observation the family was
AllowUnverified official-newer: Research 331 left the qualified ceiling at
`2.1.278` while npm `latest` and GitHub `v2.1.280` had moved to `2.1.280`.
The npm `stable` dist-tag was still `2.1.267` and is not this family's
channel.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `claude-code.headless-stream-json` | not installed | npm and GitHub `2.1.280` | qualified `2.1.220..=2.1.278`; AllowUnverified | identity stop; ceiling unchanged |
| `claude-code.response-only-stream-json` | not installed | npm and GitHub `2.1.280` | qualified `2.1.227..=2.1.278`; AllowUnverified | identity stop; ceiling unchanged |

Gemini remains deferred. Claude Agent ACP and the Claude Agent SDK exact pin
were not reopened.

## Method

Re-probed npm `@anthropic-ai/claude-code` dist-tags and the GitHub latest
release, then retrieved the npm wrapper and the `darwin-arm64` and
`linux-x64` platform tarballs for the previous ceiling `2.1.278` and official
`2.1.280` into `/tmp/cc-280`. Every tarball was SHA-256 hashed. The `2.1.278`
wrapper tarball, both platform tarballs, and both platform binaries reproduce
Research 331. npm version metadata and GitHub tag lookup prove `2.1.279` and
`2.1.281` are unpublished and `2.1.280` is published. The GitHub tag
`v2.1.280` is a lightweight commit.

Downloaded official binaries were never executed. Mapped-surface evidence was
recovered from the embedded source the binaries carry. For both platform
builds and both compared versions:

- extracted every `.option(` / `.addOption(` constructor and compared the
  selected flag's string literals after minified-identifier normalization;
- resolved input-format, output-format, effort, and permission-mode choice
  arrays, including the `manual` → `default` wire alias; and
- compared the `init` object construction and the safe-mode plugin-hook
  registration control flow.

Host `claude` was not on `PATH`. Missing host install is not a gap and was
not installed, updated, or replaced. The GitHub release body was read as
discovery only. It does not mention `--safe-mode`.

No provider prompt, login, credential, install, host update, downloaded
binary execution, watcher live work, release, or consumer change occurred.

## Identity

npm `latest` and GitHub latest agree on `2.1.280`. npm `stable` stays
`2.1.267`. npm published `2.1.280` at 2026-09-22T15:44:39.443Z with wrapper
integrity
`sha512-EZlX8jqNf+e7q9v+UoPbLYAbEGth7aDbcTytHzPYYohbP/fCfrjboCbcv85ZYGEq1Rq7Amm8hXLhuCKxLsabwA==`
and tarball SHA-256
`1326e6b8cf00404fc3f9bd101d806b3fdec9588264e5a1aa8d98d1f7afe50170`. GitHub
published `v2.1.280` at 2026-09-22T16:38:14Z with lightweight tag commit
`56f36532530f88b572854538d685fcf781141e8c`.

| Version | npm published | GitHub tag commit | darwin-arm64 SHA-256 | linux-x64 SHA-256 |
| --- | --- | --- | --- | --- |
| `2.1.278` | 2026-09-19T01:48:59.758Z | `bf7d404e26a5fb6167d21b46c93a2bf6c22ab274` | `bd245662fb8a0e321b3bf133e930371d6563c387527885f30b2613aef3ba14d6` | `5c4735937844e84f8a93306e841a5b0e12252909b07870f789b190468da147ab` |
| `2.1.280` | 2026-09-22T15:44:39.443Z | `56f36532530f88b572854538d685fcf781141e8c` | `387a5c5dcdbb815085edf0baf79591f9d8894efe922bceaf3d75b1b08055229d` | `1e08503dbdf3c2cb0d706d32f3408277388d1c76ef108673e8fe42c1b322925b` |

The `2.1.278` digests reproduce Research 331. Published stables after the
qualified ceiling are exactly `2.1.280`. `2.1.279` is unpublished between
them. Existing unpublished gaps `2.1.244`, `2.1.249`, `2.1.253` through
`2.1.256`, `2.1.262`, and `2.1.264` stay incompatible. First unpublished
stable after official latest: `2.1.281`.

Host `claude` is not installed. That is observation input only.

## Selected Protocol

The npm package remains an installer wrapper. Wrapper file count stays 7 and
platform package file count stays 4. `LICENSE.md`, `README.md`,
`bin/claude.exe`, `cli-wrapper.cjs`, `install.cjs`, and `sdk-tools.d.ts` are
byte-identical across `2.1.278` and `2.1.280`. `package.json` changes only
its version pin and `optionalDependencies` platform pins. Neither route
consumes `sdk-tools.d.ts`.

On both platform builds, selected flag names, value placeholders, and choices
stay. Constructor identifiers remangle (`new Y` → `new J` on linux-x64).
Input choices stay `text` / `stream-json`. Output choices stay `text` /
`json` / `stream-json`. Effort choices stay `low` / `medium` / `high` /
`xhigh` / `max`. Permission-mode wire values stay `acceptEdits` / `auto` /
`bypassPermissions` / `default` / `dontAsk` / `plan`, and `manual` still
maps to `default`.

The init object keeps `type`, `subtype`, `cwd`, `session_id`, `tools`,
`mcp_servers` (optional `source` already present at `2.1.278`), `model`,
`permissionMode`, `slash_commands`, and `claude_code_version`.
`stream_event`, `hook_started`, `result`, and `thinking_tokens` sites remain.

`--safe-mode` does not stay. The response-only route passes it. Headless does
not.

- `2.1.278` registration returns immediately:
  `if(kr()){t("Safe mode: skipping plugin hook registration");return}`.
- `2.1.280` linux-x64 keeps builtin sources and continues:
  `e.filter((n)=>jO(n.source))` where `jO` is `e.endsWith("@builtin")`.
- `2.1.280` darwin-arm64 does the same with `XO` and `iu="builtin"`.
- The help sentence changes from disabling `plugins` (built-in tools still
  work) to disabling `installed plugins` (built-in tools and plugins still
  work).

That is a selected-surface change on a flag the response-only route passes.
The family is not split, and neither ceiling moves.

## Decision

**Stop.** Do not claim `2.1.280`.

- keep both baselines, both claim ids, both behavior revisions, and
  `AllowUnverified`;
- leave headless `latest_qualified` at `2.1.278` and response-only
  `latest_qualified` at `2.1.278`;
- leave `2.1.279` as the existing synthetic later `UnverifiedNewer` point;
- leave `2.1.280` as `UnverifiedNewer` under `AllowUnverified`;
- keep every historical unpublished gap, watcher exact `2.1.251`, and every
  feature-specific exact version set; and
- do not flatten onto Claude Agent ACP or the Claude Agent SDK pin.

An operator ruling is required before any claim edit. Qualifying `2.1.280`
needs a decision on whether response-only isolation still holds once safe
mode loads `@builtin` plugin hooks, or a private milestone that maps that
change. This record does not make that decision.

## Sources

- npm registry: `https://registry.npmjs.org/@anthropic-ai/claude-code`
- npm tarballs: registry `dist.tarball` URLs for `2.1.278` and `2.1.280`,
  plus `@anthropic-ai/claude-code-darwin-arm64` and
  `@anthropic-ai/claude-code-linux-x64` at those versions
- GitHub release: `https://github.com/anthropics/claude-code/releases/tag/v2.1.280`
- GitHub tags: `v2.1.278`, `v2.1.280`; `v2.1.279` and `v2.1.281` absent
- Frozen corpus:
  `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.280/`
