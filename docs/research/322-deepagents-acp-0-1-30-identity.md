# 322 Deep Agents ACP 0.1.30 Identity

Status: promoted; identity evidence. The claim rebind lands in g05.074 with
this record.

Owner: Tom
Date: 2026-09-14
Card: g05.074 (Research 308 useful-newer campaign)
Authority: Contract 029; Research 153, 157, 206, and 308; the Deep Agents ACP
prepared guide; and the official npm channel.

## Answer

Official npm latest is `0.1.30`. The exact `deepagents-acp.package` point can
move from `0.1.25` to `0.1.30` as a compatible extension of the selected
`deepagents.acp` surface. The selected no-extra-argv stdio wire is byte-stable
across all six published points; the only CLI body change in the window is
dead code on the selected route; the runtime dependency `deepagents` moves
`1.12.4` through `1.13.4` with every hop classified, and its selected deltas
are additive and decoder-invisible. The `deepagents.acp.stdio-v1` behavior
revision and the empty selected argv are unchanged. This is one exact
`QualifiedOnly` point, not a range.

## Method and channel boundary

All six npm points (`0.1.25..=0.1.30`) were retrieved as exact official
tarballs into `/tmp`; every tarball's SHA-512 digest matches the registry
integrity value exactly, and the `0.1.25` baseline reproduces Research 157's
and Research 206's digests byte-for-byte. Because the package axis pins the
exact runtime dependency `deepagents`, all six published `deepagents`
points (`1.12.4`, `1.13.0..=1.13.4`) were retrieved the same way; the
`1.12.4` baseline reproduces Research 206. npm latest was rechecked at run
start and again immediately before the identity commit: `deepagents-acp`
stays `0.1.30` (published `2026-09-09T17:34:53.193Z`) and `deepagents` stays
`1.13.4`. The ACP registry was re-probed: it still carries the stale
discovery-only `deepagents` `0.1.7` / `npx deepagents-acp@0.1.7` entry (now
among 41 agents), which is never compared to the npm package axis. The
LangChain ACP docs page was re-fetched as a lead only; its digest
(`28d92130…`) differs from both Research 157's and Research 206's earlier
freezes, so docs stay out of the exact-`0.1.30` evidence chain. Host
`deepagents-acp` remains absent. No artifact was executed, installed, or run
with `npx`; no prompt, login, credential, or host mutation was used.

## Selected-surface result

The selected child is `deepagents-acp` with no extra argv. Its feeding
`dist/cli.js` is byte-stable through `0.1.27` and changes exactly once, at
`0.1.28`, and only inside `ACPFilesystemBackend.read`:

- `0.1.27..0.1.28` — the read pagination slicing moves to the new
  `normalizeReadPagination` helper and the result gains
  `totalLines`/`startLine`/`endLine`/`nextOffset` metadata. This class is
  unreachable on the selected route twice over: the CLI `main()` passes an
  explicit `backend: new FilesystemBackend({rootDir: workspaceRoot})` in the
  agent config, and `createBackend` returns `config.backend` before any
  client-capability branch, so `ACPFilesystemBackend` is never constructed;
  and the selected initialize advertises host `fs` false. `normalizeArgs` /
  `parseArgs` / `showHelp` / `showVersion` / `main`, `handleInitialize`,
  `handleAuthenticate`, `handleNewSession` (which still ignores `params.cwd`),
  `handlePrompt` (field `prompt`, slash-command interception, stop reasons
  `end_turn`/`cancelled`), `handleCancel`, `handleSetSessionMode`,
  `streamAgentResponse`, `handleAIMessage`, `handleToolMessage`,
  `requestToolPermission` (four options, per-name cache, catch-returns-allow
  fallback), `getToolCallKind`/`formatToolCallTitle`, `handleSlashCommand`,
  `createAgent`, and `createBackend` are byte-identical from `0.1.25` through
  `0.1.30`. `dist/index.d.ts` is byte-identical across the whole window.
- `main()` still constructs the server with `serverName: "deepagents-acp"`
  and no `serverVersion`, so initialize `agentInfo.version` stays the
  constructor default `0.0.1` — never the npm package axis.
- `@agentclientprotocol/sdk` stays `^1.1.0` across all six points, so the ACP
  wire layer (`AgentSideConnection`, `ndJsonStream`) resolves from the same
  declared range as the frozen corpus. The `MemorySaver` import from
  `@langchain/langgraph-checkpoint` is unchanged; in-process session
  persistence stays nondurable.

## Runtime dependency ledger (deepagents 1.12.4..1.13.4)

`deepagents-acp` pins its runtime dependency exactly per release
(`1.12.4`, `1.13.0`, `1.13.1`, `1.13.2`, `1.13.3`, `1.13.4`), so the child's
resolved library is deterministic per point. Per-hop classification of the
selected inputs (`createDeepAgent`, `FilesystemBackend`, and the middleware
stack; the CLI passes no subagents, no permission rules, and no harness
profile):

- `1.12.4..1.13.0` — **selected, additive**:
  - a new builtin `delete` tool joins the filesystem toolset
    (`FILESYSTEM_TOOL_NAMES` gains `delete`), with its own deny-pattern
    permission check that is inert without configured permission rules;
  - `write_file`'s `content` parameter stops defaulting to `""` and becomes
    required (model-facing tool schema);
  - subagent fork-mode machinery, `parentSystemPrompt` plumbing, profile
    filesystem-tool selection, and a memory-middleware construction hoist
    (all unselected: no subagents; the parent stack order and content are
    unchanged);
  - StateBackend / StoreBackend / ContextHubBackend / CompositeBackend /
    sandbox delete changes (unselected backends).
  Wire classification: `delete` flows through the unchanged generic
  `tool_call` / `tool_call_update` / permission paths with kind `other`
  (`getToolCallKind` is unchanged and has no `delete` arm), so the
  name-agnostic decoder sees no shape change. Authority note: the child's
  local-file authority now includes recursive deletion, but the route
  already records that local `FilesystemBackend` writes in the child cwd are
  not a Swallowtail bounded-write claim, and the same ACP-level
  observe-and-cancel permission flow still gates execution. No new
  Swallowtail authority is mapped.
- `1.13.0..1.13.1` — type declarations and sourcemaps only; both selected
  implementation chunks (`dist/langsmith-*`, `dist/src-*`) are
  byte-identical.
- `1.13.1..1.13.2` — **selected, additive**:
  - `normalizeReadPagination` is added and exported; `deepagents-acp`
    `0.1.28..=0.1.30` imports it at module load, and the exact `deepagents`
    pins of those releases (`1.13.2..=1.13.4`) export it, so resolution stays
    deterministic inside each package's own pin;
  - `FilesystemBackend.read` normalizes offset/limit and returns pagination
    metadata;
  - the `read_file` middleware renders a remaining-lines notice footer into
    partial-read tool results and truncates only at complete source-line
    boundaries;
  - a tool-exclusion middleware appears (unselected: the exclusion set is
    empty here);
  - read pagination for StateBackend / Store / ContextHub / sandbox (unselected).
  Wire classification: `read_file` tool results may now carry a pagination
  footer; `ToolMessage` content flows to ACP `tool_call_update` content as
  opaque text, so the decoder-invisible surface shape is unchanged.
- `1.13.2..1.13.3` — subagent fork/isolated machinery, duplicate-name
  rejection, default subagent mode `handoff`→`isolated`, and task-tool
  description wording (all unselected or model-facing text only).
- `1.13.3..1.13.4` — more subagent fork machinery, harness-profile-aware
  subagent middleware, call-count state-key exclusions, and task-tool listing
  rendering (all unselected or model-facing).

Peer dependency floors rise at `0.1.26`
(`@langchain/langgraph` `^1.2.9`→`^1.4.10`,
`@langchain/langgraph-checkpoint` `^1.0.0`→`^1.1.5`); these are install-time
ambient ranges, not selected-route constants, and the SDK range and
checkpointer import are unchanged.

## Effect on Research 153, 157, 206, and 321

Research 157's identity facts reproduce exactly at the `0.1.25` baseline and
every selected wire fact it freezes is byte-stable through `0.1.30`, so the
frozen `deepagents-acp-0.1.25` fixture corpus continues to cover the selected
wire and stays the decoder corpus. Research 206's explicit model-selection
empty deliver-now set carries over unchanged: the CLI parser, `--model`
default, `agentInfo` shape, and `createDeepAgent` model path are unchanged on
the selected surface. Research 321's Mistral Vibe rebind is independent.

## Decision and disposition

Identity-first decision: a compatible exact-point rebind. The
`deepagents.acp.package-window-1` claim moves to one maintained `0.1.30`
point with the unchanged `deepagents.acp.stdio-v1` behavior revision,
`QualifiedOnly` posture, and empty selected argv — no mechanical adaptation
and no argv change. No range, second exact point, exclusion entry, or
unverified-newer posture is added. The stale ACP registry `0.1.7` entry and
the constructor-default `agentInfo.version` `0.0.1` stay outside the claim.
The new evidence corpus is
[`deepagents-acp-0.1.30`](../../crates/swallowtail-adapter-deepagents/tests/fixtures/deepagents-acp-0.1.30/)
with the mutation-sensitive assertions in
[`deepagents_acp_0_1_30_delta_ledger.rs`](../../crates/swallowtail-adapter-deepagents/tests/deepagents_acp_0_1_30_delta_ledger.rs).

## Non-goals

- mapping the new builtin `delete` tool, `session/load`, `session/set_mode`,
  or slash commands beyond the existing observe-and-cancel policy
- mapping the Unified-Harness-era subagent fork/isolated machinery
- live initialize, prompt, or API-key use
- installing the npm package or executing any downloaded tarball
- wrapping `npx` or the programmatic server
- version-range claims or inheriting registry `0.1.7`
