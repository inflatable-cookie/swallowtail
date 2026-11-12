# Research 482: Claude Agent SDK selected flow and the native proof plan

Status: evidence and a plan. No claim moves. `claude-agent.sdk` stays qualified
through SDK `0.3.317` with native `2.1.317`, `QualifiedOnly`, with the default
pair, the registered-tool live point, the Node segment and the wire unchanged.

Route: `claude-agent.sdk`
Axes: `claude-agent.sdk.package` (`0.3.284..=0.3.317`) and
`claude-agent.sdk.native` (`2.1.284..=2.1.317`), point-for-point coupled.
Node `22.23.2..=22.23.3`, wire and sidecar source stay separate.
Not a Claude Code, ACP, or shared-schema claim.

This record closes Q-009 item 3 (the `0.3.321` residuals) and delivers the
harness and plan Q-009 items 1 and 2 need. It does not answer either of them.

## What was rechecked, and how

On 2026-11-12 the registry packument still reports `dist-tags.latest` and
`next` as `0.3.322`, with `0.3.320` absent and nothing published above
`0.3.322`. The five wrapper tarballs (`0.3.317`, `0.3.318`, `0.3.319`,
`0.3.321`, `0.3.322`) were fetched into the run scratch, and every one
reproduced its frozen SHA-256 from the accepted corpus:

| Hop | Tarball SHA-256 (reproduced) |
| --- | --- |
| `0.3.317` | `e91e5a43…85112f` |
| `0.3.318` | `7a79b4aa…b27836e` |
| `0.3.319` | `6bfb9ccb…a2d302001` |
| `0.3.321` | `c8685dc9…3ef6f5` |
| `0.3.322` | `5db6881a…28c59a93` |

All 21 packaged files of each hop also reproduced the per-file SHA-256 ledger in
`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.322/dist-inventory.json`.
Nothing was installed, extracted into a host path, or executed.

The native binding was read from each hop's shipped `manifest.json` rather than
from `latest` metadata. Every Darwin arm64 and Linux x64 binary digest in the
plan equals the manifest checksum and the size the manifest declares; where the
repository retains a CLI fixture that binds the same build, the planned digest
appears in that fixture. `2.1.318` has no retained CLI digest, so its manifest
is its only identity, and the plan says so by leaving `cli_cross_binding` null.

## Method

`scripts/claude-agent-sdk-selected-flow.py` classifies only the file the route
actually loads. For every selected surface it takes a bounded window (a
statement-aligned window, a brace-balanced definition, or the enclosing
function), then compares hops with three facts:

- the **masked sequence digest**, where every identifier that is not a member
  access, every string literal and every number becomes an opaque marker, so a
  minifier rename cannot masquerade as a change;
- the **masked multiset digest**, which separates a real edit from a pure
  reordering (a destructured options list, for example);
- the **exact byte spans** of any window whose sequence digest moved, reported
  with both file offsets.

The final token of a byte-bounded window is dropped before comparison, because
one upstream insertion can otherwise cut the window's last token in half and
make every hop look changed. Every anchor was verified to occur exactly once;
an absent or ambiguous anchor is reported as such, never as unchanged.

The classification is reproducible: the same script, run over extracted trees,
refuses a tree whose bytes disagree with the committed ledger and re-emits
`selected-flow.json`. `effigy validate:claude-agent-sdk-selected-flow` runs its
self-test, which proves the refusals and the masking against synthetic fixtures.

## Selected surfaces, hop by hop

The accepted base and every published hop agree on these surfaces, masked and
byte-bounded:

| Surface | Result |
| --- | --- |
| `query.streamInput` | identical at all five hops |
| `query.writeStreamMessage` | identical at all five hops |
| `query.endRun` | identical at all five hops |
| `query.armRunEndCeiling` | identical at all five hops |
| `query.initializeWritten` promise and resolver | identical at all five hops |
| `query.processControlRequest` `can_use_tool` branch | identical at all five hops (the permission-request fields the branch projects, including `serverPolicyAskOnly` and `decisionReasonType`, exist from `0.3.317`) |
| `query.promptHold` assignment and `writeBehindPromptHold` | identical at all five hops |
| transport argv construction | identical at all five hops |
| `Options.managedSettings` → `--managed-settings` | identical at all five hops |
| the `settingSources`/`parentManaged` resolution | identical at all five hops |
| the managed-policy key table entry for `permissions.denyInsteadOfPrompting` | present from `0.3.319`, identical afterwards |

Two surfaces do change inside the analysed span:

- `0.3.319 → 0.3.321`, `query.writePrompt`, 136 bytes removed and 163 added: the
  string-prompt path stops calling `writeAfterInitialize` and the transport
  write directly, and starts routing both branches through `afterInitialize`
  and `writeStreamMessage`. This route passes an async iterable, so
  `streamInput` is the selected path and it already wrote through
  `writeStreamMessage` at every hop.
- `0.3.319 → 0.3.321`, the recursive key sanitizer gains a telemetry-gated
  numeric-key condition:
  `o!=="__proto__"&&(o===n||!cD(o)||Number(o)<64||!uD("cleaned_key"))`. Here
  `cD` is the canonical numeric-key predicate (`/^(?:0|[1-9]\d{0,9})$/` and
  below `4294967295`) and `uD` consults the telemetry limits gate, not a
  feature flag. The sanitizer's callers are the session tag and custom-title
  writes. No turn frame, argv, callback answer or result projection passes
  through it.

`0.3.321 → 0.3.322` is five changed spans in `sdk.mjs`: the version comment,
three SDK version stamps (telemetry `cc_version`, the MCP client `clientInfo`
version and `CLAUDE_AGENT_SDK_VERSION` in two places) and one added
`/fast`-related description string in a settings schema. No selected surface
moves.

## Residual closure

Q-009 listed three `0.3.321` residuals. All three are now classified from exact
bytes:

- **`promptHold`** — unchanged from the accepted base, and unselected. Its only
  assignment site is guarded by `hasHookSetToBlock()`, which reads
  `Object.values(this.hooks ?? {})`. The sidecar passes `hooks: {}`, so
  `promptHold` stays `undefined`, `streamInput` takes its direct-write branch,
  and the hold barrier is never armed. `initializeWritten` gates the first
  write in both hops and is unchanged.
- **`initializeWritten`** — unchanged as a promise and resolver. The only hop
  that touched its neighbourhood is the string-prompt `writePrompt` change
  above, which this route does not select.
- **the `Ki` telemetry callers** — the sanitizer gained the numeric-key
  telemetry gate at `0.3.321`, unchanged at `0.3.322`. Its callers are the tag
  and custom-title writes; the gate reduces a telemetry key set, not a wire
  payload, and is not reachable from the turn path.

## Dependency edges

The closure of `sdk.mjs` is bounded at every hop:

- the root export resolves to `sdk.mjs`, and the static ESM specifiers are Node
  builtins only (`node:*` plus the builtin bare names already reviewed);
- no dynamic `import(` appears;
- no `./core-*.mjs` chunk is named, so the rotating chunk files stay tree-bound;
- `import.meta` appears twice, in bundler helpers;
- seven literal `require()` specifiers appear inside the bundled `ajv`
  validator (`ajv/dist/runtime/*`, `ajv-formats/dist/formats`, `util`). They
  execute only when a JSON schema is compiled. This route never sets
  `outputFormat` or `jsonSchema`, so those edges stay unselected. The previous
  record said "static imports are Node builtins only" without naming these
  literals; this record does, and classifies them as unselected rather than
  pretending they are absent.

## Managed policy: what the bytes prove and what they do not

Q-009 item 2 asks whether `permissions.denyInsteadOfPrompting` can reach the
native under this sidecar's options. The source settles part of it:

- the key exists from `0.3.319` as an **administrator-tier** managed-policy key,
  with the documented meaning "a call that would prompt is denied at once, the
  model is told, and the turn carries on";
- the SDK resolves settings tiers separately from `settingSources`.
  `settingSources` maps user, project and local disk tiers; the administrator
  and policy tiers are not among them, so the sidecar's `settingSources: []`
  does not exclude managed policy;
- the SDK's own *parent* tier comes only from `Options.managedSettings`, which
  the sidecar never sets, and the transport emits `--managed-settings` only for
  that option. The parent slice therefore stays empty on this route;
- what remains is the native's own administrator-tier discovery (MDM,
  `managed-settings.json`, a policy helper, remote policy). That happens inside
  the bundled binary, has no SDK-side switch, and cannot be settled from these
  bytes.

So the honest statement is: **the setting is not excluded by anything the
sidecar controls, its honour and its bypass of `canUseTool` are native behavior,
and a product ruling is still needed.** The options Q-009 names — preserve the
managed denial under a distinct SDK behavior revision, or refuse an affected
pair — remain open with their gate owner.

One related fact belongs here because it constrains any proof: the SDK
description says allow rules, the permission mode, the auto-mode classifier and
`PermissionRequest` hooks are consulted before the denial. This route sets no
allow rules, uses the selected permission mode, and installs no hooks, so the
only thing that could pre-empt the callback is the native's own policy.

## The ruling: preserve the managed denial under a distinct revision

After this record's first stage, Tom answered Q-009 item 2 (decision
`d5a14f90-e837-42b6-9818-9b5e74a25547` v2, relayed by the planner as `task232`
brief v2): preserve the managed denial under a distinct SDK behavior revision
rather than refuse an affected pair.

The distinct revision is `claude-agent.sdk-v2`
(`CLAUDE_AGENT_SDK_MANAGED_DENIAL_BEHAVIOR`), and its contract is
`tests/fixtures/claude-agent-sdk-v2/managed-denial-contract.json`. It names the
selected surface it applies to and four guarantees, each pinned by a fake
regression:

1. a managed denial never becomes a host permission request;
2. a call the host never mediated is never reported as admitted or
   host-decided;
3. a managed denial is a completed turn with no error text;
4. a callback-mediated denial stays distinguishable by its callback record and
   its recorded decision.

The revision changes no claim. Every maintained segment keeps
`claude-agent.sdk-v1`, the package and native segments stay
`0.3.284..=0.3.317` and `2.1.284..=2.1.317`, the pair map keeps its 31 pairs, and
the shipped sidecar still reports v1 for the accepted pair; a library unit test
asserts that no claim adopts v2. The probe admission declares v1 for `0.3.317`
and `0.3.318`, where the setting cannot exist, and v2 from `0.3.319` on, so a
probe run records which contract it belongs to. What the bytes still cannot prove
is whether a real native honours the administrator tier and whether that
honouring bypasses the callback on every tool class; that needs the
original-artifact run, which stays unauthorised.

## Harness and plan

`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-native-proof/probe-plan.json`
fixes the artefacts, digests, selected argv, isolation, environment allowlist,
egress rule, bounds, observations, failure classes and cleanup for the four
candidate native builds on both selected platforms. Original-artifact execution
is `enabled: false`, and the harness refuses any request to enable it without
the authority the plan names.

`crates/swallowtail-adapter-claude-agent/tests/claude_agent_sdk_native_proof_harness.rs`
proves the harness against fake peers, over the real prepared-driver launch
parameters and the shipped sidecar. It covers the initialize handshake, the
streamed user message, assistant and result frames with usage, a typed failure
result, a callback round trip that reaches the native, a pre-callback denial
that fabricates no permission request, an interrupt with its terminal reason, a
native death mid-session that turns the session terminal, exact identity
substitution refusal before any peer runs, and a credential-free environment
with one controlled loopback request and no other egress.

The harness proves the pipeline and its observation points, not the vendor's
behavior. That distinction is the reason the plan exists.

### Temporary probe-only admission

The planner granted a harness-only probe admission on 2026-11-12 (question
`e36390b98c373c94c17185abe8fcf9a5cbd14dcde0d2b720cfa8320a76caec7e`): prepare
the bounded path, keep production claims and the maintained pair maps unchanged,
and authorise no vendor execution. `probe-admission.json` records it.

What the ledger binds:

- the exact proposed tuples - `0.3.318`/`2.1.318`, `0.3.319`/`2.1.319`,
  `0.3.321`/`2.1.321`, `0.3.322`/`2.1.322` - plus the accepted
  `0.3.317`/`2.1.317` control, each with its wrapper tarball digest and its
  Darwin arm64 and Linux x64 binary digest and size;
- the selected option surface, as a digest of the option template the shipped
  sidecar passes, with per-run paths and the environment list normalised out;
- the fake peer identity: the SDK module digest and, per tuple, the fake native
  digest, so a substituted peer is refused before it runs;
- the environment allowlist, the Node requirement, the plan and option-template
  digests and the same finite bounds as the plan.

What the path refuses, proven by tests rather than asserted:

- a ledger that claims production admission, a production mode, a foreign
  authority, or original mode without the exact token;
- a peer whose package version, native version or bytes are not the admitted
  tuple - including a newer tuple dressed as the accepted pair;
- a non-loopback endpoint, before any peer starts;
- a peer that outlives the probe bound: it is killed and reported as a cleanup
  failure, with `nativeExit.join: "killed"` in the evidence.

What it does not do: it never imports the adapter, never prepares a session,
never reaches the claim table or the sidecar's pair gate, and it writes every
record with `probeOnly: true` and `productionAdmission: false`. The harness also
asserts that the maintained pair map still lacks every proposed version and that
the shipped sidecar still refuses each one, so the probe path cannot become an
ordinary admission by accident.

In harness mode the platform is a declared identity label, because the peers are
JavaScript. Only an authorised original run would exercise the real per-platform
binary; the ledger says so rather than implying the fake run proves a platform.

### The pre-execution request

`probe-admission.json` carries the request a reviewer and the operator read
before any vendor run: the exact command, the original-mode suffix that needs the
authority token, the live source digests (plan, option template, probe runner,
fake peers, the shipped sidecar asset and the analysis script), the artifact set
with per-platform digests, the isolation rules and the finite bounds. The digest
block is verified against the files on every harness run, so a probe source edit
cannot slip into an execution request unnoticed. Original mode in the committed
ledger has `original_run_authorised: false` and no token.

### Admission caveat

Both the prepared driver and the shipped sidecar admit only the maintained pair
map, so every planned hop above `0.3.317` is refused with
`sdk_version_mismatch` before the SDK loads. The harness proves the pipeline on
`0.3.317`/`2.1.317` and proves the refusal for the newer hops. An authorised
original run for a newer hop needs an explicit probe admission, either from the
authority that names those exact pairs or from the qualification the run
produces. Nothing in this task widened the map.

## Claims and boundaries

- The package and native claims, the default pair, the finite 31-pair map, the
  `0.3.259`/`2.1.259` registered-tool live point, the `0.3.297`, `0.3.298` and
  `0.3.313` exclusions and the Node segment are unchanged.
- No live provider turn, login, credential, install, host update, prompt,
  release, tag or direct push occurred. No vendor artifact was executed.
- The plan and harness are not a qualification, and this record does not extend
  any pair.

## Sources

- [Q-009](../knowledge/questions.md) (the three residuals and the two open
  gates)
- [Research 480](./480-claude-agent-sdk-0-3-322-identity.md) (accepted corpus)
- [Research 472](./472-claude-agent-sdk-0-3-317-qualification.md) (accepted base)
- [`0.3.322` fixture](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.322/README.md)
- [Native proof fixture](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-native-proof/README.md)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
