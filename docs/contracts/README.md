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
