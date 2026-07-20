# 002 Execution Layer and Access Profile Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-19

## Question

How should Swallowtail separate agent-harness control from direct model
communication while supporting both configured API access and useful existing
provider subscriptions?

## Operator Requirement

- Nucleus needs deep harness interaction.
- Soundcheck needs small bounded model tasks and should work through an existing
  local signed-in harness before API configuration.
- Either consumer may use both execution layers.
- Provider subscriptions should be usable wherever the provider or a maintained
  integration supports application or automation access, with authority and
  policy status visible.

## Authoritative Evidence

Sources accessed 2026-07-19.

### OpenAI

- [Codex authentication](https://developers.openai.com/codex/auth/) separates
  ChatGPT sign-in for subscription access from API-key sign-in for usage-based
  access. ChatGPT desktop, Codex CLI, and the IDE extension support both for
  local work.
- The same authentication guide permits Codex access tokens for trusted
  enterprise automation and explicitly directs general OpenAI API calls to
  Platform API keys.
- The [Codex SDK](https://developers.openai.com/codex/sdk/) controls local Codex
  agents. The Python SDK controls a local Codex app-server; it is not the same
  surface as a direct Responses API call.
- The [OpenAI API reference](https://developers.openai.com/api/reference/overview)
  documents Responses as the direct model surface and accepts API keys or
  workload-identity access tokens.
- [OpenAI billing guidance](https://help.openai.com/en/articles/8156019-how-can-i-move-my-chatgpt-subscription-to-the-api)
  states that ChatGPT and API service are billed and managed separately.
- [OpenCode provider documentation](https://opencode.ai/docs/providers#openai)
  supports ChatGPT Plus/Pro browser authentication as an alternative to an
  OpenAI API key.
- OpenCode's maintained
  [Codex authentication driver](https://github.com/anomalyco/opencode/blob/dev/packages/opencode/src/plugin/codex.ts)
  obtains OpenAI OAuth credentials and rewrites Responses-shaped requests to
  the subscription-backed Codex Responses endpoint.

Finding: OpenCode demonstrates ChatGPT subscription access for both harness
control and direct Responses-shaped model communication. The direct route is
product-scoped to Codex rather than the separately billed public Platform API.
OpenCode documents and maintains that integration; current OpenAI public docs
establish the Codex OAuth and endpoint ingredients but do not present the
public Platform API as subscription-included.

### Anthropic

- [Claude Code authentication](https://code.claude.com/docs/en/authentication)
  supports Claude subscription login, Console/API credentials, cloud-provider
  credentials, gateways, and provider-issued automation tokens.
- `claude setup-token` can create a long-lived OAuth token for scripts using an
  eligible Claude subscription. The documented token is scoped to model
  requests through Claude Code surfaces and has explicit feature limits.
- The same guide distinguishes subscription OAuth credentials from
  `ANTHROPIC_API_KEY` direct API credentials and gives API keys higher
  precedence when configured.
- [Claude billing guidance](https://support.claude.com/en/articles/9876003-i-have-a-paid-claude-subscription-pro-max-team-or-enterprise-plans-why-do-i-have-to-pay-separately-to-use-the-claude-api-and-console)
  states that paid Claude subscriptions do not include Claude API or Console
  access.
- Current [Agent SDK subscription guidance](https://support.claude.com/en/articles/15036540-use-the-claude-agent-sdk-with-your-claude-plan)
  says Agent SDK, `claude -p`, and third-party Agent SDK applications may draw
  from subscription usage.
- [Claude Code legal and compliance guidance](https://code.claude.com/docs/en/legal-and-compliance)
  restricts subscription OAuth to Claude Code, native Anthropic applications,
  and supported Agent SDK use. It prohibits third-party applications from
  offering Claude login or routing subscription credentials outside those
  supported surfaces.

Finding: Claude offers provider-supported subscription-backed programmatic use
through Claude Code, `claude -p`, and the Agent SDK. The Agent SDK still runs
Claude Code's agent loop, so it is a harness route even for a one-shot query.
Arbitrary direct Messages API calls with subscription OAuth are not currently
an authorized third-party route.

### Kimi

- The [Kimi Code overview](https://www.kimi.com/code/docs/en/) documents a
  membership-backed API for third-party tools using OpenAI-compatible and
  Anthropic-compatible endpoints.
- Members create normal API keys in the Kimi Code Console. Membership tier
  determines available models, context, and rate limits.
- Kimi distinguishes the membership-backed Kimi Code API from the separately
  billed Kimi Platform API; their keys and endpoints are not interchangeable.
- The [third-party agent guide](https://www.kimi.com/code/docs/en/third-party-tools/other-coding-agents)
  explicitly supports using membership API keys in other coding agents.

Finding: Kimi is a provider-supported example where a conventional API key
authenticates direct model traffic whose entitlement and metering come from a
subscription rather than pay-as-you-go API billing.

## Promoted Model

Swallowtail needs five independent axes:

1. **Execution layer** — harness or direct model inference.
2. **Operation shape** — interactive session or bounded structured run.
3. **Driver and transport** — CLI, app-server, SDK, HTTP API, local runtime, or
   another supported surface.
4. **Access profile** — credential mechanism, commercial entitlement/metering,
   and endpoint audience.
5. **Support authority** — provider-supported, integration-maintainer-supported,
   experimental/observed, or prohibited.

The axes cannot substitute for one another. In particular:

- a bounded structured run may use either execution layer
- a simple prompt sent through Codex or Claude Code is still harness-backed
- a provider API is still direct inference when it offers tools or state
- one driver may support several access profiles
- API key does not imply pay-as-you-go billing; OAuth does not imply harness
- entitlement or endpoint audience may change available models, limits,
  administration, privacy posture, and billing without changing the family
- support authority remains visible rather than being inferred from technical
  interoperability

## Consumer Consequence

Soundcheck may expose several explicit routes for the same tagging operation:

1. use a discovered, signed-in Codex or Claude Code harness
2. use ChatGPT OAuth with a Codex-scoped direct inference driver
3. use a membership-backed Kimi Code API key
4. use a usage-billed public model API

Nucleus may favor harness routes while retaining direct inference for narrow
features. Route selection and fallback policy remain consumer-owned. Swallowtail
must never cross credential, entitlement, endpoint-audience, support-authority,
or billing boundaries silently.

## Security Consequence

- Swallowtail adapters invoke provider or integration-maintainer OAuth flows,
  clients, SDKs, APIs, and credential helpers with visible support authority.
- Swallowtail does not scrape browser state or copy tokens from another
  application's credential store without an explicitly supported sharing path.
- A host resolves credential references and owns secret storage, injection,
  user consent, and remote placement.
- Credentials are used only with their authorized endpoint audience.
- Provider-prohibited routes are not implementation candidates even when they
  are technically reproducible.

## Promotion

- durable execution and access rules: Contract 006
- identity correction: Contract 005
- integration inventory dimensions: g01 card 007
