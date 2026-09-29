# 365 Claude Code 2.1.283 Identity

Status: promoted
Owner: operator-authorized family qualification
Date: 2026-09-28

## Question

Is official npm/GitHub Claude Code `2.1.283` a compatible extension of the
qualified headless `2.1.220..=2.1.281` window and the response-only v2 segment
`2.1.280..=2.1.281`, a private milestone, a new facade, or a stop? Headless
and response-only stay one family. Claude Agent ACP and the Claude Agent SDK
sidecar stay separate.

## Remaining Rank

This run covers only Claude Code headless and response-only. At observation
the family was AllowUnverified official-newer: Research 348 left both ceilings
at `2.1.281` while official npm `latest` had moved to `2.1.283`.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `claude-code.headless-stream-json` | not installed | npm and GitHub `2.1.283` | qualified `2.1.220..=2.1.281`; AllowUnverified | official-newer |
| `claude-code.response-only-stream-json` | not installed | npm and GitHub `2.1.283` | v1 `2.1.227..=2.1.278`, v2 `2.1.280..=2.1.281`; AllowUnverified | official-newer |

Gemini stays deferred. Claude Agent ACP `0.81.2` and the SDK sidecar were not
reopened.

## Method

Re-probed npm `@anthropic-ai/claude-code@latest` and the GitHub latest
release, then retrieved the npm wrapper and the `darwin-arm64` and `linux-x64`
platform tarballs for the previous ceiling `2.1.281` and published hops
`2.1.282` and `2.1.283` into `/tmp/cc-283`. Every tarball was SHA-256 hashed.
The `2.1.281` wrapper tarball and both platform binaries reproduce Research
341. npm version metadata proves `2.1.282` and `2.1.283` published and
`2.1.279` and `2.1.284` unpublished. GitHub tags `v2.1.281` through `v2.1.283`
resolve to lightweight commits. Tag `v2.1.284` is absent.

Downloaded official binaries were never executed. Mapped-surface evidence was
recovered from the embedded `// @bun @bytecode` chunks each platform binary
carries, on both platforms and all three compared versions:

- selected option constructor sites and `.choices(...)` arrays for input
  format, output format, effort, and permission mode
- the safe-mode plugin filter and its `@builtin` sentinel
- `agents-md` default mode, `isOnByDefault`, hook events, and the built-in
  registration set
- stream-JSON init schema keys
- the telemetry opt-out parser

Wrapper and platform file inventories were built with per-file SHA-256
digests and frozen in `dist-inventory.json`.

Host `claude` was not on `PATH`. Missing host install is not a gap and was
not installed. Changelog bodies were read as discovery only.

No provider prompt, login, credential, live session, host update, or
downloaded-binary execution occurred. Official latest was re-probed before
this identity freeze: still `2.1.283`.

## Identity

npm `latest` and GitHub latest agree on `2.1.283`. npm `stable` is `2.1.274`
and is not this family's channel. npm `next` equals `latest` and is ignored.
npm published `2.1.283` at 2026-09-25T18:46:11.330Z with wrapper integrity
`sha512-/8Y1pe7M15qMOU7RwUEjpFcM8XGVXNzWrWRXXp/0HlGm1k8FcOCxYMA9VR237JUUzzx5DkcQaGvELAo4si7TwA==`
and tarball SHA-256
`e9d3ee7e3c007c2e7c435ee8a64213a65f0341ae80abc51c1e9fde2614c351cb`. GitHub
published `v2.1.283` at 2026-09-25T21:50:12Z with lightweight tag commit
`7779afb12e3635f46f56ec823979d68350ae000b`.

| Version | npm published | GitHub tag commit | darwin-arm64 SHA-256 | linux-x64 SHA-256 |
| --- | --- | --- | --- | --- |
| `2.1.281` | 2026-09-23T17:01:17.780Z | `d78be9481b889e11186ec4578b4f5e9301396e25` | `a922981f6f3b55a251ef9f9dbaa0621a5f99cbcb5ca67f8a797476ccfc83f626` | `56fe3da88458465fb27d7e9299dddb3fead55750fb9c2de795f233b5eea6dce1` |
| `2.1.282` | 2026-09-24T15:56:22.706Z | `ddcb43a29b2d4da61fd22a55f6030c6d376b0545` | `fcfd837103965c64de34a6b9b94370d77a347ea71819715a27d5f0ef01775ea4` | `3afe8535c0cc33f0e24f7b25dab7a1727b8b592196f8496a8bc302ba2161eed3` |
| `2.1.283` | 2026-09-25T18:46:11.330Z | `7779afb12e3635f46f56ec823979d68350ae000b` | `d8cb1e5c79684cc12a8bfc813e3a2073406921b6245744b3009be3ab5651d21e` | `1859583ce32920595c61ef868bee52e1b1594f7486db209935e01f1e5e804ae2` |

Published stables after the previous ceiling are exactly `2.1.282` and
`2.1.283`. No new unpublished interior gap appears between them. Existing
unpublished gaps `2.1.244`, `2.1.249`, `2.1.253` through `2.1.256`,
`2.1.262`, `2.1.264`, and `2.1.279` stay incompatible. First unpublished
later stable: `2.1.284`.

Host `claude` is not installed. That is observation input only.

## Selected Protocol

The npm package remains an installer wrapper. Wrapper file count stays 7 and
each platform package file count stays 4. Across `2.1.281` through `2.1.283`
the wrapper changes only `package.json` (version pin and
`optionalDependencies`). `sdk-tools.d.ts` is byte-identical
(`b33fdb2cf0af6559a111ae602ab259767883510456492907d2cdf4eec794b417`).
Neither route consumes that file. Each platform hop changes `claude` and
`package.json` and leaves `LICENSE.md` and `README.md` identical.

The selected mapped subset is unchanged on both platform builds:

- selected option help and choice arrays are equal at `2.1.281`, `2.1.282`,
  and `2.1.283`: output `text` / `json` / `stream-json`; input `text` /
  `stream-json`; effort `low` / `medium` / `high` / `xhigh` / `max`;
  permission wire `acceptEdits` / `auto` / `bypassPermissions` / `default` /
  `dontAsk` / `plan`
- the safe-mode filter still keeps plugins whose source ends in `@builtin`
- `agents-md` stays default mode `claude-md-or-agents-md` with
  `isOnByDefault` true and hook events `session.start`, `prompt.context`,
  `agent.spawn`, `tool.call`
- the nine built-in registration names and gates stay
- stream-JSON init keys stay, including optional `mcp_servers[].source` and
  `terminal_slash_commands`
- the telemetry opt-out parser stays and remains unpinned because
  response-only `EnvironmentRef` is opaque
- response-only selected argv stays empty tools, `--safe-mode`, disabled
  slash commands and Chrome, no prompt suggestions, and empty strict MCP
- response-only v2 still launches in an adapter-owned empty temporary
  directory when no project location is supplied
- headless still passes `--permission-mode plan` and does not pass
  `--safe-mode`

Changelog notes that do not move the selected surface stay unmapped:
optional `path` on init `plugin_errors` for `--plugin-dir` failures;
`claude -p` skipping the interactive UI; the interactive auto-mode default
for third-party providers or telemetry-off sessions; combining
`--system-prompt` with its file form; and the revert of the `2.1.282`
`claude-ai` name reservation. `--client-data-url` is new at `2.1.283` only
and is not selected. `--bare` was already present at `2.1.281` and stays
unmapped.

## Decision

Compatible extension. No identity stop.

- Headless: keep baseline `2.1.220`, claim id
  `claude-code.headless.window-1`, behavior
  `claude-code.headless.stream-json.v1`, and `AllowUnverified`. Raise the
  ceiling from `2.1.281` to `2.1.283`. Qualify `2.1.282`.
- Response-only: keep claim id `claude-code.response-only.window-1`. v1
  stays `2.1.227..=2.1.278` on `stream-json.v1`. Extend v2 from
  `2.1.280..=2.1.281` to `2.1.280..=2.1.283` on `stream-json.v2`.
- Unpublished gaps stay denied. Synthetic later stable is `2.1.284`
  (`UnverifiedNewer`, v2 behavior on response-only).
- Watcher stays exact `2.1.251`. Maximum-turns stays on the Research 226
  probed set.
- Do not flatten onto Claude Agent ACP or the Claude Agent SDK sidecar.

## Sources

- [Research 341](./341-claude-code-2-1-281-builtin-hooks-and-qualification-stop.md)
- [Research 348](./348-claude-code-2-1-281-narrowed-response-only-claim.md)
- Frozen corpus: `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.283/`
- npm `@anthropic-ai/claude-code` `latest` `2.1.283` (2026-09-28)
- [GitHub v2.1.282](https://github.com/anthropics/claude-code/releases/tag/v2.1.282)
- [GitHub v2.1.283](https://github.com/anthropics/claude-code/releases/tag/v2.1.283)
