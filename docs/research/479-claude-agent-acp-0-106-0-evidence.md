# Research 479: Claude Agent ACP 0.106.0 evidence

Status: evidence only. `claude-agent.acp` remains qualified through 0.100.0;
newer points remain `UnverifiedNewer`. The permission and process boundaries
below require a route-specific decision. This record does not qualify the
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
decision `43ddf3d5-e62b-4516-8546-e53137b320be` owns the permission boundary;
the separate idle-compaction process boundary has been reported to the planner.

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
