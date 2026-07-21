# 067 Post-Portability Coverage Evidence

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../020-post-portability-coverage-expansion.md`

## Objective

Select the next high-information production proof from current authoritative
evidence after the xAI, Kimi, and owned llama.cpp tranche.

## Scope

- recheck maintained Rust SDKs and explicit supported embedding boundaries
- distinguish in-process SDK use from CLI, subprocess, FFI, and language-
  sidecar transports
- compare remaining hosted catalogue, direct-inference, harness, shared-
  protocol, and attached-runtime routes
- verify access, credential, endpoint audience, entitlement, metering, support
  authority, model catalogue, lifecycle, and topology behavior independently
- rank candidates by new architectural information and deterministic fixture
  quality
- promote a research delta and select one concrete route or one explicit
  prerequisite lane

## Out Of Scope

- provider implementation
- live authentication or paid inference
- implicit consumer routing preference
- treating provider count as architectural value

## Acceptance Criteria

- [x] current official provider or maintained-project sources are cited
- [x] at least one SDK-native candidate is accepted or rejected against an
      exact embedding boundary
- [x] the selected route exercises behavior not already covered by a cheaper
      existing profile
- [x] missing shared contracts are named before implementation
- [x] access and support authority are explicit
- [x] roadmap 020 cards 068-069 are narrowed to one coherent route

## Evidence

- Research 013 rechecks official OpenAI, Anthropic, Google, xAI, AWS, ACP,
  Qwen, and Kimi surfaces against the exact Rust embedding boundary
- Claude, Qwen, and Kimi SDKs still use non-Rust clients and provider binaries;
  Kimi's new Rust agent is an experimental Wire-mode binary, not a library
- OpenAI Rust remains community-supported; Anthropic, Gemini, and xAI publish
  no provider-supported Rust client
- ACP's proposed replacement Rust SDK remains Preview and overlaps the already
  proven owned ACP layer
- official `aws-sdk-bedrockruntime = 1.136.0` is accepted as the first true
  in-process Rust SDK route
- the selected `ConverseStream` proof adds cloud identity, SigV4, explicit
  region/service binding, typed EventStream, SDK retry/timeout defaults, and
  cloud-account billing
- Contract 019 promotes the missing shared SDK, delegated credential-provider,
  private executor, one-attempt, and control-plane separation rules
- access is provider-supported AWS SDK use with cloud-provider identity and
  cloud-account billing; IAM, model access, region, rate, quota, and readiness
  remain separate
- no model, region, credential, endpoint, provider, or consumer routing default
  is selected

## Validation

- source and release currentness checks
- contract and architecture consistency review
- `effigy qa:docs`
- `git diff --check`

## Stop Conditions

- current evidence leaves two policy-significant choices genuinely tied
- a candidate depends on private credentials, unsupported OAuth reuse, or an
  unowned serving lifecycle
- the only SDK route is an undisclosed or mislabeled language bridge

## Auto-Continuation

No. Return the evidence-backed selection to the operator if it establishes new
provider or access policy; otherwise continue to card 068.
