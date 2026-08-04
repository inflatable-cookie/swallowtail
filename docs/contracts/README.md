# Contracts

Contracts hold durable, testable rules.

## Active Contracts

- [Contract Index](contract-index.md)
- [001 Working Rules](001-working-rules.md)
- [002 Repository Authority](002-repository-authority.md)
- [003 Portable Contract Kernel](003-portable-contract-kernel.md)
- [004 Runtime Ownership Boundary](004-runtime-ownership-boundary.md)
- [005 Integration Identity and Transport Diversity](005-integration-identity-and-transport-diversity.md)
- [006 Execution Layer and Access Boundary](006-execution-layer-and-access-boundary.md)
- [007 Model Artifact and Serving Boundary](007-model-artifact-and-serving-boundary.md)
- [008 Runtime Registration and Preflight](008-runtime-registration-and-preflight.md)
- [009 Async Operation Lifecycle](009-async-operation-lifecycle.md)
- [010 Execution Host Services and Inputs](010-execution-host-services-and-inputs.md)
- [011 Runtime Conformance Profiles](011-runtime-conformance-profiles.md)
- [012 Interactive Session Options and Callback Exchange](012-interactive-session-options-and-callback-exchange.md)
- [013 Interactive Session Access Policy](013-interactive-session-access-policy.md)
- [014 Hosted Transport, Credential, And Evidence Boundary](014-hosted-transport-credential-and-evidence-boundary.md)
- [015 ACP v1 Negotiation And Client Callbacks](015-acp-v1-negotiation-and-client-callbacks.md)
- [016 Connection-Scoped Direct Sessions And Billed Cost](016-connection-scoped-direct-sessions-and-billed-cost.md)
- [017 Provider-Owned Session Load, Replay, And Host Containment](017-provider-owned-session-load-replay-and-host-containment.md)
- [018 Owned Ephemeral Model Serving Lifecycle](018-owned-ephemeral-model-serving-lifecycle.md)
- [019 Embedded SDK And Cloud Client Boundary](019-embedded-sdk-and-cloud-client-boundary.md)
- [020 Model Catalogue Observation And Availability Boundary](020-model-catalogue-observation-and-availability-boundary.md)
- [021 Provider-Owned Background Run And Temporary Retention Boundary](021-provider-owned-background-run-and-temporary-retention-boundary.md)
- [022 Provider-Managed Agent Resource And Durable Session Boundary](022-provider-managed-agent-resource-and-durable-session-boundary.md)
- [023 Harness Operation Isolation And Native Boundary](023-harness-operation-isolation-and-native-boundary.md)
- [024 Compatible Chat Codec And Provider Semantics](024-compatible-chat-codec-and-provider-semantics.md)
- [025 Provider-Owned Direct Conversation And Deletion Boundary](025-provider-owned-direct-conversation-and-deletion-boundary.md)
- [026 Realtime Media Direct Session Boundary](026-realtime-media-direct-session-boundary.md)
- [027 Planned Connection Rollover And Realtime Continuity](027-planned-connection-rollover-and-realtime-continuity.md)
- [028 Harness RPC Scheduling And UI Relay Boundary](028-harness-rpc-scheduling-and-ui-relay-boundary.md)
- [029 Interface Version Qualification And Compatibility](029-interface-version-qualification-and-compatibility.md)
- [030 Consumer-Owned Direct Tool Continuation](030-consumer-owned-direct-tool-continuation.md)
- [031 Attached Native Runtime Version And Residency](031-attached-native-runtime-version-and-residency.md)
- [032 Installed Executable Observation And Discovery](032-installed-executable-observation-and-discovery.md)
- [033 Harness Configuration Posture](033-harness-configuration-posture.md)
- [034 Negotiated Harness Session Options](034-negotiated-harness-session-options.md)
- [035 Remote ACP Connection Transport](035-remote-acp-connection-transport.md)
- [036 Crate Release And Compatibility Boundary](036-crate-release-and-compatibility-boundary.md)
- [037 Provider-Wide Prepared Integration And Bound Operations](037-prepared-consumer-integration.md)
- [038 Provider Session Management And Consumer Thread Boundary](038-provider-session-management-and-consumer-thread-boundary.md)
- [039 Bounded Single-Turn Structured-Run Projection](039-bounded-single-turn-structured-run-projection.md)
- [040 Generation-Control Application And Enforcement](040-generation-control-application-and-enforcement.md)
- [041 Input, Callback, And Provider-Tool Admission](041-input-callback-and-provider-tool-admission.md)
- [042 Harness-Managed Recovery And Active-Turn Reattachment](042-harness-managed-recovery-and-active-turn-reattachment.md)
- [043 Turn-Scoped Interactive Continuity](043-turn-scoped-interactive-continuity.md)
- [044 Observable Agent Activity And Disclosure](044-observable-agent-activity-and-disclosure.md)
- [045 Subagent Topology, Observation, And Control](045-subagent-topology-observation-and-control.md)
- [046 Provider Session Catalogue And Explicit Import](046-provider-session-catalogue-and-explicit-import.md)
- [047 Configured Provider Instance Catalogue](047-configured-provider-instance-catalogue.md)

Contract 003 defines the provider-neutral record kernel. Contracts 004-016
govern the realized runtime and current proof drivers. Contract 017 governs
exact persistent-session load and resume, versioned ordinary resume-binding
restart records, write callbacks, ambient harnesses, and optional process
containment. Its restart record is attachment-bound authority input, not a
consumer database or provider-session management binding; exact deployed-
runtime qualification applies only to an enforced isolation claim. Contract
018 governs owned ephemeral serving, now
realized by the llama.cpp proof. Contract 019 governs in-process SDK drivers,
explicit cloud-client configuration, and delegated SDK credentials. Contract
020 keeps mutable catalogue observations separate from entitlement, runtime
capability, and route selection; it also distinguishes standalone sources from
session-negotiated options and records that the current common catalogue
request has deadline but no independent cancellation control. Contract 021
makes provider-managed background
execution, required temporary retention, bounded stream reattachment, and
remote cancellation truth explicit and opt-in. Contract 022 governs provider-
hosted agent resources, durable retention, provider-managed recovery,
authoritative persisted events, and remote deletion truth.
Contract 023 makes harness isolation operation-shape neutral and keeps provider
permissions, native budgets, retained state, and optional sandboxing separate
from host deadline, cancellation, and process authority.
Contract 024 permits structural Chat Completions codec reuse while keeping
provider access, model, capability, lifecycle, evidence, retry, and fallback
semantics inside separately qualified adapters.
Contract 025 makes provider-owned direct conversations an explicit interactive
session posture and keeps regional workspace access, response storage, item
inventory, deletion truth, cancellation, resume, and cleanup independent.
Contract 026 adds a separate realtime-media interactive role with exact media
formats, bounded redacted chunks, native response cancellation, consumer-owned
device and playback truth, and joined duplex cleanup.
Contract 027 makes provider-planned connection replacement explicit and
bounded, keeps resumable provider handles private and operation-scoped, and
separates rollover from reconnect, reattachment, retry, consumer resume, and
durable state.
Contract 028 separates prompt, steering, follow-up, abort, command
acknowledgement, and extension-UI relay for harness RPCs. It keeps downstream
provider/model identity exact, ambient read intent uncontained, retry disabled,
and cleanup joined.
Contract 029 keeps adapter, artifact, SDK, wire, service, facade, instance,
route, and model versions separate. Execution binds exact observed points;
drivers support maintained baseline-to-latest windows only through ordered,
evidence-backed milestones, deprecation states, and exact exclusions. Ordered
claims may separately permit exact unverified-newer attempts without extending
their guaranteed support window; opaque claims remain exact-only.
Contract 030 adds a locally continued direct-session profile. Every provider
attempt needs explicit consumer authorization; tool execution remains
downstream while provider-private continuation stays bounded, redacted,
route-bound, ephemeral, and distinct from provider cache or consumer memory.
Contract 031 governs attach-only native model runtimes. It keeps exact runtime
version, installed and running inventory, selected route, artifact identity,
and invocation-caused model residency separate without granting installation,
model mutation, unload, or serving-lifecycle authority.
Contract 032 adds target-aware installed-executable discovery. It binds one
opaque host-approved candidate, exact version axis, deadline, cancellation,
authoritative host, qualified, unverified-newer, or incompatible
classification, and joined process cleanup without executable search,
configuration promotion, authentication, or execution authority.
Contract 033 binds ambient, provider-suppressed, and host-scoped harness
configuration independently from isolation, credentials, retention, and
working resources. Host-scoped execution remains gated until a separate opaque
host lease and service exist.
Contract 034 maps exact typed session options through version-qualified,
harness-advertised configuration channels. Provider option records remain
private; exact confirmation is required without model, route, access,
isolation, or lifecycle fallback.
Contract 035 adds an opt-in experimental remote ACP transport over one exact
host-approved HTTP/SSE or WebSocket endpoint. It keeps transport separate from
provider identity, scopes connection and affinity state, excludes
authentication and implicit recovery, and requires explicit joined close.
Contract 036 fixes the 26 public packages, coordinated pre-1.0 version,
compatible internal requirements, three-stage publication order, bounded MSRV,
package and consumer evidence, one accepted application-scale consumer proof
before first publication, and explicit human authority for every external
release mutation. Crate versions remain separate from Contract 029 provider-
interface ranges.
Contract 037 requires an adapter-local prepared normal path for every
production driver above the unchanged low-level roles. It binds adapter-owned
facts, derives immutable plan echoes, preserves explicit access provenance and
authority, exposes safe staged diagnostics, and permits typed bound operations
without flattening role lifecycle. It also preserves unverified-newer posture,
joined per-host service composition, and provider-wide deterministic evidence
before replacement candidate freeze.
Contract 038 keeps consumer thread lifecycle separate from bound provider
session management. It distinguishes attachment close, provider-native close,
archive, restore, history removal, data deletion, hard deletion, and
driver-owned cleanup; binds exact inactive targets; preserves version and
capability truth; and requires explicit destructive authority and uncertainty.
Its Kimi local-server addition keeps REST/WebSocket archive and restore
separate from ACP, requires explicit cross-transport binding import, and
qualifies no deletion.
Contract 039 permits exact provider routes to project one operation-private
session, connection, process, or provider resource into a bounded structured
run. It requires independent role qualification, exact request support,
explicit retention, one terminal outcome, and joined cleanup. Close never
implies deletion; realtime media and serving lifecycle inherit no structured
role.
Contract 040 keeps requested, planned, dispatched, accepted, effective, and
observed generation controls separate. It binds exact output maxima, reasoning
mappings, schema dialects, enforcement source, model capability, and version
milestones without prompt emulation, silent clamp, or fallback.
Contract 041 keeps finite attachments, native consumer tools, provider-owned
tools, approval or question extensions, and external search separate. It binds
exact input representation, callback strength, direct continuation, search
authority, model/version evidence, and joined cleanup.
Contract 042 keeps harness-managed retry, Swallowtail attempts, consumer
retry, active turns, stream attachments, transport connections, and session
resume separate. It requires explicit recovery and bounded reattachment
agreements, exact cursor continuity, no prompt replay, safe uncertainty, and
joined cleanup.
Contract 043 keeps one-child-per-turn harness continuation separate from
consumer-owned transactional transcript replay. Private continuation cannot
mint public load or resume; failed-turn commit, cancellation, bounds, provider
state, attached-service preservation, and joined cleanup remain exact.
Contract 044 adds operation-local observable activity identity, exact
lifecycle and disclosure fidelity, typed content streams, tool and request
correlation, provider-visible reasoning summaries, typed task-list replacement
snapshots, and bounded unknown-event truth. Existing run and turn streams
remain the transport. Consumers retain message and activity persistence,
grouping, collapsed presentation, and transcript policy.
Contract 045 adds bounded provider-owned child-work snapshots, parent
topology, child activity attribution, and typed provider collaboration
actions. It keeps visible harness actions separate from operator authority;
whole-turn cancellation and main-turn messaging cannot stand in for targeted
child control.
Contract 046 adds a read-only provider-session catalogue and a separate
consumer-authorized import operation. Candidates remain non-authoritative;
import revalidates exact route, host, access, version, model, resource, and
policy before issuing the ordinary resume binding. Load replay and consumer
persistence remain separate, with no background synchronization.
Contract 047 adds a bounded, consumer-assembled configured provider-instance
catalogue. It admits exact driver, instance, prepared-route, safe access, and
model-catalogue evidence into one immutable projection while excluding
credential and target authority. Strict derived selection readiness keeps
unknown, unavailable, failed, unsupported, and empty instances visible but
non-selectable; provider, model, route, default, fallback, refresh, and
persistence policy remain downstream.

Contract 015 now permits exact, one-shot activation of an already authorized
harness credential after ACP initialization. The first mapping is Grok Build
`0.2.114` `cached_token`; login, account or mechanism switching, API-key
fallback, and provider-private response metadata remain excluded.
It also qualifies unstable ACP form negotiation for the Claude choice-and-
Other subset. Richer forms are declined; URL authority and invented context
remain absent.
