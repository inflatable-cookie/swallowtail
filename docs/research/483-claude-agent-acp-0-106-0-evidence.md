# Research 483: Claude Agent ACP 0.106.0 evidence

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


## Interpreter binding prerequisite

The host launch currently passes an approved path to `Command::new`; neither
its fixed prefix nor the Node-environment filter binds the interpreter bytes
to what the kernel executes. A file hash followed by path spawn leaves a
replacement window. The available `nix` 0.31.3 `fexecve` wrapper is not
compiled for macOS; this host crate forbids unsafe code. The retained reviewer
confirmed that the repository has no existing descriptor-exec primitive.

A run-owned, mode-restricted staged copy would protect against replacement
of the source path, but same-user modification of the copy between its last
hash and path spawn remains possible. The Node runtime's own reported path
and version are observations, not an independently established identity.
Neither alternative meets this task's requirement to bind the exact Node
runtime. A platform-specific verified-execution or equivalent protected
immutable-byte primitive, with bounded ownership and joined cleanup, is
needed before the distinct launch can be admitted on each claimed platform.
The existing fake data-URL snapshot proves only the fake module handoff; it
does not close the interpreter race or establish the production import graph.
The runnable original-artifact probe cannot be frozen until the guarded host
path, its runtime and its final launch-policy digests exist. Vendor execution
and live provider access remain outside this task's authority.


## Captured graph and checked process scaffold

The adapter now packages a candidate captured-source loader. It resolves only
explicit captured graph edges, rejects unknown module identities and encoded
URL aliases, and serves source strings supplied to its loader rather than
reopening module paths. The fake guard locks the idle prototype and prevents
new own-method overrides on constructed fake instances. This changes object
shape and has not been shown compatible with the official sources; Node
builtins and unexpected worker realms also fail without explicit edges. The
captured graph and its calling rules still need a complete host-owned manifest
and exact transitive dependency coverage. A caller-provided loader graph is
not admission evidence.

An opaque checked-process reference now crosses preparation and prepared
session operations. Process-service preflight and start both default to
refusal, with no ordinary-start fallback. A fake host tests target and binding
mismatch, revocation between preparation and session start, successful fake
operations and cleanup; that host is a harness, not proof of local runtime
custody. The local host supplies no checked-process implementation. Earlier
stock paths and claims remain unchanged.

`scripts/prototypes/claude-acp-idle-guard/probe.mjs --original` always
refuses before provider execution. Its fake-only mode is bounded by a timeout
and output cap and exercises the fake loader tests. It is not an isolated
original-artifact probe: runtime custody decision
`fdd2eb69-e5c7-4743-bde5-fd5be30f7c1f`, real OS egress enforcement, final
interpreter and graph identities, and separate execution authority are still
needed to make that probe runnable. No self-reported marker or local path
can replace those requirements.


A future authority request should name each of the eight frozen ACP hops and
its exact SDK/native pair on Darwin arm64 and Linux x64, along with the final
guard, graph, launch-policy and Node digests. A bounded starting envelope is
one attempt per hop/platform tuple, at most three owned child processes, 120
seconds per attempt, 1 MiB per output stream and 256 MiB of run-owned scratch.
The harness must prove fake behavior first, reject all non-allowlisted inputs,
use an isolated home without ambient credentials, and enforce no egress at
the OS boundary. It must fail closed if that enforcement is unavailable.
Each attempt records timeout, admission, identity, protocol, cleanup and
isolation failures separately and joins all owned processes before returning.
These proposed limits are not execution authority or proof that this host
can enforce them; the concrete Node and launch-policy digests do not yet exist.


The local host also has a bounded source-byte capture helper. It checks a
host-supplied manifest's paths and hashes, rejects observed symlinks and
aliases, and retains the verified bytes in memory after later file changes.
Its deterministic fingerprint binds the captured names and bytes only. The
caller-supplied manifest is not official identity, parent-directory custody
is not established, and this helper does not issue a checked launch or bind
Node. A future host authority must establish the complete graph, guard,
runtime, recipe and policy and recheck that binding at start.

The host can form a deterministic candidate graph from captured source bytes
and literal import edges, refusing duplicate or disconnected entries. Its
candidate launch fingerprint includes the graph and host-supplied vendor,
shim, runtime, recipe and policy digests. The fake checked host compares that
fingerprint through preparation and prepared operations, rejects substituted
source or recipe, and keeps captured bytes after a disk change. Its scripted
ACP peer does not execute the graph; the loader fake is tested separately.
The fingerprint is not an approved tuple and supplies no runtime custody or
complete official dependency closure. Production checked start still refuses.

A provider-free process test sends host-captured fake source and graph bytes to
the packaged loader, replaces the source files before launch, and checks the
fake child executes the retained bytes with zero idle arming or firing. Its
unguarded control performs both effects; an unapproved worker import fails
before the fake entry body. The fake prepared-driver host checks the same
candidate fingerprint and refuses altered source or recipe and revoked
bindings. A separate checked fake host wraps the local process host and runs
the captured fake Node graph through prepared discovery, session creation,
prompt and joined close after disk replacement; revocation prevents another
start. This fake wrapper supplies no real interpreter custody, and does not
show that a production host can admit an official graph.

Checked prepared-driver fakes also exercise load and resume through the same
opaque binding, cancellation and joined cleanup, and an unverified 0.106.0
denial-without-callback scenario with ordered activity and continued output.
Those fakes do not prove the shipped native permission outcome or qualify a
new behavior revision.

The local host can derive a candidate Node prefix from the captured graph and
loader bytes. Its candidate fingerprint binds the actual shim and encoded
prefix digests together with host-supplied vendor, runtime and policy digests.
The combined fake prepared-child test uses that prefix after replacing the
source files. It does not attest those supplied digests or authorize the
interpreter. The candidate sends a length-delimited graph bootstrap before
ACP input, with a 32 MiB graph bound and a separately capped loader argument.
The fake checked service writes that bootstrap before exposing the process;
truncated input fails before entry effects. The real host does not implement
this checked handoff or establish the required runtime custody.

The packaged v1 wrapper manifest is derived from the frozen reachable-path
set and shipped-tree hashes for each of the eight hops; its exact key set and
digests are checked against both records. It is a conservative static wrapper
inventory, not a complete dependency or dynamic-import graph, a Node binding
or authority to use an installed package. Unknown versions fail closed.

The fake checked host now uses a separate bounded bootstrap write before
returning the process handle. The host writes through a nonblocking pipe under
a deadline without charging the normal stdin budget. Failed writes stop and
join the child; dropping a pending start requests process-tree cleanup. The
candidate loader requires an admission byte after the complete graph and fails
before entry effects for malformed, oversized or truncated frames. Fake tests
cover a child that never reads, a closed pipe, timeout, cancellation and a
small ordinary stdin budget. This does not prove that an actual runtime is
attested, or that cancellation after the admission byte cannot race entry
execution. The checked local host still refuses admission without custody.

The static wrapper manifest does not capture the complete bare-package,
builtin, dynamic-import or worker closure. Actual graph construction and
original-artifact probe execution remain unproved and unauthorized.

## Static import inventory and admission ordering

`import-edge-inventory.json` records syntax-level imports, exports, literal
`import()`, `import.meta.resolve`, direct `require()` and visible Worker
construction sites for every shipped JavaScript file in
the eight ACP archives, the three selected ACP SDKs, four selected Agent SDKs,
and the exact `diff` and `zod` packages. Each row binds a source SHA-256 and
byte offset. `import-edge-artifacts.json` records the previously unfrozen
`diff` and `zod` archive identities. `scripts/extract-claude-acp-import-inputs.py`
verifies both SHA-256 and SHA-512 before reading archive members, and
`scripts/analyze-claude-acp-imports.cjs` parses source without evaluating it.
The exact analysis tool was Node v26.11.0's embedded Acorn with SHA-256
`fc3ed7b81e58464715d0291402892f22c3d86ea75302645a330390f85d8015c9`.
This identifies the parser only; it does not qualify Node 26 as a runtime.

The inventory includes conservative caution sites for `createRequire`,
`require`, `eval`, `Function` and `Worker`. It is not a closed runtime graph:
aliased calls, constructed code, builtins, CommonJS resolution, native
launches and worker realms need separate analysis. For example every wrapper
hop contains `dist/claude-cli.js` `createRequire` sites; the selected Agent
SDK has additional dynamic-code and CommonJS sites. The present guard refuses
unknown ESM edges and does not admit the builtin and native paths. No source
inventory is a claim that the original artifacts have run under the guard.

The bootstrap writer now sends the admission byte in a separate final write.
No pending write has sent that byte; a successful write resolves its write
future in the same poll. On successful restoration the start returns without
yielding again. A restoration error can yield during later child cleanup.
Fake tests cancel while the pipe is stalled before admission and
terminate and wait for a process tree after admission, including a descendant.
This tests those orderings, not every operating-system race or interpreter
custody. The ordinary bootstrap API is not checked-launch attestation.

## Synchronous hook candidate and selected roots

The [Node 22.23.3 module documentation](https://nodejs.org/download/release/v22.23.3/docs/api/module.html#customization-hooks)
describes synchronous `registerHooks` interception for import, built-in
`require`, and `createRequire`-created require. Synchronous hooks are not
inherited into worker realms by default. The documentation is not proof of
this route's runtime or immutable interpreter identity.

The fake-only `sync-preload.mjs` serves captured ESM and CommonJS source and
allows only exact caller/specifier edges. On the current host Node, a fake
`createRequire` alias and its `require.resolve` load captured CommonJS after
the backing files are replaced; unknown paths, query aliases, denied builtins
and duplicate edges fail. The fake explicitly permits `node:module` for its
entry and does not establish confinement against direct builtins, native
addons, dynamic code or independently created workers. It does not execute
or admit official ACP bytes. Exact Node 22.23.3 fake execution remains unproved.

`sdk-root-exports.json` binds each selected SDK's root export to the frozen
archive and package.json digest: all four versions export `./sdk.mjs`.
For each ACP hop the frozen conservative reachable set includes its exact
v1 caller, which statically imports that SDK root, and `dist/claude-cli.js`,
which dynamically imports `node:module` and calls `import.meta.resolve` for
that SDK before `createRequire`. The selected `sdk.mjs` contains aliased
`createRequire` calls and `Function` identifiers. These are statically
reachable source sites, not proof that any particular call executes. The
remaining dynamic-code, addon and worker paths need call/dataflow and runtime
proof; no absence claim follows from the syntax inventory.

The first newer hop, ACP 0.101.0, reaches the wrapper caller that imports
Agent SDK 0.3.318's `sdk.mjs`; ACP 0.106.0 imports the corresponding root of
SDK 0.3.322. Each selected SDK root contains a child-process import, aliased
`createRequire` calls and `Function` identifiers. In the frozen 0.3.322
source, one resolver call appears inside the native CLI lookup when no
executable path is supplied. This locates a conditional selected-source path;
it does not prove that the branch fires, identify every native or worker
target, or bind the bytes eventually executed. Each hop retains its own
package and source hashes in the two inventories.

The conservative wrapper builtins include `node:fs`, `node:fs/promises`,
`node:path` and `node:os` for filesystem and host information; `node:module`
for module resolution; `node:child_process` for process creation; and
`node:crypto`, `node:readline`, `node:stream/web` and `node:util` for library
operations. The first three hops also include `node:string_decoder`.
This is an inventory of reachable source imports, not approval to expose any
builtin. `node:module`, filesystem access and child creation need an exact
host policy; CommonJS, worker and native paths remain closed until the
protected loaded-byte boundary and runtime custody are established.

## Trusted-host runtime custody ruling

Tom's decision `fdd2eb69-e5c7-4743-bde5-fd5be30f7c1f` v2 selects
trusted-host runtime custody for this guarded ACP invocation. It supersedes
the earlier pending custody choice; it does not make path-based spawn an
OS-enforced verified-execution primitive or cover a hostile same-user
replacement. The host must select and pin the runtime, protect its custody
through spawn, and bind it with the exact launch and captured graph.

`LocalTrustedRuntime` opens a canonical host-selected executable, verifies a
bounded SHA-256 digest, retains its file handle and rechecks identity,
contents and revocation. Fake prepared-driver tests derive their candidate
recipe from this held runtime and reject a revoked runtime; separate fixture
tests exercise mutation, path replacement, links and revocation. Candidate
launch verification also compares the host-held runtime, startup filter and
exact recipe prefix; a substituted preload prefix is refused by the fake.
The ordinary candidate start compares the pinned runtime digest and exact
prefix, then holds its custody lock through spawn; revocation takes the same
lock. Runtime files owned by another unprivileged user, group- or world-writable
files, non-executables and links are refused. The host must still ensure the
loaded interpreter dependencies are under its control. The candidate now also
records and rechecks each canonical ancestor's identity, ownership and mode;
writable, replaced and linked ancestors are refused. This is metadata checking
within trusted-host custody, not an OS-enforced protection against same-user
mutation. The fake prepared driver
exercises the candidate host method. This remains a
building block, not a registered checked grant: it depends on trusted-host
protection of the runtime path, does not identify the selected official graph,
and does not protect native, worker or direct builtin use. The captured-loader
bootstrap now uses synchronous hooks and a fake explicitly admitted
`node:module` edge to serve nested CommonJS bytes after source replacement.
This candidate graph is caller-supplied, not an official closure or host
attestation; direct builtin access and runtime dependencies remain unbounded.
At that increment the real local process service still refused checked launches.
The earlier conditional statement that the operator's custody choice is
pending is superseded; original/native proof and separate execution authority
remain open.

## Local checked registration and remaining graph gate

The local process service now implements checked preflight and start for
explicit host registrations. A registration binds an independently approved
compound fingerprint, one executable, the immutable captured recipe, exact
allowed argument vectors and a revocable runtime pin. Both checked operations
resolve that registration. Start holds the registration and runtime custody
locks across synchronous spawn; neither lock crosses bootstrap suspension.
Unknown bindings, target/argument substitution, a changed prefix, unsafe
startup environment, changed runtime dependencies and revoked custody refuse
before spawn. Ordinary start refuses a registered checked executable. Bootstrap
failure stops and joins the owned process; existing bootstrap cancellation and
descendant tests remain covering proof.

`LocalTrustedRuntime::capture_with_dependencies` pins the host-selected
non-system interpreter files and binds their exact paths and digests into its
identity. Completeness, system-loader custody and stability through spawn are
explicit trusted-host preconditions. This is not dependency discovery or
same-user-resistant execution. Root-owned sticky temporary ancestors are
accepted with their metadata recorded; writable non-sticky ancestors refuse.
The prepared integration now uses the production local checked service rather
than implementing acceptance in a fake `ProcessService`.

The production guarded factory now captures the complete official per-hop
entry/import closure:242–287 sources, including statically linked v2 modules
without claiming v2 support. The frozen syntax inventory and package exports
independently reconstruct each manifest with `build-claude-acp-guard-graphs.py
--check`; missing/extra files, formats, hashes and edges fail exact comparison.
The source reader can regenerate the same manifests from verified archives
without executing any vendor source. Native resolver identities remain bound
separately for Darwin arm64 and Linux x64.

The synchronous loader serves captured ESM, CommonJS and JSON. Its
`node:module` facade exposes only caller-bound `createRequire` and resolution;
raw caches/hooks and foreign parents refuse. Child creation accepts only the
held exact native, plus the exact root-owned `ps` binary for the session-index
caller and its bounded PID/start-time query. It rechecks source/ancestor
identity at use. Startup disables addons and string code, direct builtin access
refuses, and workers/unknown imports are not admitted. Other reviewed builtins
retain their normal capability under the trusted host; this is no general
JavaScript sandbox. Frozen SDK `Function` schema fast paths require original
compatibility proof with string compilation disabled; fake success cannot
supply that conclusion.

Real prepared-driver fake tests now use the production checked service and
manifest reader, including ESM/CommonJS/JSON, native resolution/execution of an
owned shell fake, caller-bound `ps`, source replacement after capture, absent
permission callback with denied activity followed by continued output, and
joined session cleanup. These establish integration only. They execute no
original ACP, SDK or native source and qualify no newer point.

`guarded-probe-proposal.json` retains all sixteen route/native tuples and exact
Node22.23.2 archive/executable identities. It adds a runnable, separately scoped
idle-class component stage: a first-party Rust checked-launch owner under a
Seatbelt read whitelist and deny-all network policy. The owner verifies
kernel-denied file/network counterexamples before launch. A deliberately
unguarded control must arm/fire; the production guard must veto both while
preserving settings. Controlled clocks exceed the finite timer maximum;
new/load/resume are explicitly modeled contexts, not original ACP sessions.
Owned compact promises, child exit and watchdog join are observed.

At this preparation stage only repository-owned fake component execution was authorized.
Original mode requires exact source, plan, owner executable, Node and class
identities approved separately. It executes only the idle class, no ACP
entrypoint, SDK/native/provider. Linux component execution currently refuses
without admitted confinement integration. Full native permission and lifecycle
proof still needs a separate confined driver and genuine administrator fixture
environment; task234's authority does not transfer. These are separate proof
stages, not a completed qualification. Contract029 keeps0.100.0; Contract061
cannot project the guarded permission revision as qualified until the actual
route-specific proof passes.

The initial Homebrew runtime admission refused `untrusted runtime ancestor`.
A subsequent unoptimized debug attempt exhausted the40s owner bound before a
positive witness; it earns no cleanup or qualification credit. No owned Node
process was present in the later process snapshot; absence is not joined proof.

The worker resolved the runtime path inside this run: the frozen official
Node22.23.2 Darwin arm64 executable was copied from its SHA-verified archive into
a fresh owned fixture, with no installation or PATH change on the host. Only
first-party fakes used it. `sha2` is optimized at level3 in development/test
profiles so full113MB source checks fit the finite probe; no byte check was
removed. Node22's `createRequire().resolve()` bypasses synchronous hooks, so the
facade now resolves from its captured edge table, never the filesystem fallback.
Cancellation proof waits for the child-ready PID before dropping the pending
bootstrap and observing its exit.

All15 captured-child tests passed on that exact Node22.23.2 fixture in12.160s.
This includes production checked-host preparation, ESM/CommonJS/JSON, owned
native fake and caller-bound `ps`, denied activity followed by continued output
without a permission callback, source replacement, cancellation and joined
cleanup. It does not execute the original provider.

The confined first-party component witness then passed on Darwin arm64
kernel25.6.0. Its unguarded control armed12 timers and fired15 times; the guard
armed0 and fired0. Both ran the entry, preserved settings and joined their owned
child/promise/watchdog cleanup. Kernel-denied filesystem and network
counterexamples preceded launch. Original/native activation remained false.
The tested owner SHA-256 is
`e9370d1db6fe59c40ed88580424595e762da2bf14588f43380a03f359b1e5564`;
the exact Node SHA-256 is
`18e387c90ab8a8400183e8bdd396376e1e875b91b4c874b894dcade7b35bf572`.
This establishes the bounded fake harness, not original-class, original ACP or
native behavior. The runnable component proposal still needs separate exact
execution authority. Linux confinement and full native permission/lifecycle
proof remain independent gates.

Official npm metadata rechecked2026-11-13T12:58:45Z: latest remains0.106.0;
the complete stable set above0.100.0 is unchanged. Canonical pushed main
`1cbb29f459950a76fb9104fb10947fd87694e483` adds the separate SDK fixture workflow
registration, with no ACP authority or proof transfer.

The startup filter also rejects external `OPENSSL_` configuration/module
controls before spawn, including inherited/bootstrap environment bindings;
the native facade applies the same refusal after settings inheritance.
[Node22.23.2 CLI documentation](https://nodejs.org/download/release/v22.23.2/docs/api/cli.html#openssl_conffile)
identifies `OPENSSL_CONF` as startup configuration. This prevents an unadmitted
crypto configuration from entering the guarded launch; permission settings
remain inherited unchanged.

The final startup-control closure repeated the confined fake witness with the
same12/15 control and0/0 guarded outcomes. Current tested owner SHA-256:
`1c5df7bbfbd940d77d5182d7c71a6e63753bcf14925bd878fb77ace56d20e51c`. The earlier owner above remains
historical fake evidence; this final artifact is the proposed original-class
probe owner. Neither run activated original or native provider code.

## Later stable discovery

Official npm latest moved to0.107.0 at the final recheck2026-11-13T14:40Z,
after the frozen0.106.0 identity at4332906a4 (2026-11-12). npm publication time
is `2026-11-13T13:39:38.999Z`; published git head is
`c5c120dba9c15bd88ea5d355fa3b77d1fc341392`. This is discovery metadata, not tarball or behavior
qualification. Contract029 In-Run Latest Movement keeps0.107.0 unverified for
the next family run; the guarded factory refuses it. This task retains the
accepted0.106.0 tuple set and original-execution gate. The planner owns its
follow-up; no support or authority transfers.

Rust1.100 CI required fixed-size hex decoding via `as_chunks`; the repaired
decoders retain the1.95 floor. The current proposed probe owner SHA-256 is
`807b2c75bd0d9c67c6a1e024648efab109ea4249542396b43c8ada8fe66475f9`, re-proved on the same exact Node22.23.2
with identical control/guard outcomes and compound fingerprints. Earlier owner
digests remain historical evidence, not the current activation artifact.

### Installed-runtime direction and bounded fake proof

Tom's 2026-11-13 decision `d0a0761f-3c47-4cf2-b198-242d232b20cc` v2
supersedes the exact22.23.2 implementation prerequisite. Upstream ACP requires
Node>=22; the guard additionally needs synchronous `registerHooks` and builtin
classification, checked before captured entry effects. Existing22.23.2 results
remain reference evidence. No additional runtime was staged for this change.

The installed Darwin arm64 Node26.11.0 executable has SHA256
`832eaf52cafa7ff1364c2ca378451db14fb043d8d23a5cfd18a0a103d66226fc`.
Read-only Mach-O inspection identified96 non-system library files. The retained
invocation record binds their exact paths/digests and the host's trust policy;
its SHA256 is
`d566f2d8c15c30ad3627fc6def7488f8c775c38db7b25bf8565dff6cd42fbca0`.
The host trusts root, execution user501 and administrator group80 to coordinate
runtime updates through spawn. Explicit group trust admits the Homebrew
ancestors; unknown groups, other writes, changed files/ancestors and revocation
still refuse. This is the approved trusted-host precondition, not OS verified
execution or protection against hostile same-user mutation.

The first confined attempt exposed Node's compiled-in OpenSSL configuration
read before JavaScript startup. Seatbelt refused it. The fixed checked prefix
now supplies `--openssl-config=/dev/null`, closing that startup configuration
edge without allowing the host configuration file. Ambient `OPENSSL_`, `NODE_`,
`LD_` and `DYLD_` controls remain rejected. Both raw resolver behaviors are
covered:22.23.2 can bypass synchronous hooks in `createRequire().resolve()`, while
26.11.0 observes them. The guarded facade resolves from its captured edge table
on both, rather than depending on either raw behavior.

The first-party confined owner SHA256 is
`b3c707d7ff58f75c7611f4cf27b265314010d9f8c3bf8a15bd58278cc2cabda0`.
Its fresh installed-runtime fake control armed12 timers and fired15 compactions;
the guard armed0 and fired0. Both entered, preserved settings and joined all
modeled work. The retained results report Node26.11.0, Darwin arm64 kernel25.6.0,
`nativeExecuted:false` and `routeQualified:false`. All17 captured-child tests
also passed using the actual installed interpreter and inspected library
closure, including production checked-host prepared-driver paths. New
counterexamples cover explicit group admission, changed dependency identity,
missing hook APIs and revocation. A PID-file readiness race in the cancellation
fixture was corrected to await a parseable PID before cancellation.

Covering selectors for this coherent change include
`validate:acp-guard-installed-child` (the new Darwin-only closure wrapper),
`validate:acp-guard-component-probe`, the existing currentness/proposal,
host-environment, prototype, compile/lint/MSRV/API/format/package selectors,
`validate:oxide-routing`, docs, routes and retired concepts. The installed
runtime wrapper uses the maintained Oxide helper; it never executes original
ACP/SDK/native code or mutates installed files.

The updated proposal selects an installed compatible runtime per invocation;
its22.23.2 tuples are explicitly historical references. Earlier exact-source
execution requests cannot authorize these changed bytes. Retained review and
separate exact original-component authority remain required. Whole-route
native/permission proof, Linux confinement and administrator-fixture admission
remain independent. The qualified ceiling stays0.100.0; standalone SDK/Node,
platform and exact HTTP MCP claims do not change.

### Retained component review and admission repairs

The retained reviewer accepted the c025c5c fake witness as bounded evidence and
withheld original activation. The subsequent repair rechecks registration and
runtime/native/helper revocation before sending graph bytes and serializes the
final admission-byte write under those custody locks. It releases every lock
before an async wait. A real checked-host test revokes registration or runtime
after spawn, before polling bootstrap, and observes refusal plus joined child
and process-group-owner cleanup. The observer PID is excluded; both owned
children must be present before revocation. All18 captured-child tests passed on installed
26.11.0 with the inspected closure.

The component now consumes host-owned principal lists and compares the approved
Rust closure-v2 digest and launch-policy digest before launch. It prints the
held executable digest, not a fresh path hash after execution. The fake host
resolves the macOS `admin` group; the Rust owner no longer infers its trusted
user from fixture ownership. Both the component and wrapper refuse Linux;
the installed-child selector reports not-applicable there without claiming
proof. New fake counterexamples reject wrong Rust custody, launch policy and
principal records; a matching approved record succeeds.

The Seatbelt exec allowlist now contains only selected Node, the initial owned
Rust executable, and the system process-group-owner helper `/bin/sleep` required
by the existing local host's joined-tree contract. Runtime libraries retain
read/map access only. The helper digest is included in Rust runtime custody.
Readable `/bin/echo` must fail exec before Node launch, alongside the independent
file and network canaries. The concurrent bound is4 (wrapper, Rust, Node, group
owner), with6 owned spawns across the two serial launches. Runtime inspection
bounds129 files,512 aliases,512MiB,5seconds per tool and1MiB per output stream.

The exact selected Rust1.97.1 Darwin source path and no-fallback preconditions
are retained in `darwin-spawn-path.json`. The absolute Mach-O invocation uses
`posix_spawnp` and its process-group attribute, with no pre-exec closure,
identity/session change or cwd. This is source evidence for the built owner
under trusted-host custody, not OS verified execution; Linux original execution
remains unavailable. The host coordinates loader/helper updates through
bootstrap admission.

The appended idle patch cannot intercept module top-level evaluation. Original
component admission therefore also pins the audited prefix SHA256
`9178cba38b0cd5451ff1425b2cfa762882fb13662c30fd13f7530f6673fd653b`,
which contains constants, one bounded intrinsic Set allocation and function
declarations. Prefix audit, full digest and captured graph use the same verified
in-memory class bytes; capture refuses any intervening disk replacement. The complete class digest stays mandatory; static initializers,
extends and computed members are outside this admitted source. The output now
describes the actual two-file graph, entry/class digests and absence of native
bindings/platform programs, rather than using constant native/qualification
flags as proof.

The refreshed fake owner SHA256 is
`3ac9390096301c89275cb15639e9ffa36288d279f3dedfc6f3a965361fa13a89`.
Its approved Rust custody identity is
`0878e1f6a91209a907f5d56ef195bcd556c31084b7225e022d355b9038501cb0`;
the launch policy is
`0af0ee5f8a6eed6c99abfae20559d22c05e2be1bf0a276442592425a84ff95da`.
The readable host-record digest is separately
`e8cae5c0e222aebed4af2a209977a4e349c958d8f6cbbd2e20b821ca450cfaa4`.
Both serial launches preserve the12/15 control versus0/0 guard distinction,
settings and joined modeled work. These changed bytes need fresh retained
component review and exact original authority. Wider permission/native and
qualification findings remain open; no claim or execution gate was waived.

### Conditional component disposition: confinement and prefix identity

The retained reviewer passed the repaired fake witness and conditionally
accepted scheduling a separate original-component authority request. It does
not qualify the route. The remaining confinement naming/binding and per-hop
prefix conditions are addressed here.

`launchPolicyRevisionSha256` now names the revision tag0af0…95da. It is not the
Seatbelt profile hash. Original mode separately requires an approved
`confinementPolicyTemplateSha256`, checked before spawn against the exact
profile with only the fresh owned scratch-root literal replaced by
`@OWNED_COMPONENT_ROOT@`. No other policy field is substituted. Each attempt
records its actual profile bytes, actual profile SHA256, template SHA256 and
owned root, permitting independent reconstruction. Wrong template approval is
an additional fake refusal before Node entry.

The observed installed-runtime template SHA256 is
`50970df3dc04a6efb4a5e913b1feab40bb356848f945835954f970f79f97b1ab`.
The fresh fake profile SHA256 is
`d389538b007acfc736d0415c5f041ff10ab2cd67a75d16c2b730e8cb27477c27`;
its complete source is retained with the witness. The updated first-party
owner SHA256 is
`f5fafa02ca0892fb1580d8fcaffbffb72f56b4c4e054e0e8d02c3fa29e564588`.
Matching approval succeeds; wrong Rust custody, policy revision, principals
and confinement template all refuse. Control12/15 and guard0/0 remain joined
with settings intact.

`idle-prefix-inventory.json` records a separate read of each of the eight
selected shipped modules, verifying its complete class digest before hashing
the prefix. All eight prefixes are1558bytes and match9178…653b, across the two
complete-module digest groups. The exact ledger, hop set, per-hop class and
prefix digests are regression-pinned; no endpoint inference or original module
execution was used.

The review caveats remain explicit: Rust1.97.1 `posix_spawnp` is source evidence,
not runtime exec tracing; Linux original mode refuses;26.11.0 proof is exact to
that installed invocation and22.23.2 remains historical. The inherited sandbox
permits `/bin/sleep` for the host's group owner, so the idle class could exec it;
this is not a hostile-JavaScript sandbox. `/bin/echo` remains the denied-exec
counterexample. Full native/permission, platform and exact HTTP MCP gates and
the0.100.0 ceiling remain unchanged.

### Original idle-class observations

Tom's decision `2f4d33df-aaa6-4497-80c1-8c56fc6c2ccc` v2,
answered2026-11-14, authorized the eight serial offline component checks at
immutable source `c0d591438590f77f502650a47b44e96295cf23f3`.
The grant superseded the earlier return-to-planner disposition only for these
exact idle-class inputs. Before every pair, the worker re-read the frozen
source/plan/command/owner/class bindings, inspected the actual97-file Node
and non-system library closure, recomputed the Rust custody identity including
file/ancestor identities and trusted principals, and checked prior attempts.
Retained Nucleus records showed the earlier restart sent no continuation and
the grant was unused. Each attempt was recorded before launch; none was retried.

The [machine-readable observations](483-claude-agent-acp-idle-component-observations.json)
bind the source, committed plan, detached proposal, command set, built owner,
guard, installed Node26.11.0, host custody, confinement and original class
digests. The detached proposal SHA256 is
`50076fe81644ce772a8aecd859f258f86faa63e4ce58fbac7adcfab88d349086`;
the eight-command SHA256 is
`40b5d96b6e766b68653191054e1b2626282f34be6d70ee4573e3e402e0281471`.
The same approved owner and runtime were present; neither was rebuilt,
substituted, installed or staged. Commands used this run's fresh scratch root;
the approved template permits only that owned-root substitution.

| Original ACP idle class | Attempt | Control arms/fires | Guard arms/fires | Owners / cleanup |
| --- | --- | --- | --- | --- |
|0.101.0|1|6 /5|0 /0|Both exit0; joined; fixture removed|
|0.101.1|1|6 /5|0 /0|Both exit0; joined; fixture removed|
|0.102.0|1|6 /5|0 /0|Both exit0; joined; fixture removed|
|0.103.0|1|6 /5|0 /0|Both exit0; joined; fixture removed|
|0.104.0|1|6 /5|0 /0|Both exit0; joined; fixture removed|
|0.104.1|1|6 /5|0 /0|Both exit0; joined; fixture removed|
|0.105.0|1|6 /5|0 /0|Both exit0; joined; fixture removed|
|0.106.0|1|6 /5|0 /0|Both exit0; joined; fixture removed|

Each owner verified kernel-denied network/file access and readable `/bin/echo`
execution before Node launch. Each admitted graph contained exactly the two
captured JavaScript files, no native binding and no platform program. Its
guarded outcome preserved inherited settings and locked prototype/own-method
replacement. Controlled clocks exceeded the finite timer maximum; direct fire
was also vetoed. Original controls differ from the earlier first-party fake's
12/15 outcome: the observed original result is6/5 on every hop.

The wrapper observed both owner exits; the Rust owners returned only after
Node `wait` and watchdog `join`. The first-party compact promises joined and
all eight owned fixtures were absent after successful teardown. Bounds stayed
40s per owner,10s child watchdog,30sCPU,256fds,1MiB per output stream,
64MiB scratch,4 concurrent owned processes and6 total launches per pair.
Raw per-hop output, actual generated policy bytes/hashes, preflight identities
and start/settlement records were delivered as retained per-head proof.

All eight grants are consumed. This proves the captured original idle class
under first-party host/settings/clock/compact promises on Darwin arm64, with
new/load/resume modeled rather than actual ACP sessions. No ACP entrypoint,
Agent SDK, original native binary, provider request, credentials/spend, Linux,
canonical-fixture retry, host mutation or release was executed. Rust spawn
evidence remains source-only; trusted-host custody and the deliberate
`/bin/sleep` allowance remain necessary, with no hostile-JavaScript or
same-user-resistant verified-exec claim.

The next gate is a route-specific confined prepared-driver proof of actual
ACP session paths and inherited native permission denial without a callback,
continued turn, cancellation and joined native cleanup. It needs admitted
transport/administrator-fixture inputs and separate exact ACP/SDK/native
execution authority; task234's fixture or custody authority does not transfer.
Contract029/061 still forbid a qualified projection of the guarded revision
from this component evidence. The ceiling stays0.100.0, earlier stock and
exact HTTP MCP points remain intact, and0.107.0 remains UnverifiedNewer.

### Retained component disposition and full-route source/fake preparation

The retained reviewer accepted the eight original idle-class pairs at
`530bd06df` in question
`a777c09a22c3eed3af7d90ef1e981d6fb78105d2e090dc7b2118d18286f912fc`,
as bounded component evidence only. All wider qualification findings remain.
The grant is consumed, with no retry or authority transfer.

Independent ACP source/fake preparation now uses actual prepared-driver
new/load/resume/prompt operations through checked captured Node loading. Its
owned SDK hook starts a real child peer; the peer reads owned user/managed
settings, asks before a permitted tool read, or denies without invoking the
callback, then obtains continued output from an owned loopback HTTP endpoint.
The fake persists the selected model across real load/resume requests. Ordered
adapter events distinguish activity, callback and continued output. A pending
prompt is cancelled through the wire; the SDK observes child exit0, then the
session releases its resources. Session cleanup runs even when a test assertion
fails. These are first-party software integration observations, not execution
of upstream ACP, SDK or native code, canonical administrator discovery, kernel
confinement or independent whole-subtree settlement.

Task234 owns the shared private supervision/native-entry and combined canonical
fixture proposal. The agreed manifest/event seam is private; its exact reviewed
source/digest is required before binding it into ACP execution. The ACP route
uses its upstream entry and SDK child-process facade, not Swallowtail's standalone
SDK sidecar, so the shared boundary must cover that spawn path. Software
ChildProcess waits and cooperative fake entry release cannot establish an
opaque original native entry gate or pidfd/cgroup/adoptee settlement. The
[full-route preparation procedure](../knowledge/operations/claude-agent-acp-full-route-proof.md)
names the runnable fake checks and the exact remaining composition prerequisites.
No new kernel/workflow/admin or original entrypoint/native/provider activation
follows from these tests.

### Reviewed shared-source composition preparation

Task234 source/security review5903353390 approves the software seam at
`f9025d2197333c53aed2266627dfa39d05bff7e3`, private-spawn.v1 plan
`3956dfa3fefeceffdf5501e2f7e81dbb8bd50c4c83baff08b25b3ddd89da1fe4`
and record `8a819724898208411b81977abbadf05084545a2f6c179ecb08d9775df99b00a9`.
The bounded ACP snapshot retains those exact sources. The ACP-private adapter
uses their process observation/validation, exec entry and caller-bound child
facade while preserving ACP captured loading and idle veto. Its graph records
actual Rust-owned sibling group leader/checked ACP actors, not the shared
model's driver-under-group-owner stand-in. No shared kernel/fixture branch runs.

Development refusals exposed an owned SDK prompt waiting after child failure,
and the installed Xcode Python launcher re-executing the framework app binary.
The fake now rejects a pending prompt when its child exits; the runner selects
and pins the actual installed framework interpreter directly. These repairs
need the targeted integration proof; they are not original runtime admission.
The first failed integration run also hit Nextest's120s timeout; Effigy then
reported a foreground guard closure timeout and an unknown journal. Preserve
that failed run separately from any later successful proof. No observation of
absence earns parent/adoptee wait or whole-subtree settlement credit.

The [inactive proposal](../knowledge/operations/claude-agent-acp-full-route-proposal.json)
names exact current software sources and the remaining shared host, canonical
fixture, vendor graph and Python/cache custody prerequisites. OriginalCommand
remains null and the original component grant remains spent. Ceiling0.100.0,
newer guarded permission, native/platform and exact HTTP MCP findings remain.
