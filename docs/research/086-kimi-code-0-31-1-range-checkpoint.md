# 086 Kimi Code 0.31.1 Range Checkpoint

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

Does current stable Kimi Code `0.31.1` require a new Swallowtail contract,
route, or compatibility segment beyond qualified `0.31.0`?

## Method

The checkpoint compared the exact official `0.31.0` and `0.31.1` tags,
selected ACP, default headless, REST, WebSocket, catalogue, and session source,
the official release note, npm metadata, and the signed macOS arm64 release
artifact.

The exact `0.31.1` artifact received only `--version`, `acp --help`, `web
--help`, and one ACP `initialize` request. No installation, update,
authentication, session creation, provider prompt, workspace write, or durable
provider mutation ran.

## Exact Release Evidence

| Field | `0.31.1` value |
| --- | --- |
| annotated tag | `69f0400a504518d2d6665933c6a9b2beddd6398d` |
| commit | `6b56c11697771fe596099b38bafae539820309a4` |
| tree | `a4ea9a07cd0371eabbc4769065a148a204d63db0` |
| npm integrity | `sha512-Hyly4EjzemSjla479jC47h+K98wNvRKOqGwu6mBncI/MlIafqEByUXeGl/9+DsOKdiE6fQTxkxiAcgusBay56Q==` |
| macOS arm64 ZIP SHA-256 | `f6bd417babbce6db6222417451808011e318b7a80e5d0fb53592167874376704` |

The downloaded executable reported `0.31.1`. Its archive digest matched the
official release asset. Apple code-signature inspection identifies Beijing
Moonshot Technology Co., Ltd under team id `2J9472RW75`.

The installed executable remains `0.31.0`. Swallowtail did not replace or
modify it.

## Route Delta

### ACP

The ACP package remains `0.3.6` on ACP SDK `0.23.0`. The selected event mapper
is byte-identical. The only selected server delta is a source comment recording
that caller-supplied MCP servers still reach the v1 kernel, not experimental
v2.

The exact release artifact initialized with ACP protocol version `1`, agent
version `0.31.1`, the same selected capability keys, one authentication method,
and no stderr. The maintained declared-effort behavior remains unchanged.

### Default headless

The stream-JSON renderer remains blob
`0e2f35238db066a13b53ad2cfff11bdff2f76724`. The ordinary TUI change only
passes the already-computed engine selection into UI state. The substantial
print-runner refactor remains behind `KIMI_CODE_EXPERIMENTAL_FLAG`; Swallowtail
does not set that flag.

Default `-p --output-format stream-json` therefore keeps behavior
`kimi.headless.stream-json.v1`.

### Local server

Bearer middleware, REST metadata, prompt schema, WebSocket v2 control, and
model-catalogue schema are byte-identical. The selected local-server delta is
compatible but material:

- `turn.ended` may carry optional `interruptReason`
- session lookup moves through workspace-scoped handlers
- broadcaster lookup follows the consolidated live-session helper
- provider-model refresh no longer transiently clears the catalogue during
  startup

Swallowtail already ignores unknown optional event fields and maps the existing
terminal `reason`. Wire paths and required fields remain unchanged. The exact
release still needs a private local-server behavior milestone because catalogue
and session lookup behavior changed.

## Decision

Compile roadmap g03.013 and extend all three maintained Kimi Code routes
through `0.31.1`:

- ACP: retain `kimi.acp.reasoning.declared-effort-v2`
- headless: retain `kimi.headless.stream-json.v1`
- local server: add exact `0.31.1`
  `kimi.local-server.rest-ws-v2-refresh-stable`

Versions above `0.31.1` remain visible as unverified newer. Experimental v2
headless execution remains outside the guarantee.

## Contract Result

Contracts 011, 023, 029, 032, 037, 038, 042, and 044 already govern exact
interface identity, behavior milestones, visible newer-version attempts,
bounded provider events, managed recovery, and joined cleanup. No new
operation, capability, credential authority, session authority, transport, or
public lifecycle surface is required.

Standalone Claude and Gemini maintenance remains paused.

## Sources

- [Kimi Code `0.31.1` release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.31.1)
- [Kimi Code repository](https://github.com/MoonshotAI/kimi-code)
- [Kimi Code npm package](https://www.npmjs.com/package/@moonshot-ai/kimi-code)
- [Kimi Code changelog](https://moonshotai.github.io/kimi-code/en/release-notes/changelog.html)

