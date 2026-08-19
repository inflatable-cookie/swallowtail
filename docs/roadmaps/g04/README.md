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
| Realize the persistence port and optional simple adapter. | planned | Contract 057 | after contract |
| Realize addable-route catalog, admission, and config field descriptors. | planned | Contract 057 | after contract |
| Realize library-max sign-in loops through host ports. | planned | Contracts 057, 006, 010, 014, 017 | after contract |
| Realize readiness refresh, authenticated-subject observation, and Contract 029 updates. | planned | Contracts 057, 006, 029, 032, 047 | after contract |
| Realize the model-presentation overlay without flattening catalogues. | planned | Contracts 057, 020 | after contract |
| Prove representative hosted, installed, and local-runtime shapes and publish a consumer path. | planned | Contracts 011, 037, 052, 057 | later |
| Continue Contract 029 currentness as a maintenance lane, not the title programme. | recurring | Contract 029 | evidence-gated |

## Planned Next Roadmaps

Compile the first Contract 057 implementation roadmaps. No implementation card
is ready until that compilation.

## Current Checkpoint

- g04.001 through g04.004 are complete
- Contract 057 is active; Spec 011 is archived
- annotated `v0.3.3` peels to `51d186208e75dca4c04f077dd7179ec3c2fafae9`
- facade implementation remains planned until implementation roadmaps are
  compiled against 057

## Milestones

- [001 Route Availability And Readiness Evidence](./001-route-availability-and-readiness-evidence.md) — completed
- [002 Route Readiness Spec And Contract Targets](./002-route-readiness-spec-and-contract-targets.md) — completed
- [003 Current Source Tag Before Readiness](./003-current-source-tag-before-readiness.md) — completed
- [004 Readiness And Admission Contract Promotion](./004-readiness-admission-contract-promotion.md) — completed
