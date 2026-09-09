# 300 Claude SDK Structured Provider Failure Evidence

Status: complete; promoted into Contract 019 and Card 152
Owner: Swallowtail Chatterbox
Created: 2026-09-09

## Question

Can `claude-agent.sdk` return a billing-specific failure without forwarding or
parsing raw provider error text?

## Evidence

Desktop Card 316's immutable capsule records a successful registered-MCP open,
one prompt, and a failed result whose fixed presence map has
`api_error_status=true` and `terminal_reason=true`. The sidecar at exact source
`0d120067` intentionally forwards only field presence and collapses valid
`rate_limit_event.status` values into generic progress. The numeric status,
terminal reason, and fixed rate-limit state were therefore discarded before
the capsule and cannot be reconstructed from it.

The official Claude API error contract documents `402` as `billing_error`.
It also documents the required ambiguity: `400` covers invalid requests and
organization/workspace spend limits, while `429` covers rate limits, usage-tier
monthly spend caps, and Claude Code workspace spend limits. The official Agent
SDK error surface exposes `api_error_status` and `terminal_reason` as structured
attributes while retaining provider prose separately.

Sources:

- <https://platform.claude.com/docs/en/api/errors>
- <https://github.com/anthropics/claude-agent-sdk-python/blob/main/src/claude_agent_sdk/_errors.py>
- Desktop Card 316 capsule SHA-256
  `d16b7f8d86513633704a82a6901746dba0cf8dd57d36a5948f1f43e007aa4c28`

## Decision

Project the exact safe numeric status, bounded terminal reason, and fixed
rate-limit status without forwarding error text. Emit a billing/entitlement
classification only for `402`. Preserve explicit mixed categories for `400`
and `429`; never infer quota exhaustion from either alone. Unknown values stay
generic or fail closed. No classification creates retry authority.

## Consequence

Card 152 repairs the provider-free producer path. After its exact reviewed
merge, Desktop may consume the operator's already-granted one-shot authority
for one final zero-credit diagnostic, provided the balance has not been topped
up. That run is evidence only and does not qualify registered tools or release.
