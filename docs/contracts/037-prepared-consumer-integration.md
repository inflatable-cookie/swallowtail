# Provider-Wide Prepared Integration And Bound Operations

Status: active
Owner: Tom
Updated: 2026-07-25

## Purpose

Define a small, hard-to-misconfigure integration layer above Swallowtail's
provider-neutral records and runtime roles for every production driver.
Preparation may bind adapter-owned facts, derive immutable plan echoes, and
offer typed bound execution. It does not absorb consumer intent, flatten
operation roles, or create authority.

## Layered Boundary

Swallowtail exposes two cooperating layers:

1. low-level provider-neutral records, host services, preflight, and operation
   roles
2. provider-specific prepared integrations built from those public boundaries

The low-level layer remains public and independently usable for advanced
consumers, custom hosts, remote-authoritative topology, and conformance.
Prepared integrations are additive. They do not replace or weaken the
low-level roles.

Prepared integrations belong to their adapter crates. Shared plan-derivation,
preparation-evidence, and diagnostic records belong in
`swallowtail-runtime`. Reusable conformance assertions belong in
`swallowtail-testkit`. Reusable local service composition belongs in
`swallowtail-host-local`. No umbrella crate or provider-specific type enters
core or runtime.

Codex is the first realized proof, not the boundary of this contract. Every
production driver must gain an adapter-local normal path. The low-level layer
remains the escape hatch and construction substrate.

## Production Route Coverage

Prepared coverage is measured per production driver, not per provider name or
crate. The current route set is:

- installed harnesses: Codex exec, Codex app-server, Claude Agent ACP, Gemini
  CLI ACP, Kimi Code ACP, Pi RPC, and Qwen headless
- attached harness network: OpenCode HTTP/SSE
- hosted direct and provider-owned state: Anthropic Messages, Kimi Platform,
  DeepSeek, Alibaba Model Studio conversations, OpenAI background Responses,
  and Anthropic Managed Agents
- realtime connections: xAI WebSocket, OpenAI Realtime, and Gemini Live
- embedded SDK: Bedrock Runtime and Bedrock catalogue
- local model runtimes: Ollama attached, llama.cpp attached, and llama.cpp
  owned ephemeral serving

Remote ACP is a transport composed by compatible ACP adapters. It does not
become another provider driver, select an integration family, or hide endpoint
and topology input.

## Explicit Preparation Input

A preparation request selects exactly one:

- registered adapter driver and operation shape
- configured-instance identity and revision
- authoritative execution host and host-approved service set
- approved executable, endpoint, SDK, or service target where required
- access profile and access evidence
- named operation profile

The selected operation profile may require more consumer input, including a
model route, working resource, reasoning selection, tool declarations,
external network and search policy, schema, attachments, or deadline.

Preparation never implicitly selects or falls back across:

- provider, integration family, driver, or operation shape
- model, route, endpoint, executable, SDK, or service
- credential mechanism, account, entitlement, billing, or support authority
- execution host, ownership, or topology
- writable access, network, search, approval, or tools
- prompt, workflow, retry, persistence, memory, or UI

Failure remains failure for the selected route. It does not authorize another
preparation attempt.

## Preparation Result

A successful prepared integration may:

- observe and classify one exact installed interface version under Contracts
  029 and 032
- select one adapter-private qualified behavior revision
- bind adapter, facade, ownership, configuration, and capability facts fixed
  by the selected driver and profile
- construct a consistent configured instance, access binding, requirements
  set, model route where supplied, and immutable preflight plan
- derive runtime request fields that repeat immutable plan state
- expose the expanded safe plan, exact compatibility assessment, access
  provenance, and selected named profile before provider effects
- retain the selected low-level driver or binding needed for typed bound
  execution

The prepared result remains bound to the exact target, host, instance revision,
driver, access evidence, and plan used during preparation. Drift is rejected
before operation effects. A prepared object is not a credential, entitlement,
provider-session, or durable routing object.

## Bound Operations

A prepared value may expose a typed operation method only for a low-level role
implemented by its selected driver. Examples include:

- catalogue observation
- structured-run start
- interactive-session open or resume
- direct-session open
- realtime-media connect
- provider-owned background-run create or reattach
- provider-managed agent run
- owned-serving start

There is no universal `send_prompt`, `run`, `session`, or provider operation.
Method names, request types, handles, events, callbacks, cancellation, remote
truth, and cleanup retain the semantics of the underlying role.

Bound execution removes consumer assembly of adapter-fixed descriptors,
bindings, configured-instance facts, plans, and plan-echoed request fields. It
does not remove explicit operation content or authority. The consumer still
supplies the prompt, schema, attachments, model where not fixed by the
prepared profile, tool declarations and callback execution, working-resource
authority, deadline, and other role-specific input.

The expanded safe plan or connection/start specification remains inspectable
before effects. Starting through the prepared value performs the same drift,
preflight, capability, access, cancellation, deadline, terminal-outcome, and
joined-cleanup checks as direct use of the low-level role. A convenience method
must not bypass or duplicate the low-level lifecycle.

Prepared values expose enough identity and evidence for diagnostics,
conformance, and advanced low-level use. They need not expose adapter-private
wire records or secrets.

The typed bound operation is the normal consumer path after successful
operation preparation. Extracting a plan and request to call the same
low-level role is an advanced escape hatch, not additional facade
functionality. A consumer that reconstructs the matching low-level driver when
the bound method already exists has adoption debt; Swallowtail must not answer
that debt with another operation abstraction.

## Plan-Derived Request Agreement

A runtime request field that merely echoes immutable preflight state must be:

- derived from that plan by a prepared constructor, or
- supplied explicitly through the low-level API and checked for exact
  agreement

An unrelated constructor default must not choose access, provider-state,
harness-configuration, isolation, retention, network, search, or other policy
that can contradict the plan. Pre-1.0 implementations remove such defaults
without a compatibility shim.

Derivation is limited to plan echoes and adapter-owned facts. Consumer-selected
operation content and authority remain explicit. Preparation cannot use model
catalogue defaults or ambient provider configuration to choose model,
reasoning, access, network, search, tools, or writable behavior.

Codex structured exec, app-server catalogue, and app-server interactive
session remain distinct operation paths. Read-only and bounded-workspace
sessions remain separate named profiles. The same separation rule applies to
every adapter with more than one driver or role.

## Access Evidence And Provenance

Preparation accepts access state only through one of two explicit sources:

- `Observed` — an `AccessStatus` produced by an identified host, provider, or
  consumer observation
- `CallerAsserted` — an `AccessStatus` the caller explicitly asserts for the
  selected preparation

The prepared result preserves the source kind and safe source identity.
Caller-asserted state does not become provider-observed or host-observed
evidence. Neither source changes the dimensional meaning of `AccessStatus`.

Preparation cannot turn configured, present, or delegated access into
`Ready`, `Available`, or `Allowed`; replace unknown dimensions; or infer
support authority. It performs no credential discovery, sign-in, account
selection, secret extraction, entitlement probe, billing selection, or
authentication fallback.

Access provenance contains no credential material, account identity, endpoint
token, raw response, or provider payload. A later live access probe remains a
separate, explicitly authorized operation.

An adapter may expose a canonical access-profile constructor when credential
mechanism, entitlement metering, endpoint audience, and support authority are
fixed facts of one provider-supported route. The caller still supplies the
profile identity and separate observed or caller-asserted status. Such a
constructor discovers no credential, account, entitlement, or readiness and
cannot relabel one login, billing, endpoint, or support route as another.

## Preparation Diagnostics

Preparation failures expose one safe primary diagnostic chain and one
machine-distinct stage:

- target selection or resolution
- process spawn
- bounded output
- process exit
- version parse
- compatibility classification
- access evidence
- preflight
- cleanup

The primary diagnostic retains safe causal ordering across stages. Cleanup
failure remains visible alongside the earlier failure. Stable formatting never
contains raw target paths, arguments, environment, stdin, stdout, stderr,
credentials, account identity, prompt or output bodies, or provider payloads.

Adapter-private diagnostics may retain restricted detail only through the
existing host diagnostic policy. They do not become stable public error
strings.

## Host Composition

An execution host may expose an inspectable, per-host composition of scoped
task, time, process, network, credential, working-resource, attachment, schema,
event, and diagnostic services. Composition:

- carries one exact execution-host identity
- includes only services the host explicitly supplies
- adds no capability or access authority
- owns and joins every spawned or blocking task
- contains no process-global executor or detached task
- preserves operation scope, resource ownership, limits, and cleanup order

Local composition is convenience, not topology policy. The same prepared
records support remote-authoritative services. A local service set cannot
substitute for the host bound by preflight.

Host target selection and approval may precede Contract 032 discovery. That
host action returns one opaque approved target. Installed discovery still
receives exactly one target and performs no `PATH`, installation, package
manager, endpoint, credential, or fallback search.

## Provider Profiles

A provider adapter owns facts that are fixed by its exact driver, qualified
version segment, and named operation profile. Each profile must expose its
expanded safe requirements and plan before effects.

A profile cannot hide authority. Writable resources, provider-side network,
search, tools, approvals, retention, and optional sandboxing remain explicit
and independently inspectable. Sandbox support is an opt-in capability, not a
prepared-integration prerequisite.

Provider-specific lifecycle and capability differences remain separate. A
prepared integration cannot flatten catalogue, structured run, interactive
session, realtime, serving, or other role semantics into a generic prompt
method.

Adapters may share provider-neutral family helpers for:

- installed target observation and exact version classification
- endpoint and credential-lease binding
- retained provider-resource ownership and deletion posture
- realtime connection setup and rollover posture
- explicit SDK client and delegated credential binding
- attached endpoint or owned-serving lifecycle binding

A family helper cannot register or select a provider, driver, model, target,
endpoint, credential, topology, or fallback. Provider-private facade
revisions, codecs, configuration mappings, and lifecycle behavior remain in
the adapter.

## Version And Compatibility

Prepared installed-harness and native-runtime routes preserve the exact
observed version and Contract 029 assessment. Points inside the maintained
guaranteed range are supported. An exact unverified-newer point may proceed
when the adapter's ordered claim permits it; its behavior remains
mileage-may-vary and does not extend the guaranteed range. A newer point is
not hard-denied solely because it is above the current maintained upper
milestone.

Opaque hosted-facade revisions, SDK package versions, protocol revisions, and
native runtime versions remain separate axes. Preparation cannot infer one
from another, convert an opaque claim into an ordered range, or silently
refresh any prepared version binding after drift.

## Lifecycle And Topology

Preparation and execution use the task, cancellation, deadline, and cleanup
rules in Contracts 009-010 and 032:

- every probe and preparation task belongs to an operation or host scope
- cancellation and host-monotonic deadline outcomes remain distinct
- preparation does not return success before owned work joins
- cleanup failure remains visible
- local and remote-authoritative host identities do not substitute

A prepared object may be reused only within the exact lifetime and drift rules
declared by its adapter. It never silently refreshes access, target, version,
route, or plan.

## Release And Consumer Evidence

Compile-only consumer evidence is insufficient for a release candidate that
claims a prepared normal path. Candidate evidence must execute deterministic,
credential-free preparation through the selected public packages and prove
the expanded plan and request agreement.

Live installed-binary and authentication checks remain separately gated.
Consumer repository edits and releases remain separately authorized.

An unpublished candidate may be superseded when later evidence invalidates its
normal integration path. Supersession preserves its source, package, checksum,
handoff, and validation evidence and grants no authority to publish a
replacement.

Replacement candidate freeze requires deterministic prepared-facade evidence
for every production route claimed by the package set. Existing Codex-only
consumer proof remains valid evidence for Codex but is not provider-wide
acceptance.

## Exclusions

This contract does not add:

- a generic `send_prompt` API
- provider, model, credential, endpoint, billing, or topology routing
- automatic authentication, installation, update, or version fallback
- consumer prompts, tools, policy, workflows, persistence, memory, or UI
- a container or sandbox prerequisite
- a global executor, detached task, or durable credential store
- compatibility shims for the unreleased low-level request footgun
- a central provider registry that constructs, selects, or routes operations

## Acceptance

- normal preparation cannot omit adapter-fixed facts
- plan echoes are derived or explicit and checked
- access status always carries honest provenance
- preparation stages are machine-distinct and safely formatted
- local composition owns and joins all work without widening authority
- exact target, version, host, instance, and route drift fail before effects
- low-level roles remain callable
- provider operation shapes remain separate
- every production driver has an adapter-local prepared normal path
- bound operations reuse the low-level lifecycle without hidden authority
- unverified-newer attempts remain distinct from guaranteed support
- deterministic runtime preparation is required before replacement candidate
  freeze
