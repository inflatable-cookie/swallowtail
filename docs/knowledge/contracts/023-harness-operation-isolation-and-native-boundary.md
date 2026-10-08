# 023 Harness Operation Isolation And Native Boundary

Status: active
Owner: Tom
Updated: 2026-10-07

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

This answer does not establish a shipped mechanism preserving explicit model
selection. The recorded `1.79.1` planning-lane precedence still conflicts with
the earlier explicit-model guarantee. Before qualification, settle whether
the route may document provider-configured planning-model precedence as a
version-specific limit or must preserve the explicit-model guarantee through
an actually shipped mechanism. Do not infer that forwarding `-m` proves the
effective model, silently substitute a model, change permission mode to escape
the planning lane, write persistent settings or relocate authentication/home.
Preserve current qualified points and the separate release compatibility gate.

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
