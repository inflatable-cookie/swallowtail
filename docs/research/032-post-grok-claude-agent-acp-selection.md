# 032 Post-Grok Claude Agent ACP Selection

Status: promoted
Owner: Tom
Updated: 2026-07-24

## Question

Which post-Grok route adds the most architectural information without a live
development account, implicit heavy container, or another near-duplicate
provider adapter?

## Method

Evidence was accessed 2026-07-24.

- inventoried realized Swallowtail descriptors, integration families,
  transports, conformance profiles, and maintained harness ranges
- read current first-party provider and maintained-project documentation
- read the live ACP registry aggregate
- inspected public npm metadata, exact package manifests, tagged source,
  changelog, and tests without installing or launching a package
- compared account-at-use, account-for-development, container, serving,
  licensing, support, compatibility, and fixture costs

No package was installed or executed. No login, credential, provider request,
paid inference, container, model server, or external mutation was used.

## Realized Coverage

Swallowtail has:

- 21 production driver descriptors
- 16 integration families
- 13 provider-neutral conformance profiles
- ACP v1 over bounded local NDJSON stdio
- provider-neutral remote ACP over HTTP/2 SSE and WebSocket
- hosted HTTP/SSE and WebSocket transports
- native Rust SDK, structured CLI, long-lived RPC, direct inference,
  provider-managed harness, attached runtime, and owned ephemeral runtime
  proofs

Codex, OpenCode, Kimi Code, and Ollama have multi-release evidence. Gemini CLI,
Pi, and Qwen remain exact or narrow claims. Existing direct routes cover
Anthropic, Bedrock, OpenAI, xAI, Kimi Platform, DeepSeek, Alibaba Model Studio,
llama.cpp, and Ollama shapes.

The highest remaining information gaps are:

- another maintained harness family over a reusable protocol
- composite wrapper, SDK, native-binary, wire, and model version truth
- broader harness lifecycle and permission evidence without sandbox conflation
- direct Z.AI coverage
- deeper ranges for Gemini, Pi, and Qwen
- attached vLLM or SGLang behavior without owning persistent serving

## Current ACP Evidence

The ACP registry contains 38 agents. It remains distribution evidence, not
installation, access, entitlement, compatibility, or support authority.

Material current entries include:

- Claude Agent `0.61.0`, distributed as
  `@agentclientprotocol/claude-agent-acp@0.61.0`
- Cursor `2026.07.23`, distributed as first-party platform archives and invoked
  with `cursor-agent acp`
- Gemini CLI `0.52.0`
- Qwen Code `0.21.0`, invoked with `--acp --experimental-skills`
- community Pi ACP `0.0.32`
- community GLM Agent `1.3.0`

Cursor is a new first-party registry route. Pi ACP and GLM Agent are not
provider-owned merely because their downstream models have provider names.

## Claude Agent ACP Evidence

### Artifact and composition

The exact published `0.61.0` package:

- is an Apache-2.0 ACP adapter maintained in the
  `agentclientprotocol/claude-agent-acp` repository
- requires Node 22 or later
- pins `@agentclientprotocol/sdk` `1.3.0`
- pins `@anthropic-ai/claude-agent-sdk` `0.3.217`
- invokes the Agent SDK's platform-specific native Claude Code binary
- reports the adapter version through `--version`
- forwards `--cli` commands to the nested native binary

The ACP registry separately labels the aggregate entry proprietary and names
Anthropic, Zed Industries, and JetBrains as authors. Package license, registry
label, source license, underlying SDK terms, and support authority remain
source-scoped.

The adapter repository contains mock-backed tests for initialization,
authorization, provider routing, new/load/resume sessions, cancellation,
configuration, models, modes, permissions, tools, usage, and failure recovery.
That is enough to build an independent deterministic corpus without a provider
account.

### Lifecycle and capability

The current adapter advertises ACP wire version 1 and supports:

- new, load, resume, list, fork, close, and delete session methods
- image and embedded-context prompts
- streamed text, thinking, tool, plan, usage, and session updates
- permission requests, cancellation, and bounded forced-cancel recovery
- model, effort, mode, and agent session options
- client MCP, elicitation, additional roots, and optional terminal metadata
- optional configurable downstream providers and gateways

These are separate optional surfaces. The first Swallowtail proof does not need
to claim them all.

### Access

Anthropic's current Agent SDK documentation supports:

- an Anthropic Console API key
- Amazon Bedrock
- Claude Platform on AWS
- Google Cloud Agent Platform
- Microsoft Foundry

It also says third-party developers must not offer claude.ai login or
subscription rate limits unless previously approved.

The ACP adapter advertises maintained terminal methods for Claude subscription
and Anthropic Console login when the client offers terminal-auth capability.
That proves adapter behavior. It does not prove that Swallowtail has approval
to expose Claude subscription login.

The first route therefore uses one host-approved Anthropic public-API key lease.
It does not advertise terminal-auth capability, call login or logout, reuse a
Claude subscription, or generalize stored Claude credentials. Other approved
cloud-provider routes remain separate future configured instances.

Production use needs an account and billable provider access. Deterministic
development and default tests do not.

### Configuration, tools, and isolation

The adapter reads user, project, and local Claude settings. The first route is
therefore explicit `Ambient` configuration. It cannot claim provider-suppressed
configuration merely because session `_meta` can supply some options.

The process and its descendants use explicit `AmbientHost` isolation. The
initial tool selection may be limited to provider-native read tools, with
permission requests denied or cancelled outside the frozen subset. That tool
policy is not filesystem containment or a sandbox.

No container or Swallowtail-owned model server is required.

### Compatibility pressure

The adapter has 53 published semantic versions from `0.24.0` through `0.61.0`.
Release `0.52.0` added a direct version flag. Releases `0.53.0`, `0.54.0`, and
`0.60.0` added auth logout, fast-mode configuration, and configurable provider
surfaces respectively.

The first qualified maintained window is `0.53.0..=0.61.0`, excluding
unpublished `0.58.0`.

Exact card-143 evidence rejected candidate `0.52.0`: its argument ordering
intercepts `--cli --version` as the wrapper version, so the nested native binary
cannot be observed, and it predates the tool-call-before-permission-request
ordering fix.

The qualified window has four private behavior milestones:

- `0.53.0`: baseline with safe wrapper and nested-binary observation
- `0.54.0..=0.59.0`: additive session-configuration behavior, excluding
  `0.58.0`
- `0.60.0`: additive provider capability
- `0.61.0`: additive steering metadata

The wrapper version, ACP SDK, Agent SDK, native binary, ACP wire, provider API,
and model remain separate axes. Stable versions above `0.61.0` may run only
under the existing visible unverified-newer policy.

## Candidate Ranking

| Rank | Candidate | Information and cost | Decision |
| --- | --- | --- | --- |
| 1 | Claude Agent ACP maintained range | New harness family, maintained ACP bridge, rich offline source/tests, composite version pressure, no live development account or container | select |
| 2 | Qwen Code ACP | First-party stable ACP within an existing family; useful lifecycle and range work but less provider breadth | later |
| 3 | Cursor ACP | First-party ACP and exact archives; proprietary beta with less transparent offline behavior and an access-dependent corpus | later |
| 4 | Z.AI GLM direct | New direct provider and reasoning controls; mostly repeats the realized compatible hosted-chat shape | later |
| 5 | Gemini, Pi, or Qwen range depth | Important maintenance pressure for existing routes | later maintenance lane |
| 6 | vLLM or SGLang attached runtime | Useful serving breadth and no necessary Swallowtail ownership, but model/parser matrices dominate the proof and attached serving is already represented | later |
| 7 | ACP distribution catalogue | Useful portable discovery role, but registry data grants no installation or execution authority | later contract lane |
| 8 | persistent owned serving | Adds lifecycle depth but raises model-serving and deployment weight near the Monkey boundary | defer |

## Selected Route

The next route is:

- integration family: `claude-agent`
- driver: Claude Agent ACP stdio
- execution layer: harness interaction
- operation shape: interactive session
- transport: ACP v1 over bounded NDJSON stdio
- installed artifact: `@agentclientprotocol/claude-agent-acp`
- qualified range: `0.53.0..=0.61.0`, excluding `0.58.0`
- access: Anthropic public API through one host-approved API-key lease
- endpoint audience: Anthropic public API only
- support: provider-supported Agent SDK and API access beneath an
  integration-maintainer-supported ACP adapter
- configuration: `Ambient`
- process isolation: `AmbientHost`
- serving topology: none

The first capability subset is deliberately narrower than the adapter:

- safe adapter version observation
- ACP initialize and one new interactive session
- exact consumer-selected model binding with no default-model fallback
- text prompts and ordered text, reasoning, tool, plan, usage, and terminal
  outcomes only where the corpus qualifies them
- provider-native read-tool selection
- permission rejection, active-turn cancellation, deadline, disconnect, and
  joined process cleanup

Excluded from the first proof:

- Claude subscription or terminal login
- console login, logout, gateway auth, or credential mutation
- Bedrock, Vertex, Foundry, custom gateway, or provider switching
- load, resume, list, fork, delete, and provider-owned persistent-session claims
- writes, Bash, web tools, subagents, background terminals, custom commands,
  client MCP, elicitation, and steering extensions
- provider- or host-enforced sandbox claims
- installation, update, downgrade, or package-manager execution
- implicit model, endpoint, credential, configuration, or provider fallback

## Contract Assessment

No new shared contract is required before the exact corpus.

- Contract 015 governs ACP v1 framing, negotiation, callbacks, and optional
  method separation.
- Contract 017 keeps delegated auth, sign-in, persistence, callbacks, and
  containment independent.
- Contract 023 allows explicit ambient harness execution and provider-native
  tool restrictions without a sandbox claim.
- Contract 029 governs the candidate maintained range and unverified-newer
  posture.
- Contract 032 governs safe installed adapter observation.
- Contract 033 requires explicit ambient configuration.

Card 143 found no missing shared contract. The selected process environment is
host-explicit and cleared before approved values are applied, so
`CLAUDE_CODE_EXECUTABLE` cannot silently replace the nested binary. The wrapper
and nested binary use separate safe observations from `0.53.0` onward.

## Corpus Result

Card 143 completed the following:

1. snapshotted all 11 published candidate points and the missing `0.58.0`
2. froze exact package, ACP SDK, Agent SDK, source, and native-binary evidence
3. ran network-denied, empty-home native `--version` probes for Claude Code
   `2.1.191`, `2.1.195`, `2.1.197`, `2.1.198`, `2.1.201`, `2.1.202`,
   `2.1.205`, `2.1.207`, `2.1.215`, and `2.1.217`
4. qualified four behavior segments and rejected `0.52.0`
5. froze independent raw ACP transcripts for initialization, session creation,
   exact model confirmation, reasoning, read tools, usage, permission
   rejection, cancellation, access failure, model drift, and disconnect
6. kept installation, authentication, and live provider probes separately
   gated

Eight focused corpus tests pass without a provider account.

## Risks

- frequent wrapper and nested SDK releases may create several short behavior
  segments
- the adapter's registry license label and package/source license differ
- the adapter reads ambient configuration
- exact nested native-binary identity may need a narrower observation rule
- upstream subscription login exists but is outside Swallowtail's approved
  access claim
- read-tool restriction is policy, not containment

## Promotion

- route and range decision: this research record
- completed corpus: card 143
- delivery sequence: roadmap 048 and cards 144-145
- next task: card 144, Claude Agent ACP production driver
- Grok hold: unchanged
- generation: g01 remains active at 48 roadmaps

## Primary Sources

- [Claude Agent SDK overview](https://code.claude.com/docs/en/agent-sdk/overview)
- [Claude Code CLI reference](https://code.claude.com/docs/en/cli-reference)
- [Claude Code permission modes](https://code.claude.com/docs/en/permission-modes)
- [Claude Agent ACP repository](https://github.com/agentclientprotocol/claude-agent-acp)
- [Claude Agent ACP changelog](https://github.com/agentclientprotocol/claude-agent-acp/blob/main/CHANGELOG.md)
- [`@agentclientprotocol/claude-agent-acp` package](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp)
- [ACP Registry](https://agentclientprotocol.com/get-started/registry)
- [Cursor ACP](https://cursor.com/docs/cli/acp)
- [Qwen Code settings](https://qwenlm.github.io/qwen-code-docs/en/users/configuration/settings/)
- [Z.AI chat completions](https://docs.z.ai/api-reference/llm/chat-completion)
- [vLLM online serving](https://docs.vllm.ai/en/latest/serving/online_serving/)
- [SGLang OpenAI-compatible APIs](https://docs.sglang.io/docs/basic_usage/openai_api_completions)
