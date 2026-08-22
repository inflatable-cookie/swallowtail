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
| Realize a full Pi SDK sidecar route with exact session attachment. | completed | Contracts 017, 019, 023, 029, 037, 057 | `g04.033` |
| Work through official per-route feature gaps one route and one control family at a time. | active | Contracts 011, 020, 024, 037, 040, 041, 047, 052; per-route feature programme | `g04.036` |

## Planned Next Roadmaps

- [g04.023 047 Presentation Metadata](023-047-presentation-metadata.md) — completed and merged, cards 065-067
- [g04.024 Hosted API-Key Kimi Platform Chat](024-hosted-api-key-kimi-platform-chat.md) — completed and merged through PR 31 at `a08c89a1`, cards 076-078
- [g04.025 Codex 0.149.0 Useful Newer](025-codex-0-149-0-useful-newer.md) — standing currentness, completed
- [g04.026 Qwen Headless 0.21.15 Useful Newer](026-qwen-headless-0-21-15-useful-newer.md) — standing currentness, completed
- [g04.027 Ollama 0.32.15 Useful Newer](027-ollama-0-32-15-useful-newer.md) — standing currentness, completed
- [g04.028 Claude Code 2.1.238 Useful Newer](028-claude-code-2-1-238-useful-newer.md) — standing currentness, completed
- [g04.029 OpenCode HTTP 1.18.20 Useful Newer](029-opencode-http-1-18-20-useful-newer.md) — standing currentness, completed
- [g04.030 Antigravity 1.1.17 Useful Newer](030-antigravity-1-1-17-useful-newer.md) — standing currentness, completed
- [g04.031 Oh My Pi 17.4.0 Useful Newer](031-oh-my-pi-17-4-0-useful-newer.md) — standing currentness, completed
- [g04.032 Kimi Code 0.38.0 Useful Newer](032-kimi-code-0-38-0-useful-newer.md) — standing currentness, completed
- [g04.033 Pi SDK Sidecar Route](033-pi-sdk-sidecar-route.md) — completed and merged through PR 32 at `9aac2dd1`, cards 089-092
- [g04.034 Gemini CLI 0.56.0 Useful Newer](034-gemini-cli-0-56-0-useful-newer.md) — standing currentness, completed
- [g04.035 Cursor Headless Model Parameters](035-cursor-headless-model-parameters.md) — complete, cards 095-097
- [g04.036 Ollama Attached Context Window](036-ollama-attached-context-window.md) — ready, cards 098-100

Do not roll over: 36 numbered roadmaps exist; 001-035 are complete and 036 is ready,
target 30-50.

## Current Checkpoint

- g04.001 through g04.022 are complete. PR 20 is on `main` at `281244db`
- g04.023 is on `main` at `deedc3e4` through PR 23; cards 065-067 are
  complete
- g04.024 is on `main` at `a08c89a1` through PR 31; cards 076-078 are
  complete
- g04.025 standing currentness complete. PR 19 is on `main` at `25fc3e35`
- g04.026 standing currentness is on `main` at `550ba112`: Qwen 0.21.15
  qualified through PR 21
- g04.027 standing currentness is on `main` at `0c528209`: Ollama 0.32.15
  qualified through PR 22
- g04.028 is on `main` at `0cd5735d` through PR 24: Claude Code headless
  and response-only `2.1.238`; cards 079-080 are complete
- g04.029 is on `main` at `3dd72fcf` through PR 25: OpenCode HTTP
  `1.18.20`; cards 081-082 are complete
- g04.030 is on `main` at `a8317ac4` through PR 26: Antigravity catalogue
  and headless `1.1.17`; cards 083-084 are complete
- g04.031 is on `main` at `6d86feb6` through PR 27: Oh My Pi RPC
  `17.4.0`; cards 085-086 are complete
- g04.032 is on `main` at `7889cc63` through PR 30: Kimi Code ACP,
  headless, and local-server `0.38.0`; cards 087-088 are complete
- g04.033 is on `main` at `9aac2dd1` through PR 32: the Pi SDK sidecar
  route is realized and both Pi routes are retained; cards 089-092 are
  complete
- g04.034 is complete: cards 093-094 qualified Gemini CLI `0.56.0` across
  separate ACP and headless axes for enterprise API-key access
- g04.035 is complete: cards 095-097 froze exact Cursor model-parameter
  evidence, added typed headless binding, and proved bounded dispatch
- g04.036 is ready: cards 098-100 cover exact Ollama `num_ctx` evidence,
  adapter-local binding, and deterministic native-request acceptance
- Contract 029 currentness remains standing and does not move the generation
  pointer
- Generation stays active. Rollover waits for 30-50 roadmaps
- `v0.3.3` remains `51d18620`

## Post-024 Planning Checkpoint

1. g04.033 cards 089-092 executed: the Pi SDK sidecar and Contract 017
   attachment are proved; the recorded disposition retains both Pi routes.
2. Execute g04.036 cards 098-100 for Ollama attached `num_ctx`. Contract 029
   currentness remains standing.

New route-family research does not pre-empt this sequence.

## Milestones

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md) —
  post-Pi/Gemini route-local delivery sequence
- [036 Ollama Attached Context Window](./036-ollama-attached-context-window.md) — ready, cards 098-100
- [035 Cursor Headless Model Parameters](./035-cursor-headless-model-parameters.md) — complete, cards 095-097
- [034 Gemini CLI 0.56.0 Useful Newer](./034-gemini-cli-0-56-0-useful-newer.md) — completed (standing currentness)
- [033 Pi SDK Sidecar Route](./033-pi-sdk-sidecar-route.md) — completed and merged through PR 32 at `9aac2dd1`, cards 089-092
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
- [021 Unmarked Overlay Rows](./021-unmarked-overlay-rows.md) — completed
- [022 Further Addable Inventory](./022-further-addable-inventory.md) — completed
- [023 047 Presentation Metadata](./023-047-presentation-metadata.md) — completed and merged
- [024 Hosted API-Key Kimi Platform Chat](./024-hosted-api-key-kimi-platform-chat.md) — completed and merged through PR 31 at `a08c89a1`, cards 076-078
- [025 Codex 0.149.0 Useful Newer](./025-codex-0-149-0-useful-newer.md) — completed (standing currentness)
- [026 Qwen Headless 0.21.15 Useful Newer](./026-qwen-headless-0-21-15-useful-newer.md) — completed (standing currentness)
- [027 Ollama 0.32.15 Useful Newer](./027-ollama-0-32-15-useful-newer.md) — completed (standing currentness)
- [028 Claude Code 2.1.238 Useful Newer](./028-claude-code-2-1-238-useful-newer.md) — completed (standing currentness)
- [029 OpenCode HTTP 1.18.20 Useful Newer](./029-opencode-http-1-18-20-useful-newer.md) — completed (standing currentness)
- [030 Antigravity 1.1.17 Useful Newer](./030-antigravity-1-1-17-useful-newer.md) — completed (standing currentness)
- [031 Oh My Pi 17.4.0 Useful Newer](./031-oh-my-pi-17-4-0-useful-newer.md) — completed (standing currentness)
- [032 Kimi Code 0.38.0 Useful Newer](./032-kimi-code-0-38-0-useful-newer.md) — completed (standing currentness)
- [033 Pi SDK Sidecar Route](./033-pi-sdk-sidecar-route.md) — completed and merged through PR 32 at `9aac2dd1`, cards 089-092
