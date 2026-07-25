# 029 Interface Version Qualification And Compatibility

Status: active
Owner: Tom
Updated: 2026-07-24

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

A driver compatibility claim owns one qualified support window for one
interface axis. It records:

- one inclusive baseline: the oldest release still supported
- one inclusive latest-qualified boundary
- one or more ordered support segments between those boundaries
- the behavior revision used to decode and map each segment
- whether each segment is maintained or deprecated-but-still-supported
- exact excluded releases inside an otherwise supported segment
- whether exact ordered versions above the latest-qualified boundary may run
  as unverified newer releases

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
not compatibility evidence. `latest`, ambient executable discovery, and silent
downgrade or upgrade are not claims.

Classification has three outcomes:

- qualified: inside a maintained or deprecated segment and not excluded;
  Swallowtail's frozen corpus and conformance guarantee the mapped behavior
- unverified newer: an exact valid ordered version above latest-qualified,
  where the claim permits forward attempts and the point is not excluded;
  execution is allowed but carries no compatibility guarantee
- incompatible: below baseline, inside an unsupported gap, explicitly
  excluded, a non-qualified prerelease, malformed, unordered outside an opaque
  exact segment, or rejected by a qualified-only claim

An unverified-newer attempt uses the most recent qualified adapter-private
behavior revision. This is not a provider, model, endpoint, credential, or
route fallback. The exact observed version remains bound and visible as
unverified. Required schema, lifecycle, capability, authentication, and event
drift still fails safely during execution.

Claims choose their newer-version posture explicitly. Ordered claims may
permit unverified newer attempts. Opaque claims remain qualified-only because
they cannot establish ordering. Known-bad exclusions may sit above the current
qualified boundary so a later vulnerability or protocol break can close one
exact point without pretending nearby releases are qualified.

Deprecation is not immediate removal. A deprecated segment remains executable
and observable as deprecated for the claim revision that still supports it.
Removing it moves the baseline in a later Swallowtail release and must be
called out as a compatibility-window change. A consuming application may warn
or require an upgrade, but Swallowtail does not silently substitute a route.

The claim has its own revision. Changing qualified membership, exclusions,
newer-version posture, evidence, or support authority changes that revision
and therefore invalidates stale plans.

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

Qualification may be batched into normal Swallowtail releases. A new upstream
patch does not require an immediate Swallowtail release when the claim permits
unverified newer attempts. This keeps routine qualification small while
retaining old installed harnesses deliberately. It also prevents compatibility
shims from accumulating in core. Provider-specific decoding and migration stay
inside the owning driver unless two adapters prove a shared protocol boundary.

## Preflight And Discovery

Discovery reports the exact safe version and its qualified,
unverified-newer, or incompatible classification. Qualified observations carry
their matching behavior revision and maintained or deprecated support status.
Unverified-newer observations carry the latest-qualified boundary and the
adapter-private behavior revision available for a forward attempt. Discovery
does not install, upgrade, downgrade, authenticate, or choose another driver.

Preflight checks every required exact interface point against the configured
instance and selected driver claim before provider work. Missing, substituted,
excluded, retired, unsupported-gap, or incompatible points identify the
interface-version dimension without exposing paths, tokens, raw manifests, or
provider payloads. Deprecated points pass while retaining visible deprecated
status. Permitted unverified-newer points also pass while retaining their exact
unverified classification. Consumers may warn, require confirmation, or reject
before configuration; Swallowtail does not relabel the attempt as qualified.

Hosted APIs with no trustworthy version observation use an exact dated facade
or evidence revision. They do not invent a semantic version. Runtime capability
negotiation may narrow qualified behavior or stop an unverified attempt; it
cannot convert unverified evidence into qualified support.

## Conformance

Each compatibility claim records:

- driver and transport/facade identity
- version axis, ordering scheme, baseline, and latest-qualified boundary
- milestone segments, behavior revisions, deprecation states, and exclusions
- evidence date and support authority
- frozen corpus or maintained upstream evidence
- conformance profiles and provider-specific assertions run
- known exclusions, semantic breakpoints, and newer-version posture

Default QA covers the baseline, latest-qualified boundary, both sides of every
milestone, every deprecated segment, each exclusion and its neighboring
accepted points where they exist, and one representative interior point for a
non-singleton segment. Claims that permit forward attempts also cover one exact
newer stable point, exact-version preservation, unverified diagnostics, and
runtime drift failure. Historical corpora remain in the repository while their
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
- one Swallowtail release guarantees baseline-through-latest-qualified
  installed versions with explicit intermediate milestones
- ordered newer releases may execute without becoming guaranteed support
- adding a qualified release or milestone does not require a new common
  operation API
- ranges are ordered, evidence-backed, bounded, deprecation-aware, and
  exclusion-aware
- support-floor movement is explicit and follows an observable deprecation
  period when application policy requires one
- known-incompatible and unordered unknown versions fail closed without
  installation or fallback
- permitted unverified-newer versions retain exact identity and never become
  qualified implicitly
- stale claims and configured-instance revisions invalidate preflight plans
- provider payloads and host paths stay outside stable diagnostics
