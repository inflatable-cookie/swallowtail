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
  operation; the changed load and shared handlers still require proof.

No static manifest or fake proves that the changed native binaries implement
every selected SDK behavior. HTTP MCP live honouring remains exact to the
previously evidenced points. The retained tests exercise adapter behavior
against fakes; they do not execute or qualify the vendor artifacts. Operator
decision `43ddf3d5-e62b-4516-8546-e53137b320be` version 2 chooses a distinct
ACP revision preserving the inherited permission setting. The route-owned idle veto and its bounded original component proof are retained below.

Sources: [official npm package](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp),
[upstream tags](https://github.com/agentclientprotocol/claude-agent-acp/tags),
and the exact tarball URLs and integrity values in the fixture.

## Current qualification procedure

The original source analysis and prototypes are retained at commit
98067e2e2e49f946f9eeac6d386e946c749c48ad. The exhaustive import/static-callsite
catalogues and abandoned host-admission models are archived as described in
[the provenance note](../knowledge/operations/claude-agent-acp-retired-analysis.md).
Their absence does not establish source coverage or close a qualification gate.
Frozen artifact, dependency, dist, permission and selected guard inputs remain.

Tom's decision1ec44a26 v2 authorizes finite original tests on trusted disposable
Linux x64/macOS arm64 runners. It supersedes the previous shared-host/kernel/
Python-custody/caller-catalogue prerequisites for this procedure. The actual
prepared Rust route, inherited permissions, idle veto and functional waits remain
required. See [the current procedure](../knowledge/operations/claude-agent-acp-admission-preparation.md).
The new first original attempts reached a real native Read and continued output,
but failed a callback assertion: outside-workspace Read alone did not force a
prompt. They are failed harness observations, not qualification. The corrected
fixture explicitly asks for Read; current runs must prove the callback and denial
paths without inventing native events or changing public semantics.

## Trusted-runner observations before claim promotion

At5f22062f700028b481debfe9d0179992dfa6ff0d, workflow44089786882 ran actual
frozen originals through the prepared Rust route. All56 Linux cases passed.
The macOS0.102.0/0.103.0 pair passed14/14;0.104.0/0.104.1 passed13/14.
The0.101.0/0.101.1 pair passed8/14. Remaining Mac results and repaired cases
are not inferred passes. Exact job logs and bounded packets retain the failures.

Mac originals exposed queued updates after cancelled or rejected prompt
responses, causing `update_without_turn` and degraded session close. Retaining
one non-completed finished turn as a validating sink repairs that lifecycle
boundary; normal idle updates and late callback requests still refuse. A second
failure was an allowed outside-resource Read returning a native tool error.
The corrected selected-route probe requests files inside its approved working
resource and still explicitly asks for Read permission; it does not claim an
outside-resource fix. Bounded tool-result diagnostics preserve any repeated
failure. None of these failed cases authorizes a ceiling increase.

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

The next gate is a route-specific confined prepared-driver proof of actual
ACP session paths and inherited native permission denial without a callback,
continued turn, cancellation and joined native cleanup. It needs admitted
transport/administrator-fixture inputs and separate exact ACP/SDK/native
execution authority; task234's fixture or custody authority does not transfer.
Contract029/061 still forbid a qualified projection of the guarded revision
from this component evidence. The ceiling stays0.100.0, earlier stock and
exact HTTP MCP points remain intact, and0.107.0 remains UnverifiedNewer.

