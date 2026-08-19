# 144 Primary Wave Source And Route Gate

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 260

## Question

After Research 143 selected the primary wave, which official sources, route
identities, axes, access/topology/cleanup postures, and stop conditions
should identity cards 262, 266, 270, 274, 278, 282, and 304 inherit?

## Method

Reconciled Research 143 with the current route matrix and workspace package
inventory. Re-read official CLI/ACP docs, GitHub latest stable releases, npm
`latest` package versions, and the ACP registry as discovery metadata.

No executable was installed. No provider account, login, prompt, catalogue,
or live session was used. Observed versions are not qualified claims.

## Inventory

The production route matrix still declares 39 routes. The workspace has 24
`swallowtail-adapter-*` crates, including `swallowtail-adapter-deepseek-harness`
and `swallowtail-adapter-zcode`. The README installed-harness map still lists
22 crates and omits those two. Repair that drift in the first accepted
expansion package card, not here. No production claim, matrix row, or
package is added by this record.

Existing overlap that must not be flattened:

- `pi.rpc` already owns Pi's official strict-LF JSONL RPC on `pi.package`
  through `0.84.2`
- Claude Agent, Cursor, Gemini, Grok, and Kimi already own first-party ACP
  stdio routes
- Qwen, Claude Code, Cursor, Kimi, Command Code, and Antigravity already own
  first-party headless/stream-json routes

ACP registry membership remains discovery only.

## Observed official points

These are observation timestamps, not Swallowtail bounds.

| Candidate | Official channel | Observed stable | Registry metadata (not a claim) |
| --- | --- | --- | --- |
| Cline | npm `cline` | `3.0.55` | `cline@3.0.55`, args `--acp` |
| Goose | GitHub `block/goose` | `v1.46.0` (2026-08-12) | binary `goose` args `acp` |
| Copilot CLI | npm `@github/copilot` | `1.0.80`; GitHub tag `v1.0.80` | `@github/copilot@1.0.80`, args `--acp` |
| Mistral Vibe | GitHub `mistralai/mistral-vibe` | `v2.24.2` (2026-08-18); release includes `vibe` and `vibe-acp` assets | registry still `2.24.1` / `vibe-acp` |
| Qoder CLI | npm `@qoder-ai/qodercli` | `1.1.25` | registry still `@qoder-ai/qodercli@0.2.14`, args `--acp` |
| Pi | npm `@earendil-works/pi-coding-agent` | `0.84.2` (already qualified for `pi.rpc`) | registry `pi-acp@0.0.33` is community `svkozak/pi-acp` |

## Candidate dispositions

### 1. Cline — admit two routes, one package

Official CLI `cline` documents both `--acp` (stdio ACP for editors) and
`--json` (NDJSON `ask`/`say` lines for programmatic runs), plus a prompt
argument / stdin default. Same executable, distinct selected flags.
`--auto-approve` defaults to true in CLI and false in ACP. `--id` resume,
hub/zen, TUI, kanban, schedule, and teams stay unmapped.

| Field | `cline.acp` | `cline.headless` |
| --- | --- | --- |
| Entrypoint | `cline --acp` | `cline --json` with a prompt; no TUI |
| Axis (provisional) | `cline.package` | `cline.package` |
| Package (provisional) | `swallowtail-adapter-cline` | same crate, distinct route |
| Topology | owned stdio child | owned stdio child |
| Access | provider-owned `cline auth` / env key; Swallowtail does not log in | same |
| Cleanup | join/kill the ACP child | join/kill the print child |
| First useful op | ACP initialize + one bounded prompt/session | one bounded JSON print run |
| Identity cards | 262 then 263-265 | 304 then 305-307 |

Do not combine the routes. Do not select `--auto-approve true` as a default
Swallowtail authority. Do not treat VS Code / SDK / hub as this route.

### 2. Goose — admit `goose.acp`

Official command is `goose acp` over stdio JSON-RPC. GitHub `v1.46.0`
binaries match the registry command. `--with-builtin` extensions and
`goose serve` HTTP/WebSocket stay unmapped. Auth method advertised as
`goose-provider` / `goose configure`; Swallowtail does not run configure.

| Field | Value |
| --- | --- |
| Route | `goose.acp` |
| Entrypoint | `goose acp` |
| Axis (provisional) | `goose.release` |
| Package (provisional) | `swallowtail-adapter-goose` |
| Topology | owned stdio child |
| Access | provider-owned local goose config; no Swallowtail login |
| Cleanup | join/kill the ACP child |
| First useful op | ACP initialize + one bounded prompt |
| Identity card | 266 |

Do not flatten Goose providers, recipes, or desktop onto this route.

### 3. GitHub Copilot CLI — admit `copilot-cli.acp` as public preview

Official `copilot --acp` with default `--stdio`. Docs are explicit public
preview. TCP `--port` is a different lifecycle (listener outlives one
client) and is not the first topology. Session/new does not carry
tool-filter or effort; those bind at server start. Interactive-only slash
commands stay unmapped. GitHub login or BYOK (`COPILOT_PROVIDER_*`) is
provider-owned.

| Field | Value |
| --- | --- |
| Route | `copilot-cli.acp` |
| Entrypoint | `copilot --acp --stdio` |
| Axis (provisional) | `copilot-cli.package` |
| Package (provisional) | `swallowtail-adapter-copilot-cli` |
| Topology | owned stdio child |
| Access | GitHub Copilot login or BYOK env; Swallowtail does not log in |
| Cleanup | join/kill the ACP child |
| First useful op | ACP initialize + one bounded prompt |
| Identity card | 270 |
| Maturity | public preview must remain visible on the claim |

Prerelease tags such as `1.0.81-0` are ignored.

### 4. Mistral Vibe — admit `mistral-vibe.headless` only

Official programmatic mode is `vibe --prompt … --output json|streaming`.
`--max-turns` bounds the run. `--trust` is programmatic folder trust.
`--continue` / `--resume` stay unmapped. Interactive TUI, teleport `&`,
and `--max-price` are not the route.

The same GitHub release also ships `vibe-acp`. That is a separate ACP
binary and stays a later disposition, not this roadmap.

| Field | Value |
| --- | --- |
| Route | `mistral-vibe.headless` |
| Entrypoint | `vibe --prompt` with `--output json` or `streaming` |
| Axis (provisional) | `mistral-vibe.release` |
| Package (provisional) | `swallowtail-adapter-mistral-vibe` |
| Topology | owned stdio child |
| Access | provider API key / local vibe config; Swallowtail does not log in |
| Cleanup | join/kill the prompt child |
| First useful op | one bounded programmatic prompt |
| Identity card | 274 |

### 5. Qoder CLI — admit `qoder.headless` only

Official headless is `qoder -p` / `--print` with `--output-format`
`text|json|stream-json`. Working directory `-w`. Permission modes and
`--yolo` / `bypass_permissions` stay unselected. `--input-format
stream-json` and `--session-id` stay unmapped until a later card.

npm `latest` is `1.1.25`. ACP registry still lists `0.2.14` with `--acp`.
Registry ACP is discovery only and is not this headless route.

| Field | Value |
| --- | --- |
| Route | `qoder.headless` |
| Entrypoint | `qoder --print` / `-p` |
| Axis (provisional) | `qoder.package` |
| Package (provisional) | `swallowtail-adapter-qoder` |
| Topology | owned stdio child |
| Access | `QODER_PERSONAL_ACCESS_TOKEN` or provider sign-in; Swallowtail does not log in |
| Cleanup | join/kill the print child |
| First useful op | one bounded print run with json or stream-json |
| Identity card | 278 |

### 6. Pi ACP — stop before driver

The ACP registry entry `pi-acp` is community package `pi-acp@0.0.33`
(`svkozak/pi-acp`). It speaks ACP by spawning `pi --mode rpc`. That is a
foreign wrapper over the already-qualified `pi.rpc` route, not an official
`@earendil-works/pi-coding-agent` ACP surface. A fork PR for `pi --mode acp`
is not the maintained official channel.

| Field | Value |
| --- | --- |
| Route | `pi.acp` (named only) |
| Official native ACP | absent |
| Collapse | community adapter → existing `pi.rpc` |
| Package | keep `swallowtail-adapter-pi`; do not add a wrapper package |
| Identity card | 282 records this stop |
| Driver cards 283-285 | do not start unless official native ACP appears |

This does not reopen or change `pi.rpc`.

## Primary order after this gate

1. `cline.acp` (262)
2. `cline.headless` (304) after Cline ACP closeout, including negative
3. `goose.acp` (266)
4. `copilot-cli.acp` (270)
5. `mistral-vibe.headless` (274)
6. `qoder.headless` (278)
7. `pi.acp` identity-only stop (282); no driver

Secondary wave still waits on card 286 after primary closeout.

## Contract-fit preview for card 261

Admitted first routes fit existing contracts: ACP stdio under 015 plus
process/host/cleanup 009-010, 032-033, 044, 051; headless print/JSON under
the installed structured-run shape used by Qwen/Command Code/Antigravity;
version identity under 029. No new provider-neutral contract is required
to start identity corpora.

Still adapter-private, not new contracts: Cline auto-approve default,
Copilot preview + server-start tool/effort binding, Vibe trust folders,
Qoder permission modes, Goose builtins.

Pi ACP needs no new contract because it is not admitted as a distinct
wire.

## Non-goals

- qualifying any version range
- mapping Cline hub/TUI, Goose serve, Copilot TCP, Vibe ACP, Qoder ACP,
  or community Pi wrappers
- installation, login, live inference, README repair, matrix edits

## Sources

- ACP registry snapshot 2026-08-18
- Cline CLI reference and ACP guide
- Goose `goose acp` docs / GitHub `v1.46.0`
- Copilot CLI ACP server docs (public preview)
- Mistral Vibe CLI programmatic mode; GitHub `v2.24.2`
- Qoder CLI run-in-scripts; npm `@qoder-ai/qodercli@1.1.25`
- npm `cline@3.0.55`, `@github/copilot@1.0.80`,
  `@earendil-works/pi-coding-agent@0.84.2`, `pi-acp@0.0.33`
