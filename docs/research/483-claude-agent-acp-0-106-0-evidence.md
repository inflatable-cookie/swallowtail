# Research 483: Claude Agent ACP 0.106.0 evidence

Status: selected-route original qualification evidence through 0.106.0 on Linux
x64/Darwin arm64. Preserve earlier segments/exclusions; add only the eight frozen
published points. Exact 0.101.0 retains v7 callback behavior; the seven points
from 0.101.1 use guarded inherited-permission v8. This does not qualify standalone
Agent SDK, native executable, Node, shared ACP schema, Windows or new HTTP MCP
live points. Independent review and ordinary CI remain merge requirements.

## Identity

On 2026-11-12 npm `latest` was 0.106.0. Between the 0.100.0 ceiling and that
point the complete published stable set is 0.101.0, 0.101.1, 0.102.0,
0.103.0, 0.104.0, 0.104.1, 0.105.0 and 0.106.0. Every tarball's npm SHA-512
integrity and SHA-256 were checked; npm `gitHead` matches the corresponding
upstream tag. The [fixture](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-acp-0.106.0/README.md)
retains exact identities, all package paths and hashes, adjacent deltas, and
the dependency package trees. That identity observation ran no package; the
trusted-runner original qualification below is separate.

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

The original analysis classified changed paths by literal v1 import reachability
and surface role; those analysis-only catalogues are now archived. It is an over-approximation: shared helpers can serve several
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
  operation; the original selected load/resume proof is recorded below.

No static manifest or fake proves that the changed native binaries implement
every selected SDK behavior. HTTP MCP live honouring remains exact to the
previously evidenced points. The historical fake tests alone do not qualify vendor artifacts. The trusted
original proof below exercises the selected route against those frozen bytes. Operator
decision `43ddf3d5-e62b-4516-8546-e53137b320be` version 2 chooses a distinct
ACP revision preserving the inherited permission setting. The route-owned idle veto and its bounded original component proof are retained below.

Sources: [official npm package](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp),
[upstream tags](https://github.com/agentclientprotocol/claude-agent-acp/tags),
and the exact tarball URLs and integrity values in the fixture.

## Trusted original selected-route proof

Tom's decision 1ec44a26 v2 replaced the bespoke shared-host/kernel/Python/caller-
provenance test ladder with trusted disposable Linux x64/macOS arm64 runners.
Ordinary OS/runtime libraries are trusted; rolling image metadata is observation,
not an admission pin. The [current procedure](../knowledge/operations/claude-agent-acp-admission-preparation.md)
requires the real prepared Rust/LocalProcessHost route, product custody and
permission guards, isolated settings/HOME/workspace and owned loopback model
peers. No originals or installs run on the persistent Mac; no real credentials,
provider/model spend, login/catalogue or release is involved.

At source `0f88437b60b3b127f130ea2c9ca43144241ad4c2`, all 112 behavior cases
passed: eight exact hops × seven scenarios × two platforms. The pre-claim private
admission was set only after frozen graph/native capture; it changed test
qualification gating, not product permission/mapping or the actual route. The
public claim now carries the qualified points and the runner's private override
has been removed. Normal current-head CI verifies that public selection.

The final [qualification run](https://github.com/inflatable-cookie/swallowtail/actions/runs/44115269615)
at `6805191d75d629ee453f09fb8608b6cc7231111d` passed all 112 cases with the
public claim and no private override. It uploaded nine proof packets across
the two platforms. The table below records the earlier run.

| Platform | Exact hops | Behavior results | CI source job | Packet SHA-256 prefix |
| --- | --- | --- | --- | --- |
| Darwin arm64 | 0.101.0 / 0.101.1 | 14/14; upload deadline | [job134104413586](https://github.com/inflatable-cookie/swallowtail/actions/runs/44102066351/job/134104413586) | `d1c98ec98ad7381b` |
| Darwin arm64 | 0.102.0 / 0.103.0 | 14/14 | [job134104413445](https://github.com/inflatable-cookie/swallowtail/actions/runs/44102066351/job/134104413445) | `9847f3c164b4e386` |
| Darwin arm64 | 0.104.0 / 0.104.1 | 14/14 | [job134104414263](https://github.com/inflatable-cookie/swallowtail/actions/runs/44102066351/job/134104414263) | `d4bf71ee0612b2f3` |
| Darwin arm64 | 0.105.0 / 0.106.0 | 14/14 | [job134104413501](https://github.com/inflatable-cookie/swallowtail/actions/runs/44102066351/job/134104413501) | `af4b6b53da5951de` |
| Linux x64 | 0.101.0 / 0.101.1 | 14/14 | [job134104413626](https://github.com/inflatable-cookie/swallowtail/actions/runs/44102066351/job/134104413626) | `a06673bf4b4b50ea` |
| Linux x64 | 0.102.0 / 0.103.0 | 14/14 | [job134104412763](https://github.com/inflatable-cookie/swallowtail/actions/runs/44102066351/job/134104412763) | `bfde8c24b93bcf2f` |
| Linux x64 | 0.104.0 / 0.104.1 | 14/14 | [job134104413551](https://github.com/inflatable-cookie/swallowtail/actions/runs/44102066351/job/134104413551) | `00fb418bdd990a77` |
| Linux x64 | 0.105.0 / 0.106.0 | 14/14 | [job134104414295](https://github.com/inflatable-cookie/swallowtail/actions/runs/44102066351/job/134104414295) | `9f78b6101f7285f3` |

One initial Mac pair completed every native case, then crossed the 20-minute job
limit during artifact upload. Its job remains cancelled; it is not relabelled a
CI pass. The corrected workflow gives0.101.0 and0.101.1 separate Mac jobs, keeping
setup, tests, upload and cleanup inside the same limit. All case log hashes and
packet source heads were independently re-read before claim editing. Full packet
hashes, original receipts and outputs are retained evidence; vendor binaries and
whole vendor trees are not committed or presented as small proof artifacts.

Each allow/denial scenario opens new/load/resume and runs two turns per session.
Original API traffic confirms the selected model/options and protected Read
results; actual one-shot callbacks precede the allowed result. User and managed
no-prompt denial from 0.101.1 emits no fabricated callback, reports a tool error
without protected data, and continues the same turn. Exact 0.101.0's native does
not support that key: its two inherited-key cases require real callbacks and
allowed Reads, and receive no denial credit. Cancellation, deadline, native exit
and provider error cases retain distinct terminal results and joined cleanup.
Activity precedes the continued result; native close and local cleanup stay
separate. Actual native-child exit events, Rust child waits and descriptor EOF
supply functional wait evidence, not PID disappearance, job teardown or kernel/
adoptee/tree enforcement.

Node 22.23.2 and rustc 1.100.0 are recorded provenance for these jobs; OS/arch/image,
Node/test-binary hashes, observed argv and frozen artifact receipts are in each
packet. Compatible installed Node remains the ACP host choice, not an exact
consumer pin or a standalone SDK/Node claim extension. The registry recheck on
2026-11-17 observed0.108.0 after the frozen identity commit;0.107.0 and later
stay UnverifiedNewer on the separate follow-up, outside this qualification.

## Repairs and preserved failures

Originals exposed unsupported informational auth notifications, a missing bounded
ps binding after v1 relocation, ACP exiting before actual native-child joins,
cancelled prompts with omitted usage, and queued updates after cancellation/RPC
errors. Focused regressions retain strict unknown notifications, caller binding,
malformed supplied usage and normal idle-update refusal. A finished non-completed
turn can validate/discard its queued tail, but grants no events or callbacks.

The model peer also needed corrections: force Read to ask, distinguish coalesced
new prompts from tool results, and separate auxiliary suggestion traffic from
the owned test turn. The older Mac native rejected fabricated auxiliary tools as
`No tools needed for suggestion`. Selection now binds the actual owned prompt or
its own result ID; merely advertising Read does not select a tool call. The
outside-resource hypothesis alone did not repair this fixture bug. Current Read
files live inside the approved resource with explicit ask settings retained.
Earlier failed packets remain failures at their exact heads, with bounded error
summaries and reproductions; they are not counted as passes.

The abandoned analysis/proposal catalogues are preserved by immutable Git head
and hash in [the archive note](../knowledge/operations/claude-agent-acp-retired-analysis.md).
Frozen registry/dist/dependency/native identities and production guard inputs
remain active. No runtime or CI depends on the archive, and no shared SDK task
completion or retired host-attestation packet is a prerequisite.

## Original idle-class observations

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

Those component results alone did not qualify the actual ACP/native route.
The separate trusted-runner proof above supplies that selected-route evidence;
it does not replay the consumed component grants or transfer their host/kernel
assertions. Earlier stock claims and exact HTTP MCP live points remain intact.
