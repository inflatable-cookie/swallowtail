# Long-Term Plan

Status: active
Owner: Tom
Updated: 2026-08-20

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

Card 084 realizes the bounded codec and dated K3 corpus. llama.cpp and Kimi now
pass the same structural decoder while retaining separate semantic mappings.
Card 085 realizes the separate Kimi Platform driver with one exact K3 request,
bounded authenticated catalogue, ordered reasoning/output/usage, distinct
failure classes, local cancellation and deadline, and joined credential
cleanup. Card 086 proves the unchanged hosted-direct profile under local and
remote-authoritative hosts, exact topology, source-scoped catalogue truth, one
attempt, and connection-before-credential cleanup. Roadmap 028 closes with 384
passing repository tests and three gated probes ignored. Research 019 then
selects Alibaba Model Studio's Singapore workspace-dedicated Conversations and
Responses route over another stateless DeepSeek or Z.AI Chat Completions
mapping. Contract 025 fixes a resource-free direct interactive session with
explicit provider conversation retention, exact regional workspace access,
local-only cancellation, and item-before-conversation deletion truth. Cards
088-089 realize the provider-neutral records, dated corpus, production driver,
and local plus remote-authoritative conformance. Roadmap 029 closes with 404
tests. Roadmap 030 and card 090 re-rank remaining harness, direct, protocol,
SDK, catalogue, and self-hosted pressure before another implementation.
DeepSeek V4 and Z.AI GLM-5.1 remain candidates rather than automatic defaults.
Research 020 closes that checkpoint by selecting OpenAI Realtime's GA server-
side WebSocket. Contract 026 adds a separate realtime-media direct-session role
with exact media formats, bounded redacted chunks, consumer-owned device and
playback truth, native response cancellation, and joined duplex cleanup.
Card 091 realizes the provider-neutral role, records, preflight, eleventh
profile, and dated OpenAI corpus. Card 092 adds the separate production
Realtime WebSocket driver with local plus remote-authoritative loopback
evidence. Card 093 adds provider-neutral production conformance, adversarial
failure and disconnect coverage, cancellation uncertainty, timer and cleanup
ordering, and both host identities. Roadmap 031 closes with 430 passing tests.
Roadmap 032 and card 094 close after current evidence selects Gemini Live raw
server-to-server WebSocket. Contract 027 makes one provider-planned connection
replacement explicit and opt-in, keeps resumption handles private, and
separates rollover from reconnect, reattachment, retry, consumer resume, and
durable state. Roadmap 033 and cards 095-097 own the exact preview records,
corpus, production driver, portability conformance, and closeout. Grok Build
remains later harness breadth. Remote ACP is Draft and waits for maintained SDK
and hardening evidence. Cursor's new local SDK remains beta and
lower-information; its cloud route still requires repository and remote-
mutation policy.

Roadmap 033 closes with the eighteenth production route and 443 passing tests.
The unchanged eleventh realtime profile and separate bounded-rollover pack
pass against Gemini Live production fixtures under both host identities. The
complete offline matrix preserves warning, handle, replay, failure,
cancellation, deadline, nonreuse, and two-generation cleanup truth. Roadmap 034
and card 098 close after Research 022 audits all eighteen descriptors and
eleven profiles. Pi `0.80.10` RPC is the next proof: a maintained strict-LF
process protocol with distinct prompt, steering, follow-up, abort, and
extension-UI relay. Contract 028 keeps downstream provider/model identity
exact, `AmbientHost` visible, sandboxing optional, retry disabled, and cleanup
joined. Roadmap 035 and cards 099-101 close after production conformance proves
both host identities, exact scheduling, callback expiry, failure separation,
and joined cleanup. Roadmap 036 and cards 102-104 complete DeepSeek V4
continuation evidence, Contract 030's locally continued direct-session
boundary, one exact V4 Pro corpus, production driver, and both-topology
conformance. The twentieth route uses the OpenAI-format facade, explicit
attempt authorization, consumer-executed tools, private ephemeral reasoning
continuation, explicit provider-cache acceptance, and joined credential-last
cleanup. Roadmap 037 and card 105 close after auditing all twenty routes and
twelve profiles against Contract 029. Only Pi and DeepSeek expose descriptor
claims, both as one-point windows. Remote ACP is Draft. Claude subscription-
backed third-party SDK use and Cursor's public-beta bridge retain explicit
gates.

Research 024 and Contract 031 select attach-only Ollama native API as the first
non-singleton compatibility proof. Roadmap 038 and cards 106-109 qualify stable
`0.14.0` through `0.32.1`, exact runtime version, installed and running model
observations, one selected text route, native NDJSON, and explicit
runtime-managed residency. The route installs nothing, downloads no model,
uses no cloud access, needs no container, and does not absorb Monkey.

Roadmap 039 returns compatibility work to installed harnesses. Card 110
revalidates their release histories and discovery surfaces, starting with
Codex exec and app-server because consumer devices may carry a six-month
release span. Exact baseline, behavior milestones, exclusions, and
latest-qualified points need frozen evidence before any descriptor range is
published.

These stages normally become numbered roadmaps inside the active generation.
They do not each create a generation. A generation rolls over near the 30-50
roadmap range or through explicit operator-authorized restructuring.

## g01 Disposition

g01 reaches its intended boundary at 49 roadmaps. It has realized the
foundation, two consumer adoptions, shared runtime and host boundaries,
materially different harness and direct routes, SDK and ACP transports,
self-hosted runtime shapes, compatibility ranges, and visible
unverified-newer execution.

The remaining Grok Build lane is deferred for unavailable delegated-
authentication evidence. It did not justify roadmap 050 and now lives in the
shared roadmap backlog.

The operator closed g01 and selected API stabilization, release discipline,
packaging, and consumer upgrade support as the primary g02 programme.
Evidence-led provider breadth remains secondary. g02 begins by defining the
public package, version, compatibility, MSRV, and release-authority boundary;
no registry publication or API 1.0 promise is implied.

## g02 Disposition And g03 Programme

g02 closed at 49 roadmaps after completing prepared facades, package and
candidate discipline, compatibility ranges, provider-session lifecycle,
feature-matrix closure, observable activity, consumer handoffs, structural
stabilization, validation routing, child-work topology, and Claude typed form
elicitation.

Pi RPC load and resume moved to the shared backlog behind the unchanged
maintained public cwd-bound attachment gate. Provider-session binding
persistence remains deferred. Registry publication remains outside the active
roadmap until the operator revisits it after sustained application usage.

g03 prioritizes compatibility maintenance and consumer-proven hardening. It
starts with installed harnesses and shared protocols because their release
cadence can outpace consumer application releases. Exact baselines,
intermediate behavior milestones, latest-qualified points, exclusions, and
unverified-newer posture remain separate. A version above the qualified upper
point is not denied solely for being newer.

Registry publication was later removed from the active runway. Local package
and compatibility evidence continues, but publication does not return as a
ready, paused, or backlog item. A future lane requires a new operator decision
after months of working-application usage evidence.

Research 092 and Contract 046 add explicit discovery and import of
harness-origin sessions to g03 without changing the consumer ownership
boundary. Roadmaps g03.019-g03.023 sequence a provider-neutral catalogue and
import kernel, Codex app-server, stable ACP through Kimi, OpenCode attached
HTTP, provider-wide classification, and a Nucleus handoff. Discovery
candidates carry no attachment authority. Successful import revalidates the
exact route, host, access, version, model, resource, and policy before issuing
the existing resume binding. Automatic synchronization, consumer persistence,
raw-id attachment, and provider-session management binding persistence remain
outside the lane.

Research 036 and Contract 038 open the next additive stabilization lane.
Consumer thread archive, restore, and deletion remain downstream. Swallowtail
adds only optional management of one inactive bound provider session.
Roadmaps g02.015-g02.019 sequence the provider-neutral role, Codex
archive/restore/delete, ACP and Claude Agent close/delete, OpenCode deletion,
provider-wide package evidence, and a Nucleus adoption handoff. Provider
history browsing, active-session management, implicit deletion, and publication
remain outside the lane.

Roadmaps g02.015-g02.017 are complete. Codex app-server proves exact
inactive-session archive, restore, and hard-delete mapping. Claude Agent now
proves qualified native close and provider-data deletion through production
stdio plus the same lifecycle records over explicit remote ACP without
fallback. Research 039 and card 055 qualify OpenCode provider-data deletion
with provider-defined descendants across every exact
`1.14.48..=1.18.18` release while preserving gaps and unverified-newer truth.
Roadmap g02.018 is complete. Its bound driver, prepared facade, exact segment
matrix, local and remote-authoritative host proof, failure boundary, and full
regression suite pass. Roadmap g02.019 now owns provider-wide classification,
packaged acceptance, and the Nucleus handoff. Its retained candidate gate is
paused until the lifecycle source enters canonical history.

Research 040 corrects the earlier Kimi Web authority assessment. Exact Kimi
Code `0.28.1` and `0.29.0` document a foreground local REST/WebSocket server
with OpenAPI, AsyncAPI, bearer authentication, exact metadata, reversible
archive, and restore. Roadmap g02.020 adds that route separately from ACP.
Deletion remains unsupported. Lifecycle lands before the larger interactive
surface, and no container is required.


## g03 Disposition And g04 Programme

g03 closed at 106 roadmaps after operator-authorized rollover. Compatibility
maintenance, consumer-proven hardening, recovery facades, source tags, harness
expansion, and the recurring currentness sweep are complete or rehomed.

Aider headless, Kiro headless, and OpenHands production wiring moved to the
shared backlog. Gemini requalification, Pi RPC continuity, and provider-session
management binding persistence remain deferred. Registry publication remains
outside the active roadmap.

g04 prioritizes a portable route-readiness and connection-admission library
surface. Consuming applications should be able to list addable routes, collect
or launch required credentials, admit configured instances, observe readiness
and updates, and expose the models those instances can run. Swallowtail does
not become a connection server, credential store, UI, or router. Recurring
currentness is a standing Contract 029 lane, not a g04 goal. See
[standing-lanes.md](standing-lanes.md).

## g04 Disposition And g05 Programme

g04 closed at 91 roadmaps after realizing route readiness, connection
admission, broad addable-route coverage, Pi SDK sidecar continuity, Gemini
enterprise API-key requalification, and the full 85-item per-route feature
inventory. That inventory ends with 83 closed dispositions and two parked
Bedrock items. Contract 029 currentness remains standing.

g05 starts with harness skill visibility and dependable process observability.
The first milestone inventories exact prompt-free production-harness surfaces.
Later work must separate distribution membership, host configuration, model
visibility, watcher lifecycle, process authority, activity projection, and
turn-completion enforcement before contracts or implementation. New routes,
ambient home scanning, arbitrary PID control, and prompt-only watcher claims
stay outside the programme.

## Next Task

Implement the active task named by the [roadmaps front door](README.md#next-task).
The long-term plan stays the staged adoption sequence; it does not own the
current batch pointer.
