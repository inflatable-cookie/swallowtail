# Contract Summaries

These are the working summaries that used to live in the contracts README.
They explain what each contract governs and why it exists, in delivery order.
They are not a substitute for the contracts themselves; the
[contract index](contract-index.md) is the one-line authority map, and each
contract file owns the exact rules.

## Foundation: Identity, Runtime, And Execution (003-016)

- **003** defines the provider-neutral record kernel: the smallest stable
  vocabulary that adapters and consumers share.
- **004-016** govern the realized runtime and current proof drivers:
  - **004** splits execution shapes, host authority, and consumer/runtime
    ownership.
  - **005** keeps integration family, driver, transport, instance, and model
    route separate identities.
  - **006** separates harness interaction from direct model inference, and
    access profiles from operation shapes.
  - **007** keeps model artifact, serving runtime, deployment, facade, and
    route separate.
  - **008** governs driver roles, configured instances, access state, and
    side-effect-free preflight.
  - **009** defines the async operation lifecycle: scoped handles, events,
    cancellation, terminal outcomes, and cleanup.
  - **010** scopes execution-host authority: executable launch, credentials,
    resources, attachments, schemas, and diagnostics.
  - **011** defines deterministic cross-shape runtime conformance profiles.
  - **012** covers session instructions, reasoning selection, tool
    declarations, and correlated callback lifecycle.
  - **013** separates resource, isolation, filesystem, approval, network,
    deadline, and cleanup policy for interactive sessions.
  - **014** scopes endpoint grants, credential leases, direct streaming, and
    provider usage or limit evidence.
  - **015** governs ACP wire negotiation, delegated-credential activation,
    form elicitation, and execution-host callback authority.
  - **016** adds resource-free direct sessions with connection-bound
    continuation and exact billed-cost evidence.

## Persistent State, Serving, And Transport (017-035)

- **017** governs exact persistent-session load and resume, versioned ordinary
  resume-binding restart records, write callbacks, ambient harnesses, and
  optional process containment. Its restart record is attachment-bound
  authority input, not a consumer database or provider-session management
  binding; exact deployed-runtime qualification applies only to an enforced
  isolation claim.
- **018** governs owned ephemeral serving, now realized by the llama.cpp
  proof.
- **019** governs in-process SDK drivers, explicit cloud-client configuration,
  and delegated SDK credentials.
- **020** keeps mutable catalogue observations separate from entitlement,
  runtime capability, and route selection. It distinguishes standalone sources
  from session-negotiated options and records that the current common
  catalogue request has a deadline but no independent cancellation control.
- **021** makes provider-managed background execution, required temporary
  retention, bounded stream reattachment, and remote cancellation truth
  explicit and opt-in.
- **022** governs provider-hosted agent resources, durable retention,
  provider-managed recovery, authoritative persisted events, and remote
  deletion truth.
- **023** makes harness isolation operation-shape neutral and keeps provider
  permissions, native budgets, retained state, and optional sandboxing
  separate from host deadline, cancellation, and process authority.
- **024** permits structural Chat Completions codec reuse while keeping
  provider access, model, capability, lifecycle, evidence, retry, and fallback
  semantics inside separately qualified adapters.
- **025** makes provider-owned direct conversations explicit interactive
  session postures. It keeps the realized operation-owned delete-on-close
  route separate from a contracted retained resource-free load route with
  exact binding, bounded ordered replay, preservation on close, and separately
  authorized cleanup. Regional workspace access, response storage, item
  inventory, deletion truth, cancellation, load, and cleanup remain
  independent.
- **026** adds a separate realtime-media interactive role with exact media
  formats, bounded redacted chunks, native response cancellation,
  consumer-owned device and playback truth, and joined duplex cleanup.
- **027** makes provider-planned connection replacement explicit and bounded,
  keeps resumable provider handles private and operation-scoped, and separates
  rollover from reconnect, reattachment, retry, consumer resume, and durable
  state.
- **028** separates prompt, steering, follow-up, abort, command
  acknowledgement, and extension-UI relay for harness RPCs. It keeps
  downstream provider/model identity exact, ambient read intent uncontained,
  retry disabled, and cleanup joined.
- **029** keeps adapter, artifact, SDK, wire, service, facade, instance, route,
  and model versions separate. Execution binds exact observed points; drivers
  support maintained baseline-to-latest windows only through ordered,
  evidence-backed milestones, deprecation states, and exact exclusions.
  Ordered claims may separately permit exact unverified-newer attempts without
  extending their guaranteed support window; opaque claims remain exact-only.
- **030** adds a locally continued direct-session profile. Every provider
  attempt needs explicit consumer authorization; tool execution remains
  downstream while provider-private continuation stays bounded, redacted,
  route-bound, ephemeral, and distinct from provider cache or consumer memory.
- **031** governs attach-only native model runtimes. It keeps exact runtime
  version, installed and running inventory, selected route, artifact identity,
  and invocation-caused model residency separate without granting
  installation, model mutation, unload, or serving-lifecycle authority.
- **032** adds target-aware installed-executable discovery. It binds one
  opaque host-approved candidate, exact version axis, deadline, cancellation,
  authoritative host, qualified/unverified-newer/incompatible classification,
  and joined process cleanup without executable search, configuration
  promotion, authentication, or execution authority.
- **033** binds ambient, provider-suppressed, and host-scoped harness
  configuration independently from isolation, credentials, retention, and
  working resources. Host-scoped execution remains gated until a separate
  opaque host lease and service exist.
- **034** maps exact typed session options through version-qualified,
  harness-advertised configuration channels. Provider option records remain
  private; exact confirmation is required without model, route, access,
  isolation, or lifecycle fallback.
- **035** adds an opt-in experimental remote ACP transport over one exact
  host-approved HTTP/SSE or WebSocket endpoint. It keeps transport separate
  from provider identity, scopes connection and affinity state, excludes
  authentication and implicit recovery, and requires explicit joined close.

## Release, Prepared Integration, And Consumer Surfaces (036-052)

- **036** fixes the 27-package `v0.1.x` source line, admits Muse as package 28
  in the `v0.2.0` candidate, and governs the coordinated pre-1.0 version, the
  unified Rust 1.95 floor, semantic and documented API evidence, dependency
  policy, exact Git-tag consumption, consumer proof, and explicit human
  authority for every external release mutation. The initial release excludes
  crates.io and a GitHub Release object. Crate versions remain separate from
  Contract 029 provider-interface ranges.
- **037** requires an adapter-local prepared normal path for every production
  driver above the unchanged low-level roles. It binds adapter-owned facts,
  derives immutable plan echoes, preserves explicit access provenance and
  authority, exposes safe staged diagnostics, and permits typed bound
  operations without flattening role lifecycle. It also preserves
  unverified-newer posture, joined per-host service composition, and
  provider-wide deterministic evidence before replacement candidate freeze.
- **038** keeps consumer thread lifecycle separate from bound provider session
  management. It distinguishes attachment close, provider-native close,
  archive, restore, history removal, data deletion, hard deletion, and
  driver-owned cleanup; binds exact inactive targets; preserves version and
  capability truth; and requires explicit destructive authority and
  uncertainty. Its Kimi local-server addition keeps REST/WebSocket archive and
  restore separate from ACP, requires explicit cross-transport binding import,
  and qualifies no deletion.
- **039** permits exact provider routes to project one operation-private
  session, connection, process, or provider resource into a bounded structured
  run. It requires independent role qualification, exact request support,
  explicit retention, one terminal outcome, and joined cleanup. Close never
  implies deletion; realtime media and serving lifecycle inherit no structured
  role.
- **040** keeps requested, planned, dispatched, accepted, effective, and
  observed generation controls separate. It binds exact output maxima,
  reasoning mappings, schema dialects, enforcement source, model capability,
  and version milestones without prompt emulation, silent clamp, or fallback.
- **041** keeps finite attachments, native consumer tools, provider-owned
  tools, approval or question extensions, and external search separate. It
  binds exact input representation, callback strength, direct continuation,
  search authority, model/version evidence, and joined cleanup.
- **042** keeps harness-managed retry, Swallowtail attempts, consumer retry,
  active turns, stream attachments, transport connections, and session resume
  separate. It requires explicit recovery and bounded reattachment agreements,
  exact cursor continuity, no prompt replay, safe uncertainty, and joined
  cleanup.
- **043** keeps one-child-per-turn harness continuation separate from
  consumer-owned transactional transcript replay. Private continuation cannot
  mint public load or resume; failed-turn commit, cancellation, bounds,
  provider state, attached-service preservation, and joined cleanup remain
  exact.
- **044** adds operation-local observable activity identity, exact lifecycle
  and disclosure fidelity, typed content streams, tool and request
  correlation, provider-visible reasoning summaries, typed task-list
  replacement snapshots, and bounded unknown-event truth. Its composite
  activity key binds the consumer-unique runtime operation owner to the
  operation-local activity id; provider references and activity ids alone
  never become durable keys. Existing run and turn streams remain the
  transport. Consumers retain message and activity persistence, grouping,
  collapsed presentation, and transcript policy.
- **045** adds bounded provider-owned child-work snapshots, parent topology,
  child activity attribution, and typed provider collaboration actions. It
  keeps visible harness actions separate from operator authority; whole-turn
  cancellation and main-turn messaging cannot stand in for targeted child
  control.
- **046** adds a read-only provider-session catalogue and a separate
  consumer-authorized import operation. Candidates remain non-authoritative;
  import revalidates exact route, host, access, version, model, resource, and
  policy before issuing the ordinary resume binding. Load replay and consumer
  persistence remain separate, with no background synchronization.
- **047** adds a bounded, consumer-assembled configured provider-instance
  catalogue. It admits exact driver, instance, prepared-route, safe access,
  and model-catalogue evidence into one immutable projection while excluding
  credential and target authority. Strict derived selection readiness keeps
  unknown, unavailable, failed, unsupported, and empty instances visible but
  non-selectable; provider, model, route, default, fallback, refresh, and
  persistence policy remain downstream.
- **048** adds read-only cross-process reconciliation for consumer turns or
  structured runs whose runtime handles were lost. Session reconciliation
  binds an exact durable session and runtime turn; run reconciliation binds an
  exact persisted provider-run checkpoint. Run state includes provider-input
  wait; terminal attribution stays exact. Neither role grants retry, prompt,
  import, resume, cancellation, callback, management, cleanup, or child-control
  authority. Contract 022 separately admits exact inactive recovered-resource
  cleanup for a qualified driver-owned Managed Agents session and environment.
- **049** adds explicit controlled-shutdown detachment for qualified active
  runs or turns. It stops and joins only local observation work, reports local
  `Detached` truth, preserves provider terminal uncertainty, and requires a
  durable binding plus later reconciliation. Ordinary close remains unchanged.
- **050** adds one prepared working-state restoration facade over the
  qualified reconciliation operations and stateful provider-session
  continuation recovery. It also distinguishes exact live reattachment with
  discarded non-authoritative replay from a new replacement session with
  provider context lost. It preserves the selected method and outcome
  strength, never turns ACP load into reconciliation, and never falls back
  from failed observation to broader live-session authority. A separate
  consuming sequence may attach only after eligible settled reconciliation.
  Both operations are prepared before provider work; successful observation
  remains available if attachment fails.
- **051** adds portable failure origin, kind, and recovery evidence to the
  existing safe diagnostic boundary. Exact route codes, terminal source,
  preparation stage, cleanup truth, and downstream retry policy remain
  separate; unknown evidence stays unknown.
- **052** requires traceable, task-oriented consumer and operator guidance for
  every production route and portable feature. Guides and examples explain
  realized truth without widening route capabilities, authority, access,
  versions, lifecycle, cleanup, or recovery.

## Amendments

- **015** now permits exact, one-shot activation of an already authorized
  harness credential after ACP initialization. The first mapping is Grok Build
  `0.2.114` `cached_token`; login, account or mechanism switching, API-key
  fallback, and provider-private response metadata remain excluded. It also
  qualifies unstable ACP form negotiation for the Claude choice-and-Other
  subset. Richer forms are declined; URL authority and invented context remain
  absent.
