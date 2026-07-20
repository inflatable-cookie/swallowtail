# Local Materialization And Deadline Services

Date: 2026-07-19
Status: recorded

## Result

g01 card 023 is complete.

- `swallowtail-host-local` materializes only host-approved attachment and
  schema references, with separate configurable byte limits
- inline schemas and approved schema files produce redacted, operation-scoped
  file leases
- temporary working resources remain opaque, bind to one operation scope, and
  can feed a same-scope local process
- resource and file releases are explicit, awaited, and preserve
  consumer-owned sources
- the local monotonic clock produces deadline observations without choosing
  cancellation or terminal status; cancelled waits join their worker
- safe failures and default formatting contain no raw paths or copied content

No Soundcheck source changed. Product schema meaning, validation, repair,
ranking, and proposal application remain downstream.

## Evidence

- Contract 010
- `crates/swallowtail-runtime/src/host_traits.rs`
- `crates/swallowtail-host-local/src/materialization.rs`
- `crates/swallowtail-host-local/src/deadline.rs`
- `crates/swallowtail-host-local/tests/local_services.rs`
- `crates/swallowtail-host-local/tests/local_process.rs`

## Next Lane

Card 024 expands only the Codex exec capabilities that can be translated from
the proven structured-input and host-service contracts.
