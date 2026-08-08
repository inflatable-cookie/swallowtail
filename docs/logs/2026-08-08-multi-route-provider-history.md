# 2026-08-08 Multi-Route Provider Session History

Status: closed
Owner: Tom
Milestone: g03.058
Cards: 179-180

## Decision

Advertise Contract 054 history pages on every route that can page without a
live control handle: `codex.app-server` (already proven), `opencode.http`, and
retained `alibaba.conversations`. Leave Claude Agent ACP and Kimi ACP
unsupported because their only history wire today is control-granting
`session/load`.

## Evidence

- Runtime history plan rules accept ambient working-resource harness posture
  or resource-free DirectModelInference posture matching the binding.
- OpenCode history: health + session get + ascending `session_messages` via
  shared `load_replay`, synthetic newest-first pages, qualified-server gate,
  no POST/DELETE control.
- Alibaba retained history: shared ascending items walk with load, synthetic
  pages, resource-free leases, no live handle.
- Focused validation: `swallowtail-runtime`, `swallowtail-adapter-opencode`,
  `swallowtail-adapter-alibaba-model-studio`.
- Public API baselines regenerated for the new prepared history surfaces.

## Current State

Consumers can prepare and page history on Codex app-server, OpenCode HTTP, and
Alibaba retained conversations. ACP load-as-history remains a separate
qualification gate, not an implied mapping from `load_session`.

## Next Move

Return to the g03 evidence gate unless a control-free ACP history wire is
evidenced.
