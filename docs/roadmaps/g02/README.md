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
| Prove the candidate through sustained normal-path consumer workloads and harden defects. | completed | Contract 036, consumer runtime evidence | `g02.014` |
| Separate consumer thread lifecycle from bound provider-session management. | completed | Research 036, Contract 038 | `g02.015` |
| Prove Codex archive, restore, and deletion across its maintained range. | completed | Contracts 029, 038 | `g02.016` |
| Refresh ACP close/delete and prove Claude Agent lifecycle. | completed | Contracts 015, 029, 038 | `g02.017` |
| Prove attached OpenCode session deletion across its maintained range. | completed | Contracts 014, 029, 038 | `g02.018` |
| Complete provider-wide lifecycle acceptance and a Nucleus handoff. | completed | Contracts 036-038 | `g02.019` |
| Add a separate Kimi local-server lifecycle and interactive route. | completed | Research 040-041, Contracts 029 and 038 | `g02.020` |
| Expand source-scoped model catalogue coverage without hidden session creation. | completed | Research 042-043, Contract 020 | `g02.021` |
| Close useful structured-run gaps without flattening provider lifecycle. | completed | Research 044-046, Contract 039 | `g02.022`-`g02.024` |
| Audit and close remaining provider feature-matrix `No` values in evidence-ranked families. | completed | Research 003 and 042-046, Contracts 014-043 | `g02.025`-`g02.034` |
| Close the first session-continuity tranche without flattening replay, reattachment, or native close. | completed | Research 051-052, Contracts 009, 017, 038 | `g02.028` |
| Revalidate the remaining Pi RPC load and resume cells. | paused | Research 053, Contracts 009, 017, 038 | `g02.029` |
| Audit and close provider-retention matrix gaps. | completed | Research 036 and 054-055, Contracts 021 and 038-039 | `g02.030` |
| Audit and close retained-execution and recovery matrix gaps. | completed | Contracts 009, 021-022, 042 | `g02.031` |
| Audit and close working-resource and bounded-write matrix gaps. | completed | Contracts 009-010, 013, 015, 017, 037 | `g02.032` |
| Audit and close owned-runtime lifecycle and planned-rollover matrix gaps. | completed | Research 060, Contracts 004, 009, 018, 026-027, 031 | `g02.033` |
| Classify the residual feature-matrix inventory and close or re-scope the programme. | completed | Contracts 004, 009, 011, 016, 026, 029, 037, 043 | `g02.034` |
| Expose provider-visible agent activity through exact route profiles and prepared facades. | completed | Research 063, Contract 044 | `g02.035`-`g02.040` |
| Qualify current Kimi ACP and headless ranges with live account evidence. | completed | Research 068, Contracts 011, 036-037, 044 | `g02.041` |
| Qualify the Kimi `0.31.0` local-server status milestone and installed route. | completed | Research 069, Contracts 029, 032, 037-038, 042, 044 | `g02.042` |
| Add one maintained Grok Build ACP route with exact delegated authentication. | completed | Research 070, Contracts 014-015, 023, 029, 032-034, 037, 039, 044 | `g02.043` |
| Maintain current Codex and OpenCode installed support windows as one batched tranche. | completed | Research 071, Contracts 029, 032, 036-040, 044 | `g02.044` |
| Remove error-level structural debt without changing public or provider behavior. | completed | Contract 001, AGENTS module rules, Effigy structural scan | `g02.045` |
| Reduce normal development validation latency without weakening milestone or release proof. | completed | Contract 001, Effigy task graph, operator latency feedback | `g02.046` |

## Current Checkpoint

- g01 closed at 49 roadmaps after 48 completed milestones and one backlog move
- 24 workspace crates share version `0.1.0`
- no release has been published
- Research 036 separates universal consumer-local thread lifecycle from
  optional bound provider-session management
- Contract 038 fixes inactive binding authority, independent archive, restore,
  delete and native-close capabilities, exact deletion strength, version
  posture, effect uncertainty, and consumer-owned destructive policy
- roadmap g02.015 is complete; cards 046-048 realize records, runtime role,
  and public cross-host conformance
- Research 037 and cards 049-051 freeze five Codex lifecycle revisions across
  the unchanged app-server range and complete the first production
  provider-session effect mapping
- Research 038 and card 052 pin stable ACP schema `v1.20.0`, preserve portable
  history-only deletion truth, and qualify Claude harness-data deletion across
  every supported lifecycle milestone
- card 053 completes Claude Agent stdio native close, opaque binding, explicit
  prepared deletion, exact provider-data truth, and unverified-newer gating
- card 054 completes the production failure matrix, credential-last cleanup,
  and explicit remote-ACP lifecycle portability without a remote Claude claim
- Research 039 and card 055 freeze two delete-schema revisions, eight exact
  published segments, two runtime evidence revisions, provider-data deletion
  with provider-defined descendants, missing-target rejection, and
  unverified-newer posture across all 45 qualified OpenCode releases
- card 056 completes the bound OpenCode deletion role, exact prepared binding,
  provider-data mapping, health drift gate, and joined uncertainty boundary
- card 057 completes every exact deletion segment, local and
  remote-authoritative execution, explicit unverified-newer acceptance,
  destructive failure boundaries, joined cleanup, and full adapter regression
- card 058 publishes and machine-checks the exact 22-route lifecycle matrix:
  three supported routes, two explicitly unsupported ACP routes, and seventeen
  not-applicable operation shapes
- card 059's transient candidate passed reproducibility, 20 provider-facade
  suites, 33 focused lifecycle tests, both consumer proofs, and 105 packaged
  Codex tests; card 136 later supplied broader packaged lifecycle evidence,
  so retained-candidate refresh is superseded
- Research 040 corrects the Kimi Web authority assessment: exact `0.28.1` and
  `0.29.0` expose a documented foreground REST/WebSocket server with OpenAPI,
  AsyncAPI, bearer auth, exact metadata, archive, and restore
- Contract 038 now keeps the planned `kimi-code.local-server` route distinct
  from ACP, requires explicit cross-transport binding import, and qualifies no
  Kimi deletion
- roadmap g02.020 and cards 061-065 sequence the exact corpus, lifecycle
  driver, binding import, interactive driver, and package closeout
- card 061 freezes separate non-production local-server identity, exact
  `0.28.1` and `0.29.0` compatibility, bounded REST/WebSocket v2 decoders,
  error classification, metadata corroboration, no-delete proof, and exact
  fixture provenance without live effects
- card 062 realizes attached and owned-foreground authenticated preparation,
  native archive/restore, before/after-effect truth, and joined owned-child
  cleanup without a container or containment claim
- card 063 realizes explicit ACP import authority, exact local-server target
  agreement, authenticated read-only target lookup, and a new archive/restore
  binding without widening ACP or accepting raw provider identity
- card 064 realizes the separate prepared local-server interactive route,
  exact REST prompt and WebSocket v2 lifecycle, explicit approval and question
  callbacks, cursor and resynchronization truth, task-before-effect admission,
  and joined attached and owned cleanup
- card 065 publishes the 23rd production route, exact Kimi route selection,
  four compile-tested examples, a bounded Nucleus handoff, and 21 Kimi
  local-server tests executed from extracted package artifacts
- Research 042 and card 066 reclassify all thirteen catalogue `No` values:
  six have ready upstream catalogue interfaces, two are session-negotiated,
  two are not applicable, and three require separate interface evidence
- card 067 adds Pi's route-free, tool-free `get_available_models` operation,
  typed prepared catalogue facade, bounded source-scoped metadata, deadline,
  failure, topology, and credential-last joined cleanup evidence without
  selecting or invoking a model
- card 068 adds exact Kimi local-server catalogue coverage and bounded
  negotiated model options on already-authorized Gemini and Kimi ACP sessions
- card 069 adds separate hosted OpenAI, Gemini, and xAI catalogue branches
  plus Alibaba's deployment-candidate control-plane branch
- Research 043 and card 070 correct the Qwen and Alibaba evidence gaps, add
  Qwen safe-mode control discovery, and close the 21-row solution matrix at
  16 `Yes`, two session-negotiated, two not-applicable, one caller-supplied,
  and zero `No`
- roadmap g02.021 is complete; every selected machine-readable model source
  has a Swallowtail path without hidden session creation or route inference
- Research 044 reclassifies twelve structured-run gaps: eight definite
  branches, one retained Kimi local-server branch, one not-applicable owned
  serving route, and two realtime-media exclusions
- Contract 039 permits independently qualified one-turn projection over exact
  direct, connection, ACP, RPC, or attached-server lifecycles; retention,
  callbacks, cancellation, terminal outcome, and cleanup remain exact
- the operator accepts durable Kimi local-server thread retention without any
  deletion claim
- maintained Kimi Code `0.29.2` is newer than the current `0.29.0` guaranteed
  ceiling and must be qualified before structured support extends the range
- card 072 adds independent Alibaba and DeepSeek structured roles, prepared
  operations, one-request fixtures, cancellation, topology, and matrix truth
- card 073 adds an independent xAI structured role and prepared operation,
  one `store=false` WebSocket response without continuation, exact usage and
  billed-cost evidence, cancellation, joined cleanup, and both host topologies
- card 074 adds the reusable ACP single-turn assertion pack and an independent
  Claude Agent structured role with one operation-private session, explicit
  durable transcript retention, qualified native close without deletion,
  permission-stop truth, and full lifecycle and remote-ACP regression
- card 075 adds independent Pi RPC and OpenCode HTTP structured roles and
  prepared operations: Pi executes one callback-capable `--no-session` prompt
  with prohibited retention, while OpenCode creates one temporary private
  session and confirms its deletion after exact SSE terminal evidence without
  claiming attached-server lifecycle authority
- the shared structured-harness boundary pack now covers prohibited,
  temporary-with-deletion, and durable-without-deletion retention truth; the
  solution matrix reported 15 structured `Yes` and six `No`
- Research 045 and card 076 qualify Gemini CLI headless
  `0.51.0..=0.52.0`, add a separate stream-JSON structured driver and typed
  prepared run, and preserve ACP as an independent interactive transport
- Gemini CLI now has one public facade with explicit `Acp` or `Headless`
  selection; sandbox remains optional, while durable local transcript
  retention, native exits, cancellation, deadlines, and joined cleanup remain
  visible
- the 24-route, 21-solution matrix now reports 16 structured `Yes` and five
  `No`; g02.023 and cards 074-076 are complete
- Research 046 and card 077 qualify Kimi Code `0.29.1` and `0.29.2` through
  exact tagged source, separate ACP and local-server behavior milestones,
  bounded headless and server corpora, global-event tolerance, and 64
  deterministic adapter tests
- Kimi releases above `0.29.2` remain visible unverified-newer rather than
  denied
- card 078 adds separate Kimi headless and retained local-server structured
  roles, explicit ACP/headless selection through one installed facade, durable
  provider-state truth, exact callbacks and cancellation, and joined attached
  plus owned cleanup
- card 079 closes the 25-route, 21-solution matrix at 18 structured `Yes`, two
  realtime `No`, and one owned-serving `Not applicable`; every `Yes` maps to a
  realized public prepared facade
- all 23 local package archives assemble, the extracted workspace compiles,
  and the closure-tranche structured suites execute without live credentials
- roadmaps g02.022-g02.024 and cards 071-079 are complete; the sole next task
  was the operator-held card 060 lifecycle-adoption decision
- card 060 now publishes a bounded Nucleus lifecycle handoff: local lifecycle
  stays universal, provider actions stay optional and capability-gated, and
  partial outcomes remain separate
- provider management bindings currently have no stable persistence codec;
  initial consumer adoption is same-process only and durable export/import is
  deferred
- card 080 reduces the exact CSV-aware inventory from 458 to 432 `No` cells by
  correcting 26 serving-only llama.cpp cells to `Not applicable`
- Research 047 classifies every current `No`; usage evidence is first, with
  Claude Agent ACP, Pi RPC, and OpenCode ready and both Kimi surfaces retained
  as honest upstream absence
- card 081 freezes Claude, Pi, and OpenCode usage corpora, proves the full
  existing version windows, and promotes cumulative replacement plus disjoint
  aggregation
- card 082 implements the three prepared usage paths, including optional
  reasoning tokens, disjoint aggregation, malformed and duplicate rejection,
  and exact prepared capability claims
- card 083 proves all 23 packages from the dirty source snapshot and closes
  the workspace at 935 passing tests with four intentional skips
- the later Claude Code headless solution changes the live inventory; the
  solution-level matrix then had 451 `No`, 29 `Not applicable`, and only two
  usage `No` cells, both Kimi
- roadmap g02.026 and cards 084-087 close the 48 starting
  generation-control gaps through one representative tranche
- Research 049 and card 084 classify all 48 gaps: 25 are plausible
  conversions, 20 are exact upstream absences, and three managed-agent cells
  belong to the agent-version operation shape
- card 085 selects seven conversions across OpenAI, Ollama, and OpenCode;
  xAI remains operator-held and the broader matrix runway remains explicit
- Contract 040 and card 085 freeze exact application and enforcement truth
  plus offline request corpora for all seven selected controls
- cards 086-087 realize those seven controls, close the 48-cell starting
  inventory at 41 retained generation-control `No` cells, and leave the whole
  audited matrix at 444 `No` and 29 `Not applicable`
- Research 050 and card 088 classify all 74 attachment, consumer-tool,
  approval-or-question, and external-search gaps: 45 are actionable, three
  xAI cells remain held, 25 are exact route absences, and one is realtime
  media rather than an attachment
- Contract 041 and card 089 freeze finite input, native versus provider-owned
  tools, provider-request strength, and external-search authority across Pi,
  OpenCode, and Anthropic Messages
- the OpenCode corpus covers all 45 qualified releases through four exact
  input/callback surface revisions; nine focused corpus tests pass offline
- card 090 is complete: Pi RPC realizes bounded PNG input; OpenCode realizes
  bounded PNG input plus exact one-shot permission and ordered-question
  exchange across prepared structured and opt-in interactive operations
- Anthropic Messages realizes one bounded PNG input, explicit provider-owned
  web search, and a separate resource-free consumer-tool continuation session
- card 091 closes the 74-cell input/callback inventory with six realized
  conversions and 68 retained `No` cells; the complete matrix has 437 `No`
  and 29 `Not applicable` cells
- all 970 workspace tests pass with four separately gated installed-provider
  probes skipped; 23 APIs and package archives pass from the dirty snapshot
- Research 051 and card 092 classify all 58 session-continuity gaps: seven
  fit existing contracts, four require retained hosted-session contract
  expansion, one is upstream-ordering-blocked, ten are exact route absences,
  and 36 do not fit the reusable-session operation shape
- all 20 native-close cells remain honest `No`; process exit, disconnect,
  abort, archive, and deletion stay distinct
- Research 052 and card 093 confirm no shared contract gap and freeze six
  Codex continuity segments, all ten qualified Claude Agent ACP releases, and
  seven OpenCode wire surfaces across twelve published segments
- card 094 implements Codex app-server load, Claude Agent ACP load and resume,
  and OpenCode HTTP load and resume through public prepared operations
- bounded ordered replay, replay-free resume, exact binding, all guaranteed
  version segments, safe failure, and joined cleanup pass across the three
  adapters and the full workspace; no retention or lifecycle claim widens
- the session-continuity checkpoint converted five cells and left 432 matrix
  `No` cells; session continuity fell from 58 to 53 `No` cells
- card 095 closes the 58-cell starting inventory at five conversions and 53
  retained `No` cells: 17 load, 16 resume, and 20 native close
- all 23 dirty-snapshot packages assemble; the extracted workspace compiles
  and packaged Codex, Claude Agent, and OpenCode continuity facades pass
- Research 053 and card 096 revalidate Pi `0.80.10` through current `0.82.1`
  and correct the two remaining ready classifications: public session
  attachment accepts the cwd stored in provider state and cannot preserve the
  host-leased working resource
- roadmap g02.029 and cards 097-098 are paused without changing the existing
  ephemeral Pi profile; a maintained cwd-bound attachment interface is the
  exact unpause condition
- roadmap g02.030 closes the 75 provider-retention cells at 58
  `Not applicable`, five `Yes`, and twelve retained `No`
- roadmap g02.031 closes the 59 retained-execution and recovery cells at 32
  `Not applicable`, 24 retained `No`, two `Yes`, and one `Partial`
- the retained-execution checkpoint left 334 `No` and 119 `Not applicable`
  cells before the resource/write tranche
- Research 058 and card 107 classify all 31 working-resource and bounded-write
  cells: 24 are non-applicable, six are exact selected-surface absences, and
  one Gemini ACP profile is selected
- Research 059 and card 108 freeze exact Gemini CLI `0.51.0` read-write ACP
  negotiation, callback, failure, and cleanup evidence without a sandbox or
  containment claim
- card 109 realizes the explicit prepared bounded-write profile, keeps the
  existing read-only profile unchanged, and passes focused ACP and prepared
  facade conformance
- card 110 closes the resource/write family at 24 `Not applicable`, one
  `Yes`, and six `No`; the full matrix now has 309 `No` and 143
  `Not applicable` cells
- Research 060 and cards 111/114 close the 40-cell runtime-ownership and
  planned-rollover family at 39 `Not applicable` and one retained OpenAI
  Realtime `No`; cards 112-113 are superseded because no implementation
  candidate exists
- Research 061 and card 115 classify the final 61 unaudited cells: 34 become
  `Not applicable`, nine require contract or corpus work, three require
  separate realtime routes, and fifteen retain exact billed-cost `No` values
- the full matrix now has 236 `No` and 216 `Not applicable` cells
- Contract 043, Research 062, and card 116 separate Qwen restarted-harness
  continuation from Ollama transactional transcript replay and freeze both
  exact offline corpora
- cards 117-118 realize the selected Qwen and Ollama interactive profiles,
  close the final 61-cell inventory, and stop the feature-matrix programme
- Research 063 and Contract 044 promote the next operator-selected lane:
  operation-local observable activity identity, lifecycle, disclosure,
  content streams, route fidelity, and consumer projection ownership
- roadmaps g02.035-g02.040 and cards 119-137 sequence the common kernel,
  Codex, ACP, remaining harnesses, direct-inference truth, provider-wide
  package evidence, and a consumer handoff
- card 119 adds bounded and redacted activity identity, lifecycle, content,
  disclosure, correlation, and one semantic variant on the existing ordered
  runtime stream
- the ordered buffer rejects activity identity drift, lifecycle regression,
  repeated completion, and post-completion observations without exposing
  activity data in diagnostics
- card 120 adds separate observable-activity capability truth, exact route
  profiles, prepared transport and interface-behavior evidence, thinner
  requirement matching, and failure-before-effects non-promotion
- unverified-newer routes inherit the last qualified behavior revision and
  cannot widen activity fidelity from newly observed fields
- card 121 completes reusable existing-stream activity conformance, including
  exact lifecycle, assistant, reasoning-summary, unknown, callback,
  direct-tool, bounds, redaction, and unverified-newer assertions
- roadmap g02.035 is closed without granting any provider adapter a positive
  activity profile
- Research 064 and card 122 freeze exact Codex app-server and exec activity
  segments through qualified `0.145.0`, retain stable `0.146.0` as permitted
  unverified newer, and separate readable reasoning summaries from excluded
  raw reasoning
- the app-server corpus records core `0.80.0` lifecycle plus exact later
  message-phase, plan, dynamic-tool, request-resolution, hook, patch,
  timestamp, and subagent milestones; exec retains its distinct per-kind
  lifecycle truth
- card 123 maps qualified app-server activity into stable operation-local
  identity, native lifecycle, bounded content, exact callback and request
  correlation, safe unknowns, and immutable prepared route profiles
- card 124 maps exec's distinct completion-only, start/completion, and
  replacement-update item truth without fabricating app-server lifecycle
- prepared exec retains final output and usage semantics while exposing exact
  assistant, reasoning-summary, command, file, MCP, search, task, warning,
  collaboration, and namespaced unknown activity
- roadmap g02.036 is complete; both Codex transports preserve stable
  `0.146.0` as permitted unverified newer on the `0.145.0` guarantee
- Research 065 and card 125 pin ACP stable schema `v1.20.0`, separate
  protocol and transport SDK axes, and exact Claude Agent, Gemini CLI, and
  Kimi Code activity corpora
- Claude Agent `0.62.0..=0.63.0`, Gemini CLI `0.53.0`, and Kimi Code `0.30.0`
  remain permitted unverified newer on their last qualified guarantees
- card 126 adds bounded typed shared ACP session-update decoding without raw
  JSON, provider identity, runtime policy, or transport flattening
- card 127 maps exact Claude Agent, Gemini CLI, and Kimi Code activity into
  stable operation-local identity, typed lifecycle, bounded provider display,
  plan replacements, provider-owned tools, Gemini warning classification,
  and namespaced unknowns
- prepared ACP profiles bind the exact qualified behavior revision; permitted
  newer harnesses inherit the last guarantee without widening fidelity
- roadmap g02.037 is complete
- Research 066 and card 128 account for all eight remaining production
  non-ACP harness routes and freeze native-lifecycle, partial, completion-only,
  unknown, malformed, and failure corpora
- route profiles now bind selected partial or preview options; executable
  capability alone cannot widen activity fidelity
- OpenCode retains an exact thin `1.14.51` segment, Kimi retains separate
  `0.28.1` and `0.29.x` event-schema evidence, and current newer releases
  remain permitted but unverified
- card 129 maps Pi RPC, Kimi local server, OpenCode HTTP/SSE, and Managed
  Agents through exact prepared profiles without changing callbacks,
  recovery, retention, reattachment, server ownership, or cleanup
- card 130 maps the four headless routes without manufacturing lifecycle from
  terminal prose
- card 131 machine-checks 13 production harness routes and 18 ordinary
  prepared-operation profiles, with no unexplained unavailable run or session
- roadmap g02.038 is complete
- Research 067 and card 132 classify all 13 non-harness production routes,
  14 positive text-operation profiles, and 13 not-applicable catalogue,
  inventory, realtime, and serving operations
- card 133 projects all 14 selected text-operation profiles through exact
  prepared evidence and ordered activity events
- Kimi client-visible thought updates are exposed while DeepSeek and xAI
  private continuation remains excluded
- card 134 machine-checks all 13 non-applicable prepared operations, keeps
  OpenAI Realtime and Gemini Live on their dedicated media lifecycles, and
  retains attached-runtime and owned-serving boundaries
- card 135 publishes one 55-row activity inventory with 32 available and 23
  not-applicable route operations, exact prepared and conformance links, and
  consumer-owned safe projection guidance
- route QA checks all 26 production routes plus four auxiliary catalogue
  identities without flattening lifecycle, disclosure, or version-dependent
  profiles
- card 136 proves all 23 extracted archives, 35 packaged facade suites, all 26
  production routes, ten representative activity profiles, and isolated
  Nucleus and Soundcheck compatibility without release mutation
- packaged lifecycle evidence now includes Gemini headless management: 14
  suites, five management adapters, five supported routes, three unsupported,
  and eighteen not applicable
- card 137 publishes separate bounded Nucleus and Soundcheck activity
  handoffs plus two compile-checked public runtime projection examples
- Nucleus retains separate durable message and work projections, grouping,
  collapse, thread lifecycle, authorization, retention, analytics, and UI
- Soundcheck may ignore all activity without losing final structured output
- roadmaps g02.039 and g02.040 are complete; consumer repository adoption now
  requires an operator decision and no numbered card is ready
- roadmaps g02.016-g02.019 sequence Codex, ACP/Claude Agent, OpenCode,
  provider-wide acceptance, and the Nucleus handoff
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
  prepared routes with exact region, SDK, service, access, and cloud-client
  evidence, then adds one provider-level facade over those typed branches
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
- registry publication is outside the active roadmap; no registry, tag, push,
  release, or workflow mutation occurred
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
- any future publication requires a newly authorized roadmap after months of
  consumer usage evidence, including fresh registry identity, staged upload,
  tag, and release decisions
- roadmap g02.013 and cards 037-039 complete the accepted provenance repair,
  canonical local commit, candidate replacement, and packaged acceptance
- publication is closed as active work; the retained snapshot remains local
  evidence, not a pending release
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
  reached normal Nucleus state and remain preserved
- card 042's exact bundle-path reset then completed 50 valid serial read-only
  turns across 5 launches and 10 app-server lifecycles: 35 ordinary,
  10 callback, 3 cancelled, and 2 timed out, with no failure, fallback,
  fixture drift, or leaked process
- the complete reset stopped at its accepted 60-turn, 7-launch, and 12-session
  ceiling; this remains the accepted sustained application proof
- card 043's read-only gate audit found active Soundcheck work in the exact
  assistant path plus missing deterministic data-seed, bounded deadline, and
  safe attempt-ledger support
- Soundcheck product baseline `7c135da` and proof-support source `3566419`
  passed offline, but the first card 092 native launch invalidated the
  incomplete tuple before provider execution
- local `soundcheck-library` and Poodle path sources were dirty and omitted;
  cached app schema v48 rejected the fresh schema-v50 fixture
- Soundcheck card 091 was reopened while M11 remained paused at card 087
- Soundcheck runner repair `49dfc7e` and clean local sources closed the tuple,
  runner, stale-cache, and native schema-v50 readiness gates
- deterministic seed, four screenshots, proof-only deadline, seven-field
  attempt evidence, guarded teardown, health, QA, 24 frontend tests, and
  176 Rust tests pass without provider effects
- the approved card 092 retry then stopped at zero provider attempts when
  normal startup imported host plug-ins into the isolated fixture
- Soundcheck `282fa21` now suppresses all three host-ingestion paths only in
  the validated proof profile; a rebuilt native bundle retains exactly 16
  fixtures, zero scan runs, zero helpers, and zero attempt evidence
- the operator declined further repetition of Soundcheck's fixed agent-review
  workflow; card 043 and Soundcheck cards 092-093 are superseded without live
  scale acceptance claims
- Contracts 011 and 036 now put repeated lifecycle claims in adapter or
  consumer backend scenario harnesses; native apps retain thin authenticated
  vertical smoke
- card 044 completed local candidate refresh without repeating UI or provider
  workloads or authorizing publication
- Soundcheck primary research always carries bounded external-search
  authority, so the corrected 20-attempt envelope authorizes search for all
  16 primary attempts rather than claiming only 2 search-enabled workflows
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
  — completed; cards 040-042, 044, and 045 complete, card 043 superseded by
  operator decision
- [015 Provider Session Management Foundation](015-provider-session-management-foundation.md)
  — completed
- [016 Codex Thread Lifecycle Proof](016-codex-thread-lifecycle-proof.md)
  — completed
- [017 ACP Lifecycle And Claude Agent Proof](017-acp-lifecycle-and-claude-agent-proof.md)
  — completed
- [018 OpenCode Session Deletion Proof](018-opencode-session-deletion-proof.md)
  — completed
- [019 Provider Session Lifecycle Acceptance And Handoff](019-provider-session-lifecycle-acceptance-and-handoff.md)
  — completed; cards 058 and 060 complete, card 059 superseded by card 136's
  broader packaged lifecycle evidence
- [020 Kimi Code Local Server Route](020-kimi-code-local-server-route.md)
  — completed; cards 061-065 complete
- [021 Model Catalogue Coverage](021-model-catalogue-coverage.md)
  — completed; cards 066-070 complete
- [022 Structured Run Projection And Direct Coverage](022-structured-run-projection-and-direct-coverage.md)
  — completed; cards 071-073 complete
- [023 Installed And Attached Harness Structured Coverage](023-installed-and-attached-harness-structured-coverage.md)
  — completed; cards 074-076 complete
- [024 Kimi Structured Coverage And Matrix Closeout](024-kimi-structured-coverage-and-matrix-closeout.md)
  — completed; cards 077-079 complete
- [025 Provider Feature Matrix No-Closure Programme](025-provider-feature-matrix-no-closure-programme.md)
  — completed; cards 080-083 complete
- [026 Generation-Control Feature Closure](026-generation-control-feature-closure.md)
  — completed; cards 084-087 complete
- [027 Input And Callback Feature Closure](027-input-and-callback-feature-closure.md)
  — completed; cards 088-091 complete
- [028 Session Continuity Feature Closure](028-session-continuity-feature-closure.md)
  — completed; cards 092-095 complete
- [029 Pi RPC Session Continuity](029-pi-rpc-session-continuity.md)
  — paused; card 096 complete, cards 097-098 paused behind the public Pi
  resource-binding gate
- [030 Provider Retention Feature Closure](030-provider-retention-feature-closure.md)
  — completed; cards 099-102 complete
- [031 Retained Execution And Recovery Feature Closure](031-retained-execution-and-recovery-feature-closure.md)
  — completed; cards 103-106 complete
- [032 Working Resource And Workspace Authority Feature Closure](032-working-resource-and-workspace-authority-feature-closure.md)
  — completed; cards 107-110 complete
- [033 Runtime Ownership And Connection Rollover Feature Closure](033-runtime-ownership-and-connection-rollover-feature-closure.md)
  — completed; cards 111 and 114 complete, cards 112-113 superseded by the
  negative tranche
- [034 Residual Feature Matrix Truth And Programme Checkpoint](034-residual-feature-matrix-truth-and-programme-checkpoint.md)
  — completed; cards 115-118 complete
- [035 Observable Agent Activity Kernel](035-observable-agent-activity-kernel.md)
  — completed; cards 119-121 complete
- [036 Codex Observable Activity Fidelity](036-codex-observable-activity-fidelity.md)
  — completed; cards 122-124 complete
- [037 ACP Observable Agent Activity](037-acp-observable-agent-activity.md)
  — completed; cards 125-127 complete
- [038 Non-ACP Harness Activity Coverage](038-non-acp-harness-activity-coverage.md)
  — completed; cards 128-131 complete
- [039 Direct Inference Activity Truth](039-direct-inference-activity-truth.md)
  — completed; cards 132-134 complete
- [040 Provider-Wide Activity Acceptance And Consumer Handoff](040-provider-wide-activity-acceptance-and-consumer-handoff.md)
  — completed; cards 135-137 complete
- [041 Kimi Code 0.31 Range And Live Proof](041-kimi-code-0-31-range-and-live-proof.md)
  — completed; cards 138-139 complete
- [042 Kimi Code 0.31 Local-Server Guarantee](042-kimi-code-0-31-local-server-guarantee.md)
  — completed; cards 140-141 complete
- [043 Grok Build Maintained ACP Route](043-grok-build-maintained-acp-route.md)
  — completed; cards 142-145 complete
- [044 Installed Harness Range Maintenance](044-installed-harness-range-maintenance.md)
  — completed; cards 146-149 complete
- [045 Error-Level Structural Health Stabilization](045-error-level-structural-health-stabilization.md)
  — completed; cards 150-155 complete
- [046 Validation Latency And Proof Routing](046-validation-latency-and-proof-routing.md)
  — completed; cards 156-158 complete

## Held Backlog

The [Grok Build maintained ACP range](../backlog/grok-build-maintained-acp-range.md)
is completed through g02.043. Durable
[provider-session management binding persistence](../backlog/provider-session-management-binding-persistence.md)
is deferred until a consumer needs post-restart provider management.

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
- card 042 completed the exact sustained reset with 50 valid outcomes across
  5 launches and 10 app-server lifecycles; the combined reset stopped at
  60 provider turns, 7 launches, and 12 sessions with no failed or unexpected
  isolated outcome
- cards 041-042 own the accepted Nucleus pilot and sustained hardening;
  card 043 retains Soundcheck secondary-shape evidence but is superseded
- card 044 completed one reproducible post-hardening candidate; all 23
  packages, 22 routes, both isolated consumers, and 93 packaged Codex tests
  pass without live effects
- card 045 completed bounded Codex non-zero discovery exit diagnostics
- cards 046-048 completed provider-neutral session-management records, runtime
  role, public fixtures, and deterministic conformance
- cards 049-051 own Codex lifecycle range evidence, production mapping, and
  closeout
- card 049 qualifies archive from `0.80.0`, restore from `0.92.0`,
  notifications from `0.104.0`, best-effort descendant archive from `0.123.0`,
  and strict descendant hard delete from `0.140.0`
- cards 050-051 complete the Codex low-level and prepared mapping, exact
  response and notification truth, uncertainty, topology, and cleanup proof
- cards 052-054 own ACP v1 lifecycle currentness and Claude Agent production
  proof
- card 052 completes the shared ACP close/delete corpus and exact Claude
  lifecycle classification
- card 053 completes the stdio production mapping; card 054 completes the
  cross-transport and shared-conformance closeout
- card 055 completes OpenCode deletion range evidence; cards 056-057 own the
  production path and closeout
- cards 058-060 own the exact provider-wide lifecycle matrix, packaged
  acceptance, and Nucleus adoption handoff
- cards 061-065 own the separate Kimi local-server corpus, lifecycle,
  cross-transport binding import, interactive route, and acceptance
- card 061 completes the exact local-server compatibility and selected
  protocol corpus without registering a production role
- card 062 completes the separate lifecycle role, attached and owned
  preparation, archive/restore effect boundary, and topology cleanup proof
- card 063 completes exact ACP-to-local-server import authority, target
  lookup, mismatch closure, unverified-newer acceptance, and attached plus
  owned lifecycle conformance
- cards 064-065 complete Kimi local-server interactive execution and route
  acceptance
- cards 066-070 complete source-scoped model catalogue coverage
- cards 071-073 complete the structured-run contract plus Alibaba, DeepSeek,
  and xAI direct branches
- cards 074-076 complete Claude ACP, Pi RPC, OpenCode HTTP, and Gemini CLI
  headless structured branches
- card 077 qualifies Kimi Code `0.29.2` and freezes the headless corpus for
  card 078
- cards 078-079 complete Kimi structured coverage and provider-wide
  structured-run closeout
- card 060 completes the bounded Nucleus lifecycle handoff without consumer
  edits; card 059's retained-candidate tail is superseded by card 136's
  broader package proof and the publication-lane closure
- cards 080-083 complete the historical CSV-aware audit and first
  usage-evidence tranche; the post-Claude-Code matrix retained 451 `No` cells
- cards 084-085 complete the 48-cell generation-control audit and selected
  contract; cards 086-087 complete implementation and closeout
- cards 088-089 complete the 74-cell input/callback audit and selected
  contract/corpus gate; card 090 realizes the selected Pi, OpenCode, and
  Anthropic Messages cells; card 091 completes re-audit and packaged closeout
- cards 092-095 own currentness, contract/corpus, implementation, and closeout
  for the 58-cell session-continuity family; card 093 found no shared contract
  gap and completed the exact three-route corpus gate; card 094 realizes all
  five selected prepared operations; card 095 closes package proof and selects
  Pi RPC load and resume
- card 096 closes Pi RPC currentness at an exact public-interface gate:
  provider state can redirect cwd away from the host lease; cards 097-098 are
  paused
- card 099 classifies all 75 provider-retention `No` cells: 58 are
  non-applicable, twelve are exact absences, OpenCode is a realized false
  negative, and four runtime cells are selected
- card 100 promotes exact Gemini CLI transcript deletion, optional Claude
  session cleanup, and OpenAI background-response cleanup contracts and
  freezes their offline corpora
- card 101 realizes Gemini bound history removal and opt-in cleanup, Claude
  Agent opt-in cleanup, and OpenAI background-response cleanup
- card 102 converts 58 operation-shape cells to `Not applicable`, converts
  five realized cells to `Yes`, retains twelve exact `No` cells, and selects
  retained execution and recovery next
- card 103 classifies all 59 retained-execution and recovery `No` cells: 32
  are non-applicable, 22 are exact selected-surface absences, two require
  separate routes, and three require a shared contract
- card 104 promotes Contract 042 and the exact Kimi retry and cursor corpus;
  card 105 realizes explicit headless and local-server managed recovery plus
  maximum-one local-server active-turn reattachment
- card 106 closes retained execution, refreshes the intentional Kimi API
  baseline, and selects resource and write authority next
- cards 107-108 classify the 31-cell resource/write inventory and select the
  exact Gemini ACP `0.51.0` bounded text-write callback profile; existing
  contracts are sufficient and the offline corpus is frozen
- card 109 realizes the separate read/write prepared profile, exact ACP
  capability and mode agreement, host callback dispatch, negative
  unnegotiated-write evidence, and unchanged ambient authority
- card 110 completes package proof, converts 24 non-applicable cells and one
  Gemini capability, retains six exact absences, and selects runtime ownership
  plus planned rollover next
- card 111 classifies all 40 ownership and rollover cells; card 114 converts
  39 category errors to `Not applicable`, retains one exact OpenAI Realtime
  absence, and supersedes cards 112-113
- card 115 classifies the final 61 cells, corrects 34 category errors, retains
  27 exact gaps, and selects Qwen headless plus Ollama attached interactive
  sessions
- card 116 settles the two selected continuity ownership shapes and freezes
  exact Qwen `0.19.11` plus Ollama `0.14.0..=0.32.1` corpora
- card 117 realizes separate Qwen private harness-retained continuation and
  Ollama consumer-owned transcript replay, converts only those two
  interactive cells, and leaves public resume, media, and billed cost absent
- card 118 closes the starting 61 cells at two `Yes`, 25 `No`, and 34
  `Not applicable`; the complete 660-cell feature region now holds 202 `Yes`,
  234 classified `No`, 216 `Not applicable`, four `Partial`, two
  `Caller-supplied`, and two `Session-negotiated`
- all 23 extracted package archives, the complete workspace, docs, route,
  metadata, example, and public-API gates pass; the feature-matrix completion
  programme closed with no provider implementation selected at that
  checkpoint
- cards 119-121 complete the common observable-activity records, ordered
  lifecycle enforcement, exact prepared route profile, qualified
  behavior-revision binding, and reusable provider-neutral conformance
- cards 122-124 own Codex range evidence, app-server lifecycle fidelity, and
  completion-oriented exec mapping
- cards 125-126 complete ACP currentness, exact harness corpora, and shared
  bounded activity decoding; card 127 completes exact Claude Agent, Gemini
  CLI, and Kimi Code profiles, emission, and conformance
- card 128 completes the exact eight-route non-ACP harness inventory and
  corpus freeze; card 129 completes HTTP, server, WebSocket, RPC, and managed
  projection; card 130 completes headless projection; card 131 closes all 13
  harness routes through 18 public prepared-operation profiles
- cards 132-134 complete direct-inference applicability, exact text activity,
  and realtime or serving boundaries
- cards 135-137 own the provider-wide activity matrix, extracted-package
  proof, and bounded Nucleus and Soundcheck handoffs
- Research 068 and card 138 compare signed Kimi Code `0.30.0` and `0.31.0`,
  prove authenticated headless and ACP fixed-output turns on installed
  `0.31.0`, select ACP and headless range extension, and keep the changed
  local-server broadcaster unverified
- card 139 advances the ACP and headless claim identities, guarantees both
  installed routes through `0.31.0`, retains local-server `0.29.2`, closes
  Python Kimi, and leaves Grok plus provider-session binding persistence under
  their explicit gates
- Research 069 and cards 140-141 classify and sequence the separate
  local-server `0.31.0` broadcaster milestone
- cards 140-141 freeze the full subagent-status payload as non-rendered
  progress, advance the local-server claim through `0.31.0`, and prove
  installed bearer-protected metadata and catalogue access
- Research 070 and card 142 qualify exact Grok Build `0.2.114`, promote the
  one-shot `cached_token` activation boundary into Contract 015, archive Spec
  003, and freeze sanitized signed-artifact plus ACP evidence
- Grok Build `0.2.114` created one expected durable empty local session during
  the no-prompt live gate; its existing authentication file was unchanged
- cards 143-145 own Grok discovery, interactive ACP execution, structured
  projection, conformance, package acceptance, and closeout
- card 143 completes exact stable-channel discovery, source-revision
  enforcement, qualified versus unverified-newer dispatch, both authoritative
  host topologies, and prepared configured-instance promotion
- card 144 completes exact ACP process dispatch, one-shot cached-token
  activation, durable local session attachment, bounded turns, portable
  activity, provider-request stop, cancellation, and joined credential plus
  resource cleanup through low-level and prepared paths
- card 145 completes the separate structured projection, exact and
  unverified-newer cross-host conformance, 27-route and 23-solution public
  truth, 24-package gates, and independently compiled Grok archive
- Research 071 and card 146 compare installed and stable range posture, retain
  Grok, Kimi, and Claude unchanged, and select Codex `0.146.0` plus OpenCode
  `1.18.5..=1.18.10` as one contract-ready maintenance tranche
- card 147 qualifies Codex exec and app-server through exact `0.146.0` across
  execution, lifecycle, continuity, activity, discovery, and prepared
  evidence while retaining the old baselines, gaps, prerelease rejection, and
  later-stable unverified posture
- card 148 qualifies all 51 exact OpenCode releases through `1.18.10` across
  HTTP/SSE, lifecycle, deletion, continuity, callback, usage,
  generation-control, activity, discovery, and prepared evidence; `1.18.8`'s
  unrelated OAuth callback delta does not create a dispatch milestone
- card 149 closes both range extensions through focused cross-host proof,
  independently assembled and compiled adapter archives, shared activity
  conformance, package metadata, public-API, route, release, and front-door
  truth without provider or consumer effects
- card 150 freezes 33 error-level structural findings across 23 source files,
  nine test files, and one script; five critical files lead the bounded
  decomposition sequence
- card 151 removes all five critical findings through private source
  extraction; focused adapter tests, warnings-denied lint, and the unchanged
  57-operation route matrix pass, leaving 29 high errors and no critical file
- card 152 removes all five Codex and seven OpenCode high findings; 211 focused
  package tests, warnings-denied lint, and the unchanged 24-crate public-API
  baseline pass, leaving 17 high errors
- card 153 removes all ten runtime, Claude Agent, Gemini, and Kimi high
  findings; 168 focused tests, warnings-denied lint, and the unchanged
  public-API baseline pass, leaving seven high errors
- card 154 removes the final seven high findings across Pi, Alibaba,
  DeepSeek, xAI, and provider-route validation; 112 focused tests,
  warnings-denied lint, exact route matrices, and the unchanged public-API
  baseline pass, leaving doctor warning-only at 142 findings
- card 155 closes error-level stabilization through workspace, metadata,
  public-API, route, docs, doctor, and four affected extracted-package gates;
  all pass with zero errors and no external effect
- card 156 classifies every validation selector by evidence tier and owner,
  records historical runtime variance through 56.3 minutes, freezes explicit
  focused and affected-package budgets, and leaves all validation behavior
  unchanged
- card 157 adds exact-package focused and affected-archive selectors; Pi and
  xAI pass 64 focused tests plus clippy in four seconds and independently
  packaged shared-target compilation in five seconds
- card 158 accepts the focused paths: four independently assembled adapter
  archives compile through one extracted target in five seconds versus 22.4
  seconds through separate targets; concurrent Kimi activity work leaves one
  doctor error outside this lane
- cards 024-026 own remaining harness facades
- cards 027-030 own hosted direct and provider-state facades
- cards 031-033 own realtime, SDK, and llama.cpp facades
- cards 034-036 own route guidance, packaged proof, and candidate return
- cards 011-014 own separately authorized Nucleus and Soundcheck migrations
- no consumer repository edits are in the matrix implementation lane

## Next

Reassess the next g02 product or provider milestone after concurrent
subagent-topology work closes. Warning-only reduction and publication remain
deferred.

## Generation Boundary

g02 is a long-lived generation with 46 roadmaps. It remains within its
30-50-roadmap range. Provider-wide facade work stabilizes the existing
production routes. The provider-session lifecycle lane adds bounded management
to applicable persistent-session routes without adding provider names,
consumer persistence, or implicit deletion. The Kimi local-server route adds a
second driver under an existing integration family; it does not justify a
generation rollover. Observable activity enriches existing route events and
prepared evidence without adding provider identities, a global event bus,
consumer persistence, or product UI.
