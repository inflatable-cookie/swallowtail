# Research 312: Cursor Agent 2026.09.10 Identity

Status: complete; identity evidence only. Production claim changes land in
the g05.062 claim batch after this record.

Observed 2026-09-14 on the `cursor-agent.release-date` axis:

- Host `cursor-agent --version`: `2026.08.04-aaa8809`; runtime index
  SHA-256 `65cb83494b6134b1b1c78139f24ac77d12943d3ba2e540d24e45eef17ee10bef`,
  ACP chunk `8869.index.js`
  `5c702c38a79ca379d19b6aaf7cf13545fc5b8d887842e90f02e3e942fb5da178`,
  headless chunk `5859.index.js`
  `9656406210421a9e9538135b6bdff5dc4a9ebaff1a7c0c15aa4f1821fd1487f1`.
  Every host digest equals the Research 135 frozen value. Host `--help`
  still documents `models`, hidden `acp`, `-p`/`--print`,
  `--output-format text|json|stream-json`, `--model`, `--trust`, and
  `--mode plan|ask`. Prompt-free host `acp` initialize still returns
  protocol v1, `cursor_login`, advertised load/list, image prompt, HTTP/SSE
  MCP, no `agentInfo`, zero stderr. The host was not installed, updated, or
  otherwise mutated.
- ACP registry Cursor entry: `2026.09.10`, archive
  `https://downloads.cursor.com/lab/2026.09.10-fd3934a/darwin/arm64/agent-cli-package.tar.gz`.
  Registry and official downloads agree; no newer stable exists.
- Published hops after the prior `2026.08.11-e8db854` ceiling are exactly
  `2026.08.31-4057e58`, `2026.09.02-c22c1a3`, and `2026.09.10-fd3934a`:
  every registry transition observed across checkpoints 271, 284, and 308,
  each corroborated by its official download. No publication index exists,
  so unobserved calendar dates stay unclaimed gaps; nothing is inferred.

## Official hop identities

| Version | Archive SHA-256 | Runtime index SHA-256 | Package files |
| --- | --- | --- | --- |
| `2026.08.11-e8db854` (ceiling) | `46044d6d…9841790` | `6aceb24b…1e5f0` | 434 |
| `2026.08.31-4057e58` | `a94483cf…7332fbb0` | `179d2050…06dca` | 404 |
| `2026.09.02-c22c1a3` | `3d814861…13ea51e7` | `9be0f8f8…45de5c` | 443 |
| `2026.09.10-fd3934a` | `aec0b01a…e0285423` | `230df535…e23468` | 443 |

The official `2026.09.10` archive is 174,178,929 bytes. Each extracted
runtime entry embeds its own exact `YYYY.MM.DD-revision` literal 16 times
and no other hop's literal. The recomputed `2026.08.11` digests match
Research 135 exactly.

The per-hop complete-tree inventory is frozen in the
[2026.09.10 dist inventory](../../crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-2026.09.10/dist-inventory.json).
Per-hop tree deltas are `+61/-91/~76`, `+97/-58/~90`, and `+59/-59/~78`.
`package.json` is byte-identical across all four hops; the launcher changes
once for a `HOME` guard around `NODE_COMPILE_CACHE` and is then stable.
Chunk filenames embed content hashes, so renames dominate added/removed
sets; 248 files are byte-identical through all four hops.

## Selected-surface classification

The selected CLI definitions (`models`, `acp`, `--print`,
`--output-format text|json|stream-json`, `--model`, `--trust`,
`--mode plan|ask`) are text-identical across all four hops modulo minifier
renames; no selected flag is added, removed, or respelled. The ACP
initialize construction (protocol version, `cursor_login`, load/list
advertisement, image prompt, HTTP/SSE MCP, no `agentInfo`, authenticated
session-create gate) is semantically identical at every hop, and the
load-history replay path keeps its text. The stream-json event keys
(`type`, `subtype`, `call_id`, `tool_call`, `model_call_id`, `session_id`,
`timestamp_ms`) are text-identical. The `2026.09.02` to `2026.09.10` ACP
chunk differs only by webpack chunk id, build-path embeds, and minified
names: a rebuild, not a behavior change.

New flags attach only to the unmapped worker/controller pool commands
(`--spawn`, `--pool`, `--warm-idle`, session hooks); the SEA/native
packaging refactor and the launcher guard feed no selected surface. Load
stays advertised without a proven replay; continuation recovery stays
blocked. Downloaded archives were hashed and never executed.

## Contract 029 decision

The family is a `compatible-extension` of
`cursor-agent.catalogue.calendar-release-v1`,
`cursor-agent.acp-v1.interactive-v1`, and
`cursor-agent.stream-json.structured-v1`. Keep the baseline
`2026.07.01-41b2de7` and all three claim ids. Qualify all three published
hops and raise the ceiling to `2026.09.10-fd3934a`. Dates between the seven
exact points stay incompatible. Feature-specific exact sets (Ask,
high-effort, model parameters) stay on their probed points. No provider
prompt, live session, login, installation, host update, or credential was
used.
