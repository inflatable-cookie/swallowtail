# g02 Swallowtail Stabilization, Facades, And Release Discipline

Status: active
Owner: Tom
Created: 2026-07-24

## Purpose

Turn the broad pre-release implementation proven in g01 into a deliberately
versioned, packageable, testable library surface that consumer applications can
upgrade without tracking every provider release.

g02 does not imply API 1.0, immediate crates.io publication, or a freeze on
evidence-led provider work.

## Generation Runway

| Goal | State | Governing refs | Next milestone |
| --- | --- | --- | --- |
| Define public crate, version, MSRV, release-authority, and compatibility boundaries. | completed | Contract 036, release topology architecture | `g02.001` |
| Make the selected package graph reproducibly packageable without publishing. | completed | Contract 036 | `g02.001` |
| Establish pre-1.0 public-API and change-classification evidence. | completed | Contract 036 | `g02.001` |
| Replace low-level-only consumer assembly with a contract-bound prepared integration layer. | completed | Research 034, Contract 037 | `g02.002` |
| Prove a small Codex prepared facade without flattening exec and app-server. | completed | Contract 037 | `g02.003` |
| Migrate and simplify Nucleus and Soundcheck under consumer authority. | completed | repository authority map, Contract 037 | `g02.005` |
| Prove consumer runtime preparation against packaged artifacts. | completed | Contracts 011, 036-037 | `g02.006` |
| Establish provider-wide prepared and bound-operation foundations. | completed | Spec 006, Contract 037 | `g02.007` |
| Prove the facade across installed, hosted-direct, and attached-runtime shapes. | completed | Contract 037 | `g02.008` |
| Cover every remaining harness, hosted, realtime, SDK, and local-runtime route. | completed | Contracts 014-035, 037 | `g02.009`-`g02.011` |
| Prove packaged provider-wide integration and replace the held candidate. | completed | Contracts 011, 036-037 | `g02.012` |
| Replace synthetic candidate provenance with one canonical-history final candidate. | completed | Contract 036, release topology architecture | `g02.013` |
| Prove the candidate through sustained normal-path consumer workloads and harden defects. | active | Contract 036, consumer runtime evidence | `g02.014` |

## Current Checkpoint

- g01 closed at 49 roadmaps after 48 completed milestones and one backlog move
- 23 workspace crates share version `0.1.0`
- no release has been published
- all 23 manifests carry contract-complete metadata and crates.io publication
  policy
- all 46 internal normal dependency edges carry compatible registry
  requirements alongside local workspace paths
- resolver 3, the `1.93` general floor, and Bedrock's `1.94.1` exception pass
  deterministic declared-floor and current-stable checks
- Effigy owns deterministic metadata, dependency, public-declaration,
  documentation, MSRV, content, and local package-family gates
- actual package publication, tagging, and release mutation remain
  operator-gated
- Research 033 inventories all 23 candidate public packages and the exact
  three-stage publication order
- both current consumers use core, runtime, host-local, and Codex through
  sibling paths and declare no toolchain floor
- all 23 exact crates.io names were absent but unreserved when checked
- the operator approved Spec 004; it is archived after promotion
- Contract 036 fixes all 23 public packages, crates.io, coordinated `0.1.0`,
  compatible internal requirements, bounded MSRV, and human release authority
- card 003 is complete: all 23 packages assemble from an isolated clean source
  snapshot and the extracted package family passes locked check and test
  compilation
- the first candidate `0.1.0` freezes all 23 packages, exact publication order, source
  bundle, archive and content hashes, and reproducibility evidence
- isolated exact-package checks pass for Nucleus and Soundcheck without
  changing either consumer repository
- release notes and separate Nucleus and Soundcheck upgrade and rollback
  handoffs are complete
- Research 034 shows those consumer checks prove compilation but not valid
  runtime preparation; Soundcheck still compiles with incomplete Codex plans
- Contract 037 promotes Spec 005's layered prepared-integration API,
  plan-derived requests, explicit access provenance, safe staged diagnostics,
  joined local host composition, and separate Codex operation profiles; Spec
  005 is archived
- cards 006-007 realize plan-derived session agreement, safe preparation
  failures, access provenance, joined local tasks, exact local service
  composition, and opaque executable target approval
- card 008 adds the Codex exact-target factory, retains exact compatibility and
  access provenance, derives the configured-instance target from the probe
  target, and preserves separate exec and app-server identities
- card 009 adds separate prepared catalogue, read-only session, bounded-
  workspace session, and structured-exec values with inspectable evidence,
  immutable plans, matching runtime requests, and explicit consumer choices
- card 010 executes every prepared profile across local and remote-
  authoritative hosts, retains the full low-level lifecycle and compatibility
  suite, adds public guidance, and records exact Nucleus and Soundcheck
  migration inputs
- cards 011-012 migrate all Nucleus Codex paths, remove manual preparation,
  preserve consumer behavior, and pass 18 adapter plus 1,991 server tests
- cards 013-014 replace Soundcheck's manual catalogue, host/task, configured-
  instance, requirements, access, route, plan, and structured-request assembly
  with separate prepared app-server and exec paths
- Soundcheck's isolated production module passes four deterministic tests and
  its normal health, 106 Rust tests, 13 Vitest tests, locked app check, and QA
  pass
- card 015 builds a transient 23-package candidate and runs isolated packaged
  runtime selectors: Nucleus 14 pass with two live probes ignored, Soundcheck
  four pass with one live probe ignored, and the extracted Codex adapter 89
  deterministic tests pass
- packaged evidence covers exact versions, access provenance, all prepared
  profiles, callbacks, failure-before-effects, cancellation, deadlines,
  cleanup, and redaction without credentials or provider calls
- Spec 006 inventories all 22 production routes across six implementation
  families and promotes an adapter-local prepared facade plus typed bound
  operation as the normal path
- revised Contract 037 keeps operation roles distinct, retains low-level
  access, forbids central provider routing, and preserves guaranteed versus
  unverified-newer compatibility truth
- Codex remains the first realized prepared proof; Kimi ACP, Anthropic direct,
  and Ollama native add cross-shape coverage, while breadth rollout continues
  without flattening route behavior
- cards 018-019 complete provider-neutral prepared-operation evidence,
  installed/hosted/attached assertion coverage, and typed Codex catalogue,
  structured-run, session-open, and session-resume operations
- Codex bound operations delegate to unchanged low-level roles while retaining
  inspectable plans, requests, exact and unverified-newer evidence, access
  provenance, host authority, and the low-level escape hatch
- the first cross-shape implementation tranche selects Kimi Code ACP,
  Anthropic Messages, and Ollama native for architectural information rather
  than provider preference
- card 020 completes exact Kimi prepared discovery, ambient configuration and
  isolation evidence, new, load, resume, prompt, interruption, replay,
  bounded-write, delegated-authentication, and joined-cleanup proof
- card 021 completes separate Anthropic catalogue and one-attempt inference
  preparation, exact public endpoint and credential binding, source-scoped
  catalogue evidence, streaming lifecycle reuse, and local plus remote-
  authoritative proof
- card 022 completes exact attached-runtime preparation, distinct installed
  and running observations, explicit runtime-managed residency, external
  server ownership, drift closure, and unverified-newer execution
- card 023 accepts the shared facade surface for breadth rollout: common
  evidence remains provider-neutral, native operations and preparation effects
  remain distinct, and no missing durable rule blocks continuation
- card 024 completes separate Claude Agent and Gemini CLI stdio-ACP prepared
  facades, exact installed-version evidence, read-only ambient plans,
  failure-before-effects checks, and local plus remote-authoritative proof
- Claude retains explicit consumer model selection; Gemini retains
  provider-observed model truth without an invented route
- Gemini `0.51.0` is qualified while newer stable releases remain executable
  as visible unverified observations
- remote ACP remains a separate explicit composition route with no stdio
  fallback
- card 025 completes distinct Pi RPC and Qwen headless prepared facades
- Pi retains provider-suppressed configuration, ambient read-only authority,
  restrictive RPC scheduling, and unchanged prompt, steering, follow-up,
  abort, UI callback, cancellation, and joined-cleanup lifecycles
- Qwen retains ambient configuration and authority, exact text stdin and
  stream-JSON framing, provider/model identity, and fixed native wall,
  tool-call, and turn budgets without a sandbox claim
- both installed facades record exact versions, qualify their frozen
  baselines, and admit later stable releases as visible unverified-newer
  execution
- card 026 completes the attached OpenCode HTTP/SSE prepared facade with exact
  health observation, external service ownership, separate catalogue and
  read-only session paths, operation-scoped affinity and cleanup, and no
  server-lifecycle authority
- OpenCode HTTP/SSE remains provider-specific rather than ACP; remote ACP
  remains a separate explicit transport choice with no fallback or recovery
  path between them
- all eight harness production routes now have prepared normal paths
- card 027 completes separate Kimi Platform catalogue and one-attempt K3
  preparation plus DeepSeek catalogue and consumer-authorized direct tool
  continuation
- Kimi Membership and Platform billing remain separate; DeepSeek provider
  cache acceptance and every continuation attempt remain explicit
- card 028 completes exact Alibaba Singapore workspace preparation, explicit
  provider retention, serial continuation, and item-before-conversation
  deletion
- card 029 completes exact OpenAI public-API background preparation, explicit
  temporary retention, maximum-one cursor reattachment, bounded retrieval,
  native cancellation, and remote terminal truth
- card 030 completes separate Anthropic Managed Agent preparation, operator-
  owned agent binding, driver-owned environment and session, authoritative
  recovery, callbacks, interruption, and ordered deletion
- g02.010 is complete
- card 031 completes separate xAI Responses WebSocket, OpenAI Realtime, and
  Gemini Live prepared facades without a common connection abstraction
- xAI retains explicit model selection, private connection-local
  continuation, billed ticks, and whole-session invalidation
- OpenAI retains fixed 24 kHz PCM, manual input commit, native response
  cancellation, and no planned rollover
- Gemini retains asymmetric PCM, local interruption truth, one planned
  provider rollover, and no retry or durable-resume claim
- card 032 completes separate Bedrock Runtime and control-plane catalogue
  prepared facades with exact region, SDK, service, access, and cloud-client
  evidence
- Runtime requires one explicit model route and provider; catalogue remains
  route-free and cannot infer invocation availability or entitlement
- both paths require an explicit region and delegated credential provider;
  neither consults ambient AWS configuration
- card 033 completes distinct llama.cpp attached and owned-serving prepared
  facades
- attached preparation binds one approved external endpoint and exact
  b9910/f5525f7e7 runtime without process or serving-lifecycle authority
- owned preparation couples one approved executable, exact GGUF artifact,
  route and alias, then returns a handle only after loopback publication and
  b10069/178a6c449 readiness
- owned cleanup joins the child and invalidates endpoint authority before
  artifact release; local and remote-authoritative fixture suites pass
- card 034 publishes one machine-checked 22-route matrix across six families
  with explicit target, access, version, operation, and low-level escape-hatch
  guidance
- card 035 proves 65 prepared tests across all 22 route identities from one
  extracted 23-package candidate, then passes the selected Nucleus,
  Soundcheck, and packaged Codex runtime proofs
- package metadata, public API, docs, MSRV, content, route, and repository
  gates pass
- card 036 replaces the held candidate with exact clean source commit
  `73c7f5b5b5611ef20bdcc1572deeb39ca50630e1`
- the replacement's 23 packages reproduce from its source bundle, all 22
  packaged route proofs pass, and Nucleus, Soundcheck, and packaged Codex
  runtime evidence remains green
- the superseded compile-only, provisional, and parentless provider-wide
  candidates remain retained as immutable historical evidence
- publication remains blocked; no registry, tag, push, release, or workflow
  mutation occurred
- the publication reassessment rechecked all 23 exact crates.io names as
  absent but unreserved, verified the complete candidate source bundle, and
  found no local or remote `v0.1.0` tag
- cards 037-038 require clean non-root candidate provenance, preserve dirty
  working-tree package checks separately, and advance local `main` through
  normal history to the exact source recorded in active candidate evidence
- the complete source bundle reproduces all 23 archives and audited file lists
- card 039 passes 20 packaged suites across all 22 production routes,
  Nucleus, Soundcheck, and the 89-test packaged Codex suite without credentials
  or provider calls
- the final candidate source is
  `f142d927767f49fe86f2737d822fecf182f52591`, with exact normal-history parent
  `e9ead4d35fb7754962053417bf8328e646839b32`
- the first normal-history rebuild is retained superseded because its packaged
  README carried stale release-currentness wording
- local `main` is ahead of origin only; the candidate source is not yet
  reachable from the canonical remote branch and no `v0.1.0` tag exists
- later publication still requires the exact crates.io owner identity and
  separate approval of the desired main push, staged uploads, tag push, and
  GitHub release
- roadmap g02.013 and cards 037-039 complete the accepted provenance repair,
  canonical local commit, candidate replacement, and packaged acceptance
- publication is now held behind application-scale consumer proof
- Nucleus is the primary long-lived harness proof; Soundcheck is the secondary
  bounded structured-run proof
- card 040 completes the read-only proof-envelope audit and fixes exact
  Nucleus pilot, sustained, writable, and Soundcheck comparison workloads
- Nucleus now has app-scoped isolated state, normal Agent Chat cancellation,
  proof deadline control, a native proof selector, and disposable fixture
  binding
- Nucleus commit `962d1901` promotes that companion contract and plans g05
  cards 007-010
- Nucleus cards 007-010 complete deterministic readiness and freeze the exact
  runtime, Codex, access, topology, model, fixture, workload, and stop tuple
  without a provider call
- card 041 started after explicit acceptance of the ChatGPT-backed 15-turn
  and 60-minute live envelope
- launch one confirmed the exact model and audience, then exposed a
  Swallowtail-owned prepared-tool bound before session or turn work
- commit `54fbbc2` replaces the invented 4 KiB ceiling with exact bounds from
  the consumer's bounded declarations; all 90 Codex-adapter tests pass
- card 041's narrow reset is approved: 4 physical launches and catalogue
  attempts total so the original workload can run across 3 clean launches
  without increasing model turns, provider threads, effects, or execution
  time
- launch two exposed a second Swallowtail-owned facade defect before provider
  turn work: prepared interactive sessions omitted their promised turn-deadline
  time service
- commit `a26b54f` binds time in the prepared session plan and adds a joined
  deadline-bound regression; all 90 Codex and 19 deterministic Nucleus adapter
  tests pass
- card 041's second narrow reset is approved: 5 physical launches and
  catalogue attempts total; the 15-turn, 6-thread, read-only, and cumulative
  active-time ceilings remain unchanged
- clean launch three completed 4 ordinary turns, 1 bounded callback, and 1
  normal cancellation across 2 joined sessions; safe evidence records 5
  completed, 1 cancelled, and no failed, timed-out, active, or unexpected turn
- clean launch four completed the remaining 2 ordinary and 2 callback
  successes across 2 joined sessions, bringing the workload to 10 of 12
  planned outcomes before the final launch
- clean launch five proved persisted-session recovery and controlled deadline
  interruption; card 041 closed at the exact 15-attempt, 6-session ceiling
  with all 12 planned outcomes, joined cleanup, and no fixture drift
- Soundcheck exposed an opaque non-zero Codex installed-version probe through a
  host-selected wrapper; card 045 now retains numeric status and bounded
  sanitized stderr under the stable discovery failure code without adding
  wrapper policy
- Codex supplied no stable rate, quota, usage, or billed-cost summary; card
  042's first sustained tranche stopped after a shared native bundle identity
  routed UI control outside the isolated proof environment; 10 synthetic turns
  reached normal Nucleus state, those records are preserved, and an exact
  bundle-path 60-turn reset is active
- live provider calls, workspace writes, and consumer mutations remain
  separately gated

## Milestones

- [001 Release Boundary And Package Readiness](001-release-boundary-and-package-readiness.md)
  — completed
- [002 Prepared Consumer Integration Boundary](002-prepared-consumer-integration-boundary.md)
  — completed
- [003 Codex Prepared Integration Facade](003-codex-prepared-integration-facade.md)
  — completed
- [004 Nucleus Prepared Facade Adoption](004-nucleus-prepared-facade-adoption.md)
  — completed
- [005 Soundcheck Prepared Facade Adoption](005-soundcheck-prepared-facade-adoption.md)
  — completed
- [006 Consumer Runtime Proof](006-consumer-runtime-proof-and-candidate-replacement.md)
  — completed; card 015 complete, card 016 superseded
- [007 Provider-Wide Facade Contract And Foundation](007-provider-wide-facade-contract-and-foundation.md)
  — completed
- [008 Representative Cross-Shape Facades](008-representative-cross-shape-facades.md)
  — completed
- [009 Remaining Harness Facades](009-remaining-harness-facades.md)
  — completed
- [010 Hosted Direct And Provider-State Facades](010-hosted-direct-and-provider-state-facades.md)
  — completed
- [011 Specialized Runtime Facades](011-specialized-runtime-facades.md)
  — completed
- [012 Provider-Wide Acceptance And Candidate Return](012-provider-wide-acceptance-and-candidate-return.md)
  — completed
- [013 Canonical Source Provenance And Final Candidate](013-canonical-source-provenance-and-final-candidate.md)
  — completed
- [014 Consumer-Scale Application Proof And Hardening](014-consumer-scale-application-proof-and-hardening.md)
  — active; cards 040-041 and 045 complete, card 042 active under an exact
  launch-target reset, cards 043-044 planned

## Held Backlog

The [Grok Build maintained ACP range](../backlog/grok-build-maintained-acp-range.md)
is deferred behind its exact authentication evidence gate. It is not part of
the active g02 queue.

## Batch Shape

- card 001 completed the package, public-API, consumer, and official Cargo
  evidence inventory
- card 002 promoted the release architecture and Contract 036
- card 003 completed deterministic package and compatibility gates
- card 004 completed the first non-published release candidate and exact
  consumer upgrade handoffs
- card 005 promoted Contract 037, updated architecture, archived Spec 005, and
  superseded the compile-only candidate without deleting evidence
- cards 006-007 completed provider-neutral request agreement, diagnostics,
  access provenance, joined local tasks, service composition, and exact target
  approval
- card 008 completed the Codex prepared discovery and exact-target factory
- card 009 completed the separate Codex prepared operation profiles
- card 010 completed Codex facade conformance and guidance
- cards 011-012 completed Nucleus prepared-facade adoption
- cards 013-014 completed Soundcheck prepared-facade adoption and normal
  consumer validation
- card 015 completed packaged cross-consumer runtime proof
- card 016 is superseded without execution; card 036 owns later candidate
  replacement
- card 017 promoted the provider-wide facade and bound-operation boundary
- cards 018-019 completed shared foundations and Codex reference bound
  operations
- card 020 completed the representative Kimi persistent-ACP facade
- card 021 completed the representative Anthropic hosted-direct facade
- card 022 completed the representative Ollama attached-runtime facade
- card 023 completed the cross-shape review and authoring guide
- card 024 completed Claude Agent and Gemini CLI ACP prepared facades
- card 025 completed Pi RPC and Qwen headless prepared facades
- card 026 completed the OpenCode attached harness facade and remote-ACP
  composition review
- card 027 completed Kimi Platform and DeepSeek direct-inference facades
- card 028 completed Alibaba provider-owned conversation preparation
- card 029 completed OpenAI background-run preparation
- card 030 completed the Anthropic Managed Agent lifecycle facade and closed
  g02.010
- card 031 completed xAI, OpenAI, and Gemini realtime preparation
- card 032 completed separate Bedrock Runtime and catalogue SDK preparation
- card 033 completed separate llama.cpp attached inference and owned ephemeral
  serving preparation and closed g02.011
- card 034 completed the exact route matrix, public example compilation, and
  provider-wide integration guidance
- card 035 completed extracted-package execution for all 22 prepared routes
  and retained the packaged Nucleus and Soundcheck proofs
- card 036 replaced the held candidate, refreshed release and consumer
  handoffs, retained superseded evidence, and returned one exact unpublished
  candidate to the operator
- cards 037-039 replaced parentless provenance with one normal-history source
  commit, reproduced the 23-package candidate, passed all packaged facade and
  consumer proofs, and produced the canonical hardening baseline now held by
  g02.014
- card 040 completed the read-only consumer-scale envelope and current-state
  audit
- card 041 completed the exact native pilot after reducing 2 pre-provider
  failures to deterministic Swallowtail regressions; all 12 planned outcomes
  passed at the 15-attempt and 6-session ceiling
- cards 041-043 own Nucleus pilot, Nucleus sustained hardening, and Soundcheck
  secondary proof under consumer authority
- card 044 owns candidate refresh and release-readiness reassessment
- cards 024-026 own remaining harness facades
- cards 027-030 own hosted direct and provider-state facades
- cards 031-033 own realtime, SDK, and llama.cpp facades
- cards 034-036 own route guidance, packaged proof, and candidate return
- cards 011-014 own separately authorized Nucleus and Soundcheck migrations
- no further consumer repository edits are in the facade implementation lane

## Generation Boundary

g02 is a long-lived generation with 14 roadmaps. It remains well below its
30-50-roadmap rollover range. Provider-wide facade work stabilizes the existing
production routes; it does not add provider names or reopen their qualified
protocol evidence.
