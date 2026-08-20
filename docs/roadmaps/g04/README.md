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
| Realize readiness refresh, authenticated-subject observation, and Contract 029 updates. | completed | Contracts 057, 006, 029, 032, 047 | `g04.008` |
| Realize the model-presentation overlay without flattening catalogues. | completed | Contracts 057, 020 | `g04.009` |
| Prove representative hosted, installed, and local-runtime shapes and publish a consumer path. | completed | Contracts 011, 037, 052, 057 | `g04.010` |
| Expand addable-route coverage on the proved hosted, installed, and local-runtime shapes. | completed | Contracts 011, 037, 052, 057 | `g04.015` |
| Close remaining 057/047 seams and expand addable coverage on proved shapes. | planned | Contracts 020, 037, 047, 057 | `g04.020` |

## Planned Next Roadmaps

- [g04.021 Unmarked Overlay Rows](021-unmarked-overlay-rows.md) — planned pending 020
- [g04.022 Further Addable Inventory](022-further-addable-inventory.md) — planned pending 021
- [g04.023 047 Presentation Metadata](023-047-presentation-metadata.md) — planned pending 022

Do not roll over: 19 numbered roadmaps now, 23 when this queue is filed,
target 30-50. Hosted OAuth stays parked.

## Current Checkpoint

- g04.001 through g04.020 are complete. PR 16 is on `main` at `576184e9`
- g04.020 completed: cards 056-058, handoff
  `docs/handoffs/20260820-205345-g04-020-config-ref-prepare-handoff.md`
- Hosted OAuth is parked. Contract 029 currentness is standing
- Generation stays active. Rollover waits for 30-50 roadmaps
- `v0.3.3` remains `51d18620`

## Milestones

- [001 Route Availability And Readiness Evidence](./001-route-availability-and-readiness-evidence.md) — completed
- [002 Route Readiness Spec And Contract Targets](./002-route-readiness-spec-and-contract-targets.md) — completed
- [003 Current Source Tag Before Readiness](./003-current-source-tag-before-readiness.md) — completed
- [004 Readiness And Admission Contract Promotion](./004-readiness-admission-contract-promotion.md) — completed
- [005 Connection Lifecycle Kernel](./005-connection-lifecycle-kernel.md) — completed
- [006 Addable Catalog, Admission, And Config Fields](./006-addable-catalog-admission-and-config-fields.md) — completed
- [007 Sign-In Loop And Host Ports](./007-sign-in-loop-and-host-ports.md) — completed
- [008 Readiness Refresh, Subject, And Updates](./008-readiness-refresh-subject-and-updates.md) — completed
- [009 Model Presentation Overlay](./009-model-presentation-overlay.md) — completed
- [010 First-Proof Route Inventory](./010-first-proof-route-inventory.md) — completed
- [011 Hosted API-Key Anthropic Messages](./011-hosted-api-key-anthropic-messages.md) — completed
- [012 Installed Codex App-Server](./012-installed-codex-app-server.md) — completed
- [013 Local Ollama Attach](./013-local-ollama-attach.md) — completed
- [014 Connection Lifecycle Consumer Path](./014-connection-lifecycle-consumer-path.md) — completed
- [015 Second-Proof Addable Inventory](./015-second-proof-addable-inventory.md) — completed
- [016 Hosted API-Key DeepSeek Continuation](./016-hosted-api-key-deepseek-continuation.md) — completed
- [017 Cline Stable Clippy Result Large Err](./017-cline-stable-clippy-result-large-err.md) — completed
- [018 Installed Claude Agent ACP](./018-installed-claude-agent-acp.md) — completed
- [019 Local llama.cpp Attached](./019-local-llama-cpp-attached.md) — completed
- [020 Config-Ref Prepare Handoff](./020-config-ref-prepare-handoff.md) — completed
- [021 Unmarked Overlay Rows](./021-unmarked-overlay-rows.md) — planned
- [022 Further Addable Inventory](./022-further-addable-inventory.md) — planned
- [023 047 Presentation Metadata](./023-047-presentation-metadata.md) — planned
