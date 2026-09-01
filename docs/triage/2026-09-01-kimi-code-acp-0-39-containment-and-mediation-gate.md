# Kimi Code ACP 0.39.x Containment And Mediation Gate

Status: stopped; one operator decision required; no direction accepted
Owner: Tom
Date: 2026-09-01
Source: operator gate request, completed g05.016 cards 041-042, Research 270,
and `main` at `2756ad6fa11c5d7cf48e5a5022fa107ad233f3a7`

## Purpose

g05.016 stopped the `kimi-code.acp` axis at `0.38.0` and excluded exact
`0.39.0` and `0.39.1` because the agent-core-v2 ACP terminal runner spawns
local host processes under the capabilities the route advertises, and nothing
in the adapter or the runtime contains that spawn. That was a claim decision,
not a direction decision.

This document compiles the direction decision. It re-derives the authority
failure path from the current source, names every actor that can and cannot
act on it, reduces the space to three mutually exclusive operator directions,
analyses each, marks the impossible, dishonest, inconsistent, and incomplete
shapes that are not choices, and returns exactly one question with no
sub-choice.

It is planning evidence. It authorizes no Rust, no contract amendment, no
claim change, no implementation card, no provider contact, and no coverage
claim. The recommendation below is analysis, not an accepted decision.

## Preserved Facts

These are load-bearing inputs, not conclusions of this gate.

- Kimi ACP `0.39.0` and `0.39.1` are exact excluded points and classify
  `Incompatible`. `InterfaceCompatibilityClaim::assess` tests exclusions
  before the `AllowUnverified` newer path.
- The ACP claim's newer-version posture is `AllowUnverified`
  (`selection.rs:116`), so current `main` is safe only for those two exact
  points. Any other published point above `0.38.0` falls through to the
  unverified-newer path.
- Under `terminal: false`, upstream `0.39.x` local-spawns host processes in
  the session working directory.
- `HarnessIsolation::AmbientHost` plus a `Read` or `ReadWrite` resource lease
  has no proved mediation and no proved containment.
- The ACP axis stays qualified only through `0.38.0`. Headless v2 separately
  qualifies `0.33.0..=0.39.1`; headless v1 is `0.29.0..=0.32.0`.
- No wire-shape-only requalification. Wire stability across
  `0.38.0`→`0.39.1` is real and recorded in Research 270; it is not evidence
  about authority.
- `kimi-code.local-server` is a separate Contract 029 family. No flattening.
- g05.009, card 034, the 249 proved / 518 remaining projection rows, the
  Gemini deferral, and every second family stay untouched.
- The fresh all-route Contract 029 currentness checkpoint is serially after
  this decision, not concurrent with it.

## 1. Authority Failure Path

### 1.1 The path, in current source order

Both ACP attachment shapes take the same path. Only the lease access and the
advertised `writeTextFile` value differ.

| Step | Site | What is bound |
| --- | --- | --- |
| 1 | `prepared_profile/plan.rs:113` | `OperationRequirements::with_harness_isolation(HarnessIsolation::AmbientHost)` |
| 2 | `prepared_profile/plan.rs:115` | `with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite))` for the interactive session; `provider_session_catalogue.rs:153` and `:359` bind `ResourceAccess::Read` plus the same `AmbientHost` for catalogue and import |
| 3 | `prepared_profile/plan.rs:104-109` | required host services: `Task`, `Process`, `Credential`, `WorkingResource`, `WorkingResourceIo`. No containment service kind exists to require |
| 4 | `driver/access.rs:21` / `:38` | the driver re-asserts the same ambient policy per attachment: `ambient_harness(ReadWrite)` for sessions, `ambient_harness(Read)` for catalogue and import |
| 5 | `driver/access.rs:85-93` | `WorkingResourceService::resolve(scope, ref, access, ResourceRepresentation::Filesystem)` returns one `ResourceLease` |
| 6 | `driver/access.rs:106-109` | `resource.filesystem().as_driver_value()` becomes the `cwd` string |
| 7 | `driver/access.rs:111-116` | `ProcessRequest::new(ExecutableRef::from_instance_target(..)).with_arguments(["acp"]).with_environment([isolated_environment]).with_working_resource(..)` |
| 8 | `driver/access.rs:117` | `ProcessService::start(scope, request)` returns one `Box<dyn ProcessHandle>` |
| 9 | `driver/access.rs:125-134` | `resource_io` is `Some(..)` only when access is `ReadWrite`; a `Read` lease passes `None` |
| 10 | `connection/attachment.rs:20-24` | `initialize` advertises `fs.readTextFile: false`, `fs.writeTextFile: <lease-derived>`, `terminal: false`, `auth.terminal: false` |
| 11 | upstream `acp-server/src/acp-terminal/acpTerminalRunner.ts` | at `0.38.0` two fail-closed errors; from `0.39.0` both become `this.local.spawn(command, args, { ...options, cwd: options?.cwd ?? this.cwd })` |

Step 10 is the trigger. Because `connection.terminalEnabled` is derived from
the client's advertised terminal capability and Swallowtail always advertises
`false`, the `0.39.x` local-spawn branch is not a fallback the route might
avoid. It is the only branch this route can take.

Step 11 executes with the ambient authority of the Kimi process, in the
directory produced at step 6, with no ACP message crossing the connection.

### 1.2 Which actors can act on it

`obs` = observe the spawn as it happens; `prev` = prevent it; `med` = mediate
or bound it; `canc` = cancel it; `clean` = clean it up; `attest` = produce
host-owned evidence that it did or did not happen.

| Actor | obs | prev | med | canc | clean | attest |
| --- | --- | --- | --- | --- | --- | --- |
| `KimiPreparedSession`, `KimiPreparedSessionCatalogue` | no | version only | no | no | no | no |
| `PreflightPlan` / `OperationRequirements` | no | no | no | no | no | no |
| `SessionAccessPolicy::ambient_harness(..)` | no | no | no | no | no | no |
| `ResourceLease` (`Read` or `ReadWrite`) | no | no | no | no | consumer-owned; `release` returns `NotApplicable` | no |
| `WorkingResourceIoService` | no | no | callback only | no | no | no |
| `AcpConnection` dispatch and pump | no | no | no | `session/cancel` on the turn | no | no |
| `ProcessHandle` (`swallowtail-host-local`) | no | no | no | transitively | best effort | no |
| `ProcessService` launch | no | no | no | no | process group only | no |
| Watcher / WatcherBridge (Contracts 059-060) | no | no | no | no | no | no |
| `DiagnosticObserver` and activity projection | no | no | no | no | no | provider self-report only |

Each `no` has a reason, and none of them is incidental.

**Version refusal is the only prevention that exists.** `selection.rs:38-44`
holds `ACP_EXCLUDED_AUTHORITY_VERSIONS`. Refusing the release is a different
act from containing the behavior; it prevents the run, not the spawn.

**Nothing observes it.** Under `terminal: false` the agent never issues
`terminal/create`, `terminal/output`, `terminal/wait_for_exit`, or
`terminal/release`. There is no frame for `connection/dispatch.rs` to inspect,
reject, or count. Absence of a request is not evidence of absence of a
process.

**The leases do not bound it.** A `ResourceLease` grants callback-scoped
access and fixes cleanup authority. It is not an operating-system permission.
The `Read` lease used by catalogue and import sets `resource_io: None`, so
`connection/dispatch.rs:189-191` rejects `fs/write_text_file` — and the
spawned process still writes through ordinary syscalls, because it never used
that channel. The `ReadWrite` lease is no better: it bounds one mediated text
replacement under `MAXIMUM_WRITE_BYTES` with canonical resolution under one
root, while the spawned process has the whole ambient filesystem and network.
Contract 017 already states this directly: filesystem callbacks are not a
process sandbox, and setting a working directory or passing a resource path
does not prove containment.

**Cancellation reaches it only transitively.** `session/cancel` asks the agent
to end the turn. `ProcessHandle::force_stop` reaches
`terminate_process_tree` (`process_exit.rs:93-128`), which `killpg`s the group
established at `process/launch.rs:42-66`. That is real cleanup for cooperative
descendants and nothing more. Research 259 proves a child that calls `setsid`
and closes inherited pipes survives it, and Contract 010 records the same
mechanism as "lifecycle ownership, not a security-containment claim."

**Watchers are not applicable.** `kimi_acp_descriptor()` requires no `Watcher`
or `WatcherBridge` service, and Contracts 059-060 scope those services to
watcher requests the host admits. A process the harness spawns for itself is
not a watcher request, so registering the services would change nothing.

**Nothing can attest.** The only trace of the spawn is the provider's own
`session/update` tool-call record, arriving after the fact. That is provider
self-report. Contract 023 forbids inferring an enforced posture from
provider-side evidence, so this cannot become host-owned proof either way.

**The declared posture is honest and inert.** Contract 023 defines
`AmbientHost` as the harness process *and its descendants* running with the
ambient authority of the execution host, and states that provider approval
modes and tool restrictions "do not contain the harness process, its
descendants, or unmediated filesystem and network access." `AmbientHost` is
therefore a correct label for what happens at `0.39.x`. It is not a control,
and it never becomes one.

### 1.3 The consequence stated exactly

At `0.38.0`, an ACP session under this route's advertised capabilities could
not execute a host process at all: the runner failed closed twice. From
`0.39.0`, an operation whose access policy is `ambient_harness(Read)` — the
catalogue and import shape, which advertises `writeTextFile: false` and holds
no `WorkingResourceIo` service — attaches an agent whose process authority is
unrestricted in the leased working root. The process service is threaded into
the ACP runtime provider at `start.ts` for every attachment, unconditionally;
whether a spawn occurs depends only on whether a tool is invoked.

That is an authority inversion, not a capability extension. It is the reason
`0.39.0` and `0.39.1` are excluded rather than admitted as unverified newer.

## 2. Choice Set

The operator's three candidates do not survive contact with the failure path
unsplit. A is two directions with different standing obligations, and both of
them are incomplete as originally stated. B is two directions with different
authority owners, one of which is impossible. C does not govern the cap
decision at all.

The governing set is exactly three. Exactly one of A1, A2, or B may be
selected.

| Id | Direction | Viable |
| --- | --- | --- |
| A1 | Cap `kimi-code.acp` at `0.38.0` permanently under `QualifiedOnly`. No re-open trigger. | yes |
| A2 | Cap at `0.38.0` indefinitely under `QualifiedOnly`, with one artifact-level upstream re-open trigger tracked by the currentness lane. | yes |
| B | Fund `HostEnforced` execution-host containment of the Kimi ACP process tree, keeping `terminal: false` and keeping the `QualifiedOnly` cap until containment and claim vocabulary are proved. | yes, at high cost |

Not governing choices:

| Id | Shape | Disposition |
| --- | --- | --- |
| R1 | Adapter or runtime mediation of the spawn while advertising `terminal: false`. | **impossible** |
| R2 | Requalify on wire stability, process ownership, `AmbientHost`, capability omission, or a test-only wrapper. | **dishonest** |
| R3 | Cap the ceiling while leaving the claim on `AllowUnverified` and adding exclusions release by release. | **internally inconsistent** |
| C | Negotiated terminal execution. | **incomplete alone**; a later design, not a cap decision |

### Why a cap must move the posture

`kimi_acp_claim()` binds `InterfaceNewerVersionPosture::AllowUnverified`
(`selection.rs:116`). `assess` tests exclusions first, so exact `0.39.0` and
`0.39.1` are `Incompatible` — and every *other* point above `0.38.0` falls
through to the `AllowUnverified` path. The claim's own test already records
this: unpublished `0.39.2` assesses as `UnverifiedNewer` today.

So the current shape is not a cap. It is a ceiling plus two named holes
plugged. The moment upstream publishes `0.39.2` or `0.40.0`, that release is
admissible on the most recent qualified behavior revision, carrying the same
uncontained runner, before any checkpoint can add an exclusion. Capping by
exclusion is a race against publication that Swallowtail loses by default.

R3 is therefore rejected rather than offered. A cap that has to be re-armed
after each upstream release is not a cap, and an operator selecting "cap"
cannot be given a version of it that admits the next release.

A1 and A2 each therefore carry the posture move as part of the direction, not
as a follow-on question. Their smallest follow-on claim card sets
`kimi-code.acp` to `InterfaceNewerVersionPosture::QualifiedOnly`. That is the
only mechanism in the current claim shape that closes the future-release race,
and it makes the two exclusions redundant rather than load-bearing — they stay
as recorded evidence of why the cap exists.

**Current `main` is safe only for the exact known exclusions.** `0.39.0` and
`0.39.1` cannot run. Nothing in `main` prevents a future published point from
running. Closing that is work the chosen A follow-on card does; it is not
already done.

### Why A1 and A2 still differ

With the posture folded in, A1 and A2 produce the same claim content and the
same code. They differ only in the standing obligation. A1 makes `0.39.x` a
closed question with no trigger; the axis stays `QualifiedOnly` at `0.38.0`
until a separate decision reopens it. A2 keeps one artifact-level trigger alive
in `docs/roadmaps/standing-lanes.md`, so every future Kimi checkpoint re-reads
`acpTerminalRunner.ts` from shipped artifacts rather than re-reading the
exclusion list.

Under both, `QualifiedOnly` means a newer point is `Incompatible` by default.
Under A2 the trigger authorizes a fresh identity run and a fresh claim
decision. **It never authorizes automatic admission**, and it cannot be
implemented by returning the posture to `AllowUnverified`.

### Why B splits from R1

The operator's B said "adapter/runtime mediation **or** containment." Those are
different authority owners with different feasibility, and one of them cannot
exist. They are separated rather than analysed as one option.

B also holds the cap. Selecting B does not admit `0.39.x`; it funds the work
that could later justify admitting it. The `QualifiedOnly` posture move applies
under B exactly as under A1 and A2, because the axis stays capped throughout.

### Why C is not a governing choice

Research 270 records the `0.39.0` runner change as firing when the client
advertises no terminal capability **or the invocation is not the interactive
Bash tool**. Both former errors — `ACP terminal capability is unavailable` and
`ACP runtime only supports interactive Bash tool processes` — became the same
`local.spawn`. Advertising `terminal: true` therefore routes interactive Bash
through the mediated ACP terminal and leaves every non-Bash invocation,
including the agent's Grep and Glob tools, on the uncontained branch.

C alone does not close the hole at `0.39.1`. It narrows it while adding
execution authority, which is strictly worse than the cap on both axes that
matter. And C is not independent of B: the only way C reaches containment is
by pairing with B or with an upstream change that removes the non-Bash
fallback. An option that cannot be selected without another option is not a
member of a mutually exclusive set.

C therefore leaves the governing set. **B+C** and **C plus an upstream
dependency** are recorded below as possible later designs. Neither is
selectable now, and neither governs the cap decision.

## 3. Choice Analyses

### A1 — Permanent `QualifiedOnly` cap at `0.38.0`

- **Scope.** One claim edit plus documentation. `kimi_acp_claim()` moves to
  `InterfaceNewerVersionPosture::QualifiedOnly`; segments, baseline, ceiling,
  exclusions, behavior revisions, and support statuses are unchanged. One
  roadmap disposition and one standing-lane sentence record that the axis is
  closed above `0.38.0` with no trigger.
- **Dependency direction.** Unchanged: `adapters → runtime → core`.
- **Public API / contract impact.** None. `QualifiedOnly` already exists
  (`interface_version.rs:99-104`). No contract amendment; Contract 029 already
  states that claims choose their newer-version posture explicitly.
- **Security / authority posture.** Strongest available. The route continues to
  declare `AmbientHost` honestly and can never reach a release whose harness
  executes processes outside the callback boundary — including releases that do
  not exist yet.
- **Failure / cancellation / cleanup.** Unchanged. Any point above `0.38.0`
  fails at preflight as `Incompatible` before a process starts, so there is
  nothing to cancel or clean.
- **Version-claim consequence.** `KIMI_CODE_LATEST_QUALIFIED_VERSION` stays
  `0.38.0`. The two exclusions become redundant under `QualifiedOnly` and stay
  as recorded evidence. Deprecation of the `0.28.1` legacy segment and the
  `0.29.0..=0.38.0` maintained segment is untouched. Moving the baseline later
  remains a called-out compatibility-window change.
- **Proof obligations.** One mutation-sensitive test that a published point
  above `0.38.0` — not only the two excluded ones — assesses `Incompatible`,
  and that removing an exclusion does not silently readmit it.
- **Counterexamples that would falsify A1.** Upstream restoring a fail-closed
  runner, or shipping a documented `ProviderEnforced` boundary. Under A1
  neither reopens the axis, by construction. That is the choice, and the
  operator should select it knowing so.
- **Rollback.** Trivial; one posture value.
- **Smallest follow-on card shape.** One claim card: move the ACP claim to
  `QualifiedOnly`, add the future-release assessment test, record the closed
  disposition. Focused and package validation for
  `swallowtail-adapter-kimi`; public API surface unchanged.

### A2 — Indefinite `QualifiedOnly` cap with one artifact-level trigger

- **Scope.** The same claim edit as A1, plus one standing-lane obligation.
- **Dependency direction.** Unchanged.
- **Public API / contract impact.** None.
- **Security / authority posture.** Identical to A1 while the cap holds, which
  is until a separate recorded decision.
- **Failure / cancellation / cleanup.** Unchanged.
- **Version-claim consequence.** Identical to A1. The difference is the
  standing obligation: every Kimi checkpoint must re-read
  `acpTerminalRunner.ts` and `experimental-v2.ts` from shipped artifacts before
  ranking the family. Under `QualifiedOnly` it must **not** add exclusions
  release by release — that is R3, and it is rejected.
- **Proof obligations.** A1's test, plus a trigger stated as an artifact test
  rather than a changelog claim: either the runner fails closed again for a
  terminal-less client across every invocation path, or upstream documents an
  enforced boundary meeting Contract 023's `ProviderEnforced` bar and
  Contract 017's mechanism-qualification bar. Reaching the trigger authorizes a
  fresh identity run and a fresh claim decision. It authorizes no admission,
  and it does not restore `AllowUnverified`.
- **Counterexamples.** A future release that fails closed for Bash but not for
  Grep or Glob does not satisfy the trigger; the trigger covers every
  invocation path, not the interactive one.
- **Rollback.** Trivial.
- **Smallest follow-on card shape.** A1's card plus the artifact-level trigger
  written into `docs/roadmaps/standing-lanes.md`, with the
  no-automatic-admission rule stated in the lane text.

### B — `HostEnforced` execution-host containment

- **Scope.** New execution-host authority, with the cap held throughout. At
  minimum: a containment mechanism in `swallowtail-host-local` covering the
  Kimi process and its descendants; a way for the plan to bind and the driver
  to verify that the mechanism applied; per-platform backends; Contract 017
  mechanism-qualification evidence; a Contract 029 answer for
  platform-conditional qualification. The Kimi adapter changes least: it would
  select `HarnessIsolation::HostEnforced` and a real `FilesystemBoundary`
  instead of `ambient_harness(..)`.
- **Dependency direction.** Unchanged in shape, but the new authority lands in
  `runtime` and `host-local`, not the adapter. Any design that puts containment
  inside `swallowtail-adapter-kimi` is wrong by construction: the spawn is a
  host-process concern and the adapter has no process authority beyond the
  handle it was given.
- **Public API / contract impact.** See section 5. A shared public addition is
  unavoidable for the *claim*, though not for the *mechanism*.
- **Security / authority posture.** Strongest of the change options: it bounds
  the actual failure and adds no execution authority. It also changes what the
  route promises, a consumer-visible move from ambient to enforced that
  Contract 023 forbids substituting in either direction.
- **Failure / cancellation / cleanup.** New failure classes: containment
  unavailable on this platform, containment setup failed, containment escaped.
  Contract 023 forbids falling back to `AmbientHost` when any fires. Cleanup
  must join containment teardown before releasing the resource lease, and the
  existing process-group cleanup remains, unchanged and still not a containment
  claim.
- **Version-claim consequence.** The cap and the `QualifiedOnly` posture hold
  for the whole of B; admission is a separate later decision that B's evidence
  might justify. This is where B is hardest and where an implementer is most
  likely to overclaim. A qualified `0.39.x` ACP segment would be true only on a
  host where an enforced backend exists and applied.
  `InterfaceCompatibilityClaim` (`interface_version/claim.rs:79-86`) carries
  id, axis, scheme, newer-version posture, segments, and exclusions — and no
  platform, host, or capability dimension. Contract 029 has no conditional
  qualification vocabulary either. So B cannot end in "qualified" without
  adding that dimension or splitting the axis, and neither is decided here.
- **Proof obligations.** Contract 017's mechanism-qualification section in
  full: exact root and access binding, descendant inheritance, symlink and link
  behavior, filesystem topology, special filesystems, alternate mutation
  syscalls, inherited descriptors, runtime availability on the exact deployed
  executable, cancellation, and joined cleanup. Partial enforcement cannot be
  promoted; a synthetic helper passing while the real artifact fails is
  explicitly insufficient.
- **Counterexamples — and one is already on file.** Research 011 proved the
  exact Kimi Code `0.28.1` single-executable arm64 artifact cannot run as an
  inherited macOS App Sandbox helper: re-signed with only the documented
  App Sandbox and inheritance entitlements it dies with signal 5 in V8 heap
  initialization, and the `--jitless` diagnostic then stalls on an
  ad-hoc-signed native module. Research 259 independently proves no default or
  portable macOS descendant-containment backend exists. Neither result was
  taken on a `0.39.x` artifact, so neither is dispositive today — but B must
  budget for re-running that proof against the current artifact and for it
  failing again on the same grounds. `sandbox-exec` is deprecated and falls
  under Contract 017's "deprecated, private, experimental, or undocumented
  platform interface" exclusion. Linux `cgroup.kill` and Landlock and Windows
  Job Objects exist but are per-platform and unimplemented here.
- **Rollback.** Expensive. A published `HostEnforced` route cannot quietly
  become ambient again.
- **Smallest follow-on card shape.** Not an implementation card. The smallest
  honest first step is a planning-only feasibility card: re-run the Research 011
  artifact experiment against the exact `0.39.1` artifact under the current
  documented entitlement model, on one named platform, executing nothing
  downloaded beyond that bounded experiment, and report whether Contract 017's
  runtime-availability bar can be met at all. The cap and posture move happen
  first or alongside; they are not deferred until containment succeeds.
  Compiling a containment implementation card before that result would be
  planning on an assumption two research records already contradict.

### R1 — Adapter or runtime mediation under `terminal: false`: impossible

Not "hard", not "unproved": impossible on this route as currently shaped.
Mediation requires a message to mediate. Under `terminal: false` the agent
issues no terminal request, so `AcpConnection` sees nothing, and the spawn is
internal to a process Swallowtail owns but does not supervise syscall by
syscall. There is no adapter-local or runtime-local arrangement of existing
types that interposes on it. Any design claiming otherwise is either doing
containment (that is B, and it belongs to the execution host) or doing
negotiation (that is C, and it needs the capability). A card proposing
"adapter mediation" without changing the advertised capability or adding host
containment should be rejected at review.

### R2 — Requalification by relabelling: dishonest

Each of these has been offered before, in this repository or upstream, and each
must fail review:

- wire-shape stability from `0.38.0` to `0.39.1` — real, recorded, and not
  about authority;
- process ownership and process-group cleanup — Contract 010 and Research 259
  both call it lifecycle ownership, not containment;
- omitting `terminal` instead of sending `false` — the upstream branch keys on
  `connection.terminalEnabled`, so omission takes the same branch;
- declaring `AmbientHost` more loudly — an honest label for the failure, not a
  control over it;
- a test-only or fixture-only containment wrapper — Contract 017 rejects a
  synthetic helper passing while the selected artifact does not.

### R3 — Permissive cap with a growing exclusion set: internally inconsistent

Keeping `AllowUnverified` and adding one exclusion per published release is not
a cap. Between publication and the next checkpoint, the new point is
admissible on the most recent qualified behavior revision, carrying the
uncontained runner. The gap is unbounded because it depends on when a human
next looks. Contract 029's `AllowUnverified` posture exists for axes whose
newer releases are expected to behave; this axis has proved the opposite. R3
is rejected, not offered as a cheaper A.

## Deferred Designs

Recorded so they are not re-derived, and so they are not mistaken for choices.

**B+C.** Host-enforced containment plus negotiated terminal execution. C's
mediation would give bounded, observable, cancellable execution for the
interactive Bash path while B's containment covers everything else including
the non-Bash fallback. This is the only shape in which C's added authority buys
something the cap does not already give. It is selectable only after B is
selected and its feasibility result is in.

**C plus an upstream dependency.** If upstream removes the non-Bash fallback so
that every tool invocation reaches the ACP terminal, C alone would mediate the
whole surface. That is an upstream dependency, not a Swallowtail choice, and it
would arrive through A2's trigger or a fresh identity run rather than through
this gate.

Both require, at minimum: a Contract 015 terminal amendment (the contract
currently states "Terminal support needs a later contract"), a new
`HostServiceKind` and service trait for agent-requested execution, bounded
request and output types, new failure classes, per-route scoping so advertising
`terminal: true` does not leak to every ACP route sharing the client, and
artifact-level evidence that no invocation path still reaches `local.spawn`.
None of that is authorized, costed, or decided here.

## 4. Recommendation, As Analysis Only

**A2**, on this reasoning:

1. It is the only option whose cost is bounded by what is already known. B
   requires evidence this repository does not have, and B's closest prior
   evidence — Research 011's artifact failure and Research 259's backend
   result — points at failure.
2. A2 costs one standing-lane sentence more than A1 and preserves the ability
   to reopen without a new decision, which is worth more than A1's finality
   given that the upstream change was a single-file behavior flip and could be
   flipped back.
3. Both A options close the future-release race that current `main` leaves
   open. That is the part of this decision with a deadline: it is bounded by
   upstream's next publication, not by Swallowtail's schedule.
4. Nothing about A2 blocks B later. Selecting A2 is not selecting against
   containment; it is declining to fund it on this evidence.

This is a reading of the evidence, not a decision. It is deliberately absent
from `docs/roadmaps/README.md`, from `docs/roadmaps/g05/README.md`, from
`docs/roadmaps/standing-lanes.md`, from the milestone's status, and from every
contract. The lane is paused on the question, not on the answer.

## 5. Can Existing Seams Express B Without A Shared Public Type?

Split into mechanism and claim, because the answer differs.

**Mechanism: yes.** Contract 010 already lets one `ExecutableRef` resolve to a
host-private launch recipe — one exact interpreter or native program, bounded
immutable prefix arguments, and optional bootstrap environment, all behind the
opaque reference and absent from records, events, and diagnostics. A host could
resolve the Kimi reference to a containment launcher today, with no new public
type, no adapter change, and no contract amendment.

**Claim: no, and this is provable rather than uncertain.** Three independent
blocks:

1. Contract 023 forbids exactly this inference: "A driver cannot infer an
   enforced posture from a binary name, settings file, platform, or
   installation method." A host-private recipe is invisible to the plan, so the
   driver cannot bind or verify it.
2. Contract 017's mechanism-qualification section requires the containment to
   be preflight-bound and tested. `ProcessRequest`
   (`runtime/src/process_input.rs:10-15`) carries executable, arguments,
   environment, and working resource — no containment field. `ProcessService`
   (`runtime/src/host_traits.rs:93-100`) takes scope and request and returns a
   handle — no containment parameter and no containment report.
   `HostServiceKind` (`core/src/runtime_identity.rs:201-249`) has no
   containment kind for the plan to require. So there is no seam that carries
   the binding, and none that carries the attestation.
3. `HarnessIsolation::HostEnforced` already exists
   (`core/src/session_access.rs:43`) and `SessionAccessPolicy::new` accepts it
   while rejecting `AmbientHost` (`session_access.rs:180-184`). So the
   *declaration* needs no new type — which makes this the sharpest trap in the
   whole gate. An implementer can flip `ambient_harness(ReadWrite)` to
   `SessionAccessPolicy::new(.., HarnessIsolation::HostEnforced, ..)` with a
   host-private sandboxing recipe behind the executable reference, compile
   clean, pass preflight, and have published an enforced claim with zero
   attestation. That is a declaration change wearing a containment change.

So: B's mechanism fits existing seams; B's *claim* requires a shared public
addition — a host service kind or an equivalent plan-bound containment
evidence surface — and the honest first move is to name that, not to invent it
here.

**Two uncertainties are proved rather than resolved, and both are the
operator's:**

- *Which shared shape.* An optional `HostServiceKind` plus a containment
  service trait would follow the Contract 059/060 precedent, where registration
  alone binds nothing. A plan-carried containment evidence value would follow
  the Contract 029 binding precedent. Both are constructible; neither is
  implied by current authority. No type is invented in this document.
- *Whether Contract 029 can express platform-conditional qualification.*
  `InterfaceCompatibilityClaim` has no host or platform dimension, and
  Contract 029 has no conditional-qualification vocabulary. If containment is
  per-platform, a qualified `0.39.x` ACP segment is true on some hosts and
  false on others, and today's claim shape cannot say that. This is a genuine
  gap, not a modelling preference, and it must be settled before any admission
  is compiled — not during.

Both uncertainties are consequences of selecting B. Neither needs an answer to
select A1 or A2, and neither is folded into the question below.

## The Operator Question

> Which single direction governs `kimi-code.acp` above `0.38.0`: **A1**
> permanent `QualifiedOnly` cap with no re-open trigger; **A2** indefinite
> `QualifiedOnly` cap with one artifact-level upstream re-open trigger recorded
> in the currentness lane; or **B** fund `HostEnforced` execution-host
> containment, starting with an artifact-feasibility experiment against the
> exact `0.39.1` binary, while the `QualifiedOnly` cap holds throughout?

That is the whole question. There is no sub-choice. All three move
`kimi-code.acp` to `InterfaceNewerVersionPosture::QualifiedOnly`, because
current `main` is safe only for the exact known exclusions and would otherwise
admit the next published release. A2's trigger authorizes a fresh identity and
claim decision, never automatic admission. Negotiated terminal execution is not
on the ballot: it cannot close `0.39.1` alone and cannot be selected without B
or an upstream change.

No other question is returned by this gate. Nothing else in g05 is paused on
it.

## Review Oracle

Invariant: this document is planning evidence for a stopped gate. It authorizes
no implementation and accepts no direction.

Fail any later change that:

- treats this gate as decided, or reads section 4 as an accepted direction;
- records a direction in roadmap status, contract language, the standing lane,
  or the Next Task pointer without a recorded operator answer;
- offers, records, or implements a cap that leaves `kimi-code.acp` on
  `InterfaceNewerVersionPosture::AllowUnverified` and relies on adding
  exclusions release by release — that is R3; the next published point is
  admissible before any checkpoint can react;
- treats the `QualifiedOnly` move as an open sub-question, an optional part of
  A1 or A2, or work that is already done on current `main`;
- implements A2's trigger as automatic admission, as a return to
  `AllowUnverified`, or as anything short of a fresh identity run and a fresh
  claim decision;
- reintroduces negotiated terminal execution as a top-level choice alongside
  A1, A2, or B, or presents B+C or C-plus-upstream as selectable before B is
  selected and its feasibility result is recorded;
- qualifies `0.39.x` on C alone — the non-interactive-Bash invocation paths
  still reach `local.spawn` at `0.39.1`;
- claims containment from **process ownership** — owning the Kimi
  `ProcessHandle`, spawning into a process group, or calling
  `terminate_process_tree` or `force_stop`. Contract 010 and Research 259 fix
  that as cooperative lifecycle cleanup; a `setsid` descendant survives it, and
  cleanup after a turn contains nothing during the turn;
- claims containment from **wire stability** — the byte-identical mapped ACP
  surfaces across `0.38.0`→`0.39.1` are evidence about framing, never about
  authority;
- claims containment from **omitting `terminal`** rather than sending
  `terminal: false` — upstream keys on `connection.terminalEnabled` and both
  encodings take the `local.spawn` branch;
- claims containment from **`AmbientHost`** — the posture is a correct label for
  uncontained execution and is never a control;
- claims containment from a **test-only or fixture-only wrapper** —
  Contract 017 rejects a synthetic helper passing while the selected artifact
  does not;
- claims containment from the **resource lease** — neither `Read` nor
  `ReadWrite` is an OS permission, and `resource_io: None` bounds one callback
  channel, not the process;
- claims containment from **`fs.writeTextFile: false`**, an approval mode, a
  plan mode, a tool denylist, or `mcpServers: []` — Contract 023 excludes all
  of these;
- selects `HarnessIsolation::HostEnforced` or `SessionAccessPolicy::new(..)`
  for this route without a plan-bound, driver-verifiable containment
  attestation — the type compiles today and would publish an unattested
  enforced claim;
- implements containment inside `swallowtail-adapter-kimi` — the spawn is host
  authority;
- proposes adapter or runtime mediation while the route still advertises
  `terminal: false` — see R1; there is no message to mediate;
- requalifies `0.39.0` or `0.39.1`, removes either from
  `ACP_EXCLUDED_AUTHORITY_VERSIONS`, or raises
  `KIMI_CODE_LATEST_QUALIFIED_VERSION` above `0.38.0`, before a recorded
  operator answer and the proof obligations that answer carries;
- publishes a platform-conditional qualified segment through the current
  `InterfaceCompatibilityClaim` shape — it has no host or platform dimension;
- changes `kimi-code.local-server`, `kimi-code.headless`, `kimi-platform.chat`,
  or any second family from this gate;
- touches g05.009, card 034, or the 249 proved / 518 remaining projection
  counts;
- lifts the Gemini deferral, or runs the all-route currentness checkpoint
  before this decision is recorded;
- compiles an implementation card for any direction before the operator
  answers, or compiles the follow-on card for one direction as if the others
  were closed.

## Validation Boundary

None beyond the planning batch that carries it: `effigy qa:docs`,
`effigy qa:northstar`, and `git diff --check`. This document changes no Rust,
no manifest, no fixture, no matrix, no guide, and no contract.

## Authority

- [Contract 015](../contracts/015-acp-v1-negotiation-and-client-callbacks.md)
- [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md)
- [Contract 010](../contracts/010-execution-host-services-and-inputs.md)
- [Contract 023](../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 059](../contracts/059-operation-scoped-process-watchers.md)
- [Research 270](../research/270-kimi-code-0-39-1-identity.md)
- [Research 259](../research/259-process-containment-backend-evidence.md)
- [Research 011](../research/011-kimi-macos-app-sandbox-runtime-compatibility.md)
- [completed g05.016](../roadmaps/g05/016-kimi-code-0-39-1-useful-newer.md)
- [completed card 041](../roadmaps/g05/batch-cards/041-kimi-code-0-39-1-identity.md)
- [completed card 042](../roadmaps/g05/batch-cards/042-kimi-code-0-39-1-claim.md)
- [standing lanes](../roadmaps/standing-lanes.md)
