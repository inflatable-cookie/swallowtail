# 108 Provider-Wide Session Usability Restoration

Status: promoted
Owner: Tom
Date: 2026-08-05

## Question

Which remaining prepared routes can join the common working-state restoration
facade without inventing provider continuity or retrying one-shot work?

## Existing Interactive Boundary

Four unmapped route identities already expose an ordinary prepared interactive
open path:

| Route | Prepared request | Qualified restart action |
| --- | --- | --- |
| `ollama.attached` | `OpenSessionRequest` | fresh session replacement |
| `xai.responses-websocket` | `OpenSessionRequest` | fresh session replacement |
| `anthropic.messages` | `OpenDirectContinuationSessionRequest` | adapter-local fresh session replacement |
| `deepseek.continuation` | `OpenDirectContinuationSessionRequest` | adapter-local fresh session replacement |

The ordinary Alibaba delete-on-close profile can also select fresh replacement
without adding a route identity. Its separate retained profile keeps the
stronger continuation-recovery mapping.

Ollama private transcript history, xAI connection state, and direct-provider
continuation state are process-local. Replacement restores route usability
only. It never replays prompts, tools, transcript, or provider work.

## Direct Private Continuation

Anthropic Messages and DeepSeek need provider-private reasoning or
continuation material to form later exact requests. Contract 030 keeps that
material bounded, zeroizing, non-serializable, and tied to one exact runtime
session, route, model, access profile, and configured instance. It is destroyed
on close or invalidation.

Visible assistant text and tool envelopes are not a reconstruction format for
the hidden provider material. Replaying them after a crash could change
reasoning, tool correlation, cache behavior, or provider acceptance. These
routes therefore qualify only for explicit fresh replacement.

## Realtime Boundary

`openai.realtime` and `gemini.live` expose prepared
`OpenRealtimeMediaSessionRequest` operations and
`RealtimeMediaSessionHandle`, not `InteractiveSessionHandle`. Both can open a
new usable media connection after restart, but neither can restore the lost
audio buffer, transcript, active response, provider continuation, or terminal
state.

Contract 050 needs one distinct `FreshRealtimeSessionReplacement` method and
`RealtimeSessionReplaced` outcome. Gemini planned rollover remains scoped to a
live operation under Contract 027 and grants no cross-process recovery.

## Exclusions And Stronger Gates

- catalogue, serving, and one-prompt routes remain outside automatic retry
- Gemini ACP still needs replay-readiness or exact attachment evidence
- Pi RPC still needs caller-bound and corroborated effective cwd
- Cursor needs client-visible replay failures before complete recovery
- Grok needs inspectable control flow or deterministic replay evidence
- Antigravity and Qwen need public durable attachment or load surfaces before
  they can exceed replacement

## Decision

Promote the four interactive route mappings, ordinary Alibaba replacement, and
two realtime replacement mappings. Keep all outcomes inside the existing
static, exact-once facade. Preserve every stronger-recovery gate unchanged.

No authenticated provider work or external mutation supports this decision.
The evidence is the existing prepared API, runtime contracts, and frozen route
corpora.
