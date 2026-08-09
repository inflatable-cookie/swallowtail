# 006 Pluggable Learned Idioms

Status: draft
Owner: Tom
Updated: 2026-08-09

## Purpose

Decide whether Swallowtail can own a provider-neutral mechanism for learning
and applying behavioral preferences ("idioms") from interaction signals —
the substrate Command Code's taste exposes — without absorbing host product
policy or binding to a closed model. Keyword chosen over "taste" to stay
mechanism-flavored and avoid Command Code branding.

## Scope

In:

- portable idiom record: typed symbolic constraint with confidence,
  provenance, and scope (user / project / package)
- signal ingestion from existing event and tool-call transport (accept,
  reject, and edit deltas)
- pluggable engine trait: static-rules backend (baseline), learned backend,
  registry backend
- injection into harness session context with budget and compression
  semantics
- headless posture: static rules only; learned layer opt-in where signals
  exist
- constraint-store merge semantics (push/pull, confidence updates, lint)

Out:

- taste-1 replication or any learned-model implementation in the first
  tranche
- Monkey backend implementation until contracts settle (design reference
  only)
- product policy: per-consumer meaning of idioms
- permission and trust enforcement (separate concern; host-owned authority)
- remote registry service
- self-published-style benchmark claims

## Decisions Needed

Settled keyword: **idioms** (2026-08-09). Drafted positions below are
proposals grounded in the architecture; they need operator confirmation
before contract work.

### Drafted: crate surface

New `swallowtail-idioms` crate depending on `swallowtail-core` only. Core and
runtime stay untouched. Rationale:

- follows the separate-crate pattern of the protocol boundaries
  (`swallowtail-protocol-acp`, `swallowtail-protocol-openai-chat`)
- keeps the runtime dependency floor (core, `futures-core`, `zeroize`)
- dependency direction stays acyclic: `swallowtail-idioms` -> core;
  consumer and learned backends -> `swallowtail-idioms` -> core
- entry into the workspace release set still requires the Contract 036
  architecture/package review, as with every adapter tranche

### Drafted: record schema

- `IdiomId` — opaque, stable across scopes
- `IdiomScope` — `User | Project | Package(BoundedName)`
- `IdiomConstraint` — typed: `Text(pattern)` | `File(path-pattern)` |
  `Tool(name-pattern)` | `Command(pattern)`
- `Confidence` — `u8` 0..=100, deterministic merge (new / raised / lowered /
  unchanged); no decay in the first contract (open fork below)
- `Provenance` — `Static(source-ref)` | `Learned(signal-count, last-signal)` |
  `Imported(package-ref, merge-base)`
- `IdiomSignal` — `kind: Accept | Reject | Edit`, bounded redacted target,
  opaque session correlation, scope, sequence

Signals qualify only where a consumer supplies them. Current adapter routes
have no accept/reject loop, so headless routes get static rules only.

### Drafted: engine trait boundary

- `IdiomSource::select(ctx) -> bounded IdiomSet` — scope- and
  confidence-ordered
- `IdiomSink::record(signal)` — fail-soft, optional registration on the same
  model as `DiagnosticObserver` (missing sink means no recording, never a
  failure)
- static-rules backend is the first tranche; learned and registry backends
  implement the same trait later
- injection: Swallowtail delivers a bounded `IdiomSet` at session
  preparation; prompt composition stays host-owned (vision: system prompts
  are host intent). Interactive session requests already carry optional
  redacted developer instructions — a host may map idiom text there; the
  mechanism itself does not compose prompts
- merge and packaging semantics: pure functions in `swallowtail-idioms`
  (merge new/raised/lowered/unchanged, lint bounds), transport stays
  consumer-owned

### Open forks (operator decision before contract)

Settled 2026-08-09:

1. confidence decay — **time-based decay** with fixture-clock determinism
   (Contract 055)
2. learned backend — **trait seam only**, no Monkey absorption (Contract 055)
3. distribution — **registry client in the first tranche**, transport
   host-owned (Contract 055)
4. validation evidence — **consumer-proven**: correction-loop proxy through
   Soundcheck after the static-rules proof (Contract 055)

## Acceptance Criteria

- [x] Research 117 records evidence and recommendation
- [x] spec records the mechanism boundary versus host-owned policy
- [x] four open forks settled by the operator and recorded here
- [ ] architecture notes crate placement (deferred until code lands;
      architecture records what exists)
- [x] contract governs the engine trait and record schema
- [x] roadmap and ready cards sequence a static-rules proof first, learned
      backend later (g03.062, cards 189-192; candidate lane awaiting operator
      selection)

## Promotion Targets

- `docs/architecture/system-architecture.md`
- a new contract under `docs/contracts/`
- a roadmap and ready cards under `docs/roadmaps/g03/`
- Research 117 promotion into logs on close
