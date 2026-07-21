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
child cleanup without absorbing Monkey. Roadmap 020 now rechecks SDK-native
routes for a real maintained Rust embedding or explicit supported language
boundary. If none is ready, it selects the next hosted catalogue, protocol,
harness, direct-inference, or attached-runtime route by new contract pressure.
Similar HTTP/SSE and one-shot JSONL routes remain behind higher-information
shapes.

These stages normally become numbered roadmaps inside the active generation.
They do not each create a generation. A generation rolls over near the 30-50
roadmap range or through explicit operator-authorized restructuring.
