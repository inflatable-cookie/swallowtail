# 125 ACP Activity Currentness And Corpus

Status: completed
Owner: Tom
Created: 2026-07-29
Milestone: `../037-acp-observable-agent-activity.md`
Depends on: card 124

## Goal

Freeze current ACP activity semantics and exact Claude Agent, Gemini CLI, and
Kimi Code behavior milestones before shared projection.

## Scope

1. Revalidate the maintained ACP schema, SDK, and transport authority.
2. Revalidate each selected harness's exact schema and version range.
3. Freeze bounded fixtures for message chunks, thought chunks, plans, tool
   calls, tool updates, usage, modes, commands, unknown updates, permission
   requests, and completion.
4. Preserve provider-specific fields and exclusions separately.
5. Cover stdio framing and the existing explicit remote transport corpus.
6. Make no production behavior change.

## Out Of Scope

- Grok Build activation
- provider identity inferred from ACP
- production mapping
- live harness installation or authentication

## Acceptance Criteria

- [x] current ACP authority and exact schema version are recorded
- [x] every guaranteed harness segment has fixture provenance
- [x] thought chunks are classified as provider-visible display content or
      excluded
- [x] tool and plan lifecycle semantics are explicit
- [x] stdio and remote transport identity remain separate
- [x] unknown and malformed updates have deterministic outcomes

## Validation

- ACP protocol fixture tests
- adapter fixture decoders
- `effigy qa:docs`
- `effigy check:rust`

## Stop Conditions

- Stop if current ACP semantics conflict with Contract 044.
- Keep provider-specific decoding outside the shared boundary when required.

## Auto-Continuation

Continue to card 126 only after the shared and provider-specific corpus is
complete.

## Result

- Pinned ACP v1 stable schema `v1.20.0`, schema package `1.6.0`, Rust core
  SDK `2.0.0`, and remote transport SDK `2.0.0` as separate authority axes.
- Froze shared stable-schema activity, malformed, stdio, and remote fixtures.
- Froze exact Claude Agent, Gemini CLI, and Kimi Code activity corpora across
  every guaranteed segment. Current newer releases remain permitted but
  unverified.
- Kept ACP thought classification in adapters: it may become a reasoning
  summary, warning, another display activity, or an exclusion.
- Recorded tool partial-update, plan replacement, raw-field exclusion, and
  session-metadata boundaries in Contract 044.
- Made no production, provider, authentication, or live transport change.

## Validation Evidence

- shared ACP activity corpus: 4 passed
- Claude Agent activity corpus: 2 passed
- Gemini CLI activity corpus: 2 passed
- Kimi Code activity corpus: 2 passed
- complete `swallowtail-protocol-acp` suite: 74 passed
- remote ACP transport suite: 8 passed
- Rust compile, lint, formatting, docs, and Northstar gates passed
- all 23 local packages and the extracted workspace passed
- `effigy doctor` remained at the known 111 structural findings
