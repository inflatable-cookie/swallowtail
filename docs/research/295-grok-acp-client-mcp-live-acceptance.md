# 295 Grok ACP Client-MCP Live Acceptance

Status: complete; promoted into g05 card 143
Owner: Tom
Created: 2026-09-08

## Finding

Bovine Desktop completed the Card 128 live gate against source-linked
Swallowtail `04e9b2dd9058783b7186a33a6c13dd74882a5925`. Exact Grok Build
`1.0.4` and `1.0.5` ran once each, in that order, as separate segments. Both
returned `accepts_client_mcp`.

Each capsule records client-MCP admission, `tools/list`, a live echo helper,
`tools/call`, a completed prompt with `stop_reason: end_turn`, no truncation,
and joined cleanup. Neither segment hit authentication or quota rejection and
neither was retried. No stale callback arrived; the capsule field
`stale_callback_rejected` is therefore false rather than evidence of a stale
callback test.

| Segment | Version receipt | Capsule SHA-256 |
| --- | --- | --- |
| `1.0.4` | `d846eb93d94d` | `66fa8865bf065bc5b1d84b50ee3e75218cebbf43297d486aaa5ccf82772f75fe` |
| `1.0.5` | `5115b46bc909` | `bd1f450a64eef37e316cb9926276afd4b08cf4b26cd7ba77829f2f61e1b33c5a` |

Desktop records Northstar task
`10bdda2b-d395-4dc1-baaa-5e625a6286e5`, accepted PR 169 head
`77bced726a40b3a5dc74917aa4e4b518c8e15972`, independent review comment
`5586043120`, merge `6b22c82946fcd04d418ee2a810e73970373b7888`, and canonical closeout
`c495b6a60976c4618ab8f50c820265849bfe3735`. Focused proof passed 160/160
plus four verdict fixtures. The exact prompt is retained in each committed
Desktop capsule.

## Card 128 Disposition

The `accepts_client_mcp` branch converts the four live packet cells from
`evidence_pending` to `producer_gap`. Card 118 already built the callable
route-local seam and closed while the gate was pending, so card 143 owns the
remaining qualification work.

The capsules directly prove client MCP admission, registered-tool discovery
and invocation, and consumer tool exchange for the exact maintained
`1.0.4..=1.0.5` segment. They do not carry a selected skill bundle. Card 143
must keep selected-skill delivery separate: use a bounded, distinctly labelled
ACP surface supported by frozen evidence, or return an honest route limitation.
Appending skill content to user prompt text is not an admissible substitute.

## Limits

This is route- and segment-specific evidence. It does not authorize or imply a
tag, release, provider parity, unrelated matrix movement, version extension,
or a claim that Grok followed a skill. Desktop remains the source of the
committed capsules; Swallowtail records their identities and planning
disposition rather than copying client paths or credentials.
