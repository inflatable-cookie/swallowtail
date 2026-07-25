# 027 Kimi Platform And DeepSeek Facades

Status: complete
Owner: Tom
Created: 2026-07-25
Milestone: `../010-hosted-direct-and-provider-state-facades.md`

## Objective

Add prepared direct-inference paths for the two qualified compatible-chat
providers without merging provider semantics.

## Governing Refs

- Contracts 014, 020, 024, 029-030, and 037
- current Kimi Platform and DeepSeek corpora
- card 023

## Scope

1. Share only structural endpoint, credential, catalogue, and chat-codec
   preparation where already provider-neutral.
2. Keep provider access, facade revision, route, model, reasoning, usage,
   caching, and failure mapping adapter-owned.
3. Bind catalogue, initial attempt, and consumer-authorized tool continuation.
4. Preserve Kimi membership and Platform credential/billing distinction.
5. Add no compatible-provider fallback.

## Acceptance Criteria

- [x] each facade exposes its exact endpoint audience and access profile
- [x] compatible JSON structure does not imply semantic compatibility
- [x] each tool continuation attempt is explicitly authorized
- [x] model and provider evidence cannot drift
- [x] credential leases release after joined work

## Evidence

- separate adapter-local catalogue plus K3 structured-attempt and DeepSeek
  direct-continuation prepared values
- Kimi Membership metering and alternate-model failure before effects
- DeepSeek exact endpoint, V4 Pro, cache-acceptance, and tool-result
  authorization checks
- local and remote-authoritative fixture execution
- 34 adapter tests and examples across both packages
- full Effigy validation recorded in
  `../../../logs/2026-07-25-kimi-platform-and-deepseek-prepared-facades.md`

## Validation

- both dated offline corpora
- hosted-direct and direct-tool conformance
- both host identities
- low-level regression suites

## Auto-Continuation

Yes. Continue to card 028.
