# 038 ACP v1 And Claude Agent Lifecycle Currentness

Status: promoted
Owner: Tom
Updated: 2026-07-27

## Question

Which exact stable ACP v1 records govern close and delete, and what deletion
truth is justified across Swallowtail's qualified Claude Agent
`0.53.0..=0.61.0` range?

## Method

Evidence was accessed 2026-07-27.

- pinned the latest stable ACP v1 schema tag, commit, meta record, generated
  schema, protocol documentation, and changelog
- compared stable schema `1.18.0` through `1.20.0`
- inspected Claude Agent tagged source and mock-backed lifecycle tests at every
  existing private behavior milestone
- inspected the unpublished-package `0.58.0` tag separately
- downloaded but did not install the exact ACP and Claude Agent SDK packages
- inspected exact SDK deletion implementations and JSON-RPC error mapping
- observed current Claude Agent `0.62.0` without extending the qualified range

No package was installed or executed. No login, credential, provider request,
user state, model, container, or external mutation was used.

## ACP Stable v1

The additive corpus pins:

- wire version: `1`
- schema artifact: `schema-v1.20.0`
- source commit: `5e89c71497fe07dd4ae633c181a17224f4a8956d`
- stable schema SHA-256:
  `92c1dfcda10dd47e99127500a3763da2b471f9ac61e12b9bf0430c32cf953796`

The selected stable lifecycle shape is byte-identical across schema releases
`1.18.0`, `1.19.0`, `1.19.1`, and `1.20.0`. Historical Gemini and Kimi schema
pins remain historical evidence. They are not rewritten.

`session/close` and `session/delete` remain independent optional methods.
Omitted or null capabilities forbid the matching dispatch.

Close:

- requires `sessionCapabilities.close`
- cancels ongoing work and frees active resources
- returns an empty result on success
- may reject a missing or inactive session
- does not delete durable history

Delete:

- requires `sessionCapabilities.delete`
- removes the session from later `session/list` results
- returns an empty result on success
- should silently accept an already-absent target
- leaves active deletion and later load behavior implementation-defined
- permits soft or hard provider implementations

Portable ACP truth therefore remains `HistoryRemoved`.

## Claude Tagged Range

The exact lifecycle handlers and tests are materially unchanged at:

- `0.53.0` / ACP SDK `1.0.0` / Agent SDK `0.3.195`
- `0.54.0` / ACP SDK `1.1.0` / Agent SDK `0.3.197`
- tag-only `0.58.0` / ACP SDK `1.2.1` / Agent SDK `0.3.205`
- `0.60.0` / ACP SDK `1.2.1` / Agent SDK `0.3.215`
- `0.61.0` / ACP SDK `1.3.0` / Agent SDK `0.3.217`

The unpublished wrapper package exclusion at `0.58.0` remains. Its public tag
does not make it an executable supported point.

Claude close requires an active in-memory session. It interrupts the query,
aborts and disposes owned session resources, removes the in-memory entry, and
returns an empty result. A missing or repeated close rejects. It does not call
the SDK deletion path, and tagged load tests confirm history remains usable
after close.

Claude delete:

1. tears down an active in-memory session when present
2. calls the exact Agent SDK `deleteSession`
3. returns an empty result only after SDK success

Across the pinned Agent SDK versions, the local implementation validates the
session identifier, locates one non-empty primary session JSONL, removes that
file, and recursively removes its sibling session directory. Missing and
repeated deletion reject. The ACP SDK maps ordinary handler errors to JSON-RPC
internal error `-32603`.

## Deletion Classification

Claude's selected route qualifies `ProviderDataDeleted` with
`ProviderDefinedDescendants`, not merely `HistoryRemoved`.

The stronger claim is narrow:

- it covers the Claude Agent harness's primary local session transcript
- it covers the provider-defined sibling session directory, including stored
  descendant material
- it does not claim secure erasure
- it does not claim deletion of Anthropic API service data, account analytics,
  logs, backups, or other provider retention
- it is not `ProviderHardDeleted`

This is exact provider-route evidence, not a wider ACP guarantee.

## Version Posture

Claude Agent `0.62.0` was published on 2026-07-24 at tag commit
`53a0c36ce3b0b76929d11d8b9565e319da745608`. Its lifecycle source is unchanged,
but it remains visible unverified-newer execution. This card does not
retroactively extend the full adapter support window.

## Recommendation

Card 053 may:

- use native `session/close` only after exact capability negotiation
- preserve close as active-resource cleanup, not deletion
- expose inactive-session delete as `ProviderDataDeleted`
- record `ProviderDefinedDescendants`
- treat missing and repeated deletion as provider rejection
- keep raw provider identifiers and error detail out of stable diagnostics
- use the same ACP message records over stdio and explicit remote transport

No new shared contract is needed. Contracts 015 and 038 need the exact
additive pin and Claude classification recorded before production mapping.

## Promotion

- ACP lifecycle pin and gates: Contract 015
- deletion strength and scope: Contract 038
- realized fixture boundary: system architecture
- deterministic corpus: card 052
- production mapping: card 053

## Primary Sources

- [ACP schema `v1.20.0`](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.20.0)
- [ACP stable session close](https://agentclientprotocol.com/protocol/v1/session-setup#closing-active-sessions)
- [ACP stable session delete](https://agentclientprotocol.com/protocol/v1/session-delete)
- [Claude Agent `0.53.0` handler](https://github.com/agentclientprotocol/claude-agent-acp/blob/47fee477096f9edd4aeb994679658fb0914cc0c0/src/acp-agent.ts)
- [Claude Agent `0.53.0` lifecycle tests](https://github.com/agentclientprotocol/claude-agent-acp/blob/47fee477096f9edd4aeb994679658fb0914cc0c0/src/tests/acp-agent.test.ts)
- [Claude Agent `0.61.0` handler](https://github.com/agentclientprotocol/claude-agent-acp/blob/c19bddcf7914259d6c15103a2d1580c7371e1d16/src/acp-agent.ts)
- [Claude Agent SDK package](https://www.npmjs.com/package/@anthropic-ai/claude-agent-sdk)
