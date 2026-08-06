# g03 Compatibility Maintenance And Consumer-Proven Hardening

Status: active
Owner: Tom
Created: 2026-07-31

## Purpose

Keep Swallowtail useful across real provider and harness release ranges while
hardening defects and integration friction proven by consuming applications.

g03 does not chase every upstream release. Each Swallowtail release carries
its qualified ranges. Versions above a qualified upper milestone remain
visible as unverified newer unless exact evidence requires rejection.

## Generation Runway

| Goal | State | Governing refs | First milestone |
| --- | --- | --- | --- |
| Establish a repeatable currentness inventory for installed harnesses and shared protocols. | completed | Contracts 011, 029, 036 | `g03.001` |
| Extend exact compatibility segments only where current evidence finds material drift or useful newer support. | active | Contracts 011, 029, 037 | `g03.002`, `g03.012`-`g03.015` |
| Add high-value installed harnesses through explicit catalogue, interactive, structured, and continuation roles. | completed | Contracts 005-006, 015, 020, 029, 037, 039, 043-045 | `g03.005` |
| Close useful installed-version gaps against exact current artifacts without forcing per-provider releases. | completed | Contracts 011, 029, 032, 037 | `g03.009` |
| Keep deterministic corpora and conformance aligned with behavior milestones rather than package semver alone. | completed | Contracts 011, 029, 036 | `g03.010` |
| Support host-approved interpreted installed harnesses without ambient environment inheritance. | completed | Contracts 010, 032; Research 084 | `g03.011` |
| Turn consumer-reproduced defects and integration friction into portable regression evidence. | active | Contracts 002, 037, 044-045 | `g03.003` |
| Reassess prepared-facade usability from multi-consumer proof without importing product policy. | completed | Contracts 002, 010, 037 | `g03.017` |
| Allow explicit discovery and import of harness-origin sessions without owning consumer persistence or synchronization. | completed | Contracts 017, 029, 037, 046 | `g03.019` |
| Expose configured provider instances for explicit consumer selection without central routing policy. | completed | Contracts 005-006, 008, 014, 020, 037, 047 | `g03.024` |
| Persist exact ordinary provider-session resume authority across consumer restarts without adding routing or synchronization. | completed | Contracts 017, 029, 037, 046 | `g03.025` |
| Prevent cross-operation activity projection collisions without manufacturing provider-global identity. | completed | Contracts 009, 044 | `g03.026` |
| Reconcile interrupted consumer turns from retained provider truth without retry or control authority. | completed | Contracts 017, 042, 046, 048 | `g03.027` |
| Preserve qualified provider work across controlled consumer shutdown without leaking local work. | completed | Contracts 009, 017, 042, 048-049 | `g03.028` |
| Persist and reconcile an exact Kimi operation before detaching its local observer. | completed | Contracts 017, 042, 048-049 | `g03.029` |
| Persist, reconcile, and detach an exact provider-owned structured run. | completed | Contracts 021, 042, 048-049 | `g03.030` |
| Qualify ACP retained history for honest restart reconciliation. | completed | Contracts 017, 042, 048 | `g03.031` |
| Qualify remaining durable transcript and retained-operation reconciliation candidates. | completed | Contracts 021-022, 042, 048 | `g03.032` |
| Reconcile and clean exact recovered Anthropic Managed Agent runs without conflating observation and control. | completed | Contracts 021-022, 038, 042, 048 | `g03.033` |
| Restore consumer working state through the strongest exact route method without flattening ACP continuation into reconciliation. | completed | Contracts 017, 037, 046, 048, 050 | `g03.034` |
| Extend ACP continuation recovery only where exact agent-specific load/replay evidence passes. | completed negatively | Contracts 013, 015, 017, 029, 037, 050 | `g03.035` |
| Compose read-only reconciliation with separately prepared settled-session attachment. | completed | Contracts 017, 037, 046, 048, 050 | `g03.036` |
| Promote further retained-session recovery behind exact resource and ownership gates. | completed | Contracts 009, 017, 025, 029, 037-038, 050 | `g03.037` |
| Give every prepared interactive harness route one truthful post-crash action. | completed | Contracts 017, 037, 042, 048, 050 | `g03.038` |
| Give every prepared reusable session shape one truthful post-crash action. | completed | Contracts 026-027, 030, 043, 050 | `g03.039` |
| Give every provider and harness route one portable failure interface without erasing exact diagnostics. | completed | Contracts 003-004, 006, 009, 014, 037, 044, 051 | `g03.041` |
| Give agents and operators deep traceable instructions for every production route and portable feature. | completed | Contracts 037, 044-052 | `g03.042` |
| Prepare the 27-package workspace for an initial `v0.1.0` GitHub source tag without registry publication. | completed | Contracts 009, 022, 029, 036-037, 052; Research 111 | `g03.043` |
| Ship compatible post-tag repairs as an exact CI-green `v0.1.1` source tag. | completed | Contracts 001, 009, 022, 036, 049, 052 | `g03.044` |
| Periodically reconcile deferred gates, route truth, and generation capacity. | planned | Contract 001 | recurring checkpoints |

## Current Checkpoint

- the operator promoted initial `v0.1.0` source-tag preparation; crates.io and
  GitHub Release publication remain excluded
- Research 111 finds deterministic QA and both Rust floors passing, then
  identifies stale release authority, Bedrock's legacy TLS advisory path,
  5,897 missing public-doc warnings, and missing source-consumer release polish
- Contract 036 now governs 27 source packages and exact tag consumption;
  g03.043 sequences cards 124-131
- card 125 removes Bedrock's legacy Rustls 0.21 path and adds a passing
  advisory, license, and source policy
- card 126 is complete: all 27 packages have a pinned 7,819-entry semantic API
  inventory and deny missing public docs at their roots; the review preserves
  route, access, lifecycle, recovery, and management differences instead of
  flattening them; all-feature workspace Rustdoc and broad QA pass with no
  hidden missing-doc allowances; card 127 is ready after the required review
  boundary
- card 127 is complete: the concise root front door, changelog, source-tag
  release notes, and security, support, and contribution policies match the
  27-package, 33-route reviewed surface; docs QA now rejects source-pin,
  package, or route drift
- card 128 is complete: package posture, source-only release configuration,
  matching CI, dependency refresh, and all 11 deterministic gates pass;
  Effigy's explicit first-tag/current-version mode simulates exact `v0.1.0`
  with only the changelog mutation and no release-state write
- card 130 is complete: annotated `v0.1.0` and `main` resolve to release commit
  `a8bef72b718d3d9e503da48b3af05da4b674d4ec`; no registry or GitHub Release
  mutation ran
- card 131 closes the tagged CI finding: accepted cancellation wakes the
  attachment pump and wins concurrent deadline readiness; cancellation tests
  isolate managed-resource setup contention; all six CI jobs pass on exact
  repair commit `4ffbd8f8a5302b9ce31ee37687876fcab8661f58` without moving
  the published tag
- g03.044 is active: the operator authorized `v0.1.1`; card 132 first repairs
  the Kimi fixture close-observation race found by Rust 1.90 release simulation
  before any version or tag mutation
- card 132 is complete: the synchronized fixture passes 40 repeated Rust 1.90
  runs, ten complete workspace test rounds, focused and extracted-package
  proof, and all 11 final-version release gates
- cards 133-134 are complete: all six CI jobs pass at exact release commit
  `bd3f4bbdffc403897ece4499ee0904b1e8116639`; annotated `v0.1.1` tag object
  `d7cb439ef3b6808013950d209c2ffcf7930ec81a` resolves to that commit and
  `v0.1.0` remains unchanged
- g02 closed at 49 roadmaps
- its only unfinished implementation lane, Pi RPC load and resume, moved to
  shared backlog behind the unchanged cwd-bound attachment gate
- no active spec governs g03
- g03.041 gives every safe diagnostic one portable unknown fallback, maps
  typed failure evidence across 17 adapters, and preserves exact route codes
  plus terminal source truth
- Research 110 and card 118 open g03.042: 18 route guides cover 26 of 33
  routes, 31 adapter examples cover 27 routes, and Contract 052 now requires
  traceable deep route and feature instructions
- card 119 closes the visible route holes: all 33 routes now map to a canonical
  guide and compiling example; Antigravity, Cursor, Grok, and Oh My Pi remain
  partial pending the shared deep-guide checklist
- Nucleus owns its delegated child-work, typed question, plan, and task-list
  adoption
- provider-session management binding persistence remains deferred; ordinary
  resume-binding persistence is complete in g03.025 after consumer evidence
- registry publication remains outside the active roadmap until the operator
  revisits it after sustained application usage
- the current structural scan reports 234 findings, including 22 errors; this
  known
  structural debt remains outside the active provider route batch
- Research 104 and Contract 050 select one consuming working-state restoration
  facade. Five reconciliation routes keep read-only observation; Claude Agent
  ACP and Kimi ACP use distinct live continuation recovery with no lost-turn
  state claim
- g03.034 realizes that facade across all seven selected routes. Focused tests,
  compile-checked guidance, and affected-package proof pass without live
  provider work
- the operator selected three follow-on restoration milestones inside g03:
  Cursor/Grok ACP qualification, explicit reconciliation-then-attachment, and
  gated Pi/Alibaba retained-session recovery
- card 090 independently qualified Cursor and Grok against exact load/replay
  evidence before either production mapping changed
- Gemini ACP remains blocked on replay-readiness evidence; private headless
  continuation and routes without reusable provider sessions remain outside
  this runway
- Research 105 and card 090 close g03.035 negatively: Cursor suppresses exact
  replay failures and Grok lacks complete client-visible replay proof; neither
  route gains load or continuation-recovery authority
- cards 091-092 are superseded without production changes
- operator direction expands the facade to every prepared interactive harness
  route; Research 106 and Contract 050 distinguish full recovery, exact
  reattachment with discarded replay, and fresh replacement with context loss
- g03.038 gives all 11 prepared interactive harness routes one truthful restart
  action; Cursor and Grok attach exactly, four weaker routes replace with
  explicit context loss, and Gemini remains unpromoted from replacement
- g03.036 is complete; Codex and OpenCode reconcile then load bounded replay,
  while Kimi local server reconciles an exact turn then resumes without replay
- Research 107 closes g03.037's candidate gate: Pi RPC `0.83.0` remains blocked
  on public cwd binding and corroboration; Alibaba Conversations advances to a
  separate retained-profile contract at card 098
- g03.037 is complete: Alibaba retained conversations load bounded ordered
  replay through the common continuation-recovery facade, preserve on ordinary
  close, and delete only under separate management authority; Pi remains
  blocked and the generation returns to its evidence gate
- operator selection opens g03.039: four remaining direct/attached interactive
  routes and two realtime routes may restore usability through explicit fresh
  replacement while private continuation, connection state, and interrupted
  turns remain lost
- g03.039 is complete: Anthropic, DeepSeek, Ollama, xAI, and ordinary Alibaba
  map to context-losing interactive replacement; OpenAI Realtime and Gemini
  Live map to distinct context-losing media replacement; retained Alibaba
  remains on stronger bounded continuation recovery
- Research 074 inventories 13 installed/attached harness route ids and bounds
  the first external currentness source set
- the 2026-07-31 currentness pass leaves Codex and stable ACP unchanged,
  classifies Claude Agent, Gemini, Pi, and Qwen range candidates, keeps Pi
  continuity blocked, and confirms OpenCode's optional live selector is stale
- Research 088 supersedes the coupled g03.002 tranche: standalone Claude Agent
  range maintenance moves to g03.015, while future Gemini CLI range work moves
  to deferred backlog without changing existing Gemini support
- Nucleus reproduced Codex `0.146.0` rejecting a legal numeric request ID at
  activity resolution after a typed callback answer; g03.003 repaired the
  mismatch with strict type-aware deterministic coverage
- the operator elected to complete the resulting portable request-reference
  representation contract immediately; g03.004 now preserves text versus
  signed-integer identity portably
- Research 075 promotes Cursor as the immediate high-priority installed
  harness: separate authenticated catalogue, ACP interactive, and headless
  structured roles behind one explicit facade
- Google's personal-account migration makes Antigravity the active Google
  harness lane; Gemini remains supported but further range work is deferred
  without an implied return date
- exact Qwen Code `0.21.2` is installed; delegated provider access remains a
  separate live-test gate and does not block deterministic range work
- Cursor card 011 adds the 25th workspace package with host-approved discovery,
  delegated local access, and a bounded authenticated catalogue
- Nucleus then proved request-correlation recovery through a live Codex
  `spawnAgent` call; g03.007 now admits only exact operation-local child-owned
  activity and restored the Cursor lane
- the first rerun against g03.007 proved spawn admission, then exposed Codex
  `0.146.0` child-local `turn/started` on the root-only lifecycle path; g03.008
  now projects exact child-local lifecycle without root authority and card 013
  remains next
- Research 076 and card 012 now qualify Cursor's exact installed ACP
  interactive route without a live prompt
- Research 077 and card 013 qualify the separate Cursor headless structured
  route from exact installed source; thinking deltas are provider-disclosed,
  read-only authority uses plan mode, and optional sandboxing remains separate
- card 014 closes the Cursor milestone with one explicit three-branch prepared
  facade, 30-route public truth, 24-solution feature truth, and extracted
  package proof
- Research 078 and card 015 qualify one exact Google-signed Antigravity CLI
  `1.1.9` artifact, preserve the shared `1.1.8`/`1.1.9` tag commit without
  inventing a second behavior range, and add separate discovery plus an
  authenticated identity-only catalogue
- Research 079 and card 016 qualify the exact headless stream-JSON route with
  explicit model, resource, effort, schema, permission, and optional-sandbox
  truth; bounded assistant, tool, subagent, usage, failure, and terminal
  evidence passes without a live provider prompt
- Research 080 and card 017 qualify ambient read-intent restarted continuity:
  the first turn captures one private conversation id and later turns use only
  explicit `--conversation`; mismatch, missing identity, cancellation,
  deadline, or uncertain state invalidates the handle without fallback
- card 018 closes Antigravity behind explicit catalogue, headless-run, and
  continuation facade branches; public truth now contains 32 production routes
  across 25 solutions, and the extracted 44-file package compiles independently
- Research 081 confirms installed Qwen Code `0.21.2`, records the discontinued
  Qwen OAuth posture, and selects fixture-first `0.19.11..=0.21.2` range closure
  ahead of the then-deferred standalone Claude ACP extension
- Research 082 and card 021 freeze seven stable Qwen points, unchanged stream
  declarations, and one `0.21.0` image-only catalogue-filter milestone without
  yet changing the exact `0.19.11` production claim
- card 022 promotes the two maintained Qwen segments and binds runtime stream
  version evidence to the exact preflight plan; 35 focused tests and the
  extracted 60-file package pass
- card 023 accepts installed exact `0.21.2`, adds three separately gated live
  selectors, and reconciles public route truth; catalogue access is unavailable
  from current harness authentication configuration and no prompt was run
- Research 083 selects installed/current Pi RPC `0.83.0` ahead of standalone
  Claude ACP `0.64.0`; Pi's ephemeral RPC range is now closed while its
  unchanged persisted-session cwd gate remains outside the lane
- roadmap g03.010 now guarantees six exact Pi points through installed
  `0.83.0`; its live selector exposed a separate environment-cleared npm
  launcher gap recorded in Research 084
- Research 084 is promoted into Contracts 010 and 032; g03.011 realizes one
  host-private native or interpreted launch recipe and repeats the exact
  installed Pi proof through it
- g03.011 now passes deterministic native/interpreted host proof, exact
  installed Pi `0.83.0` discovery under `env_clear()`, and two-package extracted
  compilation; interpreter and script selection remain explicit host policy
- the next maintenance checkpoint found no new consumer defect and only one
  material qualified-route drift: installed/current Grok Build `0.2.117`
- Research 085 freezes exact `0.2.115` through `0.2.117` artifacts and ACP
  initialization; exact `0.2.117` needs a private task-control behavior segment
  without adding portable authority
- card 030 freezes the four-point range corpus while preserving exact `0.2.114`
  as the production boundary; 23 focused Grok tests pass
- card 031 guarantees the two exact behavior segments through `0.2.117`, binds
  all four source revisions, and passes focused plus extracted-package proof
- card 032 classifies installed exact `0.2.117`, reconciles public route truth,
  and closes g03.012 without authentication or a provider prompt
- the next same-day checkpoint found Kimi Code `0.31.1` as the only new stable
  qualified-route candidate; Research 086 selects separate ACP, headless, and
  local-server range decisions without requiring a new contract
- card 033 freezes exact `0.31.1` artifact and source identity, preserves the
  `0.31.0` production ceiling during corpus acceptance, and passes 89 focused
  Kimi tests
- card 034 extends all three route claims through `0.31.1`, adds one exact
  local-server refresh-stable milestone, and passes focused plus extracted
  package proof
- card 035 accepts the exact signed artifact without changing installed
  `0.31.0`, reconciles public truth, and closes g03.013 without authentication
  or a provider prompt
- the next checkpoint finds Cursor Agent `2026.07.23-e383d2b` as the only
  material unqualified route drift; Research 087 confirms unchanged selected
  catalogue, ACP, and headless behavior without a provider prompt
- g03.014 fixes per-milestone opaque build enforcement, qualifies exact
  `2026.07.01-41b2de7` and `2026.07.23-e383d2b`, preserves the unsupported
  calendar gap, and leaves later dates visibly unverified
- Research 088 revalidates Claude Agent through current `0.64.0`: `0.62.0`
  leaves selected behavior unchanged, `0.63.0` adds private tool/subagent
  correlation, and `0.64.0` adds opt-in host steering plus a form marker
- Research 088 selected g03.015 as the standalone Claude Agent range lane;
  Gemini CLI requalification is deferred backlog and existing Gemini routes
  remain intact
- g03.015 qualifies Claude Agent ACP through exact `0.64.0`, accepts installed
  exact `0.63.0`, preserves six private behavior segments, and leaves later
  stable versions visibly unverified without selecting new optional authority
- Research 089 finds no new consumer regression and no non-deferred provider
  range drift; Gemini `0.53.1` remains recorded but deferred
- the only current maintenance defect is OpenCode's optional live probe, which
  still demands exact `1.14.48` despite a production claim through `1.18.10`;
  g03.016 repairs attached-probe truth without changing the route guarantee
- g03.016 now classifies OpenCode probe evidence through the production claim;
  four gated deterministic tests and 82 focused package tests pass while the
  network selector remains separately operator-gated
- Research 090 confirms Soundcheck uses bound operations while Nucleus retains
  three low-level extractions; only local deadline arithmetic and canonical
  Codex ChatGPT-subscription profile construction require new library work
- cards 044-045 add saturating local deadline derivation and one effect-free
  canonical Codex ChatGPT-subscription profile
- card 046 closes the multi-consumer usability milestone with bound-operation
  guidance, compile-tested canonical access, affected-package proof, and a
  bounded Nucleus adoption delta
- Research 091 finds no new consumer defect or non-deferred stable upstream
  drift, repairs the stale 24-package and 30-route authority counts, and leaves
  g03 evidence-gated without compiling roadmap 018
- Soundcheck then reproduced a Codex exec `0.146.0` queryless completed
  `web_search` navigation observation aborting a later valid structured result;
  g03.018 owns the exact fixture, narrow projection repair, and whole-stream
  regression
- g03.018 now accepts only completed queryless `action.type == "other"`
  lifecycle observations without content; ordinary search queries remain
  visible, and malformed actual searches remain rejected
- Soundcheck's unchanged Luna/medium structured-proposal test completed
  successfully through its consumer path; the defect is closed and g03 is
  evidence-gated again
- Research 092 and Contract 046 select explicit provider-session catalogue and
  import: candidates remain non-authoritative, import revalidates exact
  attachment identity, and existing load/replay/resume follows only after an
  ordinary binding is issued
- roadmaps g03.019-g03.023 sequence the shared kernel, Codex, stable ACP/Kimi,
  OpenCode, provider-wide acceptance, and a bounded Nucleus handoff; automatic
  synchronization, consumer persistence, and raw-id attachment remain excluded
- card 049 realizes separate catalogue/import vocabulary, bounded candidates
  and cursors, immutable plans, and typed drift-rejecting requests; 163 focused
  core/runtime tests pass and card 050 now owns the object-safe runtime roles
- card 050 realizes independently registered object-safe roles, bounded
  traversal outcomes, typed failure stages, host-service validation, prepared
  evidence, and exact revalidation-gated imported bindings; 107 focused runtime
  tests pass and card 051 now owns common cross-host conformance
- card 051 closes the shared foundation with reusable local and
  remote-authoritative fixtures, bounded negative conformance, exact prepared
  evidence, and import-to-load/resume sequencing; 248 focused tests and all
  three extracted common packages pass
- Research 093 and card 052 freeze the exact Codex catalogue/import operation
  floor at `0.105.0`, retain earlier supported app-server operations without
  catalogue claims, and keep current documentation as corroboration only; 149
  focused Codex tests pass without production changes
- card 053 maps exact cwd-scoped Codex listing and read-before-import behind the
  prepared facade, rejects drift and unavailable threads, and reuses existing
  load/replay/resume behavior unchanged; 154 focused Codex tests pass
- card 054 closes Codex import with common local and remote-authoritative
  conformance, lifecycle failure boundaries, compile-tested public guidance,
  and an independently compiling extracted package; 239 focused tests pass
- Research 094 and card 055 freeze current stable ACP v1 session listing,
  including independently gated additional directories, bounded opaque
  extensions, strict request correlation, and fail-closed projection; 93
  focused protocol tests pass
- card 056 qualifies resource-scoped Kimi ACP catalogue and explicit import
  across exact `0.28.1` through `0.31.1`; state-root or candidate drift issues
  no binding, ordered replay stays on the existing load path, and 186 focused
  Kimi/ACP tests plus both extracted packages pass
- card 057 closes g03.021 with production-driver conformance under local and
  remote-authoritative host identities, in-flight lifecycle control, honest
  Claude/Cursor non-promotion, public guidance, 272 focused tests, and two
  independently compiling packages
- Research 095 and card 058 freeze OpenCode list/status/get/messages/prompt
  closure across all 51 qualified releases: seven exact recursive surfaces,
  12 published-version segments, explicit directory/endpoint/status/child
  policy, deterministic failures, and 86 focused tests
  focused protocol tests and extracted-package verification pass
- card 059 adds an attached, directory-scoped OpenCode session catalogue and
  explicit import facade. Exact health, lookup, status, revision, directory,
  title, update-time, root, and archive evidence is revalidated before a
  binding enters the unchanged load/replay path; 195 focused tests pass
- card 060 closes g03.022 with provider-neutral contract inclusion, local and
  remote-authoritative topology proof, in-flight cancellation and deadline
  control, lease cleanup, attached-server preservation, public guidance, 172
  focused tests, and an independently compiling extracted package
- Research 096 and card 061 classify all 19 harness routes without transport
  inheritance: Codex app-server, Kimi ACP, and OpenCode HTTP are supported;
  one route is discovery-only, two attachment-only, two blocked, and 11 not
  applicable, each with an exact promotion gate
- card 062 publishes separate catalogue/import feature columns, the split
  19-route matrix, a prepared browse-select-import-load/resume guide, and
  compile-tested examples; seven extracted common and adapter packages compile
  independently
- card 063 publishes the bounded Nucleus external-thread handoff: prepared
  entry points, in-process binding ownership, replay-to-live persistence,
  duplicate and restart posture, unsupported/stale UX, and deterministic
  fixtures remain consumer-facing without a Nucleus edit
- Nucleus g05.073 then exposed a portable selection gap: Swallowtail had exact
  configured instances, access evidence, prepared routes, and model catalogues
  but no admitted projection binding them together; g03.024 and Contract 047
  own the consumer-assembled catalogue without adding routing policy
- cards 064-065 realize bounded exact admission, strict non-ready truth,
  authority-redacted records, focused and extracted-package proof, and the
  Nucleus g05.073 assembly handoff without authenticated provider work
- Research 097 shows T3 Code created fresh OpenCode roots after losing its
  provider-session mapping; OpenCode compaction retained the existing id.
  g03.025 promotes ordinary resume-binding persistence without automatic
  rotation, raw-id attachment, or management authority
- cards 066-067 add the bounded versioned record, strict attachment restore,
  same-session compaction corpus, exact-id restart proof, public adoption path,
  210 focused tests, and two independently compiling packages
- Research 098 promotes T3 Code's cross-thread message overwrite into g03.026:
  one typed operation-plus-activity key, explicit consumer runtime-id
  uniqueness, and Cursor reuse proof without provider id rewriting
- cards 068-069 expose the route-neutral `ActivityKey`, freeze repeated Cursor
  provider and fallback ids across two turns, update consumer examples, and
  pass 154 focused tests plus two extracted-package checks
- Research 099 and g03.027 separate crash recovery observation from session
  import, resume, retry, and cancellation. Card 070 realizes the portable
  boundary, exact-turn Codex mapping, and session-scoped OpenCode mapping;
  remaining routes retain exact evidence gates without expanding the main
  feature CSV
- Research 100 and g03.028 separate controlled local detachment from provider
  cancellation. Cards 071-072 add the portable handle control and first
  opt-in OpenCode read-only mapping without widening callback or owned-process
  routes; deterministic restart composition and package proof pass
- Research 101 and g03.029 add exact Kimi operation checkpoints, finite
  exact-turn reconciliation, and opt-in attached-turn detachment without
  widening callbacks, owned servers, or unverified versions
- Research 102 and g03.030 add the distinct provider-run checkpoint and
  reconciliation role, then qualify OpenAI background one-request recovery and
  opt-in structured-run detachment while preserving ordinary terminal deletion
- card 079 closes ACP retained-history reconciliation negatively: stable ACP
  `session/load`, the Claude continuity corpus through `0.61.0`, installed
  exact `0.63.0`, and Kimi ACP `0.28.1..=0.31.1` all restore a resumable
  session before or alongside replay; neither route gains Contract 048
  observation authority
- Research 103 and g03.032 reject Gemini headless reconciliation because exact
  `0.51.0..=0.52.0` listing may invoke summary inference and mutate transcript
  metadata while exposing no terminal record
- the same evidence invalidates Gemini's claimed read-only post-delete
  confirmation; g03.033 card 083 removes the public management role and binding,
  sends no list request, and preserves one operation-owned delete attempt as
  unconfirmed degraded cleanup
- Anthropic Managed Agents is the selected exact retained-run candidate:
  bounded session and persisted-event reads support active, waiting, terminal,
  interrupt, and unknown truth once an early route-bound checkpoint exists
- Contracts 022 and 048 keep read-only reconciliation separate from explicit
  inactive-only cleanup of the exact recovered session then environment; cards
  084-086 own portable vocabulary, route realization, and package acceptance
- card 085 adds the explicit recoverable Managed Agents profile, emits both
  persisted authorities before message dispatch, performs bounded exact
  session/event reconciliation, and orders inactive cleanup session before
  environment without retry or implicit interruption
- card 086 aligns prepared examples, consumer guidance, route truth,
  architecture, and Contract 048; focused, docs, and extracted-package
  acceptance pass without authenticated provider work

## Milestones

- [001 Installed Harness And Protocol Currentness Baseline](001-installed-harness-and-protocol-currentness-baseline.md) — completed
- [002 Claude And Gemini ACP Range Maintenance](002-claude-and-gemini-acp-range-maintenance.md) — superseded
- [003 Codex Request-ID Canonicalization](003-codex-request-id-canonicalization.md) — completed
- [004 Provider Request Reference Representation](004-provider-request-reference-representation.md) — completed
- [005 Cursor Installed Dual-Route Foundation](005-cursor-installed-dual-route-foundation.md) — completed
- [006 Antigravity Personal Harness Foundation](006-antigravity-personal-harness-foundation.md) — completed
- [007 Codex Operation-Local Child Activity Ownership](007-codex-operation-local-child-activity-ownership.md) — completed
- [008 Codex Child Turn Lifecycle Ownership](008-codex-child-turn-lifecycle-ownership.md) — completed
- [009 Qwen Code Installed Range Closure](009-qwen-code-installed-range-closure.md) — completed
- [010 Pi RPC Installed Range Closure](010-pi-rpc-installed-range-closure.md) — completed
- [011 Host-Approved Interpreted Executable Launch](011-host-approved-interpreted-executable-launch.md) — completed
- [012 Grok Build 0.2.117 Range Maintenance](012-grok-build-0-2-117-range-maintenance.md) — completed
- [013 Kimi Code 0.31.1 Range Maintenance](013-kimi-code-0-31-1-range-maintenance.md) — completed
- [014 Cursor Agent 2026.07.23 Range Maintenance](014-cursor-agent-2026-07-23-range-maintenance.md) — completed
- [015 Claude Agent 0.64 Standalone Range Maintenance](015-claude-agent-0-64-standalone-range-maintenance.md) — completed
- [016 Attached Harness Probe Compatibility Truth](016-attached-harness-probe-compatibility-truth.md) — completed
- [017 Prepared Facade Multi-Consumer Usability](017-prepared-facade-multi-consumer-usability.md) — completed
- [018 Codex Exec Queryless Navigation Lifecycle](018-codex-exec-queryless-navigation-lifecycle.md) — completed
- [019 Provider Session Catalogue And Import Foundation](019-provider-session-catalogue-and-import-foundation.md) — completed
- [020 Codex External Thread Discovery And Import](020-codex-external-thread-discovery-and-import.md) — completed
- [021 ACP Session List And Kimi Import](021-acp-session-list-and-kimi-import.md) — completed
- [022 OpenCode External Session Discovery And Import](022-opencode-external-session-discovery-and-import.md) — completed
- [023 Provider Session Import Acceptance And Handoff](023-provider-session-import-acceptance-and-handoff.md) — completed
- [024 Configured Provider Instance Catalogue](024-configured-provider-instance-catalogue.md) — completed
- [025 Durable Session Resume-Binding Persistence](025-durable-session-resume-binding-persistence.md) — completed
- [026 Portable Activity Key And Cross-Operation Isolation](026-portable-activity-key-and-cross-operation-isolation.md) — completed
- [027 Cross-Process Active Operation Reconciliation](027-cross-process-active-operation-reconciliation.md) — completed
- [028 Controlled Shutdown Active Operation Detachment](028-controlled-shutdown-active-operation-detachment.md) — completed
- [029 Kimi Operation Checkpoint, Reconciliation, And Detachment](029-kimi-operation-checkpoint-reconciliation-and-detachment.md) — completed
- [030 OpenAI Background Run Reconciliation And Detachment](030-openai-background-run-reconciliation-and-detachment.md) — completed
- [031 ACP Retained History Reconciliation Qualification](031-acp-retained-history-reconciliation-qualification.md) — completed
- [032 Retained Operation Reconciliation Candidate Gate](032-retained-operation-reconciliation-candidate-gate.md) — completed
- [033 Anthropic Managed Run Reconciliation And Recovered Cleanup](033-anthropic-managed-run-reconciliation-and-recovered-cleanup.md) — completed
- [034 Working-State Restoration Facade](034-working-state-restoration-facade.md) — completed
- [035 ACP Continuation Recovery Expansion](035-acp-continuation-recovery-expansion.md) — completed negatively
- [036 Reconciliation Then Attachment Composition](036-reconciliation-then-attachment-composition.md) — completed
- [037 Retained Session Recovery Promotion](037-retained-session-recovery-promotion.md) — completed
- [038 Provider-Wide Interactive Crash Recovery](038-provider-wide-interactive-crash-recovery.md) — completed
- [039 Provider-Wide Session Usability Restoration](039-provider-wide-session-usability-restoration.md) — completed
- [040 Oh My Pi RPC Foundation](040-oh-my-pi-rpc-foundation.md) — completed
- [041 Portable Failure Classification](041-portable-failure-classification.md) — completed
- [042 Complete Integration Guide System](042-complete-integration-guide-system.md) — completed
- [043 v0.1.0 Source Release Readiness](043-v0-1-0-source-release-readiness.md) — completed
- [044 v0.1.1 Source Patch Release](044-v0-1-1-source-patch-release.md) — completed

## Checkpoint

Roadmaps g03.035 through g03.038 are complete. Research 106 promotes a weaker
exact attachment boundary plus explicit fresh replacement; Contract 050 also
governs the completed settled reconcile-then-attach sequence. Roadmap g03.037
closes with a separate retained Alibaba open/load, common
continuation-recovery, preservation, continuation, and management-cleanup
path. Pi card 099 remains superseded behind its unchanged public cwd gate.
Roadmap g03.039 closes with every prepared reusable session shape mapped to one
truthful post-crash action. Roadmap g03.040 closes the distinct Oh My Pi
package and negotiated RPC v2 route with local-auth preparation, exact model
and reasoning control, typed questions, bounded PNG input, activity, usage,
cancellation, and fresh replacement. Its later operator-gated Luna/low smoke
passes through the prepared facade and freezes session-level model/thinking
lifecycle plus empty display clears. Roadmap g03.041 closes with a portable
failure classification on every safe diagnostic, evidence-backed mappings
across direct providers and harness routes, and honest `Unknown` fallback.
Terminal source, cleanup, callback, cancellation, timeout, and exact route
diagnostics remain distinct. The generation returns to its evidence gate.
Operator promotion opens roadmap g03.042. Cards 118-119 record and close seven
missing route guides plus six missing examples. Card 120 completes Contract
052 guidance for all 19 installed and attached harness routes, including
branch-specific recovery, callback, cleanup, failure, and promotion truth.
Card 121 completes the remaining 14 hosted, SDK, realtime, attached-runtime,
and owned-serving route guides. All 33 production route rows now meet Contract
052. Card 122 adds four shared runbooks, deepens plan/task activity and failure
handling, and gives all 34 feature columns plus the named portable surfaces a
canonical complete guide owner. Card 123 adds deterministic route, feature,
guide-index, and example enforcement, proves the three packages that gained
examples, closes g03.042, and returns the generation to its evidence gate.
The operator then promotes an initial GitHub source-tag release. Research 111
finds the functional baseline healthy but blocks the tag on stale registry
authority, Bedrock's legacy TLS graph, undocumented public Rust API, and
missing release-consumer polish. Contract 036 now selects 27 source packages,
Rust 1.90/1.94.1, and a separately authorized annotated `v0.1.0` tag. Roadmap
g03.043 owns cards 124-131. Card 125 removes the redundant Bedrock legacy TLS
feature, drops the vulnerable Rustls 0.21 graph, and adds a clean dependency
policy. Card 126 replaces declaration hashes with a pinned 27-package semantic
inventory and starts package-family Rustdoc closure; both protocol crates and
two support crates now deny missing docs locally. Testkit, core, runtime, and
seven hosted adapters are also closed. The first installed-harness batch adds
Antigravity, Cursor, Grok, and Qwen. The stateful harness batch adds Codex,
Claude Agent, and Kimi; 21 of 27 packages enforce the gate and 662 warnings
remain. The server and local-runtime batch adds Gemini, OpenCode, and Ollama;
24 of 27 packages enforce the gate and 255 warnings remain across llama.cpp,
Oh My Pi, and Pi.

## Generation Boundary

g03 begins at roadmap 001 and normally runs for 30-50 numbered roadmaps. A
consumer defect, provider release, or completed maintenance tranche does not
create another generation. Rollover needs a substantial run plus an explicit
sequencing reset.
