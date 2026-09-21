# 331 Oh My Pi 18.2.7 Compatible-Extension Identity

Status: promoted
Owner: Tom
Date: 2026-09-21
Authority: Contracts 017, 023, and 029; Research 327 and the version-currentness
skill; the Oh My Pi selection, prepared profiles, drivers, protocol decoder,
and frozen fixtures; and the official npm, GitHub, and host channels.

## Question

Is the Oh My Pi RPC package window a compatible extension of the existing
`18.0.0..=18.1.22` claim through every published `18.2.x` stable to official
`18.2.7`, a private milestone, a new driver/facade, or a stop?

## Remaining AllowUnverified rank

Named family only. Extracurricular currentness; not a generation rank rewrite.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Oh My Pi RPC (`oh-my-pi.package`) | not on `PATH` | `18.0.0..=18.1.22` | operator-named family; official npm and GitHub stable is `18.2.7` |

Gemini stays deferred. Pi stays a separate family:
`@earendil-works/pi-coding-agent` is `0.86.1` and `pi.package` and
`oh-my-pi.package` never substitute for each other.

## Method

Re-probed npm `@oh-my-pi/pi-coding-agent` `dist-tag latest` and the GitHub
latest release and tag on 2026-09-21. npm `latest` is `18.2.7`, published
`2026-09-21T03:12:39.903Z`. GitHub latest release is `v18.2.7`, published
`2026-09-21T02:13:02Z`, tag commit `d716bcf60ab0a2e7ece1fdf382c0d143fef1f307`.
`omp` is not on `PATH`; missing install is not a gap. The host was not
installed, updated, replaced, or executed.

Retrieved every published npm stable from the previous ceiling `18.1.22`
through `18.2.7` into `/tmp` and reproduced the registry `integrity` and
`shasum` for all nine tarballs before extraction. Downloaded artifacts were
never executed. The registry record carries no source commit (`gitHead` is
null). Extracted the complete shipped `package/` tree for each point and built
one deterministic path-and-SHA-256 inventory per hop.

Read `CHANGELOG.md` as discovery only, then inspected the shipped source that
contains or feeds the selected `--mode rpc` route: `src/modes/rpc/**`,
`src/jsonrpc/message-framing.ts`, `src/cli/flag-tables.ts`, and
`src/modes/index.ts`. Compared `docs/rpc.md` git blobs at each tag through the
GitHub contents API.

Reproduced Research 327's `18.1.22` identity exactly: npm integrity and
shasum, tarball `6eac4319763089c6fab2fd5c60469bd7697988cb4134367f89db518a31ae80ac`,
`dist/cli.js` `b8bfd4f19a36b8bcae884be7ebed68133397868c8461167f3759acf1630a2499`
at size `22437945`, file count `3156`, GitHub `23a5b9ae38864d3f785dc6cbc96eb6d674a1d32d`.

No provider prompt, login, credential, catalogue call, live session,
installation, host update, downloaded-artifact execution, release, tag,
publication, or consumer mutation occurred.

## Identity

npm `latest` and the GitHub latest release/tag agree on `18.2.7`. The 9-row
published ledger is frozen in
`tests/fixtures/oh-my-pi-18.2.7/identity.json`; the complete per-hop shipped
tree delta is in `dist-inventory.json`.

Published npm stables after the previous ceiling `18.1.22` are exactly
`18.2.0`, `18.2.1`, `18.2.2`, `18.2.3`, `18.2.4`, `18.2.5`, `18.2.6`, and
`18.2.7`. No `18.1.23` npm package or GitHub tag exists. No GitHub-only
`18.2.x` tags exist. `18.2.8` is the synthetic later-stable point. Not a Pi
package point.

Shipped file counts: `3156` → `3174` → `3186` → `3189` → `3194` → `3196` →
`2676` → `2676` → `2700`. The `18.2.5` drop is the TUI extraction into
`@oh-my-pi/pi-tui`. 1683 files are byte-identical across all nine compared
versions (digest `56c3d8deb001a043cb4c11616e679e06831db6f6d5193787eb7ed45cffc8de74`).

## Selected protocol

Selected Swallowtail argv is unchanged: `--mode rpc`, `--no-session`,
`--provider`, `--model`, `--tools read,grep,glob,todo,ask`, `--no-extensions`,
`--no-skills`, `--no-rules`, `--no-prewalk`, `--approval-mode always-ask`, and
the catalogue path `--no-tools`.

Selected RPC commands are unchanged. Wire invariants are unchanged: ready
`protocolVersion` 1 with `supportedProtocolVersions` `[1, 2]`, `maxFrameBytes`
1048576, `maxReassembledFrameBytes` 67108864, protocol 2, strict-LF JSONL,
`rpc_chunk` reassembly, `response` shape, `agent_end` `isTerminal` terminal
semantics, assistant `message_end` `stopReason: "error"` provider failure,
usage fields, tool fields, and the mapped UI methods.

`docs/rpc.md` is blob `310b44702de66b4eecd6da37660f9fc40075d973` at `v18.1.22`
and `v18.2.0`. `v18.2.1` (`b3b5d8aa`) documents stdout drain after stdin EOF
and the private backpressure spool. `v18.2.3` (`b91b20a8`, through `v18.2.7`)
documents that `login` rejects `secret: true` prompts. Neither change adds a
selected command, frame field, or required host operation.

## Selected-source ledger

`rpc-frame.ts`, `rpc-input.ts`, `rpc-messages.ts`, and `host-uris.ts` are
byte-identical across all nine compared versions. `message-framing.ts` is
Content-Length JSON-RPC used only by LSP/DAP; `--mode rpc` uses `rpc-frame.ts`.

| Hop | Verdict |
| --- | --- |
| `18.1.22→18.2.0` | unmapped: skill-invocation `prompt` field; `--no-skills` |
| `18.2.0→18.2.1` | unmapped: `rpc-output.ts` stdout spool; persistence `notice` (bounded unknown); optional `readsSkillUris`; LSP/DAP framing rewrite |
| `18.2.1→18.2.2` | unmapped: `RpcClient.handoff` comment only |
| `18.2.2→18.2.3` | unmapped: login rejects secret prompts; login is unselected |
| `18.2.3→18.2.4` | no mapped or selected-support file change |
| `18.2.4→18.2.5` | unmapped: import paths move to `@oh-my-pi/pi-tui` |
| `18.2.5→18.2.6` | no mapped or selected-support file change |
| `18.2.6→18.2.7` | unmapped: `--system-prompt-template` argv extra; selected flags stay |

## Decision

**Compatible-extension.** Raise the existing adapter-private `18.x` segment
from `18.0.0..=18.1.22` to `18.0.0..=18.2.7` on the unchanged
`oh-my-pi.rpc-v2-v18.0.0` behavior revision. Qualify published intermediates
`18.2.0` through `18.2.6`. Keep claim id `oh-my-pi.rpc.package-window-2`,
retained deprecated `17.2.9..=17.4.2`, exclusions `18.0.2`/`18.1.7`,
`AllowUnverified`, and the `oh-my-pi-rpc-17.2.9` decoder corpus.
`18.2.8` stays permitted `UnverifiedNewer`.

Not a major-line reset (still `18.x`). Not a new public operation. Not a new
driver or facade. Not a flatten onto `pi.package` or `pi.sdk-sidecar`.

This record edits no production claim.

## Sources

- npm `@oh-my-pi/pi-coding-agent` `18.1.22` through `18.2.7`
- [GitHub `v18.2.7`](https://github.com/can1357/oh-my-pi/releases/tag/v18.2.7)
- `CHANGELOG.md` and `docs/rpc.md` at every tag `v18.1.22` through `v18.2.7`
- frozen `crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.2.7/`
  and the historical `oh-my-pi-18.1.22/` corpus
- [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
- [Research 327](./327-oh-my-pi-18-1-22-identity.md)
