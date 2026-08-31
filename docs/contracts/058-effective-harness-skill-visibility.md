# 058 Effective Harness Skill Visibility

Status: active
Owner: Tom
Updated: 2026-08-31
Research: 255

## Purpose

Let a consumer observe the bounded effective skill roster visible to the model
in one exact prepared harness context. Preserve global, project-local, plugin,
provider-managed, and distribution-bundled skills without treating installed
or discovered files as effective visibility.

This contract owns the selected-harness effective overlay only. It does not
inventory installed skills. Spec 013 plans a separate host-approved inventory
surface for global, project-bound, and harness-distributed roots. That surface
must preserve its weaker discovery evidence and cannot satisfy this contract.

## Effective Roster

The portable claim is selected-context visibility, not installation:

- a skill belongs in the roster only when the exact harness surface reports it
  as available to the selected model, session, agent profile, or run
- distribution membership, package contents, a help entry, files on disk, and
  a model mentioning a skill are not visibility evidence
- operator-installed global and project-local skills are first-class roster
  members when the harness admits them
- provenance never limits inclusion; it records where the harness says the
  skill came from
- an empty complete roster, unavailable roster, and unverified roster remain
  distinct

The roster is a point-in-time observation. It is not an immutable route
capability, provider catalogue, tool list, MCP registry, prompt, or permission
grant.

## Observation Context

Every observation binds:

- exact configured instance, driver, transport, facade, interface version, and
  compatibility assessment
- selected model route and model when the harness exposes model-conditioned
  skill visibility
- execution host and harness-configuration posture
- exact working-resource presence or absence and its access posture
- selected session, run-init, or agent-profile identity supplied by the
  harness
- observation time, freshness strength, and completeness strength

Changing any bound dimension makes the observation stale. Load, resume, model
switch, profile switch, working-resource change, configuration change, or a
later turn does not inherit the roster unless the exact route proves that
lifetime.

## Skill Observation

Each bounded skill row carries:

- stable provider-local id or adapter-qualified id
- bounded display name and optional bounded description
- provenance: distribution-bundled, operator-installed global,
  project-local, plugin or extension, provider-managed, injected by
  Swallowtail, or unknown
- visibility scope: installation, project, profile, session, run, or unknown
- evidence strength: harness-declared, selected-context confirmed, or unknown
- optional exact provider namespace and opaque reference

Unknown provenance does not remove an otherwise confirmed row. Unknown
visibility strength cannot be upgraded to selected-context confirmation.
Names, descriptions, namespaces, and references are operation data. Default
formatting and diagnostics redact them.

## Observation Operation

A route may qualify one bounded read-only roster operation or expose the same
snapshot during a separately qualified no-prompt session or run-init phase.
The prepared operation fixes context, row count, row bytes, total bytes,
deadline, cancellation, and cleanup before effects.

The operation may start a host-approved harness and perform exact initialization
needed for the roster. It must not:

- send a model prompt or authorize inference
- make Swallowtail or the adapter recursively scan a home, project, plugin
  directory, or package tree; the harness may use its exact documented normal
  resolution to produce the roster
- install, update, enable, disable, or mutate a skill or harness configuration
- infer loading from a configuration file or package manifest
- initiate login or use credentials outside the selected configured instance;
  separately authorized delegated state may be used only when the route binds
  and discloses that requirement
- allocate durable provider state or retain a session unless the route
  separately qualifies and discloses that exact effect

If the harness cannot expose its effective roster without one of those effects,
the route reports unavailable. Swallowtail never fills the gap with a partial
ambient scan.

## Completeness And Freshness

A roster is complete only when the exact harness surface defines the returned
collection as the full effective set for the bound context. A filtered,
paginated, lazy, tool-invoked, or provider-account-dependent list needs its own
exact completeness rule.

Session-confirmed means the same selected context that will receive the first
prompt supplied the roster before that prompt. A prompt-bearing init event is
not prompt-free merely because the roster frame arrived before model output.

Version qualification follows Contract 029. Unverified-newer execution cannot
widen roster fields, completeness, provenance, or lifetime.

## Privacy And Authority

Selecting a harness, working resource, and ambient configuration posture
authorizes only the exact harness observation. It does not grant Swallowtail
general host inventory authority. The harness performs its own resolution;
the adapter maps the bounded result.

Consumers own whether names and descriptions are displayed, persisted, or
shared. Swallowtail owns bounds, identity, provenance preservation, redacted
formatting, and exact evidence strength. Roster membership grants no execution,
tool, filesystem, network, prompt, process, or permission authority.

## Relationship To Existing Contracts

- Contract 020 remains the model catalogue; skills do not enter it.
- Contract 032 executable discovery does not become skill discovery.
- Contract 033 binds configuration posture but grants no configuration scan.
- Contract 034 session options do not prove the effective roster.
- Contract 037 prepares the exact route and context before observation.
- Contract 041 keeps skills, tools, MCP tools, and callbacks distinct.
- Contract 047 remains the configured-instance selection snapshot.
- Contract 059's injected watcher skill may appear with
  `InjectedBySwallowtail` provenance only when this contract independently
  observes it.
- Spec 013's proposed inventory may contribute discovery and provenance only.
  Inventory membership never upgrades to harness-declared or selected-context
  confirmation without this contract's evidence.

## First Proof Disposition

Qoder headless `1.1.25` is the first evidence candidate because its selected
run init schema carries explicit `skills` and `plugins` collections alongside
model and session identity. Current fixtures prove only empty collections on a
prompt-bearing run. They do not prove a prompt-free operation, positive global
or project membership, completeness, provenance, or pre-prompt visibility.

No production route advertises this contract until that evidence closes. An
honest empty qualification leaves the contract active without a proof route.

## Conformance

Portable and route fixtures must prove:

- exact context identity and staleness after any bound dimension changes
- bounded row, collection, and total content
- complete, empty, unavailable, and unverified states remain distinct
- provenance preserved independently from visibility strength
- global and project rows retained when the harness reports them
- no file-presence, manifest, help, tool, or model-prose inference
- no prompt, model inference, Swallowtail ambient scan, mutation, or hidden
  durable state
- cancellation, deadline, process cleanup, and safe diagnostics

## Acceptance

- consumers can request the effective roster for one exact harness context
- operator-installed global and project skills are not excluded by provenance
- a route with no exact roster remains visibly unavailable
- no ambient inventory or skill mutation authority enters Swallowtail
- no production claim lands before one exact positive or complete-empty proof
