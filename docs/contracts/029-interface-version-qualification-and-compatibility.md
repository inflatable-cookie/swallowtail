# 029 Interface Version Qualification And Compatibility

Status: active
Owner: Tom
Updated: 2026-07-22

## Purpose

Keep fast-moving harness, SDK, service, protocol, and facade releases usable
across the installed-device lifetime of a consuming application. One
Swallowtail release must be able to support a maintained version window rather
than only the upstream release current on its publication date.

## Separate Version Axes

Adapter-driver release, executable or package release, embedded SDK release,
wire protocol, provider API date, schema artifact, protocol facade, configured-
instance revision, model-route revision, and mutable model alias remain
separate identities.

One version cannot stand in for another. A stable wire protocol does not prove
that every harness release using it behaves alike. A package semver does not
version an unversioned hosted API. A moving model alias is catalogue evidence,
not an interface compatibility claim.

## Exact Observation And Maintained Window

A configured instance records the exact safe version observed or selected for
each interface axis relevant to its driver. The immutable preflight plan binds
those exact points. Execution never receives only a range and guesses which
artifact or service it reached.

A driver compatibility claim owns one ordered window for one interface axis.
It records:

- one inclusive baseline: the oldest release still supported
- one inclusive latest-qualified boundary
- one or more ordered support segments between those boundaries
- the behavior revision used to decode and map each segment
- whether each segment is maintained or deprecated-but-still-supported
- exact excluded releases inside an otherwise supported segment

Segment starts are compatibility milestones. A new milestone is required when
framing, schema, lifecycle, capability, invocation, authentication, failure,
cleanup, or deprecation behavior changes while the same driver remains useful.
The driver may dispatch privately by the exact bound version and milestone
behavior revision. Consumers still use the same operation shape.

The initial claim may be a one-point window. Later evidence may add:

- a wider segment using the same driver behavior
- a new milestone segment for changed behavior
- disjoint compatible groups around known breaks or exclusions
- a closed range only when the upstream version scheme has ordering semantics,
  upstream compatibility evidence supports the interval, and conformance
  covers its boundaries and known breakpoints
- explicit exclusions for withdrawn, vulnerable, or behaviorally incompatible
  releases

Semantic versions, ordered integers, and calendar dates may define ordered
windows. Opaque versions permit exact one-point segments only. Syntax alone is
not compatibility evidence. `latest`, open-ended ranges, ambient executable
discovery, and silent downgrade or upgrade are not claims. A version newer
than the latest-qualified boundary or older than the baseline fails discovery
or preflight as incompatible.

Deprecation is not immediate removal. A deprecated segment remains executable
and observable as deprecated for the claim revision that still supports it.
Removing it moves the baseline in a later Swallowtail release and must be
called out as a compatibility-window change. A consuming application may warn
or require an upgrade, but Swallowtail does not silently substitute a route.

The claim has its own revision. Changing membership, exclusions, evidence, or
support authority changes that revision and therefore invalidates stale plans.

## Upgrade Workflow

Supporting upstream movement should normally require:

1. observe the exact interface versions and capability surface
2. add or update a frozen corpus for changed behavior
3. run the existing provider-neutral profile and adapter assertions
4. extend the latest segment when behavior is unchanged, add a milestone when
   adapter-private mapping changes, or create a new driver/facade revision when
   the public lifecycle changed materially
5. publish the new configured-instance revision and claim evidence
6. deprecate an older segment before moving the supported baseline when the
   application/device support policy requires an overlap period

This keeps routine patch qualification small while retaining old installed
harnesses deliberately. It also prevents compatibility shims from accumulating
in core. Provider-specific decoding and migration stay inside the owning
driver unless two adapters prove a shared protocol boundary.

## Preflight And Discovery

Discovery may report an observed safe version, its matching behavior revision,
and maintained or deprecated support status. Discovery does not install,
upgrade, downgrade, authenticate, or choose another driver.

Preflight checks every required exact interface point against the configured
instance and selected driver window before provider work. Missing, substituted,
excluded, retired, newer-than-qualified, or unknown points identify the
interface-version dimension without exposing paths, tokens, raw manifests, or
provider payloads. Deprecated points pass while retaining visible deprecated
status and their exact behavior revision.

Hosted APIs with no trustworthy version observation use an exact dated facade
or evidence revision. They do not invent a semantic version. Runtime capability
negotiation may narrow a qualified claim; it cannot widen it.

## Conformance

Each compatibility claim records:

- driver and transport/facade identity
- version axis, ordering scheme, baseline, and latest-qualified boundary
- milestone segments, behavior revisions, deprecation states, and exclusions
- evidence date and support authority
- frozen corpus or maintained upstream evidence
- conformance profiles and provider-specific assertions run
- known exclusions and semantic breakpoints

Default QA covers the baseline, latest-qualified boundary, both sides of every
milestone, every deprecated segment, each exclusion and its neighboring
accepted points where they exist, and one representative interior point for a
non-singleton segment. Historical corpora remain in the repository while their
segments are supported. Live probes remain separately gated.

## First Pi Mapping

The first Pi RPC claim contains one semantic-version segment whose baseline and
latest-qualified boundary are both
`@earendil-works/pi-coding-agent@0.80.10`. The configured instance and preflight
plan bind that exact point separately from strict-LF RPC framing, Pi's
downstream provider and model, Swallowtail's adapter version, and the instance
revision.

A later compatible Pi release can extend the latest-qualified boundary after
its corpus passes the same assertions. A behavior change adds a milestone and
retains the older segment while its baseline remains supported. A public
protocol or lifecycle break creates a new claim or driver revision; it does not
weaken or erase the old proof.

## Acceptance

- exact runtime observations remain distinct from compatibility claims
- separate version axes cannot substitute for one another
- one Swallowtail release may support baseline-through-latest-qualified
  installed versions with explicit intermediate milestones
- adding a qualified release or milestone does not require a new common
  operation API
- ranges are ordered, evidence-backed, bounded, deprecation-aware, and
  exclusion-aware
- support-floor movement is explicit and follows an observable deprecation
  period when application policy requires one
- unknown versions fail closed without installation or fallback
- stale claims and configured-instance revisions invalidate preflight plans
- provider payloads and host paths stay outside stable diagnostics
