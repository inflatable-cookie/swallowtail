# Research 482: Claude Agent ACP 0.106.0 evidence

Status: evidence only. `claude-agent.acp` remains qualified through 0.100.0;
newer points remain `UnverifiedNewer`. Tom chose a distinct permission
behavior revision; the process boundary still needs separate authority and
proof. This record does not qualify the
standalone Agent SDK, native executable, Node, shared ACP schema, or HTTP MCP.

## Identity

On 2026-11-12 npm `latest` was 0.106.0. Between the 0.100.0 ceiling and that
point the complete published stable set is 0.101.0, 0.101.1, 0.102.0,
0.103.0, 0.104.0, 0.104.1, 0.105.0 and 0.106.0. Every tarball's npm SHA-512
integrity and SHA-256 were checked; npm `gitHead` matches the corresponding
upstream tag. The [fixture](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.106.0/README.md)
retains exact identities, all package paths and hashes, adjacent deltas, and
the dependency package trees. No package was installed or run.

| ACP hop | ACP SDK | Agent SDK | Bundled native manifest |
| --- | --- | --- | --- |
| 0.100.0 | 2.4.0 | 0.3.316 | 2.1.316 |
| 0.101.0 | 2.4.0 | 0.3.318 | 2.1.318 |
| 0.101.1 | 2.5.0 | 0.3.319 | 2.1.319 |
| 0.102.0–0.104.0 | 2.5.0 | 0.3.321 | 2.1.321 |
| 0.104.1–0.105.0 | 2.5.0 | 0.3.322 | 2.1.322 |
| 0.106.0 | 2.6.0 | 0.3.322 | 2.1.322 |

The native values are identities carried in the respective SDK packages, not
independent qualification. The recorded Linux x64 SDK 0.3.319 binary matches
the exact 2.1.319 binary in Research 477; 0.3.322 matches the Linux x64
2.1.322 binary in Research 478. Its manifest also names Darwin and other
platform payloads; that metadata alone does not establish their behavior.

## Selected boundaries

The fixture classifies changed paths by literal v1 import reachability and
surface role. It is an over-approximation: shared helpers can serve several
operations, and dynamic resolution and semantic effects need separate review.
The 0.103.0 extraction of v1 modules and 0.105.0 session-index relocation
are tracked separately, including exact before and after hashes. File moves
must not be mistaken for proof that their behavior is unchanged.

- 0.101.0 adds idle compaction to each v1 session. An idle session with a
  one-hour cache and sufficient context can schedule an unsolicited `/compact`
  through the prompt path. Defaults are 200,000 tokens and 54 minutes; settings
  and environment can disable or adjust it. Client prompt, cancellation and
  close disarm or dispose it. This affects session process and spend authority.
- 0.101.1 pins Agent SDK 0.3.319. Its native setting description says user or
  managed `permissions.denyInsteadOfPrompting` can deny a call without asking
  `canUseTool`, then continue the turn. ACP enables user settings and uses
  `canUseTool` for ACP permission requests. Swallowtail's default behavior
  rejects an observed request and cancels; an absent callback changes what can
  be observed. The setting ignores project and local copies of that key.
- 0.102.0 changes plan update handling and background activity helpers.
  0.103.0 extracts v1 code into modules and changes shared model, elicitation
  and rendering helpers. These changes must be reviewed through their callers,
  not dismissed as a rename.
- 0.104.0 adds auth status advertisement; 0.104.1 changes auth helpers and
  the bundled SDK pin. 0.105.0 adds session status and title capability and
  changes model-selection settings precedence and session-index layout.
- 0.106.0 adds initial session config options and session/options. The selected
  load path now attempts history replay across compaction boundaries, falling
  back to the current chain on read failure. ACP SDK 2.6.0 adds the v1
  session/options request and schema. Swallowtail does not select that new
  operation; the changed load and shared handlers still require proof.

No static manifest or fake proves that the changed native binaries implement
every selected SDK behavior. HTTP MCP live honouring remains exact to the
previously evidenced points. The retained tests exercise adapter behavior
against fakes; they do not execute or qualify the vendor artifacts. Operator
decision `43ddf3d5-e62b-4516-8546-e53137b320be` version 2 chooses a distinct
ACP revision preserving the inherited permission setting. The separate
idle-compaction process boundary has been reported to the planner.

Sources: [official npm package](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp),
[upstream tags](https://github.com/agentclientprotocol/claude-agent-acp/tags),
and the exact tarball URLs and integrity values in the fixture.

## Remaining native proof

For each SDK native manifest in the fixture, first check whether retained
selected-behavior evidence matches the exact platform binary and the same
launch and settings conditions. Do not transfer a Claude Code result merely
because its executable digest matches: ACP selects it through the SDK and
supplies different options and callbacks. For every changed binary without
such evidence, retain the exact platform hash as unresolved. Marker counts,
wrapper declarations and fake-driver behavior cannot close that gap.

A follow-up needs a separately approved, isolated probe plan. First prove the
probe harness with fake ACP and SDK peers: record permission requests before
any permitted effect, ordered activity, cancellation, native close and joined
cleanup, and show that a denied or absent permission cannot be mistaken for
an approved one. Then, only with explicit authority for vendor execution and
any required credentials, network access or spend, run the exact SDK-bundled
native artifacts under controlled user and managed settings. Compare setting
absent versus enabled, the selected default and consumer-mediated permissions,
model/config inheritance, prompt/result/errors, cancellation and close. For
idle compaction, control time and cache/token inputs and observe whether an
unsolicited turn starts or can be disabled. Record platform, binary digest,
settings, process actions and observable wire order. Use no real user home or
ambient credentials, and keep network disabled except where explicitly
approved. A fake result is harness proof, not vendor qualification.

## Bounded follow-up inspection

The `native-permission-control.json` fixture identifies an exact no-prompt
control function in each independently downloaded Linux x64 and Darwin arm64
SDK 0.3.319, 0.3.321 and 0.3.322 binary. Each recorded function range has a
byte offset, length and SHA-256 under the whole binary hash. The fragments show
conditional no-prompt denial after hook handling; they do not establish the
entire caller chain, every tool outcome, or other platforms. In particular,
the absence of a marker in 0.3.318 is not proof of that earlier binary's
complete control flow. Fake ACP responses test the adapter, not native behavior.

The `idle-control-trace.json` fixture binds source ranges in 0.101.0 and
0.106.0 to the tarball trees. The session registers a timer and routes its
callback into the v1 prompt method. The timer checks an observed one-hour cache,
context threshold, active work and user input, rate limit, and elapsed deadline.
The default delay is 54 minutes and default minimum is 200,000 tokens.
`idleCompaction: false`, `autoCompactEnabled: false`, or truthy
`DISABLE_COMPACT` or `DISABLE_AUTO_COMPACT` suppress it. Settings-provided env
wins over the process environment for those flags. The adapter passes a
host-supplied environment reference; it does not itself set these controls.

Prompt, steer, cancel and model selection disarm a pending timer, and teardown
disposes it. A timer that has already entered `compact()` is not undone by
`disarm()` alone; the teardown path separately cancels and awaits queued
turns. Exact race outcomes remain unproved without a bounded vendor-side
observation. Contract 039 describes one consumer-submitted structured prompt
and joined cleanup. The existence of a suppression setting does not prove this
route enforces that authority boundary under inherited settings.

In 0.101.0 the timer's settings callback reads
`registered.settingsManager.getSettings()` rather than the inline SDK
settings supplied through session metadata. The manager resolves user,
project, local and managed files independently. An inline
`idleCompaction: false` therefore does not establish suppression at that hop.
The flags consult effective `settings.env` before process env, so a host env
flag is not a guaranteed override either. In 0.106.0,
`getCompactionSettings(configuredSettingsObject)` includes inline settings,
but managed compaction values and managed env take precedence. The frozen
inventory pins the relevant settings, timer and v1 agent files. No route-owned
control has been proved across all hops while retaining inherited settings.

The prepared-driver fake advances a controlled clock to 53 and then 54
minutes after a completed turn. At the simulated deadline it emits an
unsolicited message update; the adapter stops the connection and joins cleanup
without issuing another `session/prompt`. This proves the adapter's response
to that fake wire event, not the vendor timer, suppression or spend behavior.

Keep the claim unchanged until the selected native and idle-work gates are
resolved. A follow-up can prove a route-owned suppression control across
settings precedence and teardown races, or propose a distinct lifecycle with
explicit idle-work authority. Neither follows from the permission ruling. No
provider process or model was executed in this inspection.

## Proposed supported-control and probe prerequisite

Tom's decision `e7fdb750-688e-4326-a197-5ce89a8e300d` version 2 requires
supported idle suppression; it does not authorize an autonomous idle-work
lifecycle or overriding managed settings. The missing prerequisite is an
upstream-supported, per-session control that prevents idle compaction before
it can start while retaining user and managed permission inheritance. It must
be effective for new, load and resume, including settings reload, and provide
a way to stop and join work already fired during prompt, cancel or close races.
A control that reports success while a higher-precedence setting enables idle
work does not satisfy this boundary.

The affected published ACP hops are 0.101.0, 0.101.1, 0.102.0, 0.103.0,
0.104.0, 0.104.1, 0.105.0 and 0.106.0. Their exact tarball and per-file
hashes, pinned SDK versions and native manifests are in this record's fixture.
ACP 0.101.0's inline SDK settings do not feed its timer; host environment
flags can be shadowed by `settings.env`. ACP 0.106.0 considers inline
settings, but managed values take precedence. The 0.101.0 and 0.106.0 timer
sources differ only in a comment; the inventory binds the intervening shipped
files. This is a static control-path finding, not evidence that a hypothetical
control works at any hop or platform.

A bounded follow-up should first prove the harness with controlled clocks and
fake peers for new/load/resume, settings precedence, permission-before-effects,
ordered activity, and the pending and already-fired timer races. Next, inspect
or exercise each exact artifact and selected native platform independently,
matching its frozen digest. Acceptance requires no autonomous query or spend
under conflicting user, project, local and managed settings; an explicit
failure if the control cannot be honoured; and cancellation plus joined
cleanup when a timer is already firing. Verify the distinct permission revision
with and without a provider callback without changing earlier points. Use an
isolated home and workspace, no ambient credentials and network disabled by
default. Any vendor execution, live network, credentials or paid provider work
needs separate authority before the corresponding step. This is a proposal for
planner review, not an upstream message or permission to run the probe.


## Fake-only local guard prototype

Decision `33e074b8-eeac-4095-aa12-78871f3ff203` remains pending. The
prototype in `scripts/prototypes/claude-acp-idle-guard/` is not a supported
launch or a qualification. It snapshots four digest-pinned fake modules,
rejects startup controls in its host model, and hands source bytes to a Node
preloader and narrowly scoped module loader. The loader accepts one entry
and one caller-to-idle edge, then changes the fake idle class before the fake
entrypoint body. It vetoes both scheduling and direct fire. Replacing the
files after the snapshot does not change what this fake process loads.

The Effigy prototype selector compares a six-timer unguarded fake with a
zero-timer guarded fake, advances a fake clock beyond 30 days, and exercises
new/load/resume, repeated idle, direct fire, fake cancellation and close. It
rejects changed and missing files, duplicate imports, unsafe startup arguments,
symlink and query aliases, and wrong caller edges. The fake reports inherited
permission data unchanged. These are fake process observations, not proof of
vendor behavior, actual permission inheritance, a real clock, or OS process
cleanup. Node 26 was used only to exercise the fake; no Node support claim
follows. Module initialization effects before the patch are also outside this
fake's coverage.

The current host model is itself JavaScript and cannot secure its own Node
startup. A production admission primitive must validate the interpreter,
complete startup argv and effective environment before spawning, reject
unapproved preloads and loaders, bind immutable verified module bytes to the
actual loader, and establish host policy and symlink/race safety. A child
process marker or a hash followed by import of mutable files cannot do that.
The exact Node, host policy, loader, shim and every ACP/SDK caller and imported
module must form a separately attested tuple for each selected hop; the frozen
inventory identifies those bytes but does not establish this tuple. Managed
idle enablement would be vetoed by this candidate, so product authority is
needed before any real launch. Permission behavior and native selection remain
separate gates.

A bounded original-artifact probe, only after separate execution authority,
should run the exact integrity-pinned 0.101.0, 0.101.1, 0.102.0, 0.103.0,
0.104.0, 0.104.1, 0.105.0 and 0.106.0 tuples in isolated homes and workspaces,
with no ambient credentials and network disabled, against a controlled fake
transport and clock. First demonstrate admission and byte replacement attacks
against the fake harness; then verify each real loader edge and absence of
idle scheduling or fire under conflicting user and managed settings on
new/load/resume and prompt/cancel/close races. Observe permission callbacks,
denial without callbacks and joined cleanup independently for each selected
native artifact and platform. Any network, credential or paid-provider step
requires additional explicit authority. A missing edge, unbound runtime or
uncontrolled initialization fails closed and cannot extend the claim.


## Per-hop selected idle import

A fresh integrity-checked extraction of all eight published tarballs records
the selected v1 caller, literal import, caller digest, idle module digest and
class-method shape in `guard-import-edges.json`. The first three hops import
`./idle-compaction.js` from `dist/acp-agent.js`; from 0.103.0 the v1 caller
is `dist/v1/acp-agent.js` and its import is `../idle-compaction.js`. The
first group shares one idle module digest and the later group shares another.
The source has one selected construction and prototype methods at each hop;
this observation does not establish runtime module resolution, immutable
loading, worker-realm exclusion, permission behavior or native behavior.
The frozen inventory and registry bind each row to exact shipped bytes.
