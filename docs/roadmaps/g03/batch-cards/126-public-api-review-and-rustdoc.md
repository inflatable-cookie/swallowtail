# 126 Public API Review And Rustdoc

Status: complete
Owner: Tom
Created: 2026-08-05
Milestone: `../043-v0-1-0-source-release-readiness.md`
Depends on: card 125

## Goal

Freeze an intentional, documented first public Rust API instead of preserving
implementation detail through a source-line hash.

## Scope

1. Inventory semantic public API across all 27 packages.
2. Make implementation-only items private before the baseline.
3. Document supported items with useful contracts and examples.
4. Replace declaration hashing with semantic API evidence.
5. Deny missing public documentation in the candidate gate.

## Validation

- workspace semantic public API inventory
- `RUSTDOCFLAGS="-D missing_docs" cargo doc --workspace --no-deps --locked`
- `effigy check:examples`
- `effigy qa`

## Auto-Continuation

No. Review the resulting supported surface before consumer copy freezes it.

## Progress

- Replaced the 26-package declaration hash with a 27-package semantic
  `cargo-public-api` inventory: 7,819 normalized entries, all features, pinned
  tool and nightly versions.
- Corrected stale package metadata evidence for Oh My Pi and the Rust 1.90
  floor.
- Documented shared identity, host-reference, role-driver, and prepared-state
  macro sources. Missing-public-doc warnings fell from 5,897 to 5,182 without
  suppression.
- `swallowtail-host-local`, `swallowtail-transport-acp-remote`,
  `swallowtail-protocol-acp`, and `swallowtail-protocol-openai-chat` now pass
  and locally enforce denied missing-public-documentation builds.
- Reviewed `swallowtail-testkit` as a reusable consumer-facing conformance
  surface. Its exported assertions, scenarios, fixture constructors, and
  evidence accessors remain intentional; all now carry contract-oriented
  Rustdoc and the crate enforces denied missing docs.
- Focused testkit validation passes 83 tests, extracted package proof passes,
  and the workspace warning count falls by 353 from 5,182 to 4,829. Five of
  27 packages are closed under the release documentation gate.
- Reviewed the first `swallowtail-core` tranche: stable model identity,
  mutable catalogue observations, provider-defined catalogue extensions,
  lifecycle evidence, and qualified versus unverified-newer interface
  compatibility. These modules now enforce denied missing docs locally.
- The core tranche passes 65 focused tests and extracted package proof. It
  removes 123 warnings, leaving 848 in `swallowtail-core` and 4,706 across the
  workspace without suppression.
- Reviewed the core admission family: independent access-state dimensions,
  capability and constraint vocabulary, exact operation requirements, and
  side-effect-free preflight evidence. All four modules now enforce denied
  missing docs locally.
- The admission tranche passes 65 focused tests and extracted package proof.
  It removes another 311 warnings, leaving 537 in `swallowtail-core` and 4,395
  across the workspace without suppression.
- Reviewed configured instances and model routes, driver registration and safe
  discovery outcomes, common event envelopes and opaque extensions, redacted
  diagnostics, and the three-axis portable failure classification. These
  modules now enforce denied missing docs locally.
- The route/evidence tranche passes 65 focused tests and extracted package
  proof. It removes another 152 warnings, leaving 385 in `swallowtail-core`
  and 4,243 across the workspace without suppression.
- Completed the remaining core session-access, provider-session lifecycle,
  observable-activity, continuation, harness-RPC, realtime, attached-runtime,
  remote-ACP, and specialized policy surfaces. Crate-root denied missing docs
  now covers all of `swallowtail-core`.
- Final core validation passes 65 focused tests and extracted package proof.
  The last 385 core warnings are removed, leaving 3,858 across the workspace.
  Six of 27 packages are closed under the release documentation gate.
- Reviewed the first `swallowtail-runtime` tranche: exact role requests,
  descriptor-checked registration, host-supplied effect ports, the
  execution-host-bound service registry, and take-once operation handles.
  These five modules now enforce denied missing docs locally.
- Focused runtime validation passes 141 tests and extracted package proof. The
  semantic API inventory is unchanged. The tranche removes 260 warnings,
  leaving 1,407 in `swallowtail-runtime` and 3,598 across the workspace
  without suppression.
- Reviewed the runtime observation family: bounded activity and child-work
  records, provider metadata, semantic versus coalescible events, terminal and
  cleanup truth, and exactly-once callback exchange. These modules now enforce
  denied missing docs locally.
- Focused runtime validation again passes 141 tests and extracted package
  proof. The tranche removes 359 warnings, leaving 1,048 in
  `swallowtail-runtime` and 3,239 across the workspace without suppression.
- Reviewed the read-only recovery family: exact session and run
  reconciliation, opaque durable checkpoints, route-selected working-state
  restoration, and settled observe-then-attach sequencing. These modules now
  enforce denied missing docs locally.
- Focused runtime validation again passes 141 tests and extracted package
  proof. The tranche removes 193 warnings, leaving 855 in
  `swallowtail-runtime` and 3,046 across the workspace without suppression.
- Reviewed the runtime mutation-authority family: durable session-resume
  bindings, inactive provider-session management, typed lifecycle operations,
  and separately bound recovered driver-owned resource cleanup. These modules
  now enforce denied missing docs locally without merging resume, management,
  or cleanup authority.
- Focused runtime validation again passes 141 tests and extracted package
  proof. The tranche removes 116 warnings, leaving 739 in
  `swallowtail-runtime` and 2,930 across the workspace without suppression.
- Reviewed configured provider-instance catalogue admission and the separate
  provider-session catalogue/import family. Safe instance and candidate
  projections remain non-executable; only explicit, read-only candidate
  revalidation may issue an ordinary resume binding. Both module families now
  enforce denied missing docs locally.
- Focused runtime validation again passes 141 tests and extracted package
  proof. The tranche removes 171 warnings, leaving 568 in
  `swallowtail-runtime` and 2,759 across the workspace without suppression.
- Reviewed explicit operation policy, interactive session options, negotiated
  option evidence, immutable session-plan agreement, preparation failure
  stages, honest access provenance, and prepared operation evidence. All seven
  modules now enforce denied missing docs locally without adding defaults,
  routing, fallback, or executable authority.
- Focused runtime validation again passes 141 tests and extracted package
  proof. The tranche removes 125 warnings, leaving 443 in
  `swallowtail-runtime` and 2,634 across the workspace without suppression.
- Reviewed typed harness questions and answers, operation-local subagent
  directory projection, harness RPC scheduling and display observations,
  ordered event buffering, and bounded event delivery. These five modules now
  enforce denied missing docs locally without adding permission, tool,
  child-control, or consumer UI authority.
- Focused runtime validation again passes 141 tests and extracted package
  proof. The tranche removes 115 warnings, leaving 328 in
  `swallowtail-runtime` and 2,519 across the workspace without suppression.
- Reviewed resource-free direct continuation and realtime media coordination.
  Attempt authority, exact tool-result correlation, private continuation
  secrecy, opaque media, stream ordering, terminal truth, and bounded planned
  rollover remain distinct and explicit. Both module families now enforce
  denied missing docs at their roots.
- Focused runtime validation again passes 141 tests and extracted package
  proof. The tranche removes 125 warnings, leaving 203 in
  `swallowtail-runtime` and 2,394 across the workspace without suppression.
- Completed runtime host-input and support review: attachments, credentials,
  working-resource I/O, process requests and I/O, installed-executable
  discovery, network and serving grants, model-artifact leases, schemas,
  replay, time, cancellation, detachment, failures, and exact policy
  validators. Crate-root denied missing docs now covers all of
  `swallowtail-runtime`.
- Final runtime validation passes 141 focused tests and extracted package
  proof. The last 203 runtime warnings are removed, leaving 2,191 across the
  workspace. Seven of 27 packages are closed under the release documentation
  gate; the remaining warnings belong to the 20 provider adapters.
- Reviewed the first hosted provider batch: Anthropic Messages and Managed
  Agents, DeepSeek V4 continuation and structured runs, Kimi Platform K3, and
  xAI Models plus Responses WebSocket. Their prepared and low-level APIs keep
  route, credential, retention, recovery, cache, and continuation truth
  separate. All four crates now enforce denied missing docs at their roots.
- Focused validation passes 127 tests across the four packages, warnings-denied
  clippy passes, and all four extracted archives compile. The batch removes
  308 warnings, leaving 1,883 across 16 provider adapters. Eleven of 27
  packages are closed under the release documentation gate.
- Reviewed the broad hosted adapter batch: Alibaba Model Studio deployable
  models, structured responses, delete-on-close and retained conversations;
  Bedrock Runtime and control-plane SDK routes; OpenAI Models, background
  Responses reconciliation, and Realtime media. Their public docs preserve
  route, retention, SDK, service, recovery, and media boundaries. All three
  crates now enforce denied missing docs at their roots.
- Focused validation passes 115 tests across the three packages,
  warnings-denied clippy passes, all three extracted archives compile, and the
  semantic API baseline is unchanged. The batch removes 466 warnings, leaving
  1,417 across 13 provider adapters. Fourteen of 27 packages are closed under
  the release documentation gate.
- Reviewed the first installed-harness batch: Antigravity catalogue, headless,
  and continuation routes; Cursor catalogue, stream-JSON, and ACP routes; Grok
  structured and interactive ACP; and Qwen catalogue, structured, and
  turn-scoped session routes. Exact attachment recovery remains distinct from
  context-losing fresh-session replacement. All four crates now enforce denied
  missing docs at their roots.
- Focused validation passes 132 tests across the four packages,
  warnings-denied clippy passes, all four extracted archives compile, and the
  semantic API baseline is unchanged. The batch removes 262 warnings, leaving
  1,155 across nine provider adapters. Eighteen of 27 packages are closed under
  the release documentation gate.
- Reviewed the stateful harness batch: Codex exec and app-server work including
  catalogue, import, reconciliation, attachment, and lifecycle management;
  Claude Agent ACP including local auth, permissions, continuation, and
  lifecycle plus separate native `claude -p`; and Kimi ACP, headless, attached
  and owned local-server, cross-transport import, managed recovery,
  reconciliation, and lifecycle operations. All three crates now enforce
  denied missing docs at their roots.
- Focused validation passes 338 tests across the three packages,
  warnings-denied clippy passes, all three extracted archives compile, and the
  semantic API baseline is unchanged. The batch removes 493 warnings, leaving
  662 across six provider adapters. Twenty-one of 27 packages are closed under
  the release documentation gate.
- Reviewed the server and local-runtime batch: Gemini CLI ACP and headless plus
  hosted Models and Live; OpenCode catalogue, execution, import,
  reconciliation, replay load, resume, and lifecycle management; and Ollama
  attached runtime and model-artifact observation, inventory, inference, and
  resource-free sessions. All three crates now enforce denied missing docs at
  their roots without flattening their route or restoration boundaries.
- Focused validation passes 200 tests across the three packages,
  warnings-denied clippy passes, all three extracted archives compile, and the
  semantic API baseline is unchanged. The batch removes 407 warnings, leaving
  255 across llama.cpp, Oh My Pi, and Pi. Twenty-four of 27 packages are closed
  under the release documentation gate.
- Remaining work is package-family API review and useful Rustdoc. The warning
  count is retained as work, not hidden behind crate-wide allowances.
- Reviewed the final local-serving and RPC batch: llama.cpp attached and
  host-owned serving, Oh My Pi RPC, and Pi RPC. Their package qualification,
  catalogue, run, session, attachment, reasoning, and restoration differences
  remain explicit. All three crates now enforce denied missing docs at their
  roots.
- Focused validation passes 116 tests across the final three packages,
  warnings-denied checks pass, extracted package proof passes, examples
  compile, and the 27-package semantic API inventory remains unchanged.
- Card closure removes the final 255 warnings. All 27 packages now deny
  missing public docs, all-feature workspace Rustdoc passes with missing docs
  promoted to errors, and broad QA passes 1,459 tests plus documentation,
  guide, link, and route-matrix checks.
- The reviewed baseline remains 7,819 normalized semantic entries. No
  authenticated provider, consumer, tag, push, GitHub Release, or registry
  effect ran.
