# 2026-09-02 Claude Agent Interactive Session Parity Gap

Status: open; consumer-raised lead for operator disposition into the g05
runway. Not execution authority.
Owner: Tom
Date: 2026-09-02
Source: Bovine desktop client-handover requirement (market
`docs/triage/20260902-131606-bovine-desktop-client-handover-plan.md` at
`0c0f1446f`); Swallowtail `main` at `cac79ff9`; pinned bridge
`@agentclientprotocol/claude-agent-acp` 0.63.0; Paseo.app bundle dated
2026-08-25 and T3 Code (Nightly).app bundle dated 2026-08-29 (string-level
inspection of `app.asar`, no source).

## Purpose

The Bovine Accelerator desktop app must host a Claude Code-like chat tab:
multi-turn, read-write inside the app-managed content checkout, shell for
validators, permission modes, resume, interrupt, model and effort changes
mid-session, later client MCP tools for in-view context. The operator wants
Swallowtail to support Claude Code the way Paseo and T3 Code do. This note
records what those apps drive, what `claude-agent.acp` qualifies today, and
the exact gap. It decides nothing.

## What Paseo and T3 Code drive

Both embed `@anthropic-ai/claude-agent-sdk` in-process (Paseo also carries
ACP client code for other agents). Option and method names present in the
bundles:

| Capability | Paseo | T3 Code |
| --- | --- | --- |
| Persistent multi-turn session with writes, `cwd`, `env` | yes | yes |
| `permissionMode` + `setPermissionMode` mid-session | yes | yes |
| `canUseTool` per-call approval callback | yes (22) | yes |
| `resume`, `continue`, `resumeSessionAt`, `forkSession` | yes | yes |
| `interrupt` | yes | yes (291) |
| `setModel`, `effort`, `thinking`/`maxThinkingTokens`, `fallbackModel` | yes | yes |
| `hooks` (`SessionStart`, `PreToolUse`) | yes | yes |
| `mcpServers`, `mcpServerStatus`, `plugins`, `agents` | yes | yes |
| `allowedTools`/`disallowedTools`, `additionalDirectories`, `settingSources` | yes | yes |
| `includePartialMessages` streaming | yes | yes |
| `rewindFiles`, `enableFileCheckpointing` | yes | yes |
| `supportedCommands` (slash commands), `supportedModels`, `accountInfo` | yes | yes |
| `sandbox`, `systemPrompt`/`appendSystemPrompt` | yes | T3 systemPrompt |

## What `claude-agent.acp` qualifies today

Evidence: `docs/guides/provider-route-matrix.md:56`,
`docs/guides/provider-solution-feature-matrix.csv:5`,
`docs/guides/claude-agent-prepared-integration.md:77-192,475-517`,
`crates/swallowtail-adapter-claude-agent/src/driver/validation/session.rs:36-46`,
`src/prepared_profile/plan.rs:95-110`, `src/connection.rs:122-146`.

- Structured run: one prompt, `ReadWrite`, tools `Read, Glob, Grep, Edit,
  Write`, `acceptEdits`; native close retains the transcript.
- Interactive session: pinned to `ResourceAccess::Read` in both permission
  handling branches; tools `Read, Glob, Grep` only.
- No `Bash`/terminal tool in either shape. `mcpServers: []` hard-coded.
- Permissions: one-shot `allow_once`/`reject_once` only; "persistent choices
  remain unavailable"; the choice is fixed per prepared plan.
- Plan mode, model, and effort are session setup; "changing mode requires a
  new prepared session"; no mid-session `set_mode`/`set_config_option`.
- `load_session` and `resume_session` qualified; no catalogue, import, fork,
  rewind, checkpoint, attachments, images, slash commands, hooks, or
  subagent-transcript capability.
- Cancellation: yes. Usage: yes. Auth: inherits local Claude login; no
  in-app OAuth (by policy; `docs/triage/2026-08-21-new-route-candidates.md:527-553`
  rejects embedding the SDK and offering claude.ai login).

## Why the gap is adapter-local

- The pinned bridge 0.63.0 advertises: tool calls with permission requests,
  `session/set_mode` with `acceptEdits`, `plan`, `dontAsk`,
  `bypassPermissions` (`dist/acp-agent.js:389-393,3269-3281`), `Bash` and
  interactive/background terminals, client MCP servers, images, @-mentions,
  slash commands, edit review, subagent transcripts, `setModel` after
  creation (`dist/acp-agent.js:3257`). The wire already carries what a
  Claude Code-like session needs.
- `swallowtail-adapter-codex` already qualifies `ReadWrite` interactive
  sessions on `codex.app-server` (`src/session_access.rs:59,115`,
  `src/app_server.rs:209`), so `SessionAccessPolicy` and the runtime model
  support read-write sessions. Only the Claude adapter withholds it.

## Gap list, ranked for the Bovine consumer

| # | Gap | Bovine need | Size guess |
| --- | --- | --- | --- |
| G1 | Read-write interactive session (`Edit`, `Write`; `acceptEdits`) | blocker for a chat that edits content | M (mirror Codex session access; fixtures; matrix row) |
| G2 | Session-scoped permission policy: mode switch (`set_mode`) and "allow for this session" beyond one-shot | blocker for usability; one-shot only means prompt fatigue | M |
| G3 | `Bash`/terminal tool behind consumer mediation | should; validators and scripts; the consumer may still deny | M |
| G4 | Mid-session model and effort change via `set_config_option` | should | S–M |
| G5 | Client MCP servers on `session/new` | should for phase 2 in-view contextual chat (the app exposes scope and content as tools) | M |
| G6 | Fork, catalogue/import, rewind/checkpoint | later; Codex has catalogue; Claude has none | L |
| G7 | Images/attachments, slash commands, subagent transcripts (`_meta` capability) | nice | S each |
| G8 | Hooks (`SessionStart`, `PreToolUse`) | not needed if G2 lands; consumer mediation covers policy | — |
| G9 | Auth readiness observation surfaced as a typed state so an app can offer "sign in with Claude" by launching the harness login rather than parsing errors | should; the OAuth simplification the operator asked for lives here or in the app | S |
| G10 | Sidecar packaging guidance for Tauri consumers (Node 22 + app-pinned `claude-agent-acp`, contract-008 style frozen graph) | must for the Bovine DMG | docs S |

Codex (`codex.app-server`) already covers G1, G2-equivalents, catalogue and
history. `grok-build.acp` sessions are read-write with OAuth via
`cached_token`, but permission requests are observable only and stop the
turn (the adapter routes `session/request_permission` to rejection,
`crates/swallowtail-adapter-grok/src/**/dispatch.rs`), with no public
load/resume/management and no usage. Add as **G11: answerable Grok
permissions** if Grok is to sit behind the same chat UI; otherwise it is an
activity-only route.

## Interim path the consumer will use

Until G1–G2 land: compose Claude Code as consecutive one-prompt read-write
structured runs over a retained transcript (durable close, `load_session`
replay), with `codex.app-server` as the fully interactive harness. This is
documented as a workaround in the consumer plan, not as parity.

## Constraint

Swallowtail is in the v0.4.0 release freeze (card 050 audit dispatched
2026-09-02). Whether G1–G2 break that freeze is the operator's call.

## Disposition gate

Close when the operator either compiles G1–G2 (and chooses among G3–G5,
G9–G10) into g05 cards with a target tag the Bovine app can pin, or declines
and records the structured-run workaround as the supported Claude posture.
