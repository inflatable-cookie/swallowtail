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

## Gemini Headless Plan Transition Enforcement Design

Tom's 2026-10-09 board answer to decision
`df2315cd-fb6c-44e7-8d4e-868249400205` is “Approve offline enforcement
design; implementation returns separately”. This authorizes a bounded offline
enforcement design for Research 417's selected Plan-to-YOLO transition, reusing
completed source, identity and fake evidence.

### Frozen control ledger

Research 417 pins Gemini CLI `0.63.0` to source commit
`573846625af9e93b3b968e0e0b86bb093a4c9b16` and tree manifest
`fbb5d78fd631e4a53e26a62284f8c15d5d90a1ef2743d4d79a5517218dfc6e51`. The
selected authority hashes remain in the
[frozen authority fixture](../../../crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-headless-authority-0.63.0/authority-evidence.json);
the complete path hashes are in the
[frozen source inventory](../../../crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-0.63.0/source-tree-inventory.json).
The additional files below are read-only source analysis against that same
tree. Their hashes are from that inventory.

| Control | Frozen source | Result |
| --- | --- | --- |
| Invocation and overlays | [`--approval-mode` argument](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/cli/src/config/config.ts#L321-L343), `bdb490bbf0ea1803d74e8bc1748432096eff7063abec0447e5e7aa467e4187b8`; [`--policy` and `--admin-policy` arguments](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/cli/src/config/config.ts#L347-L362); [`--allowed-tools` option](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/cli/src/config/config.ts#L379-L386); [argument-to-settings mapping](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/cli/src/config/config.ts#L791-L826) | The CLI exposes `--approval-mode`, `--policy`, `--admin-policy`, and deprecated `--allowed-tools`; it has no mapped `--exclude-tools` option. `--allowed-tools` feeds `tools.allowed` (with settings fallback), so an allow such as `write_file` overrides Plan's default catch-all deny. `--policy` replaces the default user-policy directory when supplied. `--admin-policy` replaces the settings path list, and Gemini ignores supplemental admin paths when its system policy directory contains TOML files. |
| Policy tiering | [`packages/core/src/policy/config.ts` tiers](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/policy/config.ts#L61-L75), `522fa65371cf269bea9bce5fe558de9757db698c542f5da84b98869ce8423ee9`; [settings exclusion and allow priorities](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/policy/config.ts#L76-L78); [`tools.allowed` rule generation](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/policy/config.ts#L519-L523); [`toml-loader.ts`](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/policy/toml-loader.ts#L35-L65), `605e9e5031daa0f1b57a18f72e01edbf6cff6e2395e499a724805ca11ef8fda6` | Default → extension → workspace → user → admin. TOML priorities are `0..999` within each tier. `tools.allowed` creates a user-tier allow at `4.3`, which outranks Plan's default `1.040` catch-all deny and can allow `write_file`; `tools.exclude` creates a user-tier deny at `4.4`. User TOML can reach `4.999`, and admin rules use a higher tier, so none of these policy exclusions is a non-overridable guard. |
| Plan defaults | [`plan.toml` transition and catch-all](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/policy/policies/plan.toml#L30-L76), `cbc98437f7a0c3df44465e853863c1016d66d725da2b9a223946e802ffd2b777`; [default `.md` plan-file writes](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/policy/policies/plan.toml#L118-L195); [`read-only.toml`](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/policy/policies/read-only.toml#L28-L55), `4899746262c3beedce94ca90fdcbb50028e52232747288e5bff73c523db05bfb` | In noninteractive Plan mode, the default policy allows `exit_plan_mode` and `write_file`/`replace` at effective priority `1.070`; the latter apply only to its bounded `.md` plan-file path patterns, including designated `.gemini/tmp/.../plans` and clean relative `plan.md` or `plans/...` forms. The catch-all deny is `1.040` and the read-only list is `1.050`. That list also names topic, task, and tracker tools, so its label alone is not a filesystem read allowlist. |
| Decision and transition | [`policy-engine.ts` sorted first-match evaluation](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/policy/policy-engine.ts#L662-L686), `3b7118ceadaa36589f64def7669a503d64bd805fbff696077f5f7929361dfad2`; [dynamic rule re-sort](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/policy/policy-engine.ts#L937-L940); [noninteractive ASK_USER conversion](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/policy/policy-engine.ts#L921-L923); [`exit-plan-mode.ts`](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/tools/exit-plan-mode.ts#L261-L264), `1e1589584223a824c817a24be784cf84922fc67785c544ec99e10dfe1e889435` | The engine sorts rules by priority and uses the first matching rule. Noninteractive mode converts `ASK_USER` to `DENY` but leaves an explicit `ALLOW` intact. An allowed noninteractive exit selects YOLO through `getAllowApprovalMode()` and updates the active mode. |
| Pre-effect hook | [`scheduler.ts` hook check](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/scheduler/scheduler.ts#L590-L653), `79d3ceaceac098d96bae070c97fbd48e566d5c425cfadec5e1c9bc294ee3c629`; [later executor phase](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/scheduler/scheduler.ts#L710-L735); [`hook-utils.ts`](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/scheduler/hook-utils.ts#L25-L65), `51fcfdf0e891bd2372e993e5792011a7781a3d51de7c546cd5fdaa148d81c9dc` | The scheduler evaluates `BeforeTool` and returns its policy-violation result before policy, confirmation, or tool execution. The hook aggregator gives any `block` or `deny` decision precedence over `ask` or `allow`; see [`hookAggregator.ts`](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/hooks/hookAggregator.ts#L104-L141), `c5c16d0c17243e70a0e79f2b917cab70a399142ed77fe549a989b0cbe47713d7`. |

The selected route's [`--approval-mode plan` falls back to `default` when Plan is disabled](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/cli/src/config/config.ts#L681-L729).
Gemini also [forces `default` for an untrusted folder](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/cli/src/config/config.ts#L758-L764);
the selected [`--skip-trust` option](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/cli/src/config/config.ts#L308-L312)
addresses that fallback. The future adapter must still fail before the prompt
unless effective mode and tool registry match the prepared Plan profile.
Stream-json emits `tool_use` before the scheduler; observing that event or
cancelling the process is not a pre-effect control.

### Enforcement design

Gemini ships an internal, in-memory `BeforeTool` seam that can block the call
before its effect. `HookRegistry.registerHook` can register a `Runtime` hook
before initialization, and initialization retains runtime hooks alongside
configuration hooks; see [`hookRegistry.ts`](https://github.com/google-gemini/gemini-cli/blob/573846625af9e93b3b968e0e0b86bb093a4c9b16/packages/core/src/hooks/hookRegistry.ts#L39-L75),
`1e79ac1bb97cf1144ace5aa449cd06d400543f4292e97eeead4d6e0618e47614`. A runtime
block is not defeated by a higher-priority policy file or another hook's
allow. It does not require a settings file, extension, auth-home change, or
provider prompt.

The selected Swallowtail path launches the external CLI and has no supported
argument or IPC channel to register that runtime hook. A hook configured in
user, workspace, or system settings is persistent consumer configuration,
not an adapter-owned per-run control. Enabling an extension would change the
selected `--extensions none` invocation. `--policy` and `--admin-policy` can
replace existing consumer-selected policy paths or be suppressed by system
policy, so they are not acceptable substitutes.

The candidate enforcement contract is a provider-supported, process-local
pre-tool guard that registers an adapter-owned runtime hook in the actual
Gemini scheduler before initialization. Its prepared policy binds to the
selected Plan tool registry: always block `exit_plan_mode`; permit the exact
pre-transition tools and resource rules already approved by the prepared
profile; block every other or unknown tool before effect. Preserve Gemini's
default `write_file` and `replace` allowances only for the frozen bounded
`.md` plan-file argument patterns described above; the guard must validate
arguments as well as tool names. A user/admin allow for `write_file` cannot
expand that set. Keep provider path checks, `.gemini` write safety, output
bounds, selected cwd, environment and authentication binding. Do not use the
hook to change settings, home, provider flags outside the prepared profile,
or process containment claims.

No shipped CLI interface currently connects the external-process adapter to
that internal hook. The precise unavailable edge is process-local hook
injection while retaining all loaded consumer policy and settings. The
concrete next adaptation is a separately approved typed adapter-to-host
capability for Gemini runtime hook registration: the host must construct the
hook in memory before Gemini initialization, bind it to the prepared Plan
profile, and fail startup before the prompt if the selected runtime cannot
register or confirm the guard. This requires a supported Gemini injection
surface or a reviewed in-process integration; it is not implementable by
adding a CLI policy file or watching stream events. Do not patch or fork the
vendor runtime. Until that capability exists, consumers get no new guarantee
that the external CLI cannot transition to YOLO, and the adapter must not
describe the prepared Plan route as enforcing this guard. Any decision to
disable or narrow that route is a separate consumer-visible ruling. The
versioned strict serialization described below is only for the future typed
host boundary; it is not a mechanism the current CLI accepts.

### Consumer, preflight, and lifecycle contract

- A prepared Plan read within the selected approved resources continues
  through Gemini's defensive real-path checks and existing bounded-output
  behavior.
- Gemini's built-in `write_file` and `replace` allowances for the frozen
  bounded `.md` plan-file patterns remain in the prepared allow-set. They
  include designated `.gemini/tmp/.../plans` paths and the documented clean
  relative `plan.md` and `plans/...` forms; these rules do not authorize other
  writes.
- A model request for `exit_plan_mode` is blocked before confirmation and
  tool execution. Approval mode stays Plan; no `ASK_USER` callback, fallback
  allow, YOLO transition, or policy persistence occurs.
- Any `write_file` or `replace` call outside those exact `.md` argument
  patterns, any shell or other edit tool, an unknown tool, or an alternative
  transition is blocked before execution, even when a user or admin policy
  would otherwise allow it. An unrecognized effective tool set is a
  preflight failure, not a silent tool removal.
- Missing, disabled, malformed, or mismatched guard state fails before the
  model prompt. Cancellation and denial must leave the fake effect log empty,
  then join process and stream cleanup. Do not infer prevention from a
  cancellation after execution.
- The hook returns Gemini's normal policy-violation tool result. The adapter
  keeps its current tool-event, usage, terminal, and error projection; it
  adds no approval callback or stream field. Prove exact stream and terminal
  outcomes with fakes before any claim changes.

The proposed launch serialization must be versioned and strict, carry the
prepared tool names, authority mode, and named argument constraints for the
approved `.md` plan-file forms, and contain no prompt, concrete path,
credential, or environment value. Unknown schema versions and fields fail
before the prompt. The guard remains in process memory and is not written to
provider settings. No integration or API is approved by this design.

### Proof and release boundary

The separate implementation brief should prove, with fake-first tests:

1. baseline and user/workspace/admin allow overlays cannot run
   `exit_plan_mode`, arbitrary writes, or any other non-allowlisted tool;
2. default bounded Plan reads and the frozen `.md` plan-file writes still
   work, while other `write_file`/`replace` arguments are denied; defensive
   paths and truthful truncation/collapse semantics are preserved;
3. deny, cancellation, malformed guard, failed startup, and unknown tool
   cases produce no fake tool effects, no `ASK_USER` bypass, no mode change,
   and joined cleanup;
4. serialized events contain no new fields, parameters, or result bodies,
   and existing adapter projection remains exact.

No whole-package suite, old fake, artifact inventory, or provider proof is
repeated here. Preserve the `0.51.0..=0.61.0` headless claim, its exclusions,
and Gemini ACP independence. Under [Contract 036](036-crate-release-and-compatibility-boundary.md),
a private guard that preserves the documented Plan contract and existing
consumer event/lifecycle surface may be patch-compatible by behavior; a new
public API, changed terminal or permission lifecycle, serialized contract
change, or newly narrowed consumer guarantee is a pre-1.0 minor change. The
currentness sweep remains assigned to the minor release; this design grants
no qualification or release authority.

This is documentation and offline source analysis only: no runtime/API
implementation, artifact execution, live/provider work, credentials, install
or configuration/host mutation, qualification or tag authority. Preserve the
current `0.61.0` ceiling, older qualified points and exclusions. Gemini ACP is
independent; its evidence and authority do not settle headless enforcement.

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

## DeepSeek SDK JSONRPC Distinct-Route Design

Tom's 2026-10-09 board answer to decision
`e9216ef3-b355-4423-8f39-6cbce0104ca9` is “Approve distinct-route
contract/design study”. This accepts Research 412's distinct-route
classification and authorizes a bounded offline contract/design study for a
separate SDK JSONRPC route. Preserve the existing exact `0.1.0rc6` route,
consumers, claims, baselines and exclusions. A proposed route is not production.

The proposal is `deepseek-harness.sdk-jsonrpc`. It is a design label only:
there is no production route, claim, registry entry, or consumer API under this
name yet. It must not be substituted for `deepseek-harness.jsonrpc` or
`deepseek-harness.local-server`.

### Route and artifact identities

Keep four identities separate:

| Identity | Proposed value or rule | Boundary |
| --- | --- | --- |
| Route | `deepseek-harness.sdk-jsonrpc` | New route family. No existing route or consumer projection is renamed. |
| Runtime carrier axis | `deepseek-harness.sdk-runtime-bin` | Exact platform wheel and selected native payload. This does not identify the SDK profile. |
| SDK profile axis | `deepseek-harness.sdk-profile` | Exact complete CLI/profile/server/protocol package closure and its bundled configuration. This does not identify the native runtime that reaches it. |
| Behavior and protocol revisions | New route-local revisions, distinct from `deepseek-harness.jsonrpc-rc6-1` and `deepseek-harness.sdk-jsonrpc-v1` | Assign only when an implementation brief defines the public serialization and behavior. Never reuse the old facade revision. |

The current candidate observations are exact but unqualified: runtime wheel
`deepseek-harness-runtime-bin==0.1.5rc1`, macOS arm64, wheel SHA-256
`65f20c541a499f08c36ac3bfdafce350ca8c7cc23488c1faf4d9b9524b547156`, selected
payload SHA-256
`6f98dfe1745f6c952c6157dfade5ac2c6a8aeb672291b2c0703b21adf11cf88c`; and npm
`@deepseek-ai/dsh` `0.2.0-rc.2` with the six selected profile packages recorded
in Research 412. Neither identity is a claim. Research 412's package inventory,
runtime-bin inventory, and hop ledger are retained evidence and must be reused;
do not repeat their discovery.

The profile ledger is a closure, not a list of the six anchor packages alone.
It must freeze every direct and transitive package, every platform-selected
package, the package-manager resolution and integrity edges, and all profile,
server, protocol, and base-composition resources that affect dispatch. For
each package, retain its exact name, version, registry tarball identity and
integrity, byte count, and every unpacked member's path, size, and SHA-256.
Retain the closure manifest digest and the profile configuration digests.
Research 412's six exact npm tarballs remain anchors in this expanded ledger;
they are not evidence that the unrecorded transitive closure is complete.

Keep one exact-point evidence ledger for each axis. An observation row records
the official artifact identity, platform where relevant, artifact and
selected-member digests, immutable source or registry location, observation
date, and the selected-path change from its predecessor. A separate hole row
records the exact version or missing artifact query, axis, platform, observed
digest when present, reason it is not qualified, evidence link, and next finite
proof or decision. A hole is evidence bookkeeping, not a Contract 029 claim
classification or a supported segment. An unobserved version is not a point.
Reuse Research 412's entries for `0.1.0rc7`, `0.1.1rc1`, `0.1.2a3`,
`0.1.2rc1`, and `0.1.5rc1`, including the `0.1.2a3` package reset and the
absent PyPI `0.2.0rc2` query. Do not infer a segment across them. The old exact
`0.1.0rc6` point stays solely on `deepseek-harness.runtime-bin` and remains
unchanged.

Contract 029 applies independently to each axis: exact identity precedes a
point, each behavior revision owns its own support segments, and no segment or
maintained window follows from a vendor version label. The route's dispatch
claim is a relation over an exact runtime artifact, an exact complete profile
closure, target platform, provenance statement, and behavior revision. An
axis point alone does not make that relation qualified. No route claim exists
while provenance is absent. Once a claim is reviewed, Contract 029 classifies
an exact attempt as qualified, unverified newer only where that claim permits
it, or incompatible. A hole stays outside every qualified segment and must be
represented by the exact unsupported gap or exclusion required by Contract
029; do not invent an `evidence_pending` classification. Do not give it credit
from the old route, the local-server route, `latest`, matching versions, or
matching JSON-RPC names.

### Finite vendor-provenance gate

Before any tuple claim, one immutable vendor build attestation or equivalent
reproducible build manifest must bind all of the following in a single
verifiable record:

1. The exact platform runtime wheel digest and the selected native payload
   digest, path, and target architecture.
2. The exact complete SDK profile closure-manifest digest, including the
   locked package identities, integrity edges, profile/server resources, and
   configuration digests defined above.
3. The immutable source/build identity and build output relationship that
   connects that native payload to that closure. A repository tag, package
   name, version string, release note, marker such as `0.0.0-dev`, or an
   unbound source archive is insufficient.
4. A vendor signature or independently reproducible verification procedure
   for the record, with every input digest needed to repeat the check.

The gate is binary. Accept only if all four fields verify against the frozen
artifacts; otherwise retain the exact `evidence_pending` hole and request the
missing vendor record. Do not replace it with another version-number
comparison or repeat the already-completed inventories. Passing this
provenance gate establishes artifact identity only; it does not establish
runtime behavior, a first claim, a maintained interval, or Contract 036
compatibility.

### Preparation and preflight boundary

A future public API should use a distinct prepared type for this route, not
extend the existing `DeepSeekHarnessPreparationInput` or reuse the old
prepared-integration type. Its immutable preparation binding contains:

- configured instance and revision, execution host, and host principal;
- one exact runtime target and digest plus one complete profile-closure digest
  and the verified provenance-record digest;
- the selected provider and model, each chosen explicitly by the consumer and
  fixed process-wide for the prepared instance;
- an explicit cwd working-resource binding and an explicit host environment
  allowlist; and
- an explicit session-state binding and credential policy. Neither can be
  inferred from the process account or current directory.

Preflight verifies both artifact bindings, the provenance record, route and
protocol revisions, host/principal, provider/model, cwd, environment, and
state-resource ownership before process creation. It resolves the executable
from the exact prepared target, never from `PATH`, and passes one explicit
profile invocation. The final argv must be established against the frozen
original artifacts; the npm CLI's observed `dsh --profile sdk` expansion is
not proof that the PyPI runtime accepts that invocation. Preparation rejects
missing or mismatched artifact identities, provenance, cwd, environment,
principal, profile, provider, model, or state binding before any child starts.

Configuration authority is deliberately narrow. The invocation uses the
selected immutable SDK profile and only explicitly supplied configuration.
It does not import ambient home settings, user or project `.env` files,
ambient `$DSH_HOME/.credentials.yaml`, `$DSH_HOME/cordis.patch.yml`, arbitrary
`--patch` arguments, or inherited provider/model defaults. A typed host
credential capability may later use a short-lived route-owned handoff if the
frozen SDK requires that file; this is an explicit secret path, not ambient
home access. The process environment is an allowlist with a route-owned
`DSH_HOME`; a caller may add a setting only through a typed preparation input
recorded in preflight. Because the published profile statically admits those
home, dotenv, and overlay sources, a future implementation must disable them
or prove an equivalent host-enforced read boundary. If neither is available,
preparation fails closed. Cwd remains the explicit working resource; changing
cwd to hide dotenv files is not a substitute for controlling configuration
reads.

The selected npm protocol surface includes `initialize`, `session/prompt`,
session and agent notifications, and server `shutdown`. Static source has no
per-prompt result, per-session close, or per-prompt cancel method. Original
runtime reachability and these methods' behavior remain unproved until the
separately authorized artifact pass.

### Operation and authority matrix

| Operation or boundary | Proposed consumer contract | Reuse from the selected old route | Required adaptation or proof |
| --- | --- | --- | --- |
| Prepare and discover | Prepare one explicit runtime/profile tuple for one configured host principal. Report both axes and both digests. | Reuse typed target selection, host identity, deadline-bound discovery, access evidence, preflight-bound services, bounded diagnostics, and digest checks as design patterns. | Bind the independent profile closure and provenance digest. No old route binding or protocol facade may stand in for them. |
| Initialize | Initialize one SDK process with explicit cwd, provider, model, reasoning and token policy. These values are immutable for every session in that process. | Reuse explicit provider/model selection from the old prepared operation. | Adapt the SDK's process-wide selection. Reject changes after initialize and reject an unknown provider before accepting the handshake. Do not mount `deepseek-official` as an automatic fallback; an explicitly selected, supported provider is not a fallback. |
| Profile and configuration | Select the frozen SDK profile by one explicit invocation; accept no ambient profile, home, or launch overlay. Keep stdout exclusively for bounded JSON-RPC frames and send diagnostics to stderr. | Reuse host-owned environment references and immutable plan validation. | Disable profile/HMR reload, home `cordis.patch.yml`, CLI `--patch`, user/project dotenv, and inherited defaults, or prove host read denial. Prove stdio ownership and that no startup banner or SDK log enters stdout. The npm launcher accepting stdio after profile validation is a candidate mechanism, not proof of exact runtime reachability. |
| Credentials | Default to no credential. When a consumer explicitly selects a host credential capability, bind its provider audience and principal and expose only a redacted reference in plans, events, errors, and diagnostics. | Reuse the principle that opaque credential references and leases stay host-owned; the old route itself opens no credential lease. | Adapt SDK credential lookup so it cannot search a user home or dotenv. If the frozen SDK requires `.credentials.yaml`, use only an isolated route-owned home and a host broker's short-lived, scoped secret handoff; prove file creation, access, zeroization/deletion, and cleanup. If this cannot be enforced, authenticated preparation is unavailable. Never copy a host credential file. |
| Session creation and identity | Bind one opaque consumer session identity to one host principal, provider/model, cwd working resource, and session-state resource. Reject reuse across any changed binding. | Reuse existing typed session identity and host-bound resource patterns. | The SDK creates or reuses agents by caller session ID and persists JSONL state. Prove collision resistance, exact identity lookup, cross-session isolation, and restart behavior before advertising resume. A session ID or enqueue receipt is not completion. |
| Prompt queue and idle | Permit at most one uncompleted prompt per session. Return an accepted-message token separately from terminal completion. Complete only after the accepted message's session reaches a proven idle boundary; preserve event order and bound event count/size. | Reuse the old route's bounded framing, selected notification decoding, terminal outcome, and exact idle fold as implementation patterns only. | The SDK accepts multiple queued prompts and has no per-prompt result. Prove message/event correlation and idle cardinality. Reject a second prompt while one is pending; reject ambiguous, stale, foreign-session, or missing idle evidence. No idle inference from `{messageId}`. |
| Persistence and retention | Require `Ephemeral` or an explicit caller-owned `SessionStateRef`; disclose that the SDK writes durable session records and a projection cache. State is scoped to the bound principal and integration. | Reuse host-owned state/resource leases and bounded lifecycle evidence. | The future route must bind the SDK's `$DSH_HOME/sessions` and projection cache to the selected state resource, make retention explicit, and prove no writes outside it. Do not claim restoration until restart proof passes. Process shutdown disposes agents but does not itself erase durable session files. |
| Images | No image input in the first public route revision; reject image blocks in request validation before attachment writes. | Reuse typed content validation and bounded rejection patterns. | The SDK admits images through an attachment store. Add image capability only after file admission, size/type limits, attachment path ownership, cleanup, and provider-visible effects have their own contract and proof. |
| Tools and permission | First route revision exposes no filesystem or shell tools and no consumer permission callback. Reject unexpected tool or permission requests, cancel the process, and verify no effect. | Reuse the old route's host-selected access profile, typed process ownership, and non-secret diagnostics. | The SDK composition includes read/write/edit, shell, and bundled services with application-level approval modes. Override it to no tools or a strictly host-enforced, separately reviewed set. Never equate `ask`, `workspace-write`, or `danger-full-access` with an OS sandbox. No permission grant or shell composition is implicit. |
| Cancel, stop, shutdown, join | Cancellation ends the entire SDK process because the protocol has no per-prompt cancel or per-session close. First request orderly SDK shutdown when the exact method is accepted; otherwise close stdin, stop the owned process tree by deadline, escalate, and join all owned work. | Reuse exact child ownership, cancellation signaling, host deadline, forced stop, and joined cleanup from the existing route. | The old route's cleanup does not prove SDK subprocess or agent cleanup. Prove graceful `shutdown`, stdin EOF, SIGINT/SIGTERM, forced process-tree stop, reaping, and no descendants/leases after join. Mark an interrupted durable session incomplete; do not report it successful or silently replay its prompt. |
| Output and diagnostics | Expose only bounded, route-versioned results and typed failures. Redact credentials, session contents, home/workspace paths, raw SDK errors, and tool payloads. | Reuse bounded error projection, frame limits, unknown-event posture, and terminal cleanup reporting. | Freeze SDK notification/result mapping and serialization in a new facade. Raw SDK event names or JSON-RPC shapes do not become public API by pass-through. |

The selected source patterns above are design inputs, not inherited proof.
The old route's explicit target and Cordis binding, provider/model selection,
bounded JSON-RPC parser, single-operation idle fold, owned child, deadline,
forced stop, join, and redacted output can preserve their authority boundaries
when reimplemented against this route. They do not preserve SDK-added
process-wide defaults, arbitrary configuration layers, home credentials,
multi-session persistence, image files, or permissioned tools. Each added
boundary needs the adaptation and proof named in the matrix.

### Consumer examples and negative cases

The following is schematic public API, not current Rust code. Every name is a
design placeholder until an implementation brief and API review.

```rust
let prepared = prepare_sdk_jsonrpc(SdkJsonRpcPreparationInput {
    runtime: exact_runtime_target(runtime_digest),
    profile: complete_sdk_profile(closure_digest, provenance_digest),
    host: execution_host,
    principal: host_principal,
    cwd: approved_working_resource,
    environment: explicit_environment_allowlist,
    provider: chosen_provider,
    model: chosen_model,
    credentials: CredentialPolicy::None,
    session_state: SessionState::Ephemeral,
    tools: ToolPolicy::None,
}).await?;

let mut session = prepared.open_session(consumer_session_id).await?;
let accepted = session.enqueue_text(prompt).await?;
// Acceptance is only queue acknowledgement; it is not a completed turn.
let terminal = session.wait_for_idle(accepted, deadline).await?;
prepared.close_and_join().await?; // Stops the process and all SDK-created agents.
```

The API must reject these cases before effects, or stop and join after an
unexpected runtime effect:

- `runtime=0.1.5rc1` and `profile=0.2.0-rc.2` with no verified provenance
  statement, even when both strings appear in one configuration;
- selecting `dsh sdk` or inheriting a user's `DSH_HOME`, dotenv, credential,
  profile, or Cordis overlay instead of the prepared target and allowlist;
- changing provider or model after initialize, requesting an unregistered
  provider, or reaching `deepseek-official` because another provider was
  missing;
- sending a second prompt before the prior accepted message reaches proven
  idle, treating its message ID as final output, or accepting idle from another
  session;
- supplying image content, mounting a shell/write/edit service, approving an
  unexpected permission request, or allowing tool access beyond the prepared
  host resource; and
- reusing a durable session under another principal, provider/model, cwd, or
  state resource, or describing process shutdown as deletion of persistent
  session records.

### Fake-first, recorded proof design

No proof is run under this design task. A later proof brief should stage the
work and label fake observations separately from original-artifact
observations:

1. **Harness-only fake pass.** Exercise the proposed adapter supervisor with a
   fake JSON-RPC child and synthetic process tree. Prove target/provenance
   mismatch rejection; initialization binding; acknowledgement versus idle;
   event/session correlation; queue limits; credential redaction; no-tool and
   no-image policy; cancellation; stop escalation; join; and cleanup ordering.
   This establishes the harness behavior only. It must not claim that a vendor
   executable or profile is reachable.
2. **Frozen original-artifact reachability.** Only after the provenance record
   and a separate exact-artifact execution ruling exist, execute the frozen
   original carrier and profile in an isolated, provider-free environment.
   Record the exact wheel, payload, package-closure, invocation, and platform
   digests. Prove the selected entrypoint starts that payload, selects the SDK
   profile, and reaches JSON-RPC initialize and one queued prompt against a
   loopback fake model/tool boundary. Label every result `original_artifact`;
   fixture success cannot substitute for this stage.
3. **Selected-boundary proof.** With the same separately authorized artifacts,
   exercise explicit provider/model and cwd bindings, config/credential
   sentinels, session state and restart, message/idle cardinality, image and
   tool rejection, graceful shutdown, signal/forced stop, process-tree reaping,
   and joined cleanup. Record allowed and denied filesystem operations,
   process identities and descendants, sockets, and credential-source reads.
   A fake boundary is labelled `fixture`; an observed vendor effect is labelled
   `original_artifact` and tied to its digest.

Every stage uses a fresh task-owned scratch root with no links to a checkout,
a default-deny network boundary allowing only an explicitly observed local
fake endpoint, no real credentials, and no provider endpoint. The proof records
filesystem reads/writes/deletes, spawned process tree and exit statuses,
network attempts, credential sentinels, time bounds, and cleanup outcome.
External network, unowned home paths, real credential stores, and tool effects
outside the scratch working resource are denied and counted as proof failures.
Preserve secret-free traces and exact artifact identities with the result.

### API, serialization, and release boundary

The eventual route adds a distinct prepared integration, preparation input,
preflight binding, session/request/result types, route projection, driver and
protocol facade. It must not mutate old route IDs, the old runtime-bin axis,
the `deepseek-harness.sdk-jsonrpc-v1` facade, old protocol fixtures, baselines,
or consumer types. Add new versioned serialization for the SDK route; do not
extend an existing serialized enum in a way that changes its meaning or makes
old decoders silently accept this lifecycle. Public Rust API additions receive
their own semantic API evidence. Review every public enum/struct addition for
exhaustive-match and serde compatibility under Contract 036 before choosing
the compatibility classification.

The route introduces new lifecycle, configuration, credential, persistence,
and tool-permission decisions. It therefore cannot be treated as an urgent
SDK patch correction. Implementation belongs in a separately reviewed
pre-1.0 minor candidate (the current `v0.6.0` planning target remains
provisional), with a distinct route/API baseline and release note. Contract
036 assessment must decide the public API and serialized-shape compatibility
of the actual implementation; a design proposal cannot pre-approve that
classification or a tag.

### Separate follow-up scopes

This design task completes only the proposal. The next review should split
authority into finite work:

1. **Contract/API implementation brief:** add a new route and prepared/session
   surface using the boundaries above, after a separate ruling on any remaining
   public credential, durable-state, or permission choices. No vendor artifact
   execution or qualification is implied.
2. **Provenance and original-artifact proof brief:** request and verify the
   single vendor provenance record; then, only under separate execution
   authority, run the staged provider-free proof against its exact artifacts.
   If the vendor cannot satisfy the finite gate, preserve the exact hole and
   return the source/build correlation blocker; do not infer provenance.
3. **First-claim brief:** after implementation and proof, decide exact
   platform-specific points, behavior revision, Contract 029 segment/hole
   records, and any maintained interval from the evidence. Contract 036
   release compatibility is reviewed separately.

No runtime/API implementation, artifact execution, real credentials/login,
provider/live work, installation, host mutation, qualification or tag is
included here. Implementation, original-artifact proof, and first claimed
points require separate review and authority. DeepSeek local-server
browser-session design remains independent and supplies no SDK-route evidence
or authority.

## DeepSeek Local-Server Browser-Session Design Study

Tom's 2026-10-09 board answer to decision
`a0118612-e9e0-4eec-ac06-491cb815e6a0` is “Approve bounded offline design
study; review adaptation afterward”. This authorizes a separately reviewed
provider-free browser-session and typed-stream design study for
`deepseek-harness.local-server`, using the accepted 51-artifact identity
inventory in [Research 411](../../research/411-deepseek-harness-web-rc2-authentication-stop.md).
Reuse that evidence rather than repeating completed discovery.

Map exact shipped credential-record creation/read/write/delete/rotation,
cookie scope, audience, principal and process/browser effects. Map typed
controller/request/event flows and cancellation, stop, join, drain and cleanup
against the existing selected transport and method boundary. Separate known
source/control flow from missing owner or runtime evidence.

[Research 427](../../research/427-deepseek-harness-web-browser-session-typed-stream-boundary.md)
records the source-ledger review and proposed boundary. The selected `0.2.0-rc.2`
path creates or reads a persistent DSH browser-signing record, and its current
opaque environment and operation-scoped lease types cannot express authority
over that record or bind it to a host principal and profile. The typed Remote
controller and stream also have distinct request, event, permission, and
cancellation lifecycles from the existing 11-method facade. The proposal
therefore requires a separately versioned, explicit opt-in facade with
host-issued store/process authority; a private-only mapping is not established.
No target claim follows, and `0.2.0-rc.2` remains unqualified pending the
named owner evidence and separate implementation and qualification briefs.

Return an explicit opt-in access, lease and preflight design preserving old
unauthenticated callers and defaults, with no implicit credential acquisition,
browser launch or unauthenticated fallback. Define credential ownership,
expiration/redaction, host/principal/resource binding and failure before
effects. Classify a possible private mapping versus required public contract,
API/serialization, lifecycle and Contract 036 minor-release changes; do not
implement either classification under this design approval. Return precise
independent implementation and fake-first proof scopes for another ruling.

No vendor execution, credential access/creation/login, browser or server start,
provider/live work, installation, host mutation, runtime/API implementation,
claim increase or tag is authorized. Preserve the exact supported RC points
through `0.1.1-rc.2`; target `0.2.0-rc.2` remains unqualified. Structured-run
JSON-RPC artifact/profile design is independent and supplies no local-server
authority or qualification evidence.

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

## Copilot ACP Pre-Prompt Discovery

Tom's 2026-10-09 board answer to decision
`45b2b006-76a7-4c55-a1e7-445563473844` is “Approve bounded pre-prompt
discovery”. This authorizes a separately reviewed discovery proof against
exact Darwin ARM64 Copilot CLI `1.0.80`, `1.0.81` and `1.0.93`, using
`betterthanclay`'s existing login, to establish the selected ACP authentication
and model/configuration surface. It does not authorize a model prompt.

Prove containment and operation filtering against fakes, then persist the
exact original identity, approved authentication read paths/keychain service,
network destinations, operations, budgets and cleanup plan before execution.
Only narrowly listed vendor-owned authentication/configuration reads and
authentication/model-discovery network traffic are allowed. Deny unrelated
host-home/repository reads, unlisted destinations and all writes outside task
scratch. Never extract, copy or log secrets, switch accounts, log in again,
relocate home/authentication state or persist configuration changes. Stop
before the original if that narrow boundary cannot be established; access to
the whole home is not an alternative.

Allow initialize, authenticate when required, session/new, supported model or
configuration discovery and joined close. Start with `1.0.93`; only proceed
to older controls if its gates are safe. Each exact version has one invocation
of at most 60 seconds, with no retry or resend. No session/prompt, model
generation, permission approval or tool action is allowed. Record secret-free
protocol, endpoint, model, account-access and cleanup observations. Auto does
not establish a fixed underlying model, and local Gemma is not a substitute
provider path. If authentication, containment or observation fails, preserve
a finite evidence stop and the precise next proof.

Discovery does not consume the separately bounded three permission-proof
prompt attempts, establish permission/no-effect behavior, qualify a route or
authorize its later prompt continuation. Preserve the current exact `1.0.80`
claim until those independent gates are closed.

[Research 426](../../research/426-copilot-acp-pre-prompt-discovery-stop.md)
records the fake-only pre-prompt controls and a finite stop before original
start. The frozen artifacts' exact Keychain item, endpoint set, and prompt-free
model catalogue remain unproved; no authenticated observation or claim change
follows.

## Copilot ACP Host-Login And Auto Proof Boundary

Tom's 2026-10-09 board answer to decision
`8b098b85-fb9e-4543-b5f1-1fbfe8bc5301` is “Approve explicit host-login/Auto
proof limits”. This revises the execution prerequisites for the bounded
original-artifact permission proof only. Research 425 and 426 establish fake
preparation and the pre-start discovery stop, not original permission evidence.

Use `betterthanclay`'s existing host-owned CLI login and Auto as the requested
selection policy. Trust only the hash-verified original vendor CLI to read its
normal existing authentication state. macOS `securityd` access is not an
item-level Keychain filter; do not claim per-item mediation or a credential
broker the original does not support. The agent and harness must never
extract, copy or log tokens, invoke token-extraction commands, log in, switch
accounts, export authentication state, relocate home or change settings.

A reviewed official-destination allowlist defines network authority, rather
than asserting that every frozen-version endpoint is known in advance.
Enforce default-deny outside it and require original egress through the proven
gate. Unlisted destinations fail the attempt; they never broaden the policy.
Deny unrelated repository/home reads, writes outside task scratch and
subprocess escape. If those boundaries cannot be enforced with the original,
stop rather than claim containment or substitute patched vendor code.

Auto is a dynamic selection policy, not a fixed underlying model. Record the
actual model if exposed and otherwise report it unobserved; this proves no
model compatibility. For this proof, accept vendor-managed selection/retries
within one ACP prompt and an enforced 60-second process/network ceiling.
Internal provider request counts are not claimed known or limited to one.
This supersedes the earlier proof's pre-start fixed-model identity and
zero-fallback/internal-retry prerequisites; harness retries/resends remain
forbidden. Unsupported Auto binding stops that version before a prompt.

Use the same unused allowance: at most one original invocation and one ACP
prompt for each exact `1.0.80`, `1.0.81` and `1.0.93`, three total across all
proof tasks. Start with `1.0.93`; unsafe containment, authentication or
selection stops older attempts. Fakes must prove the selected containment,
permission and cleanup path first, with an fsynced execution record before
each original. Reject/cancel the benign scratch action, never approve it,
and verify no effects and bounded joined cleanup. Missing permission,
ambiguous cancellation, effects or surviving descendants fail the proof.
No additional reviewer live attempt is allowed.

No token extraction, new login, settings/home mutation, installation, consumer
change, automatic qualification or tag is authorized. Preserve the exact
`1.0.80` claim until original permission/cancel/no-effect evidence is reviewed
and any changed mapping boundary is separately ruled. Account quota or
entitlement failure is a finite stop, not permission to change account or plan.

Research 428 records one approved original `1.0.93` launcher command under
these boundaries. The `sandbox-exec` process exited with code 1 before ACP
`initialize`; the record does not establish whether the vendor binary reached
startup. No egress-proxy destination, ACP authentication request, session,
prompt, permission, model identity or effect was observed. Its one-invocation
allowance is consumed, so the older versions were not started and must not be
treated as covered. Fake permission controls pass, but do not establish the
exact original sandbox profile or its host-home read set. The original profile
denied all host-home reads, and no fake admitted a vendor-owned authentication
metadata read. This stricter denial could have blocked startup; the cause is
unknown because stderr was discarded and vendor-binary startup was not
observed. Permission behavior remains unproved and no route claim changes. A
renewed `1.0.93` attempt requires separate authority. The proof harness now
checks the SHA-256-pinned committed execution record before staging artifacts
or starting any original, refuses every version recorded as consumed, and
fails closed if that record is missing or changed. The guard applies to both
`--permission-proof` and the legacy `--execute` path. The fake self-test
verifies that legacy execution refuses before staging. Renewed execution
therefore requires a reviewed guard and record update under that separate
authority.

## Copilot Permission-Proof Startup Diagnostics

Research 429 repairs only the fake controls and future proof harness for
Copilot ACP. The original `1.0.93` launcher result from Research 428 remains
unknown: stderr was discarded, and launcher exit does not establish whether
the vendor binary reached initialization. The invocation remains consumed.

The fake and any future original use the same verified sandbox-profile
generator and correction-plan binding. The profile denies repository access,
home writes, and all home reads except the single read-only
`.copilot/config.json` path listed in the correction plan. Current GitHub CLI
documentation identifies that file as managed configuration including
authentication, but does not establish that frozen `1.0.93` reads it. No real
configuration file or keychain item was read. The fake substitutes a synthetic
file at that exact path and separately proves adjacent reads and writes are
denied. It adds no fake-only runtime root. Exact executable access,
`/System/Library` and `/usr/lib` mappings, and the existing `securityd`
service-lookup trust boundary remain explicit; keychain item mediation is not
claimed.

Proof harnesses that collect child stderr drain it concurrently, retain at
most 4096 bytes in memory, cap the reported total-byte count at 65536, and emit
only an allowlisted category, byte counts, truncation state, and reader status.
Raw stderr is never persisted or shown.
The launcher exit code and vendor startup status are separate: without an ACP
initialize response, original startup remains `unknown`. Fake process startup
evidence applies only to the fake. Neither this correction nor its fake
preflight renews an invocation, resets the consumed budget, or changes route
qualification. A future original attempt requires new exact authority and
must retain the bounded profile and diagnostic contract.

## Copilot ACP Corrected One-Shot Renewal

Tom's 2026-10-09 board answer to decision
`411be8ce-77a0-4a50-930f-d6aeacdffce9` is “Approve one corrected 1.0.93
attempt”. This grants exactly one additional original `1.0.93` launcher
invocation using the independently reviewed Research 429 correction. It does
not restore or reuse the consumed Research 428 invocation.

Bind the frozen Darwin ARM64 executable SHA-256
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`, the
reviewed correction-plan identity and the exact final harness/profile after
fake verification of any renewal delta. Keep `betterthanclay`'s host-owned
login, Auto policy, official-destination default-deny proxy and enforced
60-second process/network ceiling. The only added home read exception is
read-only `.copilot/config.json`, tested under the same native-fake profile;
its necessity for this exact binary remains unproved. No additional read,
egress, configuration or authentication boundary is granted.

Use a separate one-shot renewal authorization/attempt record bound to this
decision and exact identities. Atomically consume and fsync it before original
start; crash, timeout or an uncertain response cannot restore the allowance.
Preserve the immutable historical records and old consumed guards. Missing,
mismatched or used renewal authority refuses before staging/start. Prove this
final admission/record delta against fakes before the renewed original.

At most one ACP prompt may be sent, counted against the existing shared
three-prompt total. No older artifact, approval, resend, second attempt or
reviewer live spend is allowed. Persist the pre-execution record first, use
bounded allowlisted diagnostics and joined process/proxy/stderr cleanup, and
stop on unsupported selection, authentication, containment, permission or
cleanup failure. Startup remains unknown until original initialize is
observed. No token extraction, login, settings/home mutation, automatic
qualification or tag is authorized; parent qualification needs reviewed
original permission/cancel/no-effect evidence and any required mapping ruling.

Observed 2026-10-09 ([Research 430](../../research/430-copilot-acp-renewed-1-0-93-permission-proof-stop.md)):
operation `1c00be41-c012-4f12-bac8-87526f72d46b` consumed its one-shot
renewal attempt before launch. The single `1.0.93` invocation exited with
launcher code 1 after 1.289 seconds, before ACP `initialize`, with stderr
category `sandbox-denial`. No prompt, permission request, cancel, effect or
egress destination was observed, and vendor startup stays unknown. The
renewal is consumed; `1.0.81` and `1.0.80` were not started. The shared
three-prompt allowance is unchanged at three remaining. No route claim,
qualification, public API or consumer contract changed. Any further attempt
needs new exact authority after a reviewed diagnosis.

## Copilot ACP Offline Startup Repair And Study

Tom answered “Go for it” on 2026-10-09 to decision
`f3667571-27e6-4363-ba3a-14ef769fca91`, approving the bounded offline
repair/study in lead `63537795-747c-4592-a4d2-82095bd6570b`.

The work may reconstruct the Research 404/430 launch and effective-profile
differences, bind imported OS profile contents, and statically parse the
retained exact `1.0.93` artifact after checking its identity. It may repair
bounded diagnostic classification, proxy completeness and task-owned
descendant cleanup, and prove launcher phases and containment with fakes in
synthetic scratch state. Raw or normalized stderr hashes are excluded;
diagnostics retain only reviewed closed-vocabulary fields. Channel closure
alone does not prove successful exec, and process-group disappearance does
not prove descendant cleanup.

The historical `sandbox-denial` category establishes text-marker presence,
not a diagnosed sandbox operation. On the Research 431 audit host, the
`com.apple.securityd` launch-daemon label advertises Mach service
`com.apple.SecurityServer`; Research 430's profile allows the label string,
not that advertised service. Research 430 does not retain its OS build, so
this current declaration does not bind the historical run. The shipped
credential mechanism and startup cause remain unproved. Correcting the service
name does not grant access.

No original execution, artifact download, real home/config/keychain or
credential access, auth/provider traffic, additional home/service/network
grant, settings/home relocation, runtime API change, qualification or release
is authorized. Both consumed attempt records and replay guards remain
unchanged. The study returns exact startup requirements or explicit unknowns;
future vendor-state treatment, service grants and any original invocation
need separate authority after review. Completion does not resume or qualify
the retained parent, whose exact `1.0.80` claim remains unchanged.

[Research 431](../../research/431-copilot-acp-offline-startup-trace-stop.md)
reconfirms that Research 404 and 430 used the same `1.0.93` executable under
different argv, environment, home-path, and sandbox tuples. The 404 record
observes ACP `initialize`; 430 does not. The tuple changes do not isolate a
failure cause. The 430 record binds profile digest
`c00ccaadfae0c55fceffa1455952f40583a5ece13eddde64571e389828b3611a` but omits
the rendered profile and historical repository root, so its bytes cannot be
reproduced from retained inputs. The host build is also absent from the 430
record. The retained SEA payload does not expose the exact ACP startup path.

The permission harness blocks original-artifact entrypoints while
escaped-descendant containment is unproved. Process-group disappearance is
not full cleanup evidence. Both consumed attempt records and replay guards
remain unchanged. Any future original attempt needs a separate reviewed brief
and exact authority.

### Fake-Only Diagnostic And Cleanup Continuation

Task 165 adds an independent fake-only stage launcher. It reports launcher
entry and a bounded errno code on target-exec failure; close-on-exec EOF is a
separate boundary observation. EOF without the task-owned fake's explicit
startup marker remains `unknown`. Fixed stage, operation, path-role and errno
vocabularies describe compile, the controlled invalid-profile fixture, exec,
loader and native-main cases. Stderr collection remains volatile and bounded;
records contain marker facets, fixed templates, counts and joined-reader state,
never stderr text, paths, URLs, tokens, or stderr hashes. A single sandbox
marker denotes marker presence and has no inferred cause. Conflicting marker
facets remain `unknown`.

New fake-only records bind the current OS product version, build and
architecture and the imported `dyld-support.sb` closure by relative role,
size and digest. They also record a symbolic relative mirror of artifact,
record, action, scratch, synthetic-home and synthetic-repository roles. No
original artifact is staged in that mirror, and these values do not reproduce
Research 430's missing rendered profile or historical host identity. Synthetic
config probes cover read, write, parent metadata, missing-file, symlink,
repository escape, `..` and symlink home aliases. They record only fixed case
names and outcomes.

The proxy accounts for each accepted handler through completion, including
early close, timeout, cleanup cancellation and unfinished-handler rejection.
It retains no request contents, headers or arbitrary destinations. A forked
fake child leaves its parent's process group; the fake parent joins that known
direct child by PID with `waitpid`. This proves cleanup only for that controlled
lineage. General vendor-descendant enumeration and race-safe joining remain
unproved, so original-artifact admission stays fail-closed. The independent
fake launcher is not wired into an authorized original invocation; its exact
exec grant exists only in the task-owned fake profile. The effective original
profile receives no home, service, executable, network or credential authority
change. These controls do not change the exact `1.0.80` claim or either
consumed record.

### Credential-Free No-Child Diagnostic Preparation

Tom answered “Approve fake-only diagnostic preparation” to decision
`4d499c96-4046-4189-b14e-39b1a572e035` on 2026-10-09, approving lead
`63377791-fce4-4c8d-8e9e-44bee889d756`.

Preparation may use task-owned fakes and synthetic home, configuration, cache
and temporary state to prove a narrower initialization-only boundary. It must
demonstrate denial of `fork`, `vfork` and `posix_spawn`, including self-spawn,
helper execution and session escape, after the single owned launcher-to-target
exec transition. A deny-rule string or cooperative fake-child join is not
proof. Bind the current OS and imported-profile closure, replayable synthetic
layout, bounded diagnostics, one-shot records and owned-root stop/join.

[Research 432](../../research/432-copilot-acp-no-child-initialize-diagnostic.md)
records the fake-only result and its exact identities in the
[preparation record](../../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-no-child-diagnostic/preparation-record.json).
On macOS 26.6.2 build `25G83` arm64, the imported runtime-profile closure is
the single `dyld-support.sb` file recorded by role, size and digest. The
replayable profile denies `process-fork` after the import, grants `process-exec`
only to the owned stage launcher and exact target, denies Mach lookup and
network access, limits writes to the action root, and denies the current host
home, repository and record roots. Synthetic auth-config, keychain and
repository sentinels exercise those file denials; service probes cover
`com.apple.securityd` and `com.apple.SecurityServer`; a reserved fake address
and local bind exercise network denial.

The fake returns one ACP `initialize` response. Eight child-creation attempts
cover fork, vfork and posix_spawn with self-exec, system-helper, launcher and
session-escape cases; all return denied before an effect marker. A self-exec
replacement keeps the same root PID, an unlisted system helper exec is denied,
and an at-exit spawn is denied. The record also binds the timeout, cancel,
stop, direct-PID wait, stderr-reader join, unknown EOF and fsync-before-start
crash/replay controls. Its secret-bearing stderr control persists only fixed
marker facets and byte counts, with no raw text, hash, path or secret.

The proposed later diagnostic uses `--acp --stdio`, at most one initialize
request and a 60-second ceiling, with all external network and real
home/config/keychain/auth access denied. It does not use the host login or
Auto model policy and cannot establish their permission-proof compatibility.
If the no-child boundary cannot be enforced, return a concrete isolated
environment requirement; do not substitute process-group cleanup.

The record contains a data-only request with the candidate profile template,
the frozen Darwin ARM64 `1.0.93` archive and executable identities, synthetic
environment roles and a three-second cleanup bound. It sets
`execution_authorized: false`; the original executable was neither staged nor
run. Fake success establishes only the recorded profile boundary and fake
protocol behavior on that OS build. It does not show that Copilot reaches
initialize, needs no child, or works without host login, Auto model policy,
auth services or provider access. A later original attempt needs separate
exact authority and independent review after resolving those limits.

This ruling authorizes preparation and fake proof only. No original Copilot
invocation, additional credential/service access, qualification or release is
included. A later original needs separate exact authority after independent
review. Both consumed attempt records, existing replay guards and the exact
`1.0.80` claim remain unchanged; original admission stays fail-closed.

### One Isolated Original Initialize Attempt

Tom answered “Approve one isolated initialize attempt” to decision
`e05b517f-e4fc-46c4-9d52-435b2422b5cf` on 2026-10-09, approving lead
`646215a7-9eb1-4218-af35-23fba229b31a`.

Exactly one retained Darwin ARM64 Copilot `1.0.93` invocation is authorized,
binding executable SHA-256
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`
and the reviewed Research 432 preparation identities. Require macOS build
`25G83`, arm64, and the same imported-profile closure. Use only its synthetic
state environment, `--acp --stdio`, task action cwd and reviewed no-child
profile. All network, Mach lookup, real home/config/keychain/auth access and
child creation remain denied. No host login or Auto selection is included.

Fake-prove any faithful launch delta first and fsync a separate exclusive
consumed attempt record before original start. Bind exact harness, launcher,
profile, import closure, environment and artifact identities in that record.
Allow at most one initialize request, zero sessions or prompts, a 60-second
ceiling and three-second joined cleanup. No retry, resend, older artifact,
profile widening or reviewer original execution is authorized. A crash,
early failure or unsupported startup consumes the attempt. Existing records
and guards are not reset; the separate three-prompt allowance is unchanged.

Persist only bounded reviewed diagnostic fields; raw protocol/diagnostic
text, secret-bearing paths and raw or normalized stderr hashes stay excluded.
Stop on identity or containment drift, unsupported services/processes/auth,
or failed cleanup. The result is startup evidence for this isolated tuple,
not host-login/Auto permission evidence, qualification or authority to resume
the retained parent. The exact `1.0.80` claim and release identities remain
unchanged; any broader original or permission proof returns separately.

### Consumed Initialize Archive Exception

Tom answered “Approve” to decision
`f5a0b303-deee-40fd-be71-abf596f1cb0e` on 2026-10-09, accepting the single
consumed diagnostic as a limited archive-identity exception. The brief copied
Research 432's incorrect archive identity
`98640ca0de6576807f369c533c839b5742b038f105a970bdd7cb0d7efc8a7a71`, which
Research 404 identifies as the `1.0.80` archive. The executed `1.0.93`
executable matched the intended SHA-256
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1` and its
frozen-inventory archive
`f254651a3195e125b91d723c800e71e6541f8db3832d269854ae982254263eeb`.

This accepts the historical diagnostic under decision `e05b517f`; it does
not erase the failure to stop on the brief's identity mismatch. Preserve the
incorrect preparation binding, execution records, diagnostic script and
consumed allowance unchanged. Future identity mismatches still require a
stop and a separate ruling before execution.

The attempt did not reach initialization. It supplies only the recorded
startup failure with unknown cause; it establishes no authentication,
permission, cancellation or no-effect guarantee. The exact `1.0.80` claim
remains unchanged. This ruling grants no new invocation, qualification or
parent-task continuation.

[Research 433](../../research/433-copilot-acp-no-child-original-initialize-diagnostic.md)
records the one consumed original. Original-profile fakes passed the eight
child denials and the lifecycle, diagnostic, and crash/replay gates. The
exclusive attempt was fsynced before start. The original used `--acp --stdio`
and the reviewed synthetic environment on macOS 26.6.2 build `25G83` arm64.
Stage exec-boundary EOF was observed; ACP `initialize` was not reached;
`vendor_startup_status` is `unknown`; `failure_class` is `sandbox-denial`.
Stderr stored the `sandbox-denial` category, 997 bounded bytes, and the
`sandbox-marker-present` facet only. Cleanup joined.

The brief named archive
`98640ca0de6576807f369c533c839b5742b038f105a970bdd7cb0d7efc8a7a71`. The
executed bytes matched executable
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1` and archive
`f254651a3195e125b91d723c800e71e6541f8db3832d269854ae982254263eeb`. That
stop-rule breach is disclosed and not erased. The pairing is accepted for
this one historical diagnostic under the archive exception above.
[Q-007](../questions.md#q-007) is answered. Parent 119 stays blocked.
Fail-closed no-child policy may prevent vendor startup and is not a route
narrowing. Frozen records are not edited. Future identity mismatches still
stop. Further original, isolation, auth, or permission work needs separate
exact authority.


### Copilot Normal-Runtime Containment Preparation

On 2026-10-10, after the missing permission evidence and failed restrictive
startup probes were explained, Tom directed: “Do whatever you need to do to
get it done”. Proceed with a separately reviewed proof harness that contains
Copilot's normal runtime child processes and state writes rather than
requiring the provider to start under the failed no-child profile.

Prepare and exercise the replacement against task-owned fakes. Inspect
available disposable environments and implement bounded containment,
scratch state, egress control, stage diagnostics and joined cleanup. A new
isolated task-owned Linux container may be used for fake containment work;
existing VMs, accounts, host configuration and credential stores stay
unchanged. The stopped Parallels VM `macOS pristine`
(`ddf1f166-4796-4f40-925e-eaf35172c63f`) may serve as the source of a new
task-owned disposable clone, after fresh stopped-state and supported-clone
checks. Do not start or modify that source. Before starting the clone, deny
external networking and host sharing; change only the clone. Prove cleanup
and containment against fakes before any proposed vendor execution.
Establish a faithful environment for each claimed native payload;
Linux evidence does not establish Darwin compatibility. If the available
macOS host cannot enforce the required boundary, return a concrete disposable
macOS environment requirement rather than another unchanged launch.

The existing `betterthanclay` host login and Auto policy are the intended
access path. This preparation does not authorize secret extraction, credential
copying, new login or another original invocation. Produce a reviewable exact
execution plan with the required credential mechanism, normal state paths,
network audience, artifact identities and bounded permission attempts. The
three unused permission prompts and all consumed original records remain
unchanged. Bind later original execution to the reviewed concrete plan and
its separately recorded authority; preparation alone does not qualify or
resume the parent route. Preserve the exact `1.0.80` claim and released
consumers until permission, reject/cancel and no-effect evidence exists.


For this preparation, Tom's delegated direction also permits creating a new,
uniquely named task-owned Parallels host-only network. This is the narrow
exception to leaving host configuration unchanged: do not modify or reuse an
existing network's settings, host firewall, forwarding configuration, source
VM or unrelated guest. Require supported controls that bind the disposable
clone to the dedicated network, disconnect the host, omit NAT and external
uplinks, and account for both IPv4 and IPv6. Record network ownership and
configuration before starting the clone. Verify the actual guest interfaces
and routes as well as host-side topology before executing fixtures. An
adapter-disconnected readback or reserved-address timeout is not proof.
If the installed supported controls cannot establish this boundary, keep the
clone stopped and return the exact unsupported control; do not edit private
Parallels configuration or weaken containment. Original execution and guest
login remain separately gated by the reviewed plan.

### Copilot Normal Host Proof Direction

On 2026-10-10 Tom answered “Yeah keep it simple please. A vm is overkill”
to the explained alternative of testing the normal logged-in CLI against a
disposable directory with a benign action and bounded supervision. Decision
`d6c0fdf0-5abe-4562-a59f-e79520b20dca` records this ruling. His preceding direction
“Do whatever you need to do to get it done” remains completion authority.

This supersedes the VM, dedicated network and no-child prerequisites for the
next Copilot permission proof. Use the existing `betterthanclay` CLI login
and Auto policy on the normal macOS host. Trust the exact reviewed vendor CLI
with its ordinary host login reads, runtime child processes, managed state
and provider networking; do not describe a disposable working directory as
OS containment or claim default-deny egress. The prompt names only a benign
action against a task-owned sentinel. Reject or cancel every permission
request; never approve, resend or ask for broad host changes. No token
extraction, credential copying, new login or account/settings changes by the
harness. Normal vendor-managed logs/state must be disclosed.

First fake-prove and independently review the simple ACP harness, exact
artifact/launch plan, fsynced consumption record, 60-second deadline, bounded
sanitized diagnostics, sentinel observations and cleanup. Preserve all old
consumed records and the shared maximum of three unused permission prompts.
Start with one exact retained `1.0.93` permission attempt under that reviewed
plan; no failed invocation retries or extra reviewer spend. Bind remaining
version-specific attempts and currentness scope from its actual results.
Process-group exit alone does not establish arbitrary descendant cleanup;
report observed cleanup and any limitation truthfully. Host supervision is
not a security guarantee that the vendor has no ambient access.

The existing task clone remains stopped and its ownership/evidence is
preserved; no new VM or network work is needed. No compatibility claim moves
until reviewed original permission, reject/cancel and no-effect evidence
exists. Released consumers, the exact `1.0.80` claim and all original records
remain unchanged.

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
