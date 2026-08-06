# 043 v0.1.0 Source Release Readiness

Status: active
Owner: Tom
Created: 2026-08-05
Depends on: g03.042
Vision tags: source release, consumer readiness, public API, security
Contract refs: 002, 029, 032, 036-037, 052
Planning state: cards 124-128 complete; card 129 active

## Problem

Swallowtail is functionally broad and deterministic QA passes, but its release
authority, package gates, public Rust documentation, dependency policy, and
consumer front door do not describe the current 27-package source tree.

## Goal

Prepare one exact `v0.1.0` GitHub source-tag candidate. Do not publish crates,
create a GitHub Release, or create or push the tag without a later exact
operator approval.

## Execution Plan

- [x] card 124: audit and source-tag contract
- [x] card 125: Bedrock TLS repair and dependency policy
- [x] card 126: public API review, narrowing, semantic baseline, and Rustdoc
- [x] card 127: consumer front door, source install, release notes, and support
- [x] card 128: GitHub CI and source-tag candidate selector
- [ ] card 129: full deterministic candidate and external-source consumer proof
- [ ] card 130: exact operator handoff; tag creation remains separately gated

## Boundaries

- no crates.io publication
- no GitHub Release object
- no tag creation or push before explicit approval of the exact candidate
- no live provider or authenticated harness work by default
- no consumer repository edits
- no package merge or umbrella facade without separate evidence
- no mass internal rewrite solely to reduce file or duplicate counts
- no public item retained only to avoid reviewing compatibility before the
  first baseline

## Acceptance

- [x] all 27 packages declare source-only publication posture
- [x] no unaccepted normal-graph dependency advisory remains
- [x] dependency license and source policy is automated
- [x] supported public API is intentional, documented, and semantically
      baselined
- [x] root onboarding and release notes describe exact Git-tag consumption
- [x] deterministic CI covers QA, Rust floors, docs, security, and source use
- [ ] an isolated consumer compiles and prepares representative routes from the
      exact candidate source
- [ ] tag handoff names the exact commit and excluded external actions

Card 128's repository work and read-only release simulation pass all 11
deterministic gates. Effigy's explicit first-tag/current-version mode selects
the already-declared `0.1.0`, plans `v0.1.0`, omits a no-op version rewrite,
and writes no release state. Card 129 now waits for the audit work to be
reviewed and committed so its candidate evidence can bind one exact clean
commit.

## Lane Runway

Card 126 replaced declaration hashing with a pinned 27-package semantic API
inventory and brought both protocol crates plus the two smallest support crates
to denied-missing-docs closure. The reusable testkit is also closed after an
intentional export review and 83 focused tests. Package-family API and Rustdoc
review continues. The first core tranche now enforces model-catalogue and
interface-compatibility documentation locally. The second core tranche closes
access, capability, requirement, and preflight documentation locally; 4,395
warnings remained. The third core tranche closes configured routes,
registration, discovery, events, diagnostics, and failure classification
locally. The final core tranche closes session, activity, transport,
continuation, media, and specialized policy records; `swallowtail-core` now
enforces denied missing docs at its root. The first runtime tranche closes
role requests, registration, host effect ports, the host registry, and
operation handles under local denial. The second runtime tranche closes
activity, provider observations, events, terminal outcomes, and callbacks.
The third closes reconciliation, durable checkpoints, route-selected
restoration, and settled observe-then-attach sequencing. Together they remove
812 warnings. The fourth runtime tranche closes durable resume bindings,
inactive-session management, typed lifecycle operations, and separately bound
recovered-resource cleanup. Those first four runtime tranches remove 928
warnings. The fifth closes configured provider-instance catalogue admission
and explicit provider-session catalogue/import without conflating observation
with resume authority. Those first five runtime tranches remove 1,099
warnings. The sixth closes explicit operation/session policy and prepared
evidence without adding defaults or authority. Those first six runtime
tranches remove 1,224 warnings. The seventh closes typed harness interaction,
operation-local child projection, and semantic event delivery. The runtime
tranches then remove 1,339 warnings. The eighth closes resource-free direct
continuation and realtime media ordering without flattening private
continuation or planned rollover into durable state. The runtime tranches have
removed 1,464 warnings. The ninth closes host inputs, bounded I/O, leases,
replay, time, cancellation, and policy validators at the runtime crate root.
All 1,667 runtime warnings are removed. The first adapter batch then closes
Anthropic, DeepSeek, Kimi Platform, and xAI without flattening their direct,
managed, continuation, cache, catalogue, or WebSocket routes. Eleven of 27
packages now enforce the gate; 1,883 adapter warnings remain visible and
unsuppressed. The broad hosted batch closes Alibaba Model Studio, Bedrock, and
OpenAI while preserving conversation-retention, SDK control-plane/runtime,
background-run, reconciliation, and realtime boundaries. Fourteen packages
then enforce the gate; 1,417 warnings remain across 13 adapters. The first
installed-harness batch closes Antigravity, Cursor, Grok, and Qwen while
preserving catalogue, structured, interactive, continuation, and recovery
differences. Eighteen packages now enforce the gate; 1,155 warnings remain
across nine adapters. The stateful harness batch closes Codex, Claude Agent,
and Kimi without flattening exec, ACP, app-server, native headless,
local-server, import, reconciliation, or lifecycle authority. Twenty-one
packages then enforce the gate; 662 warnings remain across six adapters. The
server and local-runtime batch closes Gemini, OpenCode, and Ollama while
keeping CLI, hosted API, attached server, provider-session, local inventory,
and fresh-replacement boundaries explicit. The final batch closes llama.cpp,
Oh My Pi, and Pi while preserving attached versus owned serving and independent
RPC capability truth. All 27 packages now deny missing public docs,
all-feature workspace Rustdoc is warning-free, and the 7,819-entry semantic API
baseline remains stable. Card 127 is ready after the required public-surface
review boundary. Card 127 then replaces the root chronicle, refreshes current
release copy, adds security and contribution/support policy, and makes exact
package, route, and source-pin truth part of docs QA. Card 128 is ready; its
GitHub workflow mutation remains separately approval-gated.
