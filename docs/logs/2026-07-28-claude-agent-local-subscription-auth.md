# Claude Agent Local Subscription Auth

Date: 2026-07-28

## Decision

The Claude Agent ACP facade accepts two explicit access profiles:

- local default: `LocalUnauthenticated`, `SubscriptionAllowance`, no
  credential reference, `CredentialState::NotRequired`
- API override: `ApiKey`, `PayAsYouGo`, one credential reference,
  `CredentialState::Ready`

The profiles do not fall back to each other. Both retain the exact
`api.anthropic.com` audience and caller-asserted access evidence.

## Authority Recheck

Anthropic's 2026-06-15 update says the proposed subscription change is paused:
Agent SDK, `claude -p`, and third-party Agent SDK apps still draw from
subscription limits for now.

The current `@agentclientprotocol/claude-agent-acp` package exposes
`claude-agent-acp` as its executable. Its entry point reports the wrapper
version and otherwise starts ACP over stdio through the Claude Agent SDK.

- [Anthropic subscription update](https://support.claude.com/en/articles/15036540-use-the-claude-agent-sdk-with-your-claude-plan)
- [Claude Agent ACP package](https://github.com/agentclientprotocol/claude-agent-acp/blob/main/package.json)
- [Claude Agent ACP entry point](https://github.com/agentclientprotocol/claude-agent-acp/blob/main/src/index.ts)

## Runtime Boundary

Local access launches the approved ACP executable with the approved process
environment. The child uses authentication already held by the local Claude
installation. Swallowtail does not acquire, expose, or release a credential
lease, and the prepared plan does not require a credential host service.

API-key access keeps the existing scope- and audience-bound secret lease,
validation, and credential-last cleanup.

Structured run, interactive session, and inactive provider-session deletion
share the same distinction.

## Consumer Evidence

Figmatic now declares subscription metering and `NotRequired` credential state
when `ANTHROPIC_API_KEY` is absent. When it is present, Figmatic keeps the
pay-as-you-go API-key profile.

The remaining live prerequisite is an installed Claude Agent ACP wrapper. The
plain `claude` executable reports Claude Code version syntax and does not expose
the ACP stdio facade expected by this adapter.
