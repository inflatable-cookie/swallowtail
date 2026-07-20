# 2026-07-19 Execution Layer and Access Boundary

Status: complete
Owner: Tom

## Change

Separated four independent concerns:

- harness interaction versus direct model inference
- interactive session versus bounded structured run
- driver and transport
- credential mechanism, entitlement, endpoint audience, and support authority

Contract 006 permits subscription-backed harness and direct-inference routes
without assuming credential shape determines entitlement or endpoint scope.

## Evidence

Current evidence proves three materially different shapes:

- OpenCode uses ChatGPT OAuth for direct Responses-shaped traffic to the
  product-scoped Codex endpoint.
- Claude subscriptions support Claude Code and third-party Agent SDK use, while
  arbitrary OAuth routing to the public Messages API is prohibited.
- Kimi issues normal API keys whose direct API usage consumes membership
  benefits rather than pay-as-you-go Platform billing.

## Consequence

Soundcheck can prefer a discovered signed-in harness, a subscription-backed
direct inference route, or a usage-billed public API. Nucleus can prefer harness
drivers while using direct inference for narrow work. Consumer policy controls
every cross-layer or cross-access-profile fallback.
