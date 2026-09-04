# g05.029 Claude SDK Interactive Parity

Status: ready; card 080 read-write session and permission policy is ready; cards 081-088 are planned in priority order
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Depends on: Contracts 017, 019, 023, 029, 036, 041; Research 278 and 280; completed g05.022 and g05.023; tagged `v0.4.0`
Vision tags: Claude route, consumer parity, interactive session, discovery

## Purpose

Make `claude-agent.sdk` a full editing harness for consumers. The Bovine
Desktop requirement of 2026-09-04 (operator-confirmed) is that Claude support
must match what Paseo and T3 Code offer. Today the sidecar hard-codes
`Read`, `Glob`, `Grep`, `permissionMode: "default"`, empty MCP servers, and
rejects resume; ACP sessions bind read-only. No Claude route can drive an
editing chat.

Research 278 layered the route exactly this way: layer 1 shipped in
`v0.4.0`; layer 2 is permission modes, model and effort, resume and fork;
layer 3 is MCP; layer 4 is Bash with Contract 023 and 041 evidence. This
roadmap runs those layers in the consumer's priority order.

## Posture

Every write tool runs under an explicit `AmbientHost` posture with
consumer-mediated per-call admission (Contract 017: an ambient harness may
run write tools without a bounded filesystem claim; Contract 023: tool
allowlists do not contain the process). The read-write lease is a location
and callback scope, not containment. The default remains read-only; the
consumer opts in per prepared profile. `bypassPermissions` is never
admitted. In `acceptEdits` mode the SDK auto-approves edits, so per-call
visibility holds only in `default` and `plan`; the guide states this.

## Runway (priority order from the consumer requirement)

1. Card 080: read-write session (`Edit`, `Write`, `MultiEdit`) through
   per-call admission on a read-write lease, `permissionMode` at open, and
   `setPermissionMode` mid-session. Carrier for `v0.4.1`.
2. Card 081: Bash under mediation with intact tool input.
3. Card 082: mid-session model and effort change with confirmed values.
4. Card 083: resume, `resumeSessionAt`, and session listing; fork optional.
5. Card 084: client MCP servers on open.
6. Card 085: Grok `grok-build.acp` answerable permission requests, or an
   explicit labelled activity-only posture.
7. Card 086: identity research on discovered native Claude Code and
   discovered or minimally bundled Node against the SDK wrapper; then card
   087 moves the five SDK axes from qualified-only exact pins to Codex-style
   qualified ranges with stable-newer allowed.
8. Card 088: install guidance surfaced through discovery diagnostics for
   Claude Code, Codex, and Grok, after one provider-neutral vocabulary
   amendment.

Out of scope for the whole roadmap: hosted OAuth in the application, API-key
routes, Bedrock.

## Release Edge

Items 1 and 2 are additive under Contract 036 (default unchanged, opt-in per
profile, no public item removed), so they ship as patch `v0.4.1`. When card
080 merges, Chatterbox compiles a `v0.4.1` release-readiness roadmap on the
`v0.4.0` precedent: exact-head review, the eleven local gates, exact-SHA CI,
the external source consumer, and one operator-authorized Bovine smoke.
Cards 081-084 target `v0.4.2`; 085-088 follow their evidence. Nothing here
forces a minor.

## Batch Cards

- [080 Claude SDK Read-Write Session And Permission Policy](batch-cards/080-claude-sdk-read-write-session-and-permission-policy.md) — ready
- [081 Claude SDK Bash Under Mediation](batch-cards/081-claude-sdk-bash-under-mediation.md) — planned; after 080
- [082 Claude SDK Mid-Session Model And Effort](batch-cards/082-claude-sdk-mid-session-model-and-effort.md) — planned; after 080
- [083 Claude SDK Resume And Session Listing](batch-cards/083-claude-sdk-resume-and-session-listing.md) — planned; after 080
- [084 Claude SDK Client MCP Servers](batch-cards/084-claude-sdk-client-mcp-servers.md) — planned; after 080
- [085 Grok ACP Answerable Permissions](batch-cards/085-grok-acp-answerable-permissions.md) — planned; independent crate
- [086 Claude SDK Discovery Identity](batch-cards/086-claude-sdk-discovery-identity.md) — planned; research first
- [087 Claude SDK Qualified Ranges](batch-cards/087-claude-sdk-qualified-ranges.md) — planned; after 086
- [088 Harness Install Guidance Diagnostics](batch-cards/088-harness-install-guidance-diagnostics.md) — planned; vocabulary amendment first

## Dispatch Manifest

Promoted planning commit: the `main` commit that introduces this file.
Card 080 is approved concurrent with g05.009 cards 074, 075, 076, and 079.

| Field | Card 080 |
| --- | --- |
| Readiness | ready |
| Prerequisites | `v0.4.0` `claude-agent.sdk` route on `main`; Research 278 §8 layer 2; the fake-SDK sidecar fixture harness |
| Completion conditions | prepared profile admits an explicit write-tool set and permission mode; sidecar passes the admitted set as `tools` (never `allowedTools`), honours `permissionMode` at open, and exposes a `set_permission_mode` wire command; `canUseTool` forwards intact tool name and input for every write call; read-write lease required for any write tool; `bypassPermissions` rejected before launch; default profile byte-identical in behaviour; guide, changelog, and fixtures updated; provider-free proof that a multi-turn session edits a file in the leased cwd with every call admitted first; one PR |
| Owned mutable paths | `crates/swallowtail-adapter-claude-agent/src/sdk/**`; `crates/swallowtail-adapter-claude-agent/sidecar/**`; `crates/swallowtail-adapter-claude-agent/tests/**`; `release-baselines/public-api-0.4.0/swallowtail-adapter-claude-agent.txt` regenerated additively; `docs/guides/claude-agent-sdk-prepared-integration.md`; the `claude-agent.sdk` cells of the route and feature matrices; `CHANGELOG.md` `[Unreleased]`; this card's `## Result`; `PAPERCUTS.md` append only |
| Reserved shared closeout surfaces | `docs/roadmaps/README.md`, `docs/roadmaps/g05/README.md`, this roadmap, `docs/roadmaps/g05/batch-cards/README.md`, `docs/roadmaps/generation-index.md`, `docs/logs/README.md` |
| Forbidden paths | every other crate; `claude_code_*` and ACP modules of the Claude Agent crate; `docs/contracts/**`; version claims and SDK selection pins; Bash, MCP, resume, and model-change surfaces (later cards) |
| Approved concurrent siblings | g05.009 cards 074, 075, 076, 079 |
| Serial edges | cards 081-084 follow card 080; the `v0.4.1` release roadmap follows card 080's merge |
| Worker capability class | Rust plus Node sidecar implementation worker with process-authority discipline; frontier-tier; no provider credentials; no live Claude call |
| Acceptance evidence | provider-free fake-SDK proofs; fixture showing `tools` set and `allowedTools` absent; rejection proofs for bypass and for write tools without a read-write lease; focused and package-affected validation; additive API diff |
| Review oracle | no tool is ever auto-allowed by the sidecar; the smallest counterexample is `allowedTools` set, a write admitted on a read-only lease, `bypassPermissions` reaching the SDK, or a default profile whose behaviour changed |
| Stop conditions | the SDK cannot change permission mode mid-session without reopening (record and return to Chatterbox); any write path that bypasses `canUseTool`; a required Contract 017 or 023 reinterpretation |
| Escalation owner | operator via Chatterbox for posture questions; coordinator for mechanical blockers |

## Acceptance

- [ ] a consumer drives a multi-turn editing session on the SDK route with
      every tool call mediated before it runs
- [ ] permission mode is selectable at open and changeable mid-session
- [ ] the read-only default and `v0.4.0` behaviour are unchanged
- [ ] later cards land in priority order without reopening card 080's seam
- [ ] `v0.4.1` carries card 080's items on the Contract 036 gates
