# 023 Harness Operation Isolation And Native Boundary

Status: active
Owner: Tom
Updated: 2026-10-09

## Purpose

Represent local harness isolation honestly for every operation shape and keep
provider-native permissions, budgets, retention, and sandbox features separate
from Swallowtail lifecycle authority.

## Operation-Shape-Neutral Isolation

`HarnessIsolation` applies whenever a local harness process performs an
interactive session or structured run:

- `AmbientHost` — the harness process and descendants execute with the ambient
  authority of the selected execution host
- `ProviderEnforced` — the provider harness enforces a documented boundary
- `HostEnforced` — the execution host enforces a separately qualified boundary

The exact posture is selected in operation requirements, carried in runtime
policy, and compared during pure preflight. A direct-inference operation cannot
claim harness isolation. A driver cannot infer an enforced posture from a
binary name, settings file, platform, or installation method.

Existing interactive-session access policy remains valid. New structured-run
harness drivers must bind isolation explicitly before process work. Older
drivers may migrate without a compatibility alias; no route may silently
change its realized claim during migration.

## Permission And Tool Boundary

Provider approval modes, tool allowlists or denylists, plan modes, safe modes,
and disabled extensions limit harness behavior. They do not contain the
harness process, its descendants, or unmediated filesystem and network access.

Swallowtail may require exact provider flags as part of a driver contract. It
reports only the isolation posture independently proven by the configured
route. A read-only tool posture under `AmbientHost` remains ambient.

## Codex Trust, Workspace And Managed Network Changes

Tom, 2026-10-07: "Yeah that's fine", accepting separate provider-free
`codex.app-server` and `codex.exec` adaptations for the following upstream
changes discovered in the pre-v0.5.2 sweep:

- both routes may accept skipped automatic persisted trust for projectless
  working directories; Swallowtail must not restore that persistence;
- app-server may accept path normalization and linked `.git` read-only
  protections within approved writable roots; Swallowtail must not bypass
  those protections or expand consumer-approved access;
- exec may honour host-managed application network restrictions, including
  system and macOS MDM allowlists; Swallowtail must not disable or weaken them
  with `ignore_managed_requirements`.

This is adaptation authority, not qualification evidence. Each route needs
its own frozen source and deterministic regression proof before a claim
moves. Prove projectless and marked-directory startup, app-server root and
alias boundaries, and exec allowed/denied network-policy outcomes as
applicable. Preserve read-only defaults, existing qualified points, and honest
failure projection. A provider restriction is not proof of process containment
or a new isolation posture.

Return any unproven boundary, further authority change, consumer-contract
narrowing, or public API/lifecycle change for a separate ruling. This authority
includes no live provider turns, credentials, installations, host or workflow
mutation, or release/tag action. The exec findings are retained in
[Research 370](../../research/370-codex-exec-currentness-stop.md); app-server
has a separate evidence record and qualification path.

### Codex App-Server Protected AWS Directory

Tom, 2026-10-07: "Approve", accepting Codex app-server `0.159.0`'s
read-only protection for an existing top-level `.aws` directory inside the
consumer-approved writable root. Document that exact version-specific
workspace-write limitation; do not add a write exception or bypass the
protection. Upstream protects AWS configuration because it can select
executable credential helpers.

Preserve earlier qualified behaviour and segments, read-only defaults, and
the approved-root boundary. Qualification must prove ordinary root writes,
protected `.aws` write denial, no access expansion, and honest failure
projection, alongside the previously authorized startup/alias proofs and
remaining per-hop semantic review. Name any necessary behaviour revision
from that evidence. This accepts the specified consumer-visible limitation;
it does not settle Contract 036 patch compatibility or waive any release
gate. Further authority changes, consumer narrowing, public API/lifecycle
changes, or live-proof needs still return for a separate ruling. The existing
exclusions on live work, credentials, host/workflow mutation and release/tag
actions remain in force.

## Gemini Headless Currentness Adaptation

Tom, operator board, 2026-10-07: "Approve bounded adaptation with
authority-preserving proof" (decision `de317828-daf8-4bb3-a2b3-d75bd9f5a833`).
This authorizes a separate provider-free `gemini-cli.headless` adaptation for
[Research 371](../../research/371-gemini-cli-0-63-0-headless-currentness-stop.md).
Preserve the selected authority and existing consumer contract: no automatic
transition from Plan Mode into implementation, no `ASK_USER` approval bypass,
defensive read checks honoured, and truthful bounded/truncated tool-output
semantics. Prove those boundaries with frozen selected-route evidence and
deterministic regressions before qualifying newer versions.

Preserve older qualified segments and keep ACP independent. A justified
private behaviour milestone requires evidence that the public contract stays
intact. An inability to preserve those promises, further authority change,
consumer narrowing, new public API/lifecycle need, or live-proof requirement
returns for a separate ruling. This includes no live providers, credentials,
installation, host/workflow mutation, or release/tag action. It does not
approve the distinct ACP restrictions in Research 372.

## Gemini ACP Permission And Filesystem Restrictions

Tom, operator board, 2026-10-07: "Accept documented restrictions and bounded
adaptation" (decision `b0a9a93a-282b-4029-baa2-57f7f2cfd198`). This accepts
Gemini CLI ACP `0.63.0`'s exact permission/filesystem restrictions as documented
version-specific limits and authorizes separate provider-free adaptation and
qualification of the selected route. Evidence is
[Research 372](../../research/372-gemini-cli-0-63-0-acp-currentness-stop.md).

Preserve reject-and-cancel: no new consumer approval callback, `YOLO` override,
or permission bypass. Honour `.gemini` configuration-write protections,
redirection requiring `ASK_USER`, defensive real-path checks, and protected
`.env.*` reads, including upstream's named example/template exceptions.
Document the resulting limits; prove ordinary bounded writes and exact
refused/cancelled outcomes with deterministic mode-specific regressions before
a claim moves. Report provider tool-output bounds and truncation honestly.

Preserve older qualified segments, exclusions, and independent headless
behaviour; name any necessary behaviour revision from evidence and assess
Contract 036 release compatibility separately. HTTP MCP live honouring remains
bound to exact `0.61.0`; no newer live claim follows from unchanged mapping.
Further authority changes, consumer narrowing, new public API/lifecycle needs,
or required live proof return for a separate ruling. This authority includes
no live providers, credentials, installation, host/workflow mutation, or
release/tag action, and waives no release gate.

## Claude Code Response-Only Linked Instructions

Tom, 2026-10-07: "Accept" (decision
`a9f8a8cd-ca40-46b1-8b3c-e197944171bd`). This accepts Claude Code
`2.1.282` and newer releases' reduced symlink-linked project-instruction read
set as a documented version-specific limit for `claude-code.response-only`
and authorizes provider-free qualification under the existing public
response-only contract.

Preserve the upstream restriction. Do not restore blocked instruction reads
or expand filesystem authority. Record the changed read boundary explicitly;
use a private behaviour milestone where Contract 029 requires it, supported
by frozen artifact evidence and selected-route proof before changing claims.
Preserve older qualified segments and exclusions. The current guide's
ambient instruction discovery is not a guarantee that every linked file is
read, and provider flags are not host containment.

Do not infer an unidentified internal resolver function from bundled hook
symbols or treat release notes alone as conformance proof. Further narrowing,
new public API/lifecycle needs, or required live proof return for a separate
ruling. Assess Contract 036 patch compatibility independently. This ruling
confers no qualification, live spend, credential, installation, host/workflow
mutation, or release/tag authority and waives no evidence gate.

## Codex App-Server Managed Policy Qualification

Tom ruled on 2026-10-08, answering “Accept and approve all” to decision
`d9a1fe6e-6481-433c-b643-ada177613e07` in the seven-decision batch.
Provider-free qualification may accept these selected upstream boundaries:

- managed model-provider revalidation can refuse `model/list` or `turn/start`,
  including retained threads;
- unresolved home-relative denials and invalid deny globs fail closed without
  materializing write grants;
- safe-to-replay bootstrap GET requests may fall back to the host system proxy,
  subject to applicable provider-network policy;
- explicitly configured Windows MxC may be inherited only within the approved
  host/resource and managed-policy boundary, without automatic setup or a
  stronger isolation claim;
- `application.network` restricts provider/API HTTP independently of the tool
  sandbox's `networkAccess=false`.

Each path requires exact selected-route proof, truthful refusal and joined
cleanup, with no bypass or access expansion. Source-only Windows evidence does
not qualify runtime isolation. Preserve older qualified points and document
justified private milestones. Further narrowing, public API/lifecycle changes,
unpreserved authority or live-proof needs return for a separate ruling.
Contract 036 patch compatibility, including the protected `.aws` limitation,
remains a separate assessment.

## Claude Code Headless Safety And Stream Adaptation

Tom's 2026-10-08 “Accept and approve all” answers decision
`0e2d4a38-33d4-43cd-b475-e901e41fe36b`. A provider-free private adaptation may
qualify the selected `2.1.287` forked-skill stream change and `2.1.290`
permission/safety rechecks after `PreToolUse` input rewrites. Preserve read-only
plan-mode authority, the approved resource boundary, bounded truthful stream
and usage, and the existing public lifecycle. Honour rechecks without bypass.

Complete every published-hop semantic review and route-facing proof before
moving claims; preserve older qualified segments. Further narrowing, new
public API/lifecycle or live-proof needs return for a separate ruling.
Contract 036 patch compatibility is assessed independently.

Tom's 2026-10-09 “Approve the two.” answers decision
`7c273552-55eb-4f52-b608-ca8430c46429`. Extend the retained headless
adaptation to exact `2.1.294` instruction-form prompt/agent hook safety and
`Stop` / `SubagentStop` behaviour. Honour shipped action blocking and stop
decisions while preserving selected user, project and local settings,
read-only plan-mode authority, approved resources, bounded truthful stream
and usage, and joined cancellation/cleanup. Exact selected-path semantic
evidence and deterministic fake-process proof precede any claim increase;
release notes or strings alone do not establish compatibility. Preserve
older segments and the separate SDK, ACP and response-only routes. No
settings/home relocation, disabled consumer hooks, artifact execution,
credentials, provider turns, installation or release authority is included.
Further narrowing, public API/lifecycle changes, missing selected-source
proof or live-proof needs return separately.

## Antigravity Headless Denial And Child Status

Tom's 2026-10-08 “Accept and approve all” answers decision
`4d072a6a-4dd8-47a8-9598-efa64a8f8895`. Provider-free adaptation may represent
`1.2.15` denial semantics with exact denial/failure fixtures and a private
behavior milestone. Denial must not trigger alternate-command or tool
workarounds; existing operations and the no-approval-bypass boundary remain.

For `1.3.1` child status, obtain exact selected-stream evidence first. Map a
proven error field to `Failed` within the existing vocabulary. TUI release
notes, missing fields or disassembly alone cannot prove a successful child.
Tom answered the two follow-up decisions on 2026-10-08:
“Approve both from your previous message”. Decision
`48b9bd88-5406-49bb-a158-a433fab35992` authorizes preserving reported child
identities with `SubagentStatus::Unknown` when the selected stream provides
no usable child status. Do not infer completion from child identity, the
enclosing step, or whole-run success. Keep tool-step, outer-run and child
outcomes distinct. Prove the fallback through provider-free fixtures and a
justified private behavior milestone. Preserve older qualified points and
released contracts; any inability to do so without a public API, lifecycle
or authority change returns for another ruling.

Decision `24e5b91b-4969-4990-aaae-8ffcdbebebff` accepts documented soft denial
as a route limitation: an approval-required tool may be denied while the
outer run continues and exits zero. Run completion does not prove every
requested tool executed. Preserve provider denial without alternate-tool
workarounds or approval bypass. No typed soft-denial observation is
established by the selected stdout stream. Do not invent a stdout event or
parse unspecified human-readable stderr notices into a denial outcome.
Structured denial mapping remains unclaimed pending exact selected evidence
and its concrete Contract 029 proof/adaptation follow-up; synthetic tool or
run errors do not establish it.

These rulings authorize bounded provider-free adaptation and documentation,
not qualification by themselves. Windows sandbox artifact/runtime and
approved-environment retry/authentication proof remain gates. Contract 036
patch compatibility remains a separate assessment.

Preserve older points and `AGY_CLI_MODEL_API_MAX_RETRIES=0`. Complete all-hop
semantic evidence without transferring catalogue proof. New public API,
Windows/runtime evidence, authentication/network boundaries or live-proof
needs return separately. No Antigravity live usage is authorized.

## Antigravity Windows Offline Sandbox Proof

The later 2026-10-09 Windows scheduling ruling in
[Contract 036](036-crate-release-and-compatibility-boundary.md#windows-compatibility-qualification)
defers this provider-specific Windows proof to the whole-codebase Windows
sweep at a time Tom chooses. The subsequent ruling resumes the non-Windows
Antigravity work without this proof as its completion requirement. The authority
below records the original Windows scope; it is not a current dispatch instruction. No original VM shutdown, clone setup or
artifact execution follows from the scheduling ruling. Reuse retained evidence
when the future sweep is explicitly scoped.

Tom's 2026-10-08 board answer to decision
`155265da-be30-4028-92cf-0ee064d1612c` is “Authorize bounded proof - I have a
windows VM we can use”. A separate proof may execute the exact Antigravity
`1.2.17` x64 and ARM64 Windows PE artifacts in explicitly identified,
operator-approved disposable Windows environments. Record the VM identity,
Windows version, architecture, execution mode and access path before execution;
an unspecified VM or another host does not satisfy this boundary.

Prove the fixture harness and containment against fakes, then persist the
execution plan and result record before any actual artifact attempt. Use
only task-owned scratch and fake provider/tool fixtures. Deny outbound
provider traffic, access no real credentials or authentication stores, and
make no host installation, persistent configuration or elevation changes.
Command and elevation probes stay inside the approved disposable environment.

Verify the original shipped PE and its published identity. Fake or patched
artifact behavior does not establish shipped sandbox enforcement. If the
selected path requires real authentication, provider traffic or unavailable
sandbox facilities, stop with the exact missing proof. Record native versus
emulated execution and never transfer proof between architectures or deployment
modes. An unavailable platform remains an evidence gate, not a removed claim.

This is provider-free proof authority, not a live provider turn, permission
bypass, broader platform claim, production adaptation, release or tag approval.
The separate resource-resolution and retry control-flow gates remain.

Tom identified the available environment in the same conversation as Windows
11 ARM64 in Parallels on his machine, and said he has no x64 access currently.
Only ARM64 execution can be proved there. Keep native x64 evidence pending;
x64 emulation on ARM64 is a distinct deployment and cannot supply native x64
proof. Environment preparation that changes VM configuration or provisions a
disposable environment needs separate authority under the no-configuration-
change boundary; availability alone does not establish isolation.

Tom's 2026-10-08 chat answer “approve” to decision
`a8182378-017f-4797-8a1d-568f02cd495f` authorizes a disposable native ARM64
clone of the identified Parallels Windows 11 VM
`67f62782-c64a-4c02-ba9a-7d9b24d107ce`. Leave the original suspended VM,
configuration and disks untouched. In the clone only, disable all network
adapters and host shared folders, profile, clipboard, application and device
integration before boot. Use a fresh unprivileged local test account and
task-owned scratch with fake authentication/provider/tool fixtures. Stage only
verified tools and original artifacts without host-wide installation or real
credential-store access. Prove no outbound provider traffic or host-file access
before vendor execution; never actually elevate the provider/tool probes or
alter original-host security. Record process/VM identities and exits and retain
the clone/results for independent review; deletion requires separate scope.
If cloning or containment requires touching the original, stop. Native x64
remains unproved and emulation is not a substitute.

## Copilot ACP Offline Artifact Proof

Tom's 2026-10-08 “Accept and approve all” answers decision
`8b713287-49bf-4b77-a172-01148166ee38`. A separate provider-free, no-network
artifact harness may investigate exact stable permission requests,
denial/cancellation and absence of tool effects at the preserved `1.0.80`,
first affected stable `1.0.81` and current stable, observed `1.0.93` when asked.
Re-probe official channels before selecting the target.

Prove containment and the harness against fakes first; persist the execution
record before exact-artifact execution. Use task-owned isolated scratch,
fake authentication/model responses, and no real credentials or outbound
network. No live provider turn, spend, installation or host update is granted.
If the selected path cannot be exercised without real authentication/network,
stop for separate authority. An evidenced authority/lifecycle narrowing needs
its own mapping ruling before adaptation or claim movement. Fake-provider
proof does not establish live permission or MCP honouring. A prerelease issue
report does not qualify a final stable.

## DeepSeek Harness Artifact/Profile Mapping Study

Tom's 2026-10-08 board answer to decision
`f3ca9685-76b4-4e89-88f3-45f82bf8b4b2` is “Approve bounded mapping study;
decide adaptation after exact evidence”. This permits provider-free inspection
of exact shipped wrappers, runtime artifacts and source for the existing
structured-run route. The npm CLI `0.2.0-rc.2` and PyPI runtime-bin `0.1.5rc1`
are independently identified artifacts; their version numbers or JSON-RPC
labels do not establish compatible executable identity or provenance.

Reuse retained evidence. Freeze exact executable selection/profile invocation
and published runtime-bin hops from `0.1.0rc6`; map initialize, prompt, idle,
shutdown, forced cancellation and join, plus approved host, Cordis, provider,
model, working-resource, authentication, tool and permission boundaries.
Return either exact evidence for a same-contract private mapping with a
proposed behavior milestone/axis ledger, or a concrete distinct-route/axis
contract proposal for another ruling. No runtime or public API implementation,
claim change, artifact execution, live work, authentication, installation or
tag authority is included. Preserve the existing exact `0.1.0rc6` point,
released consumers and separation from the local-server route until the
classification is independently reviewed and adaptation is authorized.

## Copilot ACP Bounded Authenticated Proof

Tom's 2026-10-08 board answer to decision
`5b8ae2a2-8f68-40c7-8c4b-58c00652d5e2` is “Approve bounded authenticated
proof and name existing access/model”. The offline artifact proof could not
reach permission exchange: exact `1.0.80`, `1.0.81` and `1.0.93` each required
authentication at `session/new`.

[Research 425](../../research/425-copilot-acp-authenticated-proof-preparation.md)
records preparation of that proof. The offline fake now tests a
host-owned one-use in-memory delegation channel and fake ACP `authenticate`
exchange, fail-closed account/model/audience/entitlement checks, cancelled
permission, abandoned pending wait, and joined no-effect cleanup. The
secret-free plan schema lists one bounded record template for each exact
artifact and keeps live execution disabled.
The fake does not establish any frozen artifact's ACP authentication,
Auto/model observation, entitlement, audiences, retry bounds, or permission
behavior. These remain prerequisites for the separately reviewed live
continuation; no original artifact or real credential was accessed.

A separately scoped proof may use one bounded ACP prompt per exact version,
three total, after the operator identifies the existing approved account or
access profile and model. Bind delegated credential mechanism, entitlement
and exact authentication/provider network audiences before dispatch. The
answer did not itself identify that account, profile or model. On 2026-10-09,
Tom completed CLI login and identified the GitHub account as “betterthanclay”.
He reported Auto and a local Gemma 12B model as available selections. Use the
existing account as the proof access reference; Auto is the candidate GitHub-
hosted selection, conditional on exact shipped-artifact support, selected-model
observability and bounded retry/fallback evidence. Local Gemma is a separate
provider path and does not substitute for this authenticated proof. The login
removes the missing-account prerequisite; it does not establish entitlement,
credential delegation, network audiences or those model/effect budgets. No
additional account creation, subscription or substitute credential authority
is implied. Do not extract
secrets, create or switch accounts, log in, relocate authentication/home state,
or change settings. Access identifiers are sufficient; never request tokens.

Prove containment and the harness against fakes, then persist an execution
record before each live invocation. Reject or cancel a benign task-owned
action requiring permission; never approve it. Observe exact permission,
cancellation and absence of tool effects, with bounded host stop/join. No
retry, resend or additional reviewer live spend is allowed. Establish selected
retry/fallback and effect budgets first; stop for a ruling if they cannot be
bounded. No installation, consumer mutation, qualification or tag authority
is included. Keep the existing exact `1.0.80` claim until independent review
establishes the selected shipped behavior and any mapping change is ruled.
This proof belongs to the minor currentness sweep, not the urgent SDK patch.

## Command Code Explicit Model Precedence

Tom's 2026-10-08 “Accept and approve all” chooses the recommendation in decision
`f92bc2d0-ebac-4ca0-99fb-5db67018e9e1`: the consumer's explicit `-m` model
remains authoritative when `featureModels.planning` is configured. Provider-free
investigation may prove a non-persistent private per-invocation mechanism
preserving that selection on `1.73.0` and later points.

No persistent `--config` update, authentication/home relocation, new operation
or silent model substitution is allowed. No safe mechanism is assumed merely
from this ruling. If none can be proven without a new API, authority or
consumer-visible narrowing, return the exact proposed adaptation for a
separate ruling before moving claims. Preserve older qualified points;
Contract 036 compatibility remains independent.

Tom's subsequent 2026-10-08 board answer to decision
`f04070a3-4a62-4214-bda8-788ccfb3b912` is “We have to work with whatever
command code ship”. Use official shipped Command Code artifacts. Do not build
the proposed upstream model-lane prototype, patch or fork the provider, or
claim a hypothetical startup fix. No upstream publication or patched-runtime
qualification is authorized.

Tom's later 2026-10-08 chat answer “approve” to decision
`3d693da8-b1cb-4c10-a405-a3a085e0a583` accepts a version-specific change to
the model guarantee: in plan mode on `1.73.0` and later official shipped
artifacts, configured `featureModels.planning` may take precedence over the
base model forwarded with `-m`. This supersedes the explicit-model guarantee
for those newly qualified points only; preserve older qualified behavior.

A bounded one-family contract/adapter adaptation must report requested versus
effective model truthfully, prove conflicting configuration cases and add a
justified private behavior milestone. Forwarded argv is not effective-model
evidence. If existing public vocabulary cannot express this boundary, return
the exact API change before implementation. Keep plan permission and no-bypass
semantics; no persistent settings writes, authentication/home relocation,
silent model substitution, provider patch/fork or live work. This accepts a
consumer-visible guarantee change for the separate minor release under
Contract 036, not the urgent SDK patch or tag authority.

## Pi SDK Node Bundled TLS Roots

Tom's 2026-10-08 chat answer “approve” to decision
`1e7a9fff-9472-4e5b-93db-250779aa5c8e` accepts Node `22.23.3`'s shipped
bundled TLS-root update as a version-specific provider trust boundary for the
Pi SDK Node axis. Preserve exact `22.23.2`, the approved Pi package, wire and
sidecar-source tuple, host environment and normal certificate/hostname
verification. Do not restore removed roots, inject CAs, disable verification
or change proxy, authentication or network policy.

Before qualification, prove exact root identities/removals and selected Pi
HTTP/TLS trust-source and failure paths through bounded task-owned offline
fixtures, and complete the remaining Node-hop semantic review. No actual
provider endpoint compatibility or live evidence follows from root inventories
or fake fixtures. Custom/system trust needs, affected required endpoints,
further authority or public-contract narrowing return for a separate ruling.
No real credentials, provider traffic, installation, host mutation or tag
approval is included. This axis belongs to the separate currentness minor;
Claude's Node proof does not qualify Pi's provider-HTTP trust path.

## Kiro ACP Explicit Environment

Tom's 2026-10-08 “Accept and approve all” answers decision
`8079018e-2ec0-45fd-aaf3-0351cc57716f`. Qualification may accept `2.24.0` and
later Kiro's removal of automatic project `.env` loading as a documented
version-specific boundary. Only explicitly host-approved delegated process
environment is inherited; preserve all older qualified points.

Swallowtail must not implicitly read project `.env`, inject its values or
change persistent configuration. A consumer requiring those values needs a
separate adaptation defining their admission into the approved environment.
Further authority, public API/lifecycle or live-proof needs return for a
separate ruling. Frozen identity and provider-free route proof must precede
qualification; Contract 036 patch compatibility remains separate.

## Kiro ACP Owner-Only State

Tom's 2026-10-08 board ruling, “Accept owner-only boundary; require exact ACP
proof”, answers decision `73ff6b53-f8be-4d7f-9031-c8b0a81e3ff1`.
Qualification may accept Kiro `2.27.0` and later owner-only home and prior-session
state as a documented provider boundary. Preserve the approved execution
principal and delegated environment, local-account authentication,
`session/new.cwd`, read-only working resource and reject-and-cancel policy.
Preserve all older qualified points.

Qualification still requires exact selected ACP entrypoint reachability,
affected paths, symlink and ownership treatment, failure behavior and
session-access effects. Vendor documentation and filename strings alone do
not establish those effects. Provider-free static control-flow proof may
establish only the behavior it actually maps. If it cannot establish the ACP
effects, retain the existing `2.21.4` claim and record the finite missing proof
with a concrete isolated provider-free runtime harness for separate approval.
Do not infer compatibility, weaken ownership permissions or repeat an
unchanged strings-only inspection.

Cross-user access, a change of principal, authentication or home, mutation
outside approved working resources, and a public API or lifecycle change need
another ruling. This ruling authorizes no artifact execution, credential
access, live provider work, installation, host mutation or release/tag action.
Contract 036 release compatibility remains separate.

## Kiro ACP Isolated Owner-State Proof

Tom's 2026-10-08 board answer, “Approve isolated proof and name the Linux ARM64
environment”, answers decision `99de6453-9492-4537-a346-ce7803dbfea3`.
It approves bounded offline proof comparing original Kiro `2.26.1` control
with `2.27.0`, `2.27.1` and `2.28.0`. The disposable native Linux aarch64 GNU
VM or container must be explicitly designated before vendor execution.
Tom's later 2026-10-08 chat designates a fresh Linux container through local
Colima or Docker (decision `e49192c0-3483-4275-a34c-7f4fd56891a8`). Use a
task-owned native Linux ARM64 GNU container named `swallowtail-kiro-owner-proof`,
reached through the local container CLI. Verify native architecture and GNU
runtime; emulation is not evidence. A task-owned fresh Colima profile may
supply the backend when neither local backend is running. Starting that
isolated backend and staging public images or artifacts is preparation, not
permission for provider traffic. Do not alter existing profiles, install host
software or change host settings. Verify external egress and all host mounts
are denied before fake or vendor execution; retain the isolated environment
for review. Return unavailable containment or required host changes separately.

Deny external egress and host mounts before execution. Use two synthetic
unprivileged UIDs and task-owned home, working directory, temporary files and
session state inside that environment. Access no real credentials or
authentication store, and make no provider or tool turns. Prove containment
against fakes and persist the execution record before running original
artifacts. Observe only ACP initialization, session creation and owner-state
path, symlink, access and failure effects. Stop if real authentication or
unavailable containment prevents exact reachability.

Preserve the existing `2.21.4` claim, public lifecycle and exact HTTP MCP
guarantees. Evidence does not automatically qualify newer versions. No host
installation or mutation, consumer repin, live work or release/tag authority
is included. Contract 036 compatibility assessment remains separate.

## Native Sandbox Boundary

Provider-native sandboxing is optional. A configured route may select
`ProviderEnforced` only when the exact provider version, invocation, platform,
and deployment satisfy its documented boundary and deterministic conformance
evidence.

Container, VM, App Sandbox, Landlock, or another host mechanism is never an
implicit prerequisite for harness communication. `AmbientHost` remains a valid
explicit posture. There is no fallback from `ProviderEnforced` or
`HostEnforced` to `AmbientHost` when setup, startup, or qualification fails.

## Native Budget Boundary

Harness-native wall-time, turn, tool-call, retry, and output bounds are
provider behavior. They may be exact required invocation inputs and may produce
typed provider failures or progress evidence. Their documented exemptions and
scope remain provider-specific.

They do not replace:

- the Swallowtail monotonic deadline
- consumer cancellation
- process stop and force-stop authority
- bounded event and output transport
- joined task and process cleanup

A driver may use both layers. It must keep terminal causes distinct and must
not report one provider-native bound as proof that another bound or host
deadline fired.

Contract 059 watcher work uses the same independent host deadline, stop, and
join authority. A provider background-task setting, native task id, or command
activity does not satisfy watcher ownership or the same-turn completion gate.

Provider-managed retry is disabled unless separately accepted. A harness flag
that retries indefinitely is not enabled by a generic deadline or CI context.

## Configuration And Retention

Harness configuration, credential state, runtime transcript state, and working
resources are distinct authorities.

- delegated harness authentication may use an exact host-approved environment
  without exposing stored credentials
- a config-suppression mode does not imply isolated credential or runtime state
- local harness transcript retention requires explicit temporary or durable
  provider-retention acceptance
- process exit does not prove transcript deletion
- retained state does not grant Swallowtail resume, load, enumerate, mutate, or
  delete authority

If a later route uses operation-scoped harness runtime state, its host lease and
cleanup must be contracted separately. No adapter deletes ambient harness state
by path convention.

## First Qwen Mapping

The Qwen Code `v0.19.11` headless proof uses:

- `HarnessInteraction` plus `StructuredRun`
- exact `AmbientHost` isolation
- explicit safe mode, approval posture, tool exclusions, model route, and
  native wall/tool/turn bounds
- a host deadline and process cancellation independent of native budgets
- explicit durable local harness retention with no resume or deletion claim
- no provider sandbox, container, persistent retry, background work, or route
  fallback

Qwen `--sandbox` remains a later `ProviderEnforced` profile. It is not enabled
or required by the ambient proof.

## Conformance

Applicable fixtures prove:

- missing or mismatched structured-run isolation fails before process work
- direct inference rejects a harness-isolation requirement
- provider tool restrictions cannot satisfy an enforced isolation requirement
- provider-native budget, host deadline, cancellation, and process failure
  remain distinct terminal causes
- durable local retention is explicit and creates no resume or deletion claim
- an unavailable enforced profile does not retry as ambient
- public diagnostics expose no raw configuration, credential, transcript path,
  argv prompt, stdout, or stderr payload

The existing one-shot structured-CLI profile remains the execution profile.
This contract adds an isolation and native-bound assertion pack rather than a
new transport profile.

## Exclusions

This contract does not standardize provider tool names, approval modes, budget
vocabulary, sandbox implementations, configuration files, transcript formats,
or credential stores. It does not authorize repository writes, provider
fallback, or consumer routing policy.

## Pi Sidecar Replay Failure And Cleanup Repair

Tom answered “Approve” on 2026-10-09 to decision
`21daeebe-1d60-4dfe-8f5b-7bac5c6ab685`, authorizing a bounded repair of the
recurring late-replay load failure and cleanup hang.

A `replay_item` after the `session_replay` response is a protocol failure.
The pump may still hold the armed collector (`replay_incomplete` count
mismatch) or the load path may already have taken it (`replay_unexpected`).
Pump shutdown and command registration share the pending lock, so a later
`state` command cannot register after that drain and wait for a pump that
will never answer. Fake-child controls force both orderings without a sleep
race. Load returns no handle; process, resource, and credential cleanup still
join. The fake host occupies its exact owned tree on start; wait reaps that
tree and attests `OwnedTreeEmpty`. Proofs assert that empty tree and
attestation, not only wait/resource/credential order. Host-local remains
`RootOnly`; real-host cleanup classification is unchanged.

Raising CI timeouts, skipping the failing test, weakening assertions or
waiting for lower machine load does not establish correctness. Keep provider
versions, compatibility claims, public API, permissions and lifecycle
promises unchanged. Any required change to those boundaries returns for a
separate ruling. No vendor execution, live provider work, credentials,
installation, host mutation or release authority is included.
