# Selected ACP invocation preparation

The [private manifest](claude-agent-acp-invocation-manifest.json) derives24 caller
profiles from the [reviewed source record](claude-agent-acp-source-flow.json):
eight hops, each with native auth-status, SDK transport and registry-ps. It binds
caller file/hash, exact callsite bytes, imported API, argument expressions,
graph identity and native image identities. CLI delegation, auth logout and
SDK utility execution are excluded. This is a selected probe contract, not a
new public route or a general process sandbox.

**Actual bindings remain unfilled.** `actualPlan` and `originalCommand` are null.
The manifest's closed admission and0.100.0 ceiling are invariant. Source
expressions, synthetic declarations and matching metadata cannot establish a
host observation or an execution grant.

## Exact reference before a launch

The trusted host must independently construct an exact reference before the
untrusted caller runs. Never create that reference by copying the request it is
supposed to constrain. Each reference names one hop, one logical probe context
(`new`, `load`, `resume`, `prompt`), canonical fixture label/digest, source-bound
profile, and a unique launch/entry allowance. These context labels add no ACP
wire operation. No phase order or process count is inferred: a prompt can reuse
a query and have zero new SDK starts. Enumerate every physical launch explicitly.

The concrete launch tuple includes executable and image hash/size, argv, exact
effective env/cwd/three-pipe stdio, detached/shell, timeout, windowsHide, abort
handle and three descriptor identities. Absent Node options are resolved in
this tuple, while the source profile preserves whether stdio was explicit or
an execFile default. Registry-ps has its own held utility identity; it never
spends a native transport allowance. A raw `ps` name is not the resolved held
executable path. No PATH search or environment inheritance is inferred.

The current production facade knows the captured import parent, not a trusted
runtime callsite offset. A future source-bound mediation adapter must establish
that identity before effect; caller-supplied JSON cannot establish it. The
manifest records that observation as unresolved rather than treating its source
callsite hash as runtime provenance.

This first owned subprofile accepts auth `auth status --json` and the existing
four-cell bounded registry query (timeout1000, decimal PID CSV<=4096, C/UTC).
The SDK subprofile accepts only the source-shaped stream-json base plus stdio
permission callback flag. Its **full ACP option tail is unresolved**: replay
args, tools, settings, resume and other actual option branches are not guessed.
Any additional tail refuses with `sdk_option_tail_unbound`. Per-call settings
overrides, client extraArgs, executableArgs, custom spawn and alternate native
resolver branches also refuse. This does not disable inherited user/managed
settings; their actual fixture/runtime custody remains a separate requirement.
Only concrete, independently source-resolved values may extend this subprofile
through review. The current manifest supplies no usable original launch plan.

## Reservation and pre-entry

`scripts/claude-acp-invocation.py` supplies a pure `OwnedPreEntry` metadata model:

1. Validate the frozen source manifest and exact packet fields. Only
   `owned-metadata-counterexamples`, `originalActivation:false` packets exist.
   Unknown profiles, mixed hops, unresolved values and unsupported branches refuse.
2. Spend a launch allowance **before** comparing the complete observed tuple,
   fixture and context with the reference. Mismatch revokes the scope; it does
   not refund the attempt. Extra fields and boolean/number substitutions refuse.
3. Before payload effect, spend its entry allowance and compare the tuple again,
   plus fresh actor identities, wrapper argv, entry source/configuration hashes
   and fixture digest. A PID/start, parent/group, env, FD or entry change refuses.
   Revocation, duplicate release and entry without reservation refuse.

The actor relation is the actual ACP requirement: prepared Rust owner parents
the group owner and checked ACP as siblings; the group owner has its own PID as
PGID; checked ACP joins that group; each entry is checked ACP's child in the
same group. Monitor parentage is not invented. The model requires exact command
vectors; a `ps` commandText string is not reparsed into argv evidence. Actual
host launch custody must bridge that observation gap. Namespace/controller,
credential, source/runtime, software wait and kernel settlement bindings belong
to the shared host, not these synthetic records.

No method launches, releases an OS process, observes the host or forwards a
callback. Successful comparisons still return `originalAllowed:false`,
`hostObserved:false` and, at pre-entry, `kernelEnforced:false`. Its in-memory
one-use sets are not durable admission or a restart-safe grant ledger. Copied
JSON, invented PIDs/hashes or a new model instance cannot confer runtime custody.

## Shared entry boundary

The source requirement retains the accepted f902 software entry identity as
history. It is not a claim that its Python/runtime inputs are currently admitted.
The [startup integration](claude-agent-acp-startup-integration.md) consumes only
the exact migration accepted at SDK240 commit3a9ca6b6, review5919931970. Its
owned actual sibling checks are separate from this unchanged metadata model.
The 24 profiles are eight hops times three caller kinds, not24 observed launches
or a phase/process schedule. Full option-tail and runtime caller provenance
remain unresolved; source-shaped startup success supplies neither.

The next real packet also needs full ACP source option resolution, same-invocation
trusted caller/entry and descriptor bindings, actual host/kernel/transport and
combined canonical fixture admission, opaque native/Python closure and separate
original execution authority. Source matching never waives the required native
permission-without-callback, continued-turn or lifecycle proofs.

## Covering proof

```sh
effigy build:acp-invocation --output docs/knowledge/operations/claude-agent-acp-invocation-manifest.json
effigy check:acp-invocation --record docs/knowledge/operations/claude-agent-acp-invocation-manifest.json
effigy compile:acp-invocation
effigy validate:acp-invocation
effigy check:python-format scripts/claude-acp-invocation.py scripts/tests/claude-acp-invocation.py
effigy validate:oxide-routing
effigy qa:docs
effigy skill run northstar/retired-concepts
```

Owned counterexamples exercise each hop/context/caller, exact tuple and fixture
drift, unsupported settings/argv/environment branches, the wrong stand-in actor
graph, PID reuse, duplicate allowances and revocation. They execute no vendor
module, provider turn, kernel controller or canonical host setup. Unchanged
source-flow and older software suites are not repeated for this preparation.
