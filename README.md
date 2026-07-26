# Swallowtail

Shared Rust infrastructure for discovering, connecting to, and driving AI
models and agent harnesses across host applications.

Swallowtail owns portable integration mechanisms. Applications retain their
prompts, tools, authority, workflows, persistence, and product state.

Status: foundation. The repository is in strict Northstar posture. The
provider-neutral core, pure preflight, executor-neutral runtime, thirteen synthetic
conformance profiles, host-approved local process/endpoint/credential services,
twenty-two production driver routes, including separate SDK-native Bedrock Runtime
and control-plane routes, across Alibaba Model Studio, Anthropic, Bedrock,
Claude Agent, Codex, DeepSeek, Gemini, Kimi, llama.cpp, Ollama, OpenAI,
OpenCode, Pi, Qwen, and xAI
are validated. Codex app-server supports both
unchanged read-only sessions and one explicit host-resolved bounded workspace
profile.

## Start Here

```sh
effigy tasks
effigy doctor
effigy test --plan
```

Then read [docs/README.md](docs/README.md). Applications integrating Codex
should start with the
[prepared-integration guides](docs/guides/README.md). Multi-provider
applications should start with the exact
[22-route matrix](docs/guides/provider-route-matrix.md).
Consumer-scale application proof is the active stabilization lane. The shared
evidence foundation, Codex typed bound operations, and Kimi Code ACP prepared
facade are complete. Anthropic Models and Messages now supplies the first
hosted-direct prepared facade. Ollama native supplies the attached-runtime
proof. Claude Agent and Gemini CLI now add separate read-only stdio-ACP
prepared paths. Pi RPC, Qwen headless, and OpenCode attached HTTP/SSE complete
prepared coverage for all eight harness routes. Kimi Platform, DeepSeek,
Alibaba Model Studio, and OpenAI background Responses now add distinct hosted
direct and provider-state facades; specialized adapters retain their low-level
production APIs. Separate xAI Responses WebSocket, OpenAI Realtime, and Gemini
Live facades now retain their native text, audio, cancellation, invalidation,
and rollover behavior. Bedrock Runtime and control-plane catalogue now add
separate SDK-native facades with explicit region and credential-provider
configuration. llama.cpp now completes all 22 prepared routes with distinct
attached inference and owned ephemeral-serving paths. The exact cross-route
matrix and unified integration guidance are complete. All 22 prepared routes
now pass from extracted package artifacts, alongside the Nucleus and
Soundcheck Codex proofs. One reproducible provider-wide `0.1.0` candidate is
retained as the application-hardening baseline; its exact canonical source,
parent, artifacts, and validation digests are recorded in candidate evidence.
Prior candidates remain superseded evidence. Publication is held until a
working consumer passes an accepted sustained workload through its normal
product path. No external release state has changed.

## Current Direction

- provider-neutral identities, capabilities, model catalogs, references,
  events, and diagnostics first
- interactive sessions and bounded structured runs as distinct execution
  shapes
- provider-specific behavior exposed honestly through capabilities
- multiple transport-specific drivers per integration family where needed
- harness control, direct model APIs, SDKs, CLIs, protocols, and local runtimes
  treated as distinct routes
- credential mechanism, entitlement, endpoint audience, and support authority
  kept explicit per driver instance
- local and remote execution hosts treated as equal topologies
- Nucleus and Soundcheck as initial consumers, not Swallowtail authorities

The twenty-three-crate Rust workspace contains `swallowtail-core`, `swallowtail-runtime`,
`swallowtail-testkit`, `swallowtail-host-local`, `swallowtail-protocol-acp`,
`swallowtail-protocol-openai-chat`, `swallowtail-transport-acp-remote`,
`swallowtail-adapter-alibaba-model-studio`,
`swallowtail-adapter-claude-agent`, `swallowtail-adapter-codex`,
`swallowtail-adapter-deepseek`, `swallowtail-adapter-opencode`,
`swallowtail-adapter-anthropic`, `swallowtail-adapter-bedrock`, `swallowtail-adapter-gemini`,
`swallowtail-adapter-kimi`, `swallowtail-adapter-kimi-platform`,
`swallowtail-adapter-llama-cpp`, `swallowtail-adapter-ollama`,
`swallowtail-adapter-pi`,
`swallowtail-adapter-openai`, `swallowtail-adapter-qwen`, and
`swallowtail-adapter-xai`. Provider behavior stays isolated in adapters.

The Soundcheck structured-run and Nucleus interactive-session lanes are
accepted in both consumers. Codex app-server
transports preflight-bound session options and declared dynamic-tool callbacks
without executing tools. Local and remote-authoritative fixtures prove
host-bound open, resume, callbacks, interruption, failure, and joined cleanup.
Soundcheck and Nucleus Agent Chat are accepted consumers. The bounded
workspace-write runtime required by Nucleus task execution is complete without
widening read-only chat. Provider expansion is now the active lane. The shared
hosted foundation and attached OpenCode HTTP/SSE proof are complete. The
provider-supported Anthropic Models and Messages direct driver is complete.
ACP v1 and Gemini CLI `0.51.0` authority, fixtures, bounded protocol transport,
read-only filesystem callback, and production driver are realized. Shared
lifecycle and topology conformance passes. The exact llama.cpp b9910 attached
deployment, operator-supplied model fixture, production driver, and
self-hosted conformance proof are complete without model or server ownership.
The post-tranche checkpoint selected xAI Responses WebSocket as the next proof.
Contract 016 now governs resource-free direct sessions, connection-bound
continuation and cleanup, and exact provider-billed-cost evidence. Deterministic
xAI WebSocket fixtures freeze the exact upgrade/auth boundary, serial
`store=false` turns, private continuation, exact billed ticks, failure,
disconnect, and close behavior. The production session driver now holds one
approved endpoint and credential lease across serial turns, emits exact billed-
cost evidence, rejects parallel turns before a second frame, and invalidates
the connection on cancellation, timeout, provider lifetime failure, or loss.
Provider-neutral direct-session conformance is complete. The xAI proof passes
under local and remote-authoritative hosts with 227 repository tests. Kimi Code
`0.28.1` currentness is promoted in Research 006. Contract 017 now separates
provider-owned load replay from resume, binds persistent sessions to their
resource and access authority, and keeps write callbacks, tool approval,
delegated login, and process containment independent. The deterministic Kimi
ACP corpus now covers new, load, resume, prompt, cancellation, bounded writes,
drift, auth failure, and disconnect without changing shared ACP framing.
Kimi process isolation is now optional rather than a prerequisite for harness
communication. Research 012 confirms the common control-plane pattern: T3 Code
and similar orchestrators map provider-native permissions or sandboxes and may
otherwise relay to the harness with ambient host authority. Swallowtail now
records `AmbientHost`, `ProviderEnforced`, and `HostEnforced` separately with
no silent fallback. Research 007 found no qualifying dynamic mechanism on the
current local host, and the operator selected a native deployment-owned macOS
App Sandbox helper instead of a container as the first optional host proof.
Research 009 fixes that seamless shape: one user-selected project grant,
persisted bookmark authority, isolated Kimi state, inherited descendants, and
no broad home access. Research 010 repairs a
later currentness error: the existing `0.28.1` corpus already targets the
maintained TypeScript successor. Its exact source commit, arm64 archive and
executable digests, signature inputs, isolated state, exclusions, and upgrade
gate are now frozen without changing shared ACP behavior. The independent
host-owned ephemeral llama.cpp lane is now complete. Research 008 and Contract
018 fix the b10069 single-model proof, read-only artifact leases, loopback endpoint handoff,
readiness-before-handle, and joined owned-child cleanup without taking model
acquisition, persistent serving, or Monkey authority. Provider-neutral artifact
identity, preflight binding, read-only lease ports, start deadlines, and scoped
endpoint handoff are realized. The local host now verifies exact approved
regular-file artifacts and their SHA-256 digests, publishes only scoped
loopback serving endpoints, and invalidates those endpoints before releasing
artifact authority after child join. Owned conformance passes under local and
remote-authoritative host identities. The exact llama.cpp b10069 owned driver
adds bounded startup supervision, health/build/route readiness before handoff,
and joined graceful-or-forced teardown without a live model. Production
conformance now covers both topologies, readiness timeout, route mismatch, safe
diagnostics, and ordered lease release while the b9910 attached server remains
externally owned. Roadmap 019 is complete with 257 passing repository tests.
The native macOS proof confirms dynamic project-grant propagation and
descendant containment for a compatible helper, but the exact Kimi `0.28.1`
artifact cannot retain its V8 and native-module runtime under the documented
inherited-helper signature. That exact `HostEnforced` profile is unavailable.
The explicit ambient Kimi ACP production mapping and conformance are complete.
The driver keeps
provider-owned load replay separate from replay-free resume, binds attachments
to exact resource and access posture, mediates bounded text replacement, and
joins process, callback, resource, and delegated-credential work without a
filesystem, descendant, or provider-tool network containment claim. A separate
persistent ACP extension proves load, replay, resume, bounded writes, delegated
auth, topology, redaction, and ordered cleanup without widening Gemini's
baseline. Roadmap 018 is complete. Research 013 accepts the provider-supported
`aws-sdk-bedrockruntime = 1.136.0` route as the first real in-process Rust SDK
proof. Contract 019 forbids ambient SDK configuration and fixes delegated cloud
credentials, private executor ownership, explicit region/endpoint binding, and
one inference attempt. The production Bedrock Runtime driver now binds one
exact host, endpoint, region, delegated credential provider, route, model, and
output bound. Typed `ConverseStream` projection, cancellation, full-stream
deadline, and credential release finish inside joined operation-private Tokio
work. Local and remote-authoritative hosted-direct conformance passes without
AWS access. Roadmap 020 and cards 067-069 are complete. Research 014 and
Contract 020 now fix the separate native Bedrock control-plane catalogue
boundary: one non-paginated `ListFoundationModels` request through
`aws-sdk-bedrock = 1.148.0`, with source-scoped observations that cannot imply
runtime capability, entitlement, or route selection. Card 071 completes the
provider-neutral records and generated SDK fixtures. The separately registered
production driver now binds its own descriptor, configured instance, regional
endpoint, access profile, delegated provider, deadline, one request, one SDK
attempt, and bounded projection. Local and remote-authoritative fixtures prove
deadline signalling, joined private execution, drift rejection, redaction,
credential release, and the absence of implicit routes or provider identity.
Roadmap 021 and cards 070-072 are complete. Research 015 and roadmap 022 select
OpenAI Responses background mode as the next proof because the provider-owned
run can outlive one SSE attachment and supports bounded cursor reattachment
and native cancel. Required temporary provider retention remains explicit even
with `store=false`. The route uses the OpenAI public API only; ChatGPT, Codex,
harness, subscription, and community OAuth access remain separate. Contract
021 and card 074 fix optional provider-managed background execution, explicit
temporary retention, maximum-one stream reattachment, provider cancellation
truth, and a deterministic public-API corpus. The production OpenAI driver now
binds one exact public endpoint, API-key lease, model route, output bound,
deadline, create attempt, reattachment, bounded retrieve, and native cancel.
Local and remote-authoritative conformance preserves ordered output, usage,
rate, request, failure, cancellation-race, deadline, redaction, and joined-
cleanup truth. Ordinary harness and direct runs remain attached with retention
prohibited and reattachment disabled. Roadmap 023 is complete. Research 016
selects Claude Managed Agents as the next high-information proof: a provider-
hosted remote harness with explicit durable retention, provider-managed
rescheduling, authoritative persisted events, callbacks, interruption, and
remote deletion truth. Contract 022 fixes a resource-free subset with one
operator-owned agent and driver-owned environment and session. It grants no
repository, provider filesystem, external sandbox network, or local-container
authority. Roadmap 025 and cards 077-079 own the proof. Cursor Cloud Agents
remains behind a separate repository and remote-workspace authority decision.
Card 077 realizes the minimum shared durable-retention, managed-recovery,
owned-resource deletion, structured-run tool/callback, exact preflight, and
dated REST/SSE fixture boundaries. The empty-host limited environment, pinned
session override, authoritative event reconciliation, callback, interrupt,
and ordered deletion transcript pass without live access. Card 078 adds the
production driver with exact provider-agent preflight identity, bounded
callbacks and recovery, native interrupt, usage evidence, ordered deletion,
safe failure, and joined cleanup. Card 079 adds the tenth provider-neutral
profile and proves the production driver under local and remote-authoritative
host identities. Roadmap 025 is complete. Full repository QA passes with 330
tests; three installed/live probes remain separately gated. Research 017 now
selects stable Qwen Code `v0.19.11` headless over unfinished remote ACP,
policy-bound Cursor Background Agents, and the experimental Qwen daemon.
Contract 023 makes structured-run harness isolation explicit without turning
safe mode, tool restrictions, native budgets, or optional sandboxing into a
containment claim. Card 080 adds the shared binding and a pinned offline Qwen
corpus with text stdin, stream JSON, a read-only tool registry, native terminal
exits, durable local retention, and explicit `AmbientHost`. Its 110 focused
tests pass without a Qwen binary, credential, provider request, or container.
Card 081 adds the separately registered production driver with exact preflight,
frozen argv, stdin-only content, bounded stream JSON, typed usage, native budget
truth, cancellation, deadlines, safe diagnostics, and joined cleanup. Card 082
passes the unchanged one-shot structured-CLI profile and a separate Contract
023 assertion pack under local and remote-authoritative host identities. It
makes no sandbox, container, resume, transcript-deletion, provider-fallback, or
direct-inference claim. Roadmap 026 is complete with 360 passing repository
tests. Research 018 revalidates direct Kimi Platform, DeepSeek, Z.AI, and
Alibaba/Qwen and selects the current Kimi K3 public API. Contract 024 keeps
shared Chat Completions reuse structural: provider access, model, reasoning,
catalogue, error, retry, and lifecycle semantics remain separately mapped.
Roadmap 028 now owns the common codec, offline Kimi K3 corpus, production
driver, and hosted-direct conformance. The bounded common codec and dated K3
corpus are realized: fragmented SSE, comments, `[DONE]`, common JSON envelopes,
structural unknowns, K3 reasoning/output/usage, safe errors, model mismatch,
and disconnect behavior pass offline. llama.cpp now uses the same codec without
changing its attached or owned driver behavior. The separate Kimi Platform
driver now binds one host-approved API-key audience, authenticated catalogue,
exact K3 route, reasoning mode, output bound, and one streaming attempt. Its
offline production fixtures prove ordered reasoning/output/usage, distinct
failure and disconnect classes, cancellation, deadline, redaction, joined
connection work, and awaited credential release. The unchanged hosted-direct
profile now passes under local and remote-authoritative execution hosts with
exact topology, source-scoped catalogue truth, one attempt, and no fallback or
detached work. Roadmap 028 is complete with 384 passing repository tests; three
installed or live probes remain separately gated. Research 019 selects Alibaba
Model Studio's Singapore workspace-dedicated Conversations and Responses route
over another stateless DeepSeek or Z.AI mapping. Contract 025 now governs
explicit provider-conversation retention, exact regional workspace access,
local-only cancellation, and item-before-conversation deletion. The shared
session-only provider-state policy, conversation and aggregate-item deletion
kinds, dated corpus, production Alibaba driver, and local plus remote-
authoritative conformance are realized. Roadmap 029 is complete. Full repository
QA passes with 404 tests; three installed or live probes remain separately
gated. Research 020 re-ranks the remaining routes and selects the provider-
supported OpenAI Realtime GA WebSocket because continuous media exchange is the
largest missing mechanism. Contract 026 keeps realtime media inside a separate
direct interactive role with bounded redacted chunks, exact formats, consumer-
owned device and playback truth, native response cancellation, and joined
duplex cleanup. Card 091 realizes the role, records, pure preflight, eleventh
synthetic profile, and dated OpenAI Realtime offline corpus. Card 092 adds a
separately registered production WebSocket driver with exact public API-key
access, fixed PCM16 formats, two serial responses, native response
cancellation, ordered typed evidence, and joined credential-last cleanup under
both host identities. The production driver now passes the eleventh common
profile plus deterministic parallel, provider-failure, format-drift,
disconnect, cancellation-uncertainty, deadline, timer, and cleanup-failure
coverage under both host identities. Roadmap 031 is complete. Full repository
QA passes with 430 tests; three installed or live probes remain ignored and
doctor remains at the inherited 19 oversized-file findings. Roadmap 032 now
closes after selecting Gemini Live preview as the next realtime portability
proof. Contract 027 makes one provider-planned connection rollover explicit,
bounded, private-handle-backed, and distinct from reconnect or consumer resume.
Card 095 realizes the provider-neutral bounded-rollover policy, exact
preflight/request agreement, an assertion pack over the unchanged realtime
profile, and a dated Gemini Live raw-WebSocket corpus with private redacted
handle lifecycle. Card 096 adds the separate production driver with exact
preview preflight, host-approved query-key WebSocket access, manual asymmetric
audio, two serial turns, one confirmed private-handle rollover, local-only
interruption, safe failures, and joined credential-last cleanup. Card 097 is
complete with unchanged-profile and bounded-rollover conformance, local and
remote-authoritative topology, the full failure matrix, and 443 passing tests.
Roadmap 034 and card 098 close the post-Gemini-Live coverage checkpoint.
Research 022 audits all eighteen production
descriptors and eleven common profiles. Maintained Pi `0.80.10` RPC is the next
proof. Contract 028 keeps prompt, steering, follow-up, abort, extension UI,
downstream provider/model identity, ambient authority, retry, and cleanup
separate. The first route needs no container or sandbox and makes no
containment claim. Roadmap 035 and cards 099-101 own the records, frozen
corpus, production driver, and conformance. DeepSeek V4 reasoning/tool
continuation remains the next direct-contract research target rather than a
stateless compatible breadth adapter.

Card 099 now adds exact interface-version bindings and maintained compatibility
windows under Contract 029. Exact runtime versions remain separate from a
driver's supported baseline, latest-qualified boundary, behavior milestones,
deprecated segments, and exclusions. One application release can therefore
serve a deliberate range of installed harness versions without changing
consumer operations. The shared Pi RPC scheduling, restrictive ambient policy,
acknowledgement, and bounded UI records are realized as an assertion pack over
the unchanged eleven profiles. The frozen `0.80.10` corpus passes offline.
Card 100 adds the separately registered production process driver with exact
provider/model argv, restrictive startup-state validation, supervised strict-
LF transport, prompt, steering, follow-up, UI relay, native abort, deadlines,
and joined credential-last cleanup. Card 101 completes cross-topology
scheduling, callback-expiry, late-response, failure, and cleanup conformance
without changing the long-lived RPC profile. Roadmap 035 is closed.
Full repository QA passes with 466 tests and three gated probes ignored.

Roadmap 036 completes the DeepSeek V4 direct-continuation lane. Research 023
selects the exact OpenAI-format endpoint and `deepseek-v4-pro`; the Anthropic
facade is excluded from the first proof because it maps unsupported models and
ignores fields. Contract 030 defines a resource-free locally continued direct
session: every provider attempt needs explicit consumer authorization, tools
remain consumer-executed, and provider reasoning stays private, bounded,
ephemeral, and route-bound. Provider disk-cache posture is explicit. Card 103
now realizes provider-neutral continuation bounds, pure request-plan agreement,
redacted attempt/tool/continuation records, the additive twelfth profile, and
the exact V4 Pro buffered-plus-SSE offline corpus. The separately registered
production driver now proves authenticated catalogue access, consumer-owned
tool exchange, private continuation replay, three bounded attempts, exact
usage/cache/finish/request evidence, cancellation, deadline, failure, and
credential-last cleanup under both host identities. Full QA has a 489-test
inventory: 486 pass and three gated probes remain ignored. Roadmap 037 and card
105 close the coverage and compatibility-window checkpoint. Research 024
confirmed only Pi and DeepSeek published descriptor claims, both as one-point
windows; remote ACP was Draft at that checkpoint. Contract 031 and roadmap 038
select attach-only
Ollama native API as the first non-singleton compatibility proof. Stable
`0.14.0` through `0.32.1` now drives a production attach-only catalogue and
structured-run route with exact runtime discovery, installed and running model
observation, native NDJSON, and explicit inference-caused residency. The route
adds no container, model acquisition, cloud access, or Monkey authority.
Portability and full repository QA now pass with 522 tests. Research 025
revalidates every installed production harness pin and selects both Codex
drivers for the first maintained-range retrofit. Current candidate floors are
exec `0.122.0` and app-server v2 `0.110.0`; latest stable is `0.145.0`.
Version-specific corpora and experimental-gate enforcement now pass. Contract
032 and card 111 add explicit safe
observation of one host-approved installed executable with authoritative
topology, exact classification, cancellation, deadline, and joined cleanup.
Card 112 freezes exact Codex exec and app-server release corpora, stable versus
experimental schemas, rejection neighbors, and gate-enforcing transcripts.
Card 113 now publishes independent exact-observation claims: exec
`0.122.0..=0.145.0`, and app-server `0.110.0..=0.130.0` plus
`0.131.0..=0.145.0`. Joined target-aware discovery, milestone dispatch, and
stable-versus-experimental negotiation pass focused validation. Cross-topology
range conformance and full repository QA now pass with a 549-test inventory:
545 pass and four separately gated probes remain ignored. The older
January-to-April audit found the selected app-server v2 surface in exact
`0.80.0` tagged source. Deprecated six-month segments are now compiled without
a v1 driver, container, temporary credential home, or implicit fallback.
Legacy exec will expose ambient configuration and pre-`0.99.0` durable
retention explicitly; current isolated behavior remains separate. Shared
harness-configuration posture is now realized independently from isolation,
credentials, retention, and working resources. Exact ambient and
provider-suppressed bindings pass structured-run and long-lived harness
profiles; host-scoped execution stays closed until a separate host lease
exists. Exact January-to-April exec and app-server corpora now distinguish
source-generated schemas, upstream-published schemas, default versus explicit
stdio, ambient versus suppressed configuration, and retained versus ephemeral
execution. Both production drivers now select those private behavior segments
only from an immutable exact executable-version binding. Legacy policy and
capability mismatches fail before harness work; current isolation and
experimental gates remain unchanged. Full six-month range conformance and
roadmap closeout now pass. Current release and schema evidence selects OpenCode
HTTP as the second installed-harness range proof. Its candidate envelope is
`1.14.48..=1.18.4`: 45 stable releases retain the selected six routes. Their
recursively closed schemas produce 18 exact surfaces and 20 contiguous
published segments. The descriptor publishes that closed claim. Exact
instance, requirement, plan, health, and session bindings select only private
frozen behavior revisions. Cross-topology conformance and full QA pass without
changing the attached HTTP/SSE operation shape. The next lane is a fresh
provider-coverage checkpoint. Qualified windows now mean guaranteed support,
not a hard execution ceiling. Ordered Codex and OpenCode claims permit exact
newer stable releases as visibly unverified and dispatch them through their
latest qualified private behavior. They do not count those releases as
supported. Prereleases, gaps, exclusions, older points, malformed versions,
and runtime drift remain closed. Research 028 selects exact Kimi Code `0.28.1`
and `0.29.0` as separate qualified ACP behavior segments. Contract 034 adds the
narrow missing boundary for typed reasoning setup through version-qualified
private harness options. Roadmap 043 owns the corpus, installed discovery,
private dispatch, and cross-topology proof without a container or sandbox
requirement. The typed negotiation records, exact `0.28.1`/`0.29.0` behavior
corpus, host-approved installed discovery, private version dispatch,
initialization corroboration, and confirmed new-session reasoning setup are
complete. Stable newer releases remain executable but visibly unverified.
Cross-topology conformance now proves baseline, latest, unverified-newer,
dynamic-option rejection, and the unchanged persistent ACP lifecycle under
both authoritative host identities. Full repository QA inventories 606 tests:
602 pass and four gated probes remain ignored. Roadmap 043 is complete.
Research 029 closes roadmap 044 and selects remote ACP after the Active
transport RFD gained a maintained Rust HTTP transport crate. Contract 035
keeps the proof provider-neutral, unauthenticated, opt-in experimental, and
free of implicit recovery or transport fallback. Provider-neutral remote ACP
connection, affinity, bound, and version records are now realized alongside
the thirteenth conformance profile. Independent raw HTTP/SSE and WebSocket
corpora freeze both lifecycle shapes, including connection-private cookie
affinity, without depending on a production client. The reusable client is
realized with
explicit HTTP/2 SSE or WebSocket selection, bounded private cookies, frames,
correlation, and streams, and host-owned joined runtime work. Card 135 owns
the completed cross-topology matrix, maintained-server cross-check, redaction
audit, and full closeout. Roadmap 045 and cards 133-135 are complete with 629
tests inventoried. Research 030 kept g01 active at 47 roadmaps and selected
Grok Build as the next
maintained-range ACP harness proof. Research 031 now freezes the exact
`0.2.0` and `0.2.111` artifacts and all 111 published `0.2.x` points, but
qualifies no release. Direct `0.2.111 --no-auto-update --version` is the sole
safe discovery candidate. Exact authentication behavior still needs
activation-only evidence, and bundled guidance disproves the planned bounded
read-only claim. The operator placed roadmap 047 on hold because no Grok
account is available. Its evidence remains intact and its viable route stays an
explicit ambient harness relay with separately opt-in sandboxing. Roadmap 048
now selects Claude Agent ACP as the post-Grok proof. Research 032 fixes ACP v1
stdio, qualified adapter range `0.53.0..=0.61.0` excluding unpublished
`0.58.0`, Anthropic public-API-key access, `Ambient` configuration, and
`AmbientHost` isolation. Exact Agent SDK `0.3.195..=0.3.217` and nested Claude
Code `2.1.195..=2.1.217` evidence stays separate from the wrapper and ACP wire.
The deterministic corpus passes without a live account or container. The
separate production driver now observes the exact wrapper version, dispatches
the four qualified behavior milestones, permits stable newer versions only as
visible unverified executions, and runs the frozen read-only ACP session with
joined deadline, cancellation, process, resource, and credential cleanup.
Claude subscription login remains excluded because Swallowtail has no separate
approval to offer claude.ai access. The unchanged long-lived ACP profile and
adapter-local matrix now pass under local and remote-authoritative host
topologies. Full QA inventories 658 tests: 654 pass and four gated probes remain
ignored. Roadmap 048 is complete. Roadmap 049 and card 146 complete the
49-roadmap g01 generation-disposition checkpoint. g01 is closed with 48
completed roadmaps and the blocked Grok lane moved to the shared backlog.
g02 now prioritizes API stabilization, release discipline, packaging, and
consumer upgrade support. Its first milestone inventories the 23-crate package
graph and current Cargo rules before promoting a release contract. No crates.io
publication, tag, API 1.0 promise, or provider implementation is implied.
Research 033 completes that inventory. The operator approved its package,
registry, version, and MSRV recommendation. Spec 004 is archived, and Contract
036 now governs all 23 public packages, coordinated pre-1.0 compatibility, the
bounded MSRV policy, package evidence, and explicit human release authority.
Card 003 now realizes those rules across every manifest and adds deterministic
Effigy metadata, dependency, public-declaration, documentation, MSRV, content,
and package-family gates. All 23 packages assemble from a clean isolated source
snapshot and the extracted family compiles against the locked graph without
publishing. Card 004 freezes one exact reproducible `0.1.0` candidate and
completes isolated Nucleus and Soundcheck upgrade and rollback handoffs.
Milestone g02.001 is complete. Registry preflight, staged publication, tags,
pushes, releases, workflows, and consumer edits remain behind one explicit
operator decision. Research 034 then found a release-blocking usability gap:
Nucleus and Soundcheck compile while ordinary Codex integration still requires
duplicated low-level preparation and can omit runtime-only bindings. Contract
037 now promotes Spec 005's two-layer prepared integration boundary without
weakening the low-level roles. Roadmaps g02.002-g02.006 sequence plan-derived
requests, joined local host composition, separate Codex exec and app-server
facades, consumer-owned Nucleus and Soundcheck simplification, and packaged
runtime proof. The first `0.1.0` candidate is superseded with its evidence
retained until a replacement closes that runway. The shared request,
diagnostic, access-provenance, joined-task, local-service, and exact-target
foundations and the Codex exact-target prepared factory are now complete.
Separate prepared catalogue, read-only session, bounded-workspace session, and
structured-exec profiles now pass deterministic local and remote-authoritative
conformance with public usage and low-level escape-hatch guidance. Nucleus has
completed prepared-facade adoption. Soundcheck's prepared catalogue and exec
migration now passes exact-version preparation, normal health, 106 Rust tests,
13 frontend tests, the locked app check, and QA. Packaged cross-consumer
runtime proof also passes across a transient 23-package candidate without live
credentials or provider calls. Spec 006 and revised Contract 037 now extend
that normal-path requirement to all 22 production driver routes. The adapters
will expose typed prepared operations by runtime family without a universal
prompt API, provider router, hidden authority, or mandatory sandbox.
Kimi Code ACP, Anthropic Messages, and Ollama native form the first
cross-shape proof. Codex prepared catalogue, structured-run, session-open, and
session-resume methods now delegate directly to the unchanged low-level roles,
and Kimi Code ACP now adds exact prepared discovery plus bound new, load,
resume, prompt, and interruption paths without a containment claim. Anthropic
Messages now adds separate prepared catalogue and one-attempt inference paths
with exact endpoint, API-key, route, output-bound, streaming, and cleanup
truth. Ollama native now adds exact attached-runtime discovery, distinct
installed and running observations, selected artifact evidence, explicit
runtime-managed residency, and no server or model-management authority. The
three-shape facade review accepts the common evidence and two-phase authoring
pattern without flattening native operations. Claude Agent and Gemini CLI ACP
now add exact executable preparation, visible configuration and version
posture, plan-derived read-only sessions, and no implicit remote-ACP fallback.
Pi RPC now adds exact installed discovery and a provider-suppressed ambient
read-only prepared session while retaining native scheduling and UI relay.
Qwen headless now adds exact installed discovery and one ambient structured
run with explicit provider, model, stdin, stream-JSON, and native budgets.
Later stable Pi, Qwen, and OpenCode releases remain executable as visible
unverified-newer observations. OpenCode now adds exact attached-service
preparation plus separate catalogue and read-only session paths without server
lifecycle or remote-ACP fallback. Kimi Platform now adds separate catalogue and
one-attempt K3 preparation while rejecting Membership and Kimi Code access.
DeepSeek now adds separate catalogue and explicit consumer-owned tool
continuation with private reasoning replay and visible cache acceptance.
Alibaba Model Studio now adds exact regional workspace preparation with
explicit durable retention and item-before-conversation deletion. OpenAI
background Responses now adds exact public-API preparation, visible temporary
retention, maximum-one cursor reattachment, bounded retrieval, native
cancellation, and unchanged remote terminal truth. Anthropic Managed Agents
now adds separate provider-hosted harness preparation with an operator-owned
agent, driver-owned environment and session, authoritative history recovery,
callbacks, interruption, and ordered delete-on-close. Realtime provider
facades now add three separate prepared connection paths: xAI serial text with
private continuation and billed ticks, OpenAI manual 24 kHz PCM with native
response cancellation, and Gemini asymmetric PCM with one planned rollover.
Bedrock SDK facades now add separate Runtime inference and control-plane
catalogue preparation with exact region, SDK, service API, endpoint audience,
access, and delegated credential-provider evidence. Catalogue results cannot
select Runtime routes. llama.cpp now adds separate attached catalogue and
inference preparation plus owned ephemeral serving. The attached path retains
external-server ownership; the owned path couples an approved executable,
exact GGUF artifact, route, loopback readiness, and ordered teardown. All 22
production routes now have adapter-local prepared normal paths. The held
replacement candidate returns only after route-matrix and provider-wide
packaged evidence.

Publication remains held after that candidate. The active lane is
application-scale proof through normal consumer paths. Nucleus now has the
credential-free isolated profile, bounded deadline, native cancellation,
terminal persistence, disposable fixture binding, Effigy selectors, and
sanitized evidence needed for the first pilot. Its exact source and pilot
tuple are frozen. The first approved launch confirmed the exact model and
ChatGPT audience, then stopped before provider-session or turn work on a
Swallowtail prepared-tool bound. That defect is fixed with deterministic
coverage. The next launch exposed and fixed a second Swallowtail facade defect:
prepared interactive sessions omitted the time service required by their
promised turn deadlines. The repaired pilot then completed all 12 planned
ordinary, callback, cancellation, restart-recovery, and deadline outcomes at
the exact 15-attempt and 6-session ceiling, with joined cleanup and no fixture
drift. The first sustained read-only Nucleus tranche stopped after native UI
control resolved a shared bundle identity outside the isolated proof
environment. Ten marked synthetic turns reached normal Nucleus state and are
preserved. An exact rebuilt-bundle reset is active under higher turn, launch,
and session ceilings. Exact provider rate, quota, usage, and billed-cost
evidence remains unavailable on this subscription-backed path. Soundcheck
previously exposed an opaque
non-zero Codex installed-version probe through a host-selected wrapper.
Swallowtail now retains numeric status and bounded sanitized stderr under the
stable discovery failure code without rejecting wrappers. Writable proof
remains separately gated.
