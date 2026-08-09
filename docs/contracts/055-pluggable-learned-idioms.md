# 055 Pluggable Learned Idioms

Status: accepted
Owner: Tom
Updated: 2026-08-09

## Purpose

Give consumers a provider-neutral mechanism to learn, store, select, and
deliver behavioral preferences ("idioms") from interaction signals — the
substrate Command Code's taste exposes — without composing product prompts,
absorbing host product policy, or binding to a closed model. The keyword
"idioms" replaces "taste" to stay mechanism-flavored and avoid Command Code
branding.

## Boundary

Idioms are preference, not authority. The mechanism never:

- composes, edits, or injects prompt text — it delivers a bounded `IdiomSet`
  at session preparation and the host owns prompt composition (vision:
  system prompts are host intent)
- enforces permissions or trust — allow/ask/deny, modes, and safety breakers
  remain host-owned authority outside this contract
- changes harness behavior — adapter routes never expose taste, mods, or
  learned preference surfaces; the Command Code route keeps taste/mods out
  per its route decision
- absorbs a learned backend — no Monkey or model dependency is named;
  learned backends implement the same trait later
- persists signals or owns registry state — retention and transport are
  consumer-owned

Signals qualify only where a consumer supplies them. Current adapter routes
have no accept/reject loop, so headless routes receive static rules only.

## Records

`swallowtail-idioms` owns pure provider-neutral records and functions:

- `IdiomId` — opaque, stable across scopes
- `IdiomScope` — `User | Project | Package(BoundedName)`
- `IdiomConstraint` — typed: `Text(pattern) | File(path-pattern) |
  Tool(name-pattern) | Command(pattern)`
- `Provenance` — `Static(source-ref) | Learned(signal-count, last-signal) |
  Imported(package-ref, merge-base)`
- `IdiomSignal` — `kind: Accept | Reject | Edit`, bounded redacted target,
  opaque session correlation, scope, sequence

## Confidence And Decay

`Confidence` stores a value in `0..=100` and an `as-of` monotonic time.
Effective confidence is a pure deterministic function of stored value,
elapsed time, and one fixed decay rate; the same record and elapsed time
always yield the same effective confidence. Fixture clocks pin decay in
conformance tests.

Merge operates on effective confidence at merge time with deterministic
outcomes: new, raised, lowered, or unchanged. A record never changes without
a signal or a merge.

## Engine Trait

- `IdiomSource::select(ctx) -> bounded IdiomSet` — ordered by scope then
  confidence; output is always bounded
- `IdiomSink::record(signal)` — fail-soft and optional, on the same model as
  `DiagnosticObserver`: no registered sink means no recording, and a failing
  sink never fails the operation
- the static-rules backend is the first tranche; learned and registry
  backends implement the same trait without widening it

Injection: the mechanism delivers a bounded `IdiomSet` at session
preparation. A host may map idiom text into optional redacted developer
instructions or its own prompt layer; the mechanism does not.

## Registry Client

The first tranche includes a portable registry client surface mirroring
taste-style pull/push:

- package references and namespaces as portable records
- pull merges remote learnings locally; push merges local learnings remotely;
  merge outcomes follow the confidence merge semantics above
- responses are bounded and typed

Network transport is host-owned through existing host ports; the crate has
no HTTP client and no raw network authority.

## Conformance

Portable and route tests must cover:

- merge outcomes: new, raised, lowered, unchanged
- decay determinism under fixture clocks
- lint bounds and malformed-record rejection
- selection ordering, bounded output, and scope resolution
- fail-soft sink behavior and missing-sink no-op
- registry merge without transport authority
- no prompt composition and no permission/trust side effect

## Acceptance

- consumers can learn, store, select, and deliver idioms without composing
  prompts or touching permission authority
- headless routes remain static-rules-only
- the crate graph stays acyclic and free of learned-model dependencies
- correction-loop proxy is measured through Soundcheck after the
  static-rules proof; vendor-style benchmark claims stay out of contracts
- focused validation passes without live provider work
