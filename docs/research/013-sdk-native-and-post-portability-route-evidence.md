# 013 SDK-Native And Post-Portability Route Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-21

## Question

Which route adds the most architectural information after the xAI, Kimi, and
owned llama.cpp tranche, and does a maintained Rust SDK now justify the
SDK-native lane deferred by Research 005?

## Method

Official provider documentation, maintained-project documentation, registry
release records, and current Swallowtail contracts were checked on 2026-07-21.
No credential store or provider account was inspected. No authenticated or
paid inference request was made.

Candidates were compared on actual Rust embedding, execution placement,
credential and endpoint authority, retry and streaming behavior, catalogue
shape, lifecycle, support authority, and deterministic fixture quality.

## SDK Boundary Recheck

### Harness SDKs And Rust Agent Binaries

The remaining harness SDKs still do not provide a maintained in-process Rust
client boundary:

- Claude Agent SDK supports Python and TypeScript. Other languages are directed
  to the Claude Code CLI, and both SDK packages bundle the native CLI binary.
- Qwen Code's Python and TypeScript SDKs remain experimental clients around an
  external `qwen` process. The TypeScript SDK requires the CLI and the Python
  SDK explicitly fixes process transport.
- Kimi Agent SDK provides Go, Node.js, and Python clients while retaining Kimi
  Code as the execution engine.
- the new `MoonshotAI/kimi-agent-rs` artifact is an experimental, separately
  released Rust implementation of Kimi's Wire server. It is a standalone
  statically linked binary over JSON-RPC stdio, not a public embeddable Rust
  library. It also has a distinct manual API-key and credential-store posture.

These are useful harness and subprocess routes. Calling any of them an
SDK-native Swallowtail driver would hide a CLI, language sidecar, or protocol
boundary.

Evidence:

- [Claude Agent SDK overview](https://code.claude.com/docs/en/agent-sdk/overview)
- [Qwen Code Python SDK](https://qwenlm.github.io/qwen-code-docs/en/developers/sdk-python/)
- [Qwen Code TypeScript SDK](https://qwenlm.github.io/qwen-code-docs/en/developers/sdk-typescript/)
- [Kimi Agent SDK](https://github.com/MoonshotAI/kimi-agent-sdk)
- [Kimi Wire mode and Rust agent](https://moonshotai.github.io/kimi-cli/en/customization/wire-mode.html)
- [Kimi Agent Rust repository](https://github.com/MoonshotAI/kimi-agent-rs)

### Direct Provider SDKs

The direct providers already in Swallowtail's named inventory do not currently
offer a provider-supported Rust SDK:

- OpenAI lists Rust under community libraries, not official SDKs.
- Anthropic's current client SDK list covers Python, TypeScript, C#, Go, Java,
  PHP, and Ruby.
- Google lists official Gemini SDKs for Python, JavaScript/TypeScript, Go, and
  Java. Its guidance sends unsupported languages, including Rust, to the
  language-neutral API.
- xAI's current quickstart offers its Python SDK plus Python and JavaScript
  OpenAI-compatible clients. It does not publish a Rust SDK.

Those APIs remain valid HTTP, SSE, WebSocket, or generated-client candidates.
They do not add the missing provider-maintained Rust embedding boundary.

Evidence:

- [OpenAI libraries](https://developers.openai.com/api/docs/libraries#rust)
- [Anthropic client SDKs](https://platform.claude.com/docs/en/cli-sdks-libraries/overview)
- [Gemini API libraries](https://ai.google.dev/gemini-api/docs/libraries)
- [Gemini partner integration](https://ai.google.dev/gemini-api/docs/partner-integration)
- [xAI quickstart](https://docs.x.ai/developers/quickstart)

### ACP Rust SDK

ACP has a Rust client surface, but the current v1 SDK replacement proposal is
still Preview. Swallowtail already owns and proves bounded ACP v1 framing,
correlation, callbacks, Gemini, and Kimi portability. Adopting the preview SDK
now would add dependency and version churn while largely repeating a proven
transport. Recheck after the replacement SDK is stable or when it exposes a
capability the owned protocol layer cannot represent cheaply.

Evidence:

- [ACP Rust SDK v1 proposal](https://agentclientprotocol.com/rfds/rust-sdk-v1)

## Accepted SDK-Native Candidate

AWS publishes and supports an official SDK for Rust. Amazon Bedrock has
separate generated Rust crates for its control plane and runtime. The current
`aws-sdk-bedrockruntime` release is `1.136.0`, published 2026-07-08, and exposes
typed `ConverseStream` requests and EventStream responses in-process.

This is a real Rust embedding:

- the Swallowtail adapter links the SDK crate directly
- no provider CLI, language runtime, FFI library, container, or sidecar is
  required
- the SDK owns AWS request construction, SigV4 signing, HTTP, TLS, EventStream,
  and typed service errors
- the adapter still owns Swallowtail lifecycle, bounded event projection,
  cancellation, cleanup, and provider-neutral diagnostics

The default SDK posture cannot be inherited silently. It normally searches
environment variables, shared files, web identity, container credentials, and
instance metadata. It also defaults to three attempts and has no total
operation timeout. The first Swallowtail route must instead receive one exact
host-authorized credential provider, set region and endpoint explicitly, fix
maximum attempts to one, and bind all work to the operation deadline.

The SDK uses Tokio privately and its default HTTPS client uses Hyper, rustls,
and `aws-lc-rs`. That concrete runtime and TLS stack may remain adapter-private,
but all work must run inside host-scoped joined work. Swallowtail still creates
no global executor.

Evidence:

- [AWS SDK for Rust Bedrock examples](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/rust_bedrock-runtime_code_examples.html)
- [Bedrock API and Rust SDK crates](https://docs.aws.amazon.com/bedrock/latest/userguide/api-reference-overview.html)
- [`aws-sdk-bedrockruntime` 1.136.0](https://docs.rs/crate/aws-sdk-bedrockruntime/1.136.0)
- [AWS Rust credential providers](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credproviders.html)
- [AWS Rust retries](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/retries.html)
- [AWS Rust timeouts](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/timeouts.html)
- [AWS Rust HTTP client](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/http.html)

## New Architectural Information

Bedrock Runtime exercises boundaries not covered by the existing production
drivers:

- an official provider-maintained SDK linked into the adapter process
- cloud-provider identity and SigV4 signing rather than a bearer API key or
  harness-owned login
- region, partition, service audience, endpoint, and model route as separate
  bindings
- typed AWS EventStream rather than adapter-owned JSON, SSE, WebSocket, ACP, or
  subprocess framing
- SDK-owned retry, timeout, identity caching, HTTP, and async-runtime defaults
  that Swallowtail must make explicit or disable
- cloud-account billing and IAM authorization separated from model access,
  region availability, quota, and runtime readiness
- one gateway integration family serving model routes from several underlying
  model providers without collapsing gateway, provider, route, or model
  identity

Bedrock's control-plane `ListFoundationModels` operation is also distinct from
runtime inference. It reports provider name, model lifecycle, modalities, and
streaming support. The first proof should not combine it with inference: it
requires a second service client and endpoint audience, and catalogue presence
does not prove IAM permission, entitlement, regional availability, or
invocation success for one route.

Evidence:

- [Bedrock Converse API](https://docs.aws.amazon.com/bedrock/latest/userguide/conversation-inference.html)
- [Bedrock Runtime operations](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_Operations_Amazon_Bedrock_Runtime.html)
- [ListFoundationModels](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_ListFoundationModels.html)
- [Bedrock model access](https://docs.aws.amazon.com/bedrock/latest/userguide/foundation-models-reference.html)
- [Bedrock regional model availability](https://docs.aws.amazon.com/bedrock/latest/userguide/models-region-compatibility.html)
- [Bedrock model lifecycle](https://docs.aws.amazon.com/bedrock/latest/userguide/model-lifecycle.html)

## Comparison

| Candidate | New information | Main contract pressure | Rank |
| --- | --- | --- | --- |
| Bedrock Runtime Rust SDK | in-process SDK, cloud identity, SigV4, region/service binding, typed EventStream, SDK defaults | explicit SDK configuration, delegated credential-provider binding, private executor ownership, one-attempt enforcement | 1 |
| Kimi Rust Wire agent | current Rust harness binary, Wire 1.10, smaller process footprint | new provider protocol, manual API-key authority, experimental artifact drift | 2 |
| Bedrock catalogue | separate control plane, provider/model/lifecycle evidence | second endpoint/service audience and catalogue-versus-entitlement rules | 3 |
| ACP Rust SDK | maintained shared client implementation | preview replacement and overlap with proven ACP layer | 4 |
| another hosted HTTP/SSE route | provider and access breadth | mostly repeats Contract 014 transport | 5 |

## Recommendation

Select Amazon Bedrock Runtime `ConverseStream` through
`aws-sdk-bedrockruntime = 1.136.0`.

The first route is direct structured inference for one exact configured model
route and region. It uses one host-approved Bedrock Runtime endpoint, one
host-authorized delegated AWS credential provider, text input/output, one
consumer-owned positive output-token bound, one SDK attempt, no tools, no
guardrail, no prompt resource, no inference profile, no cross-region routing,
no catalogue, no retry, and no ambient AWS configuration.

Access is provider-supported AWS SDK access using cloud-provider identity and
cloud-account billing. IAM authorization, credential validity, model access,
region availability, quota, rate, and runtime readiness remain separate
observations. The route does not reuse Amazon product login, model-provider API
keys, or subscription credentials.

This does not establish a new implicit consumer provider preference. Contract
006 already represents cloud-provider identity and cloud-account billing, the
driver selects no default model or region, and live AWS access remains gated.

## Required Promotion

Contract 019 must settle before production:

- exact SDK artifact identity without confusing SDK, service API, transport,
  configured instance, or model route
- adapter-private async runtime and HTTP implementation inside host-scoped
  joined work
- no default credential chain, region discovery, endpoint discovery, retry, or
  timeout behavior
- delegated use of one exact host-authorized SDK credential provider without
  exposing credentials through common runtime records
- one-attempt inference despite SDK defaults
- typed EventStream order, bounded projection, cancellation, deadline, and
  cleanup
- distinct Bedrock runtime and control-plane drivers

## Concrete Sequence

1. Contract 019 plus a version-pinned typed ConverseStream fixture corpus.
2. Bedrock Runtime structured-run driver and hosted-direct conformance.
3. Recheck the Bedrock control-plane catalogue as a separate driver.
4. Recheck Kimi Rust Wire after its experimental artifact and protocol settle.
5. Recheck ACP's Rust SDK after the replacement proposal leaves Preview.

## Promotion

- durable SDK and cloud-client behavior: Contract 019
- completed runtime delivery: g01 roadmap 020 and cards 067-069
- next evidence gate: roadmap 021 card 070 for the separate Bedrock control
  plane catalogue
