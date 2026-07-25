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

Contract 003 defines the provider-neutral record kernel. Contracts 004-016
govern the realized runtime and current proof drivers. Contract 017 governs
persistent-session, write-callback, ambient-harness, and optional process-
containment work; exact deployed-runtime qualification applies only to an
enforced isolation claim. Contract 018 governs owned ephemeral serving, now
realized by the llama.cpp proof. Contract 019 governs in-process SDK drivers,
explicit cloud-client configuration, and delegated SDK credentials. Contract
020 keeps mutable catalogue observations separate from entitlement, runtime
capability, and route selection. Contract 021 makes provider-managed background
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
Contract 036 fixes the 23 public packages, coordinated pre-1.0 version,
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
