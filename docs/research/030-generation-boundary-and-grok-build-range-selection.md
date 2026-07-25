# 030 Generation Boundary And Grok Build Range Selection

Status: promoted
Owner: Tom
Updated: 2026-07-24

## Question

Which post-remote-ACP proof adds the most useful provider coverage, and should
it remain in g01?

## Method

Sources were accessed 2026-07-24. Evidence includes the live ACP registry,
stable and draft ACP documentation, exact Rust SDK package source, official xAI
Grok Build documentation, public npm package metadata, and realized Swallowtail
descriptors and conformance profiles.

No package was installed or executed. No provider login, credential, account,
model request, update, sandbox, container, or remote agent was used.

## Realized Inventory

Swallowtail still has 21 production driver descriptors across 16 integration
families and thirteen common conformance profiles. It now also has one shared
provider-neutral remote ACP transport over HTTP/2 SSE and WebSocket.

Material remaining gaps:

- first-party harness breadth beyond the existing Codex, OpenCode, Gemini,
  Kimi, Pi, and Qwen routes
- maintained version windows for fast-moving installed harnesses still pinned
  at one or two exact releases
- a real provider-specific remote ACP composition
- interactive authentication status and mutation
- a portable agent-catalogue role
- persistent owned serving beyond the ephemeral llama.cpp proof

Qualified support and unverified-newer execution remain separate. Codex,
OpenCode, Kimi Code, and Ollama have multi-release evidence. Gemini CLI, Pi,
and Qwen remain exact or narrow harness claims. Hosted API facades and
compile-time SDK pins do not become installed runtime ranges.

## ACP Evidence Delta

### Stable v1 grew while v2 became Draft

ACP v1 has stabilized session list, resume, close, delete, logout, usage,
message ids, request cancellation, configuration categories, additional
workspace roots, and implementation metadata. ACP v2 is separately available
as a draft.

The Rust SDK package reached `agent-client-protocol = 2.0.0` on 2026-07-23.
Its exact changelog says the SDK major keeps the stable v1 wire unchanged.
Draft v2 remains feature-gated. SDK `2.0.0`, schema `1.5.0` or later, ACP wire
v1, and draft ACP v2 are different axes.

Swallowtail's current v1 subset remains valid. New stable optional methods do
not silently widen Gemini, Kimi, remote transport, or future agent
capabilities. Draft v2 is observed but not qualified.

### Registry does not supply a remote provider endpoint

The completed ACP Registry publishes schema `1.0.0`. The live aggregate
contained 38 agents on 2026-07-24. Its distribution shapes are local binary,
`npx`, and `uvx` packages. The manifest has no provider-hosted remote ACP
endpoint field.

A provider-specific remote ACP adapter therefore still lacks authoritative
endpoint, authentication, agent-version, support, and lifecycle evidence.
The shared transport remains ready for composition when such a route appears;
it should not be forced into a local agent proof.

Registry presence is mutable distribution evidence. It does not prove an
installed executable, supported version range, authentication readiness,
entitlement, capability, model route, safe installation, or execution
authority. Automatic installation remains outside the selected lane.

### Interactive auth remains immature

The ACP Authentication Methods RFD and the new `auth/status` RFD remain Draft.
Registry admission requires an agent-managed or terminal authentication method,
but that is not current authenticated state and does not authorize a client to
run login, inject environment secrets, or call logout.

The next proof should reuse pre-existing host-approved delegated
authentication. Interactive login and status orchestration remain later work.

## Grok Build Evidence

xAI documents Grok Build as a first-party coding harness usable through:

- interactive TUI
- headless plain, JSON, or streaming JSON
- `grok agent stdio` over ACP

The ACP example assumes Grok is already authenticated locally or
`XAI_API_KEY` is present. The CLI exposes `grok version`, explicit versioned
updates, and `--no-auto-update`. Permissions, tool allow/deny rules, and
sandbox profiles are documented as separate controls.

The live ACP registry entry was:

- id: `grok-build`
- version: `0.2.111`
- distribution: `@xai-official/grok@0.2.111`
- invocation: `agent stdio`

Public npm metadata contained 175 total package versions. The stable `0.2`
candidate envelope contains 111 exact releases from `0.2.0` through
`0.2.111`, with `0.2.48` unpublished. Releases span 2026-05-26 through
2026-07-22 and often publish daily.

This is useful compatibility pressure, not proof of one continuous supported
range. Package semver, registry manifest version, ACP wire, bundled schema,
SDK, model configuration, and harness behavior remain independent. Card 137
must freeze exact artifacts and find real milestones, gaps, and exclusions
before any claim is published.

Source disagreement reinforces that rule: the live registry labels Grok Build
proprietary while npm package metadata labels the launcher Apache-2.0. Neither
value should be silently promoted into a universal product or artifact claim.

## First Grok Boundary

The candidate production route is:

- integration family `grok-build`, distinct from direct `xai`
- harness interaction over ACP v1 stdio
- exact host-approved executable and installed version observation
- pre-existing delegated interactive OAuth
- subscription allowance
- integration-maintainer support authority
- exact model selection only when the qualified ACP surface confirms it
- provider-native restrictive read-only tools and permission mode
- explicit `AmbientHost` isolation; no sandbox or containment claim
- explicit ambient configuration posture
- isolated host-approved Grok state environment
- durable local session retention if the qualified artifact creates it
- `--no-auto-update`
- local and remote-authoritative execution-host topology

The first route excludes:

- API-key injection or direct xAI API reuse
- login, device authorization, logout, or auth-status orchestration
- installation, update, downgrade, package-manager execution, or registry
  auto-install
- workspace writes, shell execution, web search, subagents, cross-session
  memory, plugins, hooks, MCP mutation, or custom model endpoints
- provider- or host-enforced sandbox claims
- implicit model, provider, route, credential, or configuration fallback
- a continuous compatibility range inferred from semver

Contracts 012, 013, 015, 017, 023, 029, 032, 033, and 034 already govern this
subset. No new shared contract is required before the range corpus. A material
lifecycle or authority difference found in card 137 is a stop condition, not
permission to stretch those contracts.

## Ranking

| Rank | Candidate | Information gained | Decision |
| --- | --- | --- | --- |
| 1 | Grok Build maintained ACP range | first-party xAI harness, daily-release range pressure, exact discovery, provider permissions without sandbox conflation | select |
| 2 | ACP Registry runtime catalogue | portable agent discovery and distribution observation | later; no current consumer discovery requirement and installation metadata needs a separate role |
| 3 | ACP v1 lifecycle expansion and v2 | protocol maintenance | observe in Grok corpus; do not widen every adapter or qualify Draft v2 |
| 4 | Gemini, Pi, or Qwen range depth | useful compatibility maintenance | later; represented providers and shapes |
| 5 | provider-specific remote ACP | composition of the new transport | wait; no authoritative provider endpoint |
| 6 | interactive ACP auth | current-state and login lifecycle | wait; methods and status remain Draft |
| 7 | persistent owned serving | durable local runtime authority | later; heavy and close to Monkey ownership |

## Generation Decision

Keep g01 active.

Roadmap 047 is one coherent compatibility-and-provider lane. It takes g01 to
47 numbered roadmaps, inside the documented 30-50 range. It does not justify
an automatic g02 rollover. Its closeout must return to a deliberate generation
checkpoint before another material lane is compiled.

## Promotion

- durable boundaries: existing Contracts 012, 013, 015, 017, 023, 029, 032,
  033, and 034
- superseding artifact evidence: Research 031 corrects the channel claim,
  qualifies no release, and rejects the bounded read-only premise
- implementation sequence: g01 roadmap 047 and cards 137-141
- evidence gate: card 138 requires operator-authorized activation-only
  delegated-auth evidence or maintained documentation matching the artifact
- operator hold: roadmap 047 and cards 138-141 are held because no Grok
  account is available
- active continuation: roadmap 048 and card 142 re-rank remaining routes
- next generation checkpoint: after roadmap 048

## Primary Sources

- [ACP RFD lifecycle](https://agentclientprotocol.com/rfds/updates)
- [ACP Registry](https://agentclientprotocol.com/get-started/registry)
- [ACP Authentication Methods RFD](https://agentclientprotocol.com/rfds/auth-methods)
- [Grok Build overview](https://docs.x.ai/build/overview)
- [Grok Build CLI reference](https://docs.x.ai/build/cli/reference)
- [Grok Build headless and ACP](https://docs.x.ai/build/cli/headless-scripting)
- [Grok Build permissions](https://docs.x.ai/build/features/permissions)
- [Grok Build settings](https://docs.x.ai/build/settings)
- [`agent-client-protocol` package](https://crates.io/crates/agent-client-protocol)
- [`@xai-official/grok` package](https://www.npmjs.com/package/@xai-official/grok)
