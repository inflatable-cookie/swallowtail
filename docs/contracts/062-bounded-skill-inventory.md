# 062 Bounded Skill Inventory

Status: active
Owner: Tom
Updated: 2026-08-31

## Purpose

Let a consumer request one bounded, read-only inventory of skill descriptors
from explicit host-approved global, project-bound, and harness-distribution
roots. Preserve source authority, traversal evidence, completeness, freshness,
duplicates, and conflicts without treating discovery as model visibility.

Contract 058 separately owns the effective roster reported by one exact
selected harness context. An inventory row never satisfies that stronger
claim by itself.

## Boundary And Ownership

The initial surface is provider-neutral and descriptor-only:

- `swallowtail-runtime` owns portable source, descriptor, snapshot,
  completeness, conflict, limit, and pure assembly records
- the execution host owns root approval and resolution, canonicalization,
  traversal, bounded reads, containment, deadlines, cancellation, and joined
  cleanup
- `swallowtail-host-local` may later implement the concrete local host service;
  this contract does not prescribe one public filesystem API
- each adapter owns its harness-distribution source declarations, decoder
  identity and revision, and mapping from source metadata into portable rows
- consumers own which global and project roots to approve, the exact working
  resource, refresh requests, and whether authorized metadata is displayed,
  persisted, or shared
- `swallowtail-testkit` may later own provider-free conformance fixtures and
  adversarial counterexamples

Runtime and core do not enumerate provider package layouts. Adapters do not
scan the host. Consumers cannot turn a raw path into authority by constructing
a portable record.

## Portable Records

The provider-neutral family is:

- `SkillInventoryRequest`: exact prepared source bindings and operation limits
- `SkillInventorySourceBinding`: one approved source identity, kind, authority
  binding, and decoder binding where applicable
- `SkillInventoryLimits`: the effective fixed-or-lower positive maxima
- `DiscoveredSkillDescriptor`: one bounded descriptor-only inventory row
- `SkillInventorySourceObservation`: one source's observation identity,
  completeness, safe reasons, and optional observed-at time
- `SkillInventoryConflict`: one bounded exact-id or decoder-declared conflict
- `SkillInventorySnapshot`: immutable rows, source observations, conflicts,
  applicability, and limits from one successful request
- `SkillInventoryFailure`: terminal request failure that publishes no snapshot

`SkillInventorySourceKind` has exactly `HostApprovedGlobal`, `ProjectBound`,
and `HarnessDistribution`. `SkillInventoryCompleteness` has exactly
`Complete`, `Partial`, and `Unavailable`. `SkillInventoryEvidence` has only
`DiscoveredDescriptor` in this contract. `SkillInventoryConflictKind` has
exactly `QualifiedIdMismatch`, `DecoderDeclaredDuplicate`, and
`DecoderDeclaredShadow`.

`SkillInventoryFailure` distinguishes `InvalidBinding`, `HostServiceFailed`,
`Cancelled`, `DeadlineExceeded`, and `CleanupFailed` without carrying unsafe
host detail. `SkillInventorySafeReason` has exactly `LimitExceeded`,
`UnsafeEntry`, `UnreadableEntry`, `MalformedDescriptor`, `DecoderRejected`,
`SourceUnavailable`, and `AdditionalFailuresOmitted`. Opaque ids and
adapter-qualified extensions remain bounded values, not provider enums in
runtime or core.

## Approved Sources

Every request names one to 32 opaque host-approved source bindings in a finite
ordered set. Source identities are unique within the request; a duplicate
rejects preparation. Source order is deterministic input order, not
precedence.

### Global

A global source binds one opaque host-approved root reference and root
revision. There is no default home, user configuration directory, environment
lookup, or ambient search.

### Project

A project source binds one opaque host-approved root reference to the exact
normalized working-resource identity, revision, execution host, and read-only
access posture prepared for the operation. The root must remain beneath that
working resource. A changed, absent, or mismatched resource rejects the source
binding before traversal.

### Harness Distribution

An adapter may declare a known distribution source only for one exact
configured-instance identity and revision, driver, facade, qualified interface
version, and decoder identity and revision. The host resolves that declaration
to an opaque approved root. The adapter cannot supply an unchecked path or
search package trees.

Layout qualification follows Contract 029. An unverified-newer interface does
not inherit an older distribution path or decoder claim.

Opaque root references and resolved paths are operation-private. They do not
enter public rows, default formatting, diagnostics, or persistence records.

## Prepared Inventory Operation

The request fixes before effects:

- operation, execution-host, and inventory-request identities
- every exact source binding and its source kind
- the working-resource binding when any project source is present
- configured-instance, driver, facade, interface-version, and decoder bindings
  for harness-distribution sources
- all library-owned maxima and any lower host maxima
- deadline, cancellation, and cleanup behavior

A stale, cross-host, cross-resource, cross-instance, cross-version, or
cross-decoder binding fails closed before rows exist. Preparation grants only
the bounded descriptor reads declared by the exact decoder. It grants no
general filesystem or provider authority.

## Fixed Limits

These are library-owned maxima. A caller cannot raise them. A host may apply a
smaller positive maximum when preparation records it and results preserve the
resulting partial or unavailable truth.

| Dimension | Maximum |
| --- | ---: |
| Approved source roots per request | 32 |
| Traversal depth beneath one root | 8 |
| Visited entries per root | 4,096 |
| Visited entries per request | 16,384 |
| Descriptor candidates per request | 2,048 |
| Bytes read from one descriptor | 256 KiB |
| Descriptor bytes read per request | 16 MiB |
| Emitted skill rows per request | 1,024 |
| Bounded omission or failure notes | 256 |
| Opaque ids, source ids, decoder ids, and namespaces | 256 UTF-8 bytes each |
| Display name | 256 UTF-8 bytes |
| Description | 2,048 UTF-8 bytes |
| Opaque reference | 512 UTF-8 bytes |

Every collection and text field rejects growth beyond its bound. Reaching a
traversal, candidate, byte, row, or note maximum never silently truncates a
source and calls it complete.

## Traversal And Containment

The authoritative host canonicalizes and validates each approved root before
work. Traversal is deterministic: directories use host-private normalized
relative entry names in ascending byte order, independent of filesystem
enumeration order. Depth and visit counts apply before an entry can be decoded.

The host may follow an entry symlink only when its canonical target remains
beneath the same approved root. It never follows a cross-root or escaping
symlink. It tracks host-private file identities to stop symlink and directory
cycles. Invalid traversal components, canonicalization failure, an escaping
target, or a cycle omit that entry and make the source partial with a safe
bounded reason.

Unreadable or concurrently removed entries are omitted and make the source
partial. A root that cannot be opened is unavailable. Raw names, relative
paths, canonical paths, file identities, filesystem errors, and descriptor
contents remain host-private.

The decoder declares the exact descriptor filenames or entry shapes it may
read. Traversal does not recursively read arbitrary skill bodies, prompts,
tools, executable content, configuration, environment files, or neighboring
metadata.

## Decoder And Descriptor Semantics

One emitted row contains only:

- an adapter- or source-qualified opaque stable inventory id
- bounded display name and optional bounded description
- provenance: `HostApprovedGlobal`, `ProjectBound`, or
  `HarnessDistribution`
- exact source identity, source kind, decoder identity, and decoder revision
- optional bounded declared namespace and opaque reference
- `DiscoveredDescriptor` evidence and the source observation identity

If the source format lacks a stable id, the decoder derives an opaque id from
the exact source identity, decoder revision, and host-private normalized
descriptor key. A display name is never an identity key. Public strings are
valid bounded UTF-8; malformed or oversized metadata omits that candidate and
makes the source partial.

The first surface exposes neither raw skill bodies nor a public body digest.
The host may use a private digest to construct an opaque observation identity,
but that does not authorize content disclosure or imply semantic equality.
Default formatting and diagnostics redact display names, descriptions,
namespaces, and references.

## Completeness, Failure, And Cancellation

Each source result is exactly one of:

- `Complete`: every candidate admitted by the exact decoder was examined
  within all bounds; zero rows is a complete empty source
- `Partial`: useful rows exist or the source was entered, but a fixed limit,
  unsafe entry, unreadable entry, malformed descriptor, or decoder rejection
  prevented a complete observation
- `Unavailable`: the approved root or decoder could not produce an observation

The whole snapshot is complete only when every requested source is complete.
Its bounded safe reasons identify the source and one closed reason class:
`LimitExceeded`, `UnsafeEntry`, `UnreadableEntry`, `MalformedDescriptor`,
`DecoderRejected`, or `SourceUnavailable`. Provider payloads, raw I/O errors,
paths, and descriptor content do not enter those reasons.

Deterministic rows admitted before a source-local limit may remain in a partial
snapshot. Exhausted failure-note capacity adds the fixed
`AdditionalFailuresOmitted` marker and preserves partial truth.

Invalid or stale preparation, host-service failure, cancellation, deadline,
or cleanup failure is terminal request failure. Cancellation and deadline stop
new admission, join all work, and publish no successful snapshot or rows.
Progress observations remain bounded and non-authoritative.

## Snapshot Identity And Freshness

One successful request returns an immutable point-in-time snapshot bound to:

- request and execution-host identity
- the exact ordered source identities, root revisions, and source kinds
- working-resource identity and revision when present
- configured-instance identity and revision, driver, facade, qualified
  interface version, and decoder revisions for distribution sources
- effective operation limits
- one opaque observation identity per source

An observed-at timestamp is present only when the host supplies its qualified
UTC time service. It is evidence of observation time, not current filesystem
truth. This contract creates no active watcher, universal freshness duration,
or background refresh loop.

A changed source declaration, root revision, working resource, configured
instance, interface qualification, decoder revision, or superseding source
observation makes the old snapshot stale for composition. A rescan returns a
replacement snapshot; it does not mutate the old one. Files may change after
the read, so even a complete snapshot claims only its observation point.

## Duplicates, Conflicts, And Precedence

Rows remain separate by qualified inventory id and provenance. Equal display
names are allowed and are never deduplicated.

A bounded conflict record exists only when:

- the same qualified inventory id resolves to differing descriptor metadata;
  or
- an exact decoder reports format-defined duplicate or shadow metadata

Conflict records preserve all involved row ids and sources. Precedence is
`Unknown` unless the exact decoder reports distribution-local precedence.
Even known distribution-local precedence does not choose a model-effective
winner. Source order, global versus project provenance, and consumer display
order cannot manufacture precedence.

## Composition With Contract 058

Inventory and effective visibility remain separate snapshots. There is no
automatic join, display-name join, or fallback from one to the other.

An optional consumer overlay may relate them only when:

- execution host, configured-instance identity and revision, driver, facade,
  qualified interface version, working resource, and configuration posture
  agree exactly
- neither source snapshot is stale for that context
- the adapter supplies an exact mapping between the inventory id and the
  harness roster id or opaque reference

The overlay preserves both evidence states. An effective row without an
inventory match remains `EffectiveNotInventoried`. An inventory row without an
effective match remains `VisibilityUnknown` when the effective observation is
partial, unavailable, unverified, or differently scoped.

`DiscoveredNotEffective` is permitted only for an exact mapping against a
complete Contract 058 roster for the same bound context. It means absent from
that one observation, not disabled, unusable, or globally shadowed. Contract
058 alone owns harness-declared and selected-context confirmation.

## Privacy And Authority

Inventory authorizes no install, update, removal, enablement, disablement,
configuration mutation, prompt injection, skill execution, tool admission,
MCP registration, provider call, credential access, network access, or model
visibility claim.

Approved roots are capabilities, not public paths. Descriptor metadata is
operation data. Consumers choose its product use under their own disclosure
policy; Swallowtail preserves bounds, provenance, redacted diagnostics, and
evidence strength.

## Relationships

- Contract 010 owns capability-scoped host services and safe diagnostics.
- Contract 029 owns exact interface-version qualification.
- Contract 032 executable discovery does not become skill inventory.
- Contract 033 configuration posture grants no root or scan authority.
- Contract 037 prepares the exact operation and joins host work.
- Contract 047 supplies configured-instance identity, not distribution roots.
- Contract 058 exclusively owns selected-harness effective visibility.
- Contract 059 watchers do not refresh inventory snapshots.
- Contract 061 may describe route support for inventory but grants no scan.

## First Proof Disposition

The first implementation tranche is provider-free:

1. add runtime records, fixed limits, immutable snapshot assembly, and
   fail-closed identity checks
2. add testkit fixtures for approved global, project-bound, and
   harness-distribution sources over a deterministic in-memory host
3. prove escape, symlink cycle, malformed metadata, every fixed limit,
   cancellation, deadline, stale composition, duplicates, and false
   effective joins
4. add a concrete host-local traversal service and adapter decoder only after
   the portable proof passes

No production route advertises inventory before its exact source declaration,
decoder, host service, and version evidence qualify independently.

## Conformance

Portable and route fixtures must prove:

- every root is explicitly host-approved and bound to the right authority
- ambient home, project, package, and provider scans are impossible
- deterministic traversal, depth, entry, byte, row, text, and note limits
- canonical containment across ordinary entries, symlinks, and cycles
- complete empty, partial, unavailable, cancelled, timed-out, and stale remain
  distinct
- duplicates and conflicts retain provenance without an inferred winner
- mixed host, resource, instance, version, decoder, or observation evidence
  fails closed before composition
- raw paths, bodies, provider payloads, and unsafe diagnostics never escape
- inventory cannot become Contract 058 confirmation without an exact complete
  same-context mapping
- every terminal path cancels or stops and joins host work

## Acceptance

- consumers can request a bounded descriptor inventory from all three source
  kinds without flattening their authority
- callers cannot widen roots, traversal, bytes, rows, text, or disclosure
- discovery, completeness, freshness, conflict, and effective truth stay
  separate
- a provider-free implementation can proceed without a new product-policy
  decision
