# 083 Direct Provider Compatibility Evidence

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../027-direct-provider-compatible-codec-checkpoint.md`

## Objective

Revalidate four current direct-provider families, identify their honest shared
codec seam, and select one bounded breadth proof or one explicit prerequisite.

## Governing References

- Research 003 and 017
- Contracts 005, 006, 011, 014, 016, and 020
- roadmap 027
- realized Anthropic, xAI, OpenAI, Bedrock, and llama.cpp direct routes

## Scope

- official current Kimi Platform, DeepSeek, Z.AI, and Alibaba Model
  Studio/Qwen documentation
- native, OpenAI-compatible, and Anthropic-compatible request and stream
  surfaces where provider-supported
- endpoint audience, key kind, entitlement, metering, regional or workspace
  binding, and support authority
- catalogue, streaming, reasoning, tools, structured output, usage, rate,
  quota, errors, cancellation, retention, and state behavior
- comparison against existing hosted-direct codecs and conformance profiles
- one dated Research 018 delta and a concrete selected-route recommendation

## Acceptance Criteria

- [x] every claim uses current official provider evidence with access date
- [x] Kimi membership-backed access stays distinct from Kimi Platform access
- [x] Z.AI Coding Plan stays distinct from the general API; Alibaba region and
      workspace authority remain explicit
- [x] wire compatibility is separated from provider semantics and lifecycle
- [x] shared codec candidates name exact reusable and provider-specific layers
- [x] the selection maximizes provider breadth without inventing fallback or
      default routing
- [x] missing contracts are promoted before implementation cards are compiled
- [x] one next task remains, or operator input is requested for a genuine policy
      tie

## Validation

- official-link and currentness review
- contract and architecture consistency review
- `effigy qa:docs`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- official docs do not settle endpoint, credential, entitlement, or support
  authority
- compatibility requires exposing raw provider payloads through stable APIs
- provider choice would establish consumer routing, billing, or credential
  policy
- the proposed proof repeats an existing adapter without a concrete breadth or
  reuse gain

## Auto-Continuation

No. Compile implementation only after the evidence selects an exact route and
shared boundary. Ask the operator when the remaining choice is product policy.

## Completion Evidence

- Research 018 revalidates all four providers from official sources accessed
  2026-07-21
- Kimi Open Platform pay-as-you-go keys remain distinct from Kimi Membership,
  Kimi Code, and regional platform keys
- Z.AI Coding Plan and Alibaba Coding Plan stay outside custom direct-inference
  use; general API, region, workspace, endpoint, and key identity remain exact
- Contract 024 limits reuse to bounded Chat Completions wire structure and
  leaves semantic mapping inside each provider adapter
- Kimi Platform K3 is selected with exact model, endpoint audience, catalogue,
  reasoning, output-bound, one-attempt, and no-fallback posture
- roadmap 028 and cards 084-086 compile the fixture, driver, conformance, and
  closeout runway
