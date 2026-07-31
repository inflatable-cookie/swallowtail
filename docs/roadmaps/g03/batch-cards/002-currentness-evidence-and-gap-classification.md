# 002 Currentness Evidence And Gap Classification

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../001-installed-harness-and-protocol-currentness-baseline.md`
Depends on: card 001

## Objective

Revalidate the bounded high-risk source set and classify exact compatibility
work without changing production behavior.

## Scope

1. Check current official provider or maintained-project documentation,
   schemas, changelogs, tags, and published artifacts selected by card 001.
2. Compare current evidence with each exact qualified segment and corpus.
3. Classify each result as compatible extension, behavior milestone, breaking
   drift, evidence-only refresh, externally blocked, or no action.
4. Update Research 074 with direct evidence and dates.
5. Identify shared contract deltas before any implementation roadmap.

## Out Of Scope

- production code or range descriptor changes
- live authenticated model work unless separately gated later
- inferring compatibility from semver
- unbounded provider inventory expansion
- consumer edits or publication

## Selected Source Set

Research 074 bounds the first currentness comparison to:

1. Codex CLI for both exec and app-server claims
2. stable ACP plus the Claude Agent wrapper
3. Gemini CLI for its separate ACP and headless claims
4. Pi coding agent RPC
5. Qwen Code headless

Check OpenCode current publication evidence only far enough to classify its
stale exact-`1.14.48` live selector. Do not reopen the freshly qualified range
unless the maintained server surface has moved materially.

Kimi, Grok, OpenCode range implementation, and Claude Code headless stay out
of the first full comparison. Their repository evidence was refreshed on
2026-07-30 and no consumer-reproduced defect currently changes priority.

## Acceptance Criteria

- [x] every selected surface has current authoritative evidence or an explicit
  evidence gap
- [x] protocol, executable, package, schema, and model versions stay separate
- [x] support claims remain exact and milestone-based
- [x] unverified-newer execution stays visible unless evidence requires denial
- [x] contract gaps are promoted before implementation compilation
- [x] card 003 has a bounded ranked candidate set

## Validation

- source and version cross-check
- deterministic artifact metadata checks where available
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, only when evidence and contracts make tranche selection mechanical.

## Evidence

- Research 074 records the 2026-07-31 official publication and tagged-source
  pass for Codex, stable ACP, Claude Agent, Gemini CLI, Pi, Qwen, and OpenCode.
- Codex and stable ACP remain at their qualified points.
- Claude Agent, Gemini, Pi, and Qwen are ranked provider-specific range
  candidates. Pi continuity remains separately blocked.
- OpenCode publication remains `1.18.10`; its optional live selector is stale.
- No shared architecture or contract delta is required. Card 003 is ready.
