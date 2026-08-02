# Provider Session Import Route Classification

Date: 2026-08-02
Roadmap: g03.023
Card: 061

## Change

Research 096 splits the public solution inventory into 19 distinct harness
routes and classifies each against catalogue, exact lookup, bounded replay,
public continuation, resource binding, activity truth, and exact-version
evidence.

Only Codex app-server, Kimi Code ACP, and OpenCode HTTP support the complete
profile. Gemini headless is discovery-only. Claude Agent ACP and Kimi local
server are attachment-only. Cursor ACP and Pi RPC remain blocked. Eleven
routes are not applicable because their selected operation exposes no reusable
external provider-session identity.

## Boundary

No route inherits support from its provider family, another transport, stable
ACP wire support, private headless continuation, or provider-state retention.
Every partial route retains a named source, authority, replay, continuation,
resource, or range-evidence promotion gate.

## Evidence

- 19 route rows reconcile to 3 supported, 1 discovery-only, 2
  attachment-only, 2 blocked, and 11 not applicable
- `effigy qa:docs` passed
- `effigy qa:northstar` passed
- `git diff --check` passed
- no implementation, provider effect, or consumer edit

## Next

Card 062 publishes this classification in the route and feature matrices,
adds prepared import examples, and proves the selected packages assemble from
extracted targets.
