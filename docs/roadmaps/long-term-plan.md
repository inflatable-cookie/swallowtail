# Long-Term Plan

Status: active
Owner: Tom
Updated: 2026-07-21

## Sequence

1. **Foundation** — standalone authority and native repository validation.
2. **Contract kernel** — pure provider-neutral records and conformance fixtures.
3. **Integration landscape** — inventory current harness, API, SDK, CLI,
   protocol, local-runtime, and remote-service surfaces.
4. **Runtime boundary** — host ports, lifecycle traits, cancellation, and event
   delivery after several materially different adapter shapes constrain the
   design.
5. **Codex proofs** — extract structured CLI and interactive app-server
   mechanisms as separate drivers without consumer policy.
6. **Soundcheck adoption** — prove bounded structured runs in the smaller
   consumer.
7. **Nucleus adoption** — prove persistent interactive sessions and remote host
   placement.
8. **Cross-adapter proofs** — implement materially different harness, direct
   API, SDK/protocol, and local-runtime drivers before stabilizing the runtime.
9. **Coverage expansion** — add explicit provider/transport drivers and
   conformance evidence without widening the common API unnecessarily.
10. **Release discipline** — compatibility policy, versioning, and consumer
    upgrade evidence after the API earns stability.

Each stage requires its own promoted contracts and bounded roadmap cards.

The first cross-adapter tranche and xAI follow-up are complete: OpenCode attached HTTP/SSE,
Anthropic direct HTTP/SSE, Gemini CLI ACP, and llama.cpp attached self-hosted
HTTP/SSE plus xAI connection-scoped WebSocket. Kimi Code's second-agent ACP
protocol proof is complete. The exact Kimi Code `0.28.1` successor, native
arm64 artifact, isolated state, feature exclusions, and upgrade gate are
frozen. A native App Sandbox proof shows the project grant and descendant
boundary work for a compatible helper, but the exact Kimi artifact cannot run
under the documented inherited-helper signature. That blocks only the optional
host-enforced profile. Harness communication now proceeds through an explicit
ambient route, matching the control-plane posture used by T3 Code and similar
orchestrators. Provider- or host-enforced isolation remains a separate exact
capability with no ambient fallback. Kimi now passes a separate persistent ACP
conformance extension under local and remote-authoritative host identities
without widening Gemini's baseline.
Owned llama.cpp now passes production conformance for read-only artifact
leases, readiness handoff, local and remote-authoritative topology, and joined
child cleanup without absorbing Monkey. Roadmaps 020-021 complete separate
provider-supported Bedrock Runtime and control-plane SDK routes. Roadmap 022
selects OpenAI Responses background mode as the next high-information shape:
one provider-owned asynchronous direct run, explicit temporary retention,
recoverable SSE cursor, and native cancel. Roadmap 023 owns its contract,
deterministic corpus, production driver, and conformance. Cursor Cloud Agents
remain later behind an explicit repository, GitHub integration, remote-
workspace, artifact, and deletion-authority choice. Roadmap 023 closes the
OpenAI production driver and conformance proof. Research 016 and Contract 022
select Claude Managed Agents as the next high-information shape: a provider-
hosted harness with versioned agent configuration, driver-owned environment
and session resources, durable retention, provider-managed rescheduling,
authoritative persisted events, callbacks, and remote deletion truth. Roadmap
025 completes the resource-free proof. It requires no local container, consumer
repository, provider filesystem, or external sandbox network. Cursor remains
later behind explicit repository and remote-mutation authority. Similar
HTTP/SSE and one-shot JSONL routes remain behind higher-information shapes.

The managed-harness proof closes with ten provider-neutral conformance profiles
and 330 passing repository tests. Research 017 then selects stable Qwen Code
`v0.19.11` headless over unfinished remote ACP, policy-bound Cursor Background
Agents, and the experimental Qwen daemon. Contract 023 keeps structured-run
isolation, provider permissions, native budgets, durable local transcript
state, and optional sandboxing separate. Card 080 freezes the first read-only
`AmbientHost` route with text stdin, bounded stream JSON, explicit tool posture,
and no container. Cards 081-082 complete the production driver and unchanged
one-shot-profile proof under local and remote-authoritative hosts. Full QA
passes with 360 tests. Roadmap 027 revalidates direct Kimi Platform, DeepSeek,
Z.AI, and Alibaba/Qwen compatibility seams. Research 018 and Contract 024
select a smaller structural Chat Completions codec plus one exact Kimi
Platform K3 proof. The codec must pass independent llama.cpp and Kimi corpora;
provider access, model, reasoning, errors, catalogue, retry, and lifecycle stay
outside it. Roadmap 028 owns cards 084-086. DeepSeek V4, Z.AI general API, and
Alibaba Model Studio remain later separate mappings.

These stages normally become numbered roadmaps inside the active generation.
They do not each create a generation. A generation rolls over near the 30-50
roadmap range or through explicit operator-authorized restructuring.
