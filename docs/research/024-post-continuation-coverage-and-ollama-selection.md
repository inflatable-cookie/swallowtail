# 024 Post-Continuation Coverage And Ollama Selection

Status: promoted
Owner: Tom
Updated: 2026-07-23

## Question

What should follow the twentieth production route, and how should Swallowtail
start proving maintained interface-version windows rather than exact current
points only?

## Method

Sources were accessed 2026-07-23. Official provider documentation, maintained
project repositories, tagged source, release records, and public release
pointers are evidence. No account, credential, browser login, paid inference,
binary installation, model download, container, remote agent, or live model
request was used.

The audit starts from twenty production descriptors and twelve common
profiles. It ranks new runtime pressure, authority clarity, compatibility
evidence, operational weight, and overlap with realized mechanisms.

## Realized Route Inventory

| Route | Layer, operation, and transport | Access and topology | Profile |
| --- | --- | --- | --- |
| Codex exec | harness structured run; JSONL CLI | delegated harness auth; local or remote-authoritative process host | one-shot structured CLI |
| Codex app-server | harness interactive session; stdio JSON-RPC | delegated harness auth; local or remote-authoritative process host | long-lived RPC harness |
| OpenCode server | harness interactive session; attached HTTP/SSE | delegated harness auth; attached local or remote-authoritative endpoint | attached network harness |
| Anthropic Messages | direct structured run; HTTP/SSE | public API key; hosted endpoint | hosted direct API |
| Anthropic Managed Agents | remote harness structured run; REST/SSE | public API key; provider-managed resources | provider-managed remote harness |
| Bedrock Runtime | direct structured run; embedded Rust SDK/EventStream | delegated AWS provider; regional hosted service | hosted direct API |
| Bedrock catalogue | direct catalogue; embedded Rust control-plane SDK | delegated AWS provider; regional hosted service | catalogue assertion pack |
| Gemini CLI ACP | harness interactive session; ACP v1 stdio | isolated harness API-key state; local or remote-authoritative process host | long-lived ACP harness |
| Gemini Live | direct realtime session; raw WebSocket | public API query key; hosted preview endpoint | realtime-media direct session |
| Kimi Code ACP | persistent harness session; ACP v1 stdio | delegated membership auth; local or remote-authoritative process host | persistent ACP harness |
| Kimi Platform K3 | direct structured run; HTTP/SSE | Platform API key and pay-as-you-go access; hosted endpoint | hosted direct API |
| llama.cpp attached | direct structured run and catalogue; attached HTTP/SSE | local unauthenticated endpoint; external deployment | attached self-hosted |
| llama.cpp owned | direct structured run, catalogue, and serving lifecycle; process plus HTTP/SSE | host-approved artifact and process; ephemeral local or remote-authoritative deployment | owned self-hosted |
| OpenAI background Responses | direct structured run; HTTP/SSE with cursor reattachment | public API key; temporary provider retention | hosted direct API plus background assertions |
| OpenAI Realtime | direct realtime session; WebSocket | public API key; hosted GA endpoint | realtime-media direct session |
| Qwen Code headless | harness structured run; streaming-JSON CLI | harness-owned auth; local or remote-authoritative process host | one-shot structured CLI |
| xAI Responses WebSocket | direct interactive session; WebSocket | public API key; hosted endpoint | connection-scoped direct session |
| Alibaba Model Studio conversation | direct interactive session; HTTP/SSE | regional workspace API key; provider conversation retention | provider-conversation assertion pack |
| Pi RPC | harness interactive session; strict-LF JSONL stdio | delegated downstream auth; local or remote-authoritative process host | long-lived RPC harness plus scheduling/UI assertions |
| DeepSeek V4 | locally continued direct session; buffered HTTP plus SSE | Open Platform API key and balance; hosted endpoint | locally continued direct session |

The twelve provider-neutral profiles are:

1. one-shot structured CLI
2. long-lived RPC harness
3. long-lived ACP harness
4. persistent ACP harness
5. attached network harness
6. hosted direct API
7. provider-managed remote harness
8. connection-scoped direct session
9. realtime-media direct session
10. attached self-hosted
11. owned self-hosted
12. locally continued direct session

Background execution, provider conversations, catalogues, planned connection
rollover, harness isolation, RPC scheduling, and callback UI remain additive
assertion packs. They are not hidden extra profiles.

## Compatibility Posture

Only Pi and DeepSeek publish Contract 029 compatibility claims through their
driver descriptors.

| Interface group | Current qualification | Maintained-window posture |
| --- | --- | --- |
| Pi RPC | semantic package `0.80.10` | one-point maintained window; descriptor claim and live probe |
| DeepSeek OpenAI facade | opaque `deepseek-openai-chat-2026-07-22` | one-point maintained window; descriptor claim; live auth remains gated |
| Codex exec and app-server | `codex-cli 0.144.6` evidence point | no descriptor claim; installed-version range unqualified |
| OpenCode HTTP | executable `1.14.48` | exact runtime rejection; no descriptor claim or range |
| Gemini ACP | executable `0.51.0`, ACP v1, schema `v1.19.0` | exact runtime rejection; protocol and harness axes separate; no range |
| Kimi ACP | executable `0.28.1`, ACP v1, schema `v1.19.1` | exact runtime rejection; protocol and harness axes separate; no range |
| Qwen headless | package `0.19.11` and source commit | exact event rejection; no descriptor claim or range |
| llama.cpp attached and owned | builds `9910` and `10069` | exact build/facade rejection; no range |
| Bedrock Runtime and catalogue | SDK crates `1.136.0` and `1.148.0` | exact dependency pins; no declared SDK range |
| Anthropic direct and managed | API header `2023-06-01`; beta `managed-agents-2026-04-01` | dated hosted facades; no descriptor claims |
| Kimi Platform | `kimi-k3` corpus dated 2026-07-21 | dated hosted facade; no descriptor claim |
| xAI WebSocket | guide revision 2026-04-23; evidence snapshot 2026-07-20 | dated hosted facade; no descriptor claim |
| OpenAI background and Realtime | facades dated 2026-07-21 and 2026-07-22 | dated hosted facades; no descriptor claims |
| Gemini Live | `v1beta` preview corpus dated 2026-07-22 | exact preview facade/model; no descriptor claim |
| Alibaba Model Studio | Singapore conversation facade dated 2026-07-22 | dated regional facade; no descriptor claim |

No production driver currently claims a non-singleton maintained range.
Exact fixture dates remain useful evidence but are not substitutes for
machine-readable compatibility claims. Hosted unversioned APIs need dated
facade claims, not invented semantic versions.

The highest maintenance risk is installed software: Codex, OpenCode, Gemini,
Kimi, Qwen, Pi, and local serving runtimes can differ across client devices.
These routes need deliberate baseline, latest-qualified, milestone,
deprecation, exclusion, and live-probe policy. A broad retrofit should follow
one production proof of the complete Contract 029 workflow.

## Current Candidate Evidence

### Ollama is the best next proof

Ollama exposes a native local HTTP API, exact `/api/version`, installed-model
inventory through `/api/tags`, running-model inventory through `/api/ps`,
model details through `/api/show`, and NDJSON chat streaming. Local access
needs no credential. Cloud access and local sign-in are separate surfaces.

The project states that its native API is not strictly versioned but is
expected to remain stable and backward compatible; deprecations are announced
in release notes. Stable `v0.32.1` is the latest non-prerelease point. `v0.32.2`
is marked prerelease and `v0.32.3-rc0` is a release candidate, so neither
belongs in the first claim.

Tagged API documentation preserves the selected version, tags, ps, show, and
text-chat subset from `v0.14.0` through `v0.32.1`. Those stable releases span
2026-01-10 through 2026-07-16. `v0.18.0` and `v0.30.0` provide intermediate
qualification points. Later tool-index and thinking-level changes do not enter
the text-only first subset.

This route adds four useful pressures:

- the first non-singleton compatibility window
- exact service-version discovery before preflight
- native installed versus running model observation
- invocation-caused model residency on an externally owned deployment

The route is attach-only. It needs no container and does not install Ollama,
download a model, accept a license, create or delete a model, stop the server,
unload resident state, or use Ollama Cloud.

Evidence:

- [API placement and compatibility posture](https://docs.ollama.com/api/introduction)
- [exact runtime version](https://docs.ollama.com/api-reference/get-version)
- [installed models](https://docs.ollama.com/api/tags)
- [running models](https://docs.ollama.com/api/ps)
- [model details](https://docs.ollama.com/api-reference/show-model-details)
- [native chat](https://docs.ollama.com/api/chat)
- [local and cloud authentication separation](https://docs.ollama.com/api/authentication)
- [stream failure behavior](https://docs.ollama.com/api/errors)
- [v0.14.0 baseline](https://github.com/ollama/ollama/releases/tag/v0.14.0)
- [v0.18.0 checkpoint](https://github.com/ollama/ollama/releases/tag/v0.18.0)
- [v0.30.0 checkpoint](https://github.com/ollama/ollama/releases/tag/v0.30.0)
- [v0.32.1 latest qualified candidate](https://github.com/ollama/ollama/releases/tag/v0.32.1)

### Grok Build is current but overlaps realized harness shapes

Grok Build supports interactive TUI, headless streaming JSON, and ACP stdio.
Browser OIDC, device auth, external auth providers, and API keys are distinct.
The public stable pointer currently reports `0.2.111`, and the CLI accepts an
exact version install plus `--no-auto-update`.

That is useful first-party xAI harness breadth, but ACP and one-shot streaming
JSON already have multiple production proofs. It follows the compatibility
and attached-runtime lane unless a consumer makes Grok harness access a
priority.

Evidence:

- [Grok Build overview](https://docs.x.ai/build/overview)
- [headless and ACP use](https://docs.x.ai/build/cli/headless-scripting)
- [enterprise authentication](https://docs.x.ai/build/enterprise)
- [stable release pointer](https://x.ai/cli/stable)

### Remote ACP moved backward

The Streamable HTTP and WebSocket transport RFD is Draft, not Active. It still
requires HTTP/2, dual connection/session identities, long-lived connection and
session streams, mandatory WebSocket client support, and implementer-owned
reconnect, affinity, and liveness behavior. Protocol-version headers and
resumability remain future hardening.

It is not ready for a production shared-transport claim.

Evidence:

- [RFD status updates](https://agentclientprotocol.com/rfds/updates)
- [current transport proposal](https://agentclientprotocol.com/rfds/streamable-http-websocket-transport)

### Claude local access remains authority-split

The Agent SDK is provider-supported for API-key, Bedrock, Vertex, and Foundry
access. Its current overview says unapproved third parties must not offer
Claude.ai login or subscription rate limits. A newer Help Center notice says
Agent SDK, `claude -p`, and third-party app use currently draw from subscription
limits while a billing change is paused.

Those first-party statements do not define one clean third-party subscription
route. An API-key-only SDK proof is possible later. Subscription-backed
Swallowtail access remains gated on explicit provider authority.

Evidence:

- [Agent SDK overview and third-party restriction](https://code.claude.com/docs/en/agent-sdk/overview)
- [current subscription notice](https://support.claude.com/en/articles/15036540-use-the-claude-agent-sdk-with-your-claude-plan)
- [Claude Code authentication precedence](https://code.claude.com/docs/en/authentication)

### Cursor remains a beta bridge

Cursor's public-beta SDK supports local and cloud agents, streaming, custom
tools, custom stores, cancellation, and auto-review. Local headless calls run
tools without approval unless auto-review is enabled. The Rust integration
still needs a TypeScript or Python bridge and a provider package whose local
runtime changes quickly. The cloud route adds durable resources, billing,
archive, deletion, and remote-workspace policy already represented elsewhere.

Evidence:

- [SDK release](https://cursor.com/changelog/sdk-release)
- [SDK lifecycle and permission updates](https://cursor.com/changelog/sdk-updates-jun-2026)

### Z.AI is compatible breadth with strict plan authority

The general API and GLM Coding Plan use separate endpoints. Coding Plan quota
is restricted to supported tools and cannot authorize arbitrary application
inference. The general API offers the clearer future Swallowtail route but
largely repeats compatible hosted chat already covered by Kimi Platform and
DeepSeek.

Evidence:

- [general and Coding Plan endpoint separation](https://docs.z.ai/api-reference/introduction)
- [Coding Plan restrictions](https://docs.z.ai/devpack/faq)
- [subscription terms](https://docs.z.ai/legal-agreement/subscription-terms)

### vLLM and SGLang remain heavier deployment breadth

Both expose broad compatible serving and operational surfaces. Their useful
Swallowtail route remains attach-only, with deployment administration excluded.
They add less new information than Ollama's native catalogue and compatibility
window and usually carry materially heavier runtime setup.

Evidence:

- [vLLM serving surface](https://docs.vllm.ai/en/latest/serving/online_serving/openai_compatible_server/)
- [SGLang serving documentation](https://docs.sglang.ai/)

## Ranking

| Rank | Route | New pressure | Decision |
| --- | --- | --- | --- |
| 1 | Ollama native attached | first maintained range, exact runtime version, installed/running catalogue, residency truth, local NDJSON | select |
| 2 | Grok Build `0.2.111` | first-party xAI harness breadth | later; ACP and JSONL overlap |
| 3 | Claude Agent SDK API-key-only | foreign SDK bridge and local agent loop | later; subscription authority unresolved |
| 4 | Cursor local SDK | local/cloud bridge, custom stores and tools | later; public beta and heavier bridge |
| 5 | Z.AI general API | GLM semantics and access separation | later compatible breadth |
| 6 | remote ACP | shared remote protocol | wait; RFD is Draft |
| 7 | vLLM or SGLang attached | deployment breadth | later; operationally heavier |

## Decision

Select an attach-only Ollama native driver.

The bounded first proof is:

- maintained semantic-version window `0.14.0` through `0.32.1`
- qualification points `0.14.0`, `0.18.0`, `0.30.0`, and `0.32.1`
- one text-only behavior segment; no inferred tool or thinking compatibility
- exact `/api/version` observation before plan binding
- host-approved loopback native endpoint with local unauthenticated access
- bounded `/api/tags`, `/api/ps`, and `/api/show` observation
- one exact operator-selected installed model tag and digest
- one resource-free direct structured run through streaming `/api/chat`
- explicit output bound, one attempt, no retry, no fallback
- explicit acceptance that inference may load or refresh runtime-managed model
  residency; no unload or restoration claim
- local and remote-authoritative host conformance through the same driver seam

Excluded:

- Ollama installation, update, sign-in, cloud endpoint, cloud model, and API key
- model pull, push, copy, create, delete, unload, or keep-alive administration
- owned server lifecycle, artifact acquisition, Monkey authority, and container
- tools, thinking, vision, embeddings, generation, OpenAI compatibility, and
  Anthropic compatibility
- broad `0.x` compatibility outside the tested closed window
- prerelease `0.32.2`, `0.32.3-rc0`, and unknown newer releases
- live model inference from default QA

## Required Promotion

Contract 031 fixes attached native-runtime version, catalogue, and residency
truth. Roadmap 038 and cards 106-109 own records, the multi-release corpus,
production driver, portability conformance, and closeout.

After that proof, re-audit installed harness ranges. Codex should be the first
retrofit candidate because both production drivers depend on client-device
releases and currently expose no descriptor compatibility claim.
