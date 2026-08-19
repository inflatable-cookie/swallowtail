# g04.005 Connection Lifecycle Kernel

Date: 2026-08-20
Roadmap: `../roadmaps/g04/005-connection-lifecycle-kernel.md`
Cards: `../roadmaps/g04/batch-cards/013-lifecycle-core-records.md`,
`../roadmaps/g04/batch-cards/014-lifecycle-store-port.md`,
`../roadmaps/g04/batch-cards/015-host-local-simple-store-adapters.md`

## Result

The Contract 057 kernel exists.

`swallowtail-core` holds topology, addable-route descriptors, field
descriptors, enablement, overlay markers, redacted subject observations, and
admitted-instance records. Topology is not `ExecutionLayer`. Overlay markers
require a validated model id. Subject records default to redacted.

`swallowtail-runtime` owns `ConnectionLifecycleStore`. Enablement is stored
independently of access-status dimensions. The trait never takes secret bytes.

`swallowtail-host-local` ships in-memory and JSON-file adapters. JSON on disk
carries references only and refuses secret-byte fields. Several instances of
one family round-trip as distinct ids.

Additive public API lives in `release-baselines/public-api-unreleased/` for
core, runtime, and host-local. `public-api-0.3.3` is unchanged. No production
adapter crate changed. `PlannedConnectionRolloverPolicy` is untouched.

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-81ea2a8a`
Worker branch: `t3code/connection-lifecycle-kernel`

PR: https://github.com/inflatable-cookie/swallowtail/pull/4

## Next

Await review and operator-authorised merge. g04.006 stays planned until this
kernel lands.
