# g04 Route Readiness And Connection Admission

Status: active
Owner: Tom
Created: 2026-08-19

## Purpose

Give consuming applications a portable library surface for discovering addable
routes, admitting configured connections, collecting or launching required
credentials, observing readiness and updates, and exposing the model list those
connections can actually run.

g04 does not ship a connection server, UI, router, or secret store.
Swallowtail remains mechanism. Persistence is a port with an optional simple
adapter. Poodle, T3 Code, Nucleus, and later consumers own presentation chrome
and selection policy.

## Generation Runway

| Goal | State | Governing refs | First milestone |
| --- | --- | --- | --- |
| Inventory existing instance, access, discovery, catalogue, version, and prepared-facade records against the consumer connection lifecycle. | completed | Contracts 005-006, 008, 014, 020, 029, 032, 037, 047; Spec 011 | `g04.001` |
| Fold inventory into Spec 011 and name contract targets without facade code. | completed | Spec 011; Research 168 | `g04.002` |
| Pin the post-g03 source tree as an immutable tag before facade implementation. | completed | Contract 036 | `g04.003` |
| Promote the readiness/admission contract after that tag. | completed | Contract 057; 006, 008, 010, 014, 015, 017, 029, 032, 037, 047 | `g04.004` |
| Realize the persistence port and optional simple adapter. | completed | Contract 057 | `g04.005` |
| Realize addable-route catalog, admission, and config field descriptors. | completed | Contract 057 | `g04.006` |
| Realize library-max sign-in loops through host ports. | completed | Contracts 057, 006, 010, 014, 017 | `g04.007` |
| Realize readiness refresh, authenticated-subject observation, and Contract 029 updates. | planned | Contracts 057, 006, 029, 032, 047 | after sign-in |
| Realize the model-presentation overlay without flattening catalogues. | planned | Contracts 057, 020 | after sign-in |
| Prove representative hosted, installed, and local-runtime shapes and publish a consumer path. | planned | Contracts 011, 037, 052, 057 | later |
| Continue Contract 029 currentness as a maintenance lane, not the title programme. | recurring | Contract 029 | evidence-gated |

## Planned Next Roadmaps

- compile readiness refresh, subject observation, and overlay projection
  after g04.007 merges

## Current Checkpoint

- g04.001 through g04.007 implementation is on the worker branch
- g04.007 PR awaits review; merge is a separate operator-authorised action
- refresh, overlay, and first-proof stay behind that merge
- `v0.3.3` remains `51d18620`

## Milestones

- [001 Route Availability And Readiness Evidence](./001-route-availability-and-readiness-evidence.md) — completed
- [002 Route Readiness Spec And Contract Targets](./002-route-readiness-spec-and-contract-targets.md) — completed
- [003 Current Source Tag Before Readiness](./003-current-source-tag-before-readiness.md) — completed
- [004 Readiness And Admission Contract Promotion](./004-readiness-admission-contract-promotion.md) — completed
- [005 Connection Lifecycle Kernel](./005-connection-lifecycle-kernel.md) — completed
- [006 Addable Catalog, Admission, And Config Fields](./006-addable-catalog-admission-and-config-fields.md) — completed
- [007 Sign-In Loop And Host Ports](./007-sign-in-loop-and-host-ports.md) — completed
