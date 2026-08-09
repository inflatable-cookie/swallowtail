# Pluggable Idioms

Use `swallowtail-idioms` to learn, store, select, and deliver behavioral
preferences ("idioms") — the substrate Command Code's taste exposes — as a
provider-neutral consumer mechanism. Idioms are typed constraints with
confidence and provenance, scoped to user, project, or package. New to the
shared vocabulary? Read [Key Concepts](key-concepts.md).

This package is additive unreleased source after `v0.3.1`. Consumers must pin
an explicitly reviewed commit containing it. Immutable `v0.3.1` and earlier
tags do not contain the package.

## What The Mechanism Owns

- portable records: `Idiom`, `IdiomSignal`, typed `IdiomConstraint`, and
  `Provenance` (static, learned, or imported)
- `Confidence` with deterministic time-based decay under fixture clocks
- merge outcomes: new, raised, lowered, unchanged, on effective confidence
- lint over stores so malformed registry payloads fail closed
- `IdiomSource::select` returning bounded, scope-then-confidence ordered
  sets, plus the `StaticRulesSource` backend
- a fail-soft `IdiomSink` recorder on the `DiagnosticObserver` model
- registry pull/push merge with no transport authority

## What The Host Owns

- prompt composition: the mechanism delivers a bounded `IdiomSet`; the host
  maps it into its own prompt or instructions layer
- permission and trust enforcement: idioms are preference, never authority
- learned backends: any model-based source implements the same trait later;
  the crate carries no learned-model dependency
- transport and retention for registry packages and signal history

## Session Preparation

Build a source from portable records, then resolve a bounded delivery at
session preparation:

```rust
use swallowtail_idioms::{IdiomScope, MonotonicInstant, StaticRulesSource,
    prepare_session_idioms};

let source = StaticRulesSource::new(records);
let delivery = prepare_session_idioms(&source, IdiomScope::Project,
    MonotonicInstant::from_ticks(0), 8);
```

Selection includes user-scope records plus records matching the context
scope, ordered by scope then effective confidence, never exceeding the
bound. Headless routes have no accept/reject loop, so they receive static
rules only.

## Recording Signals

Recording is optional and fail-soft: no registered sink means no recording,
and a failing sink never fails the operation.

```rust
use swallowtail_idioms::IdiomRecorder;

let recorder = IdiomRecorder::none(); // no-op until a sink is registered
recorder.record(signal);
```

## Registry Pull And Push

Packages are portable records; merge follows the confidence outcomes.
Transport, auth, and wire bounds are host-owned — the crate has no HTTP:

```rust
use swallowtail_idioms::{pull_package, RegistryNamespace, RegistryPackage,
    RegistryPackageRef};

let package = RegistryPackage::new(
    RegistryPackageRef::new(RegistryNamespace::new("myorg")?, "cli")?,
    remote_records,
)?;
let outcome = pull_package(&local_store, &package, now);
```

See the compiled example in the package (`examples/prepared_session.rs`) for
the full consumer path.
