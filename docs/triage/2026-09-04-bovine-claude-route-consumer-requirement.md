# 2026-09-04 Bovine Claude Route Consumer Requirement (after v0.4.0)

Status: open; consumer-raised lead for the Swallowtail operator thread. Not
execution authority.
Owner: Tom
Source: Bovine Desktop harness chat (Market Card 253, restore ruling
2026-09-04); the earlier parity note `2026-09-02-claude-agent-interactive-parity-gap.md`
(since promoted/pruned in this repo); Swallowtail `v0.4.0` route matrix.

## Problem

The operator was explicit that Claude support in Bovine Desktop must match
what Paseo and T3 Code offer (persistent editing sessions, permission modes,
resume, interrupt, mid-session model change). `v0.4.0` added
`claude-agent.sdk` (Node sidecar over the official Claude Agent SDK,
subscription auth, interactive session, interrupt, consumer-mediated tool
admission) but its guide makes it **read-only** (`Read`/`Glob`/`Grep`; writes,
Bash, plugins, subagents, checkpoints "outside this route"). `claude-agent.acp`
sessions are also read-only; only its one-prompt structured run is read-write.
So no Claude route can drive an editing chat today.

## Required, in priority order (acceptance, not inference)

1. **Read-write interactive session** (SDK route preferred): `Edit`, `Write`
   (and `MultiEdit` where the SDK exposes it) admitted through consumer-
   mediated tool admission; working resource leased read-write to the app's
   checkout. Acceptance: a multi-turn session edits a file in the leased cwd
   and the consumer sees each tool call before it runs.
2. **Session-scoped permission policy**: `permissionMode` at open and
   `setPermissionMode` mid-session (`acceptEdits`, `plan`, `default`; bypass
   excluded), plus per-call `canUseTool` mediation. One-shot allow/reject
   alone is insufficient.
3. **Bash/terminal** admitted behind consumer mediation with intact tool
   input (Bovine denies Git, allows validators).
4. **Mid-session model and effort change** with confirmed effective values.
5. **Resume** (`resume` / `resumeSessionAt`) and session listing for the
   app's transcript store; fork optional.
6. **Client MCP servers** on open (for later in-app contextual tools).
7. **Grok** (`grok-build.acp`): answerable permission requests, or an explicit
   activity-only posture the app can label.
8. **Discovery, not bundling**: accept a discovered locally installed native
   Claude Code and a discovered or minimally bundled Node runtime for the
   sidecar. Bovine will never ship the ~300 MB native Claude binary; it bundles
   only the sidecar package and, if unavoidable, Node. Version posture like
   Codex: qualified ranges with stable-newer allowed, so a client's `claude`
   need not match one exact patch (`v0.4.0` pins exact `2.1.259` / Node
   `22.23.2`, which a client machine will not reliably match).
9. **Install guidance** surfaced through discovery diagnostics: the vendor-
   recommended dependable install command per harness (Claude Code, Codex,
   Grok) for a client Mac, so the app can offer "install" when absent.

Out of scope: hosted OAuth in the app, API-key routes, Bedrock.

## Interim posture in Bovine

Codex is the fully interactive harness; Claude runs as consecutive one-prompt
read-write ACP runs over a retained transcript, with the SDK route offered
read-only for Q&A if adopted. Bovine will switch to the SDK session the moment
items 1–2 land in a tagged release.
