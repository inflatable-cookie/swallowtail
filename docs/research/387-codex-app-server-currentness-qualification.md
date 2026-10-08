# Research 387: Codex App-Server 0.161.0 Identity and Qualification

Status: qualified through official stable `0.161.0`

Owner: version-currentness lane  
Date: 2026-10-08  
Task: swallowtail#094

## Question and result

Can the `codex.app-server` route extend from its prior `0.155.1` ceiling through
current official stable `0.161.0` for the v0.5.2 version sweep?

Yes. This qualification applies only to `codex.app-server`. It preserves the
existing baseline, claim IDs, older segments, feature milestones, and
exclusions, then adds the selected behavior revisions supported by the frozen
source review and approved Contract 023 rulings. `codex.exec` remains
independently qualified through `0.155.1`.

## Identity and method

At the current channel re-probe on 2026-10-08, official npm
`@openai/codex` `latest` and GitHub latest non-prerelease release agree on
`0.161.0` / `rust-v0.161.0`; npm `0.162.0-alpha.20` remains an excluded
prerelease. The original artifact identity snapshot retains its observed
`0.162.0-alpha.18` tag value. The [artifact identity ledger](../../crates/swallowtail-adapter-codex/tests/fixtures/codex-app-server-0.161.0/published-artifacts.json)
freezes the package metadata, selected-channel stables and platform-package
identities. The selected runtime is
`@openai/codex-darwin-arm64@0.161.0/bin/codex`, 245,096,256 bytes, SHA-256
`12ac11d2c7eee27cfae34393986d7b7c9ed0dea537cb749831cdd7033893e6de`. The
public artifact was hashed and inspected offline; it was never executed.

The [tagged-source inventory](../../crates/swallowtail-adapter-codex/tests/fixtures/codex-app-server-0.161.0/source-inventory.json)
freezes 13 release tags and all 12 published stable hops after `0.155.1`, with
deterministic complete source-tree file counts and manifest digests. The
[selected-source review](../../crates/swallowtail-adapter-codex/tests/fixtures/codex-app-server-0.161.0/selected-source-review.json)
records exact changed-file and schema sets, source blobs and digests,
per-hop classifications, and upstream regression names. Its mutation-sensitive
checks assert exact inventory keys and values. Changelog notes were discovery
only. Provider-internal and unmapped changes remain bounded by the full tree
inventory and the exact selected path/file lists; new methods, thread-goal
operations, prediction notifications, and other unselected RPCs are not mapped.

## Segment shape

The `codex.app-server.cli-window-2` claim keeps baseline `0.80.0` and its
existing claim ID. Prior supported ranges through `0.155.1` remain in place.
The selected post-ceiling segments are:

| Range | Behavior revision | Status |
| --- | --- | --- |
| `0.156.0..=0.156.1` | `codex.app-server.v2.managed-policy-workspace-roots` | Deprecated |
| `0.157.0..=0.157.1` | `codex.app-server.v2.managed-network-workspace-roots` | Deprecated |
| `0.158.0` | `codex.app-server.v2.path-alias-workspace-roots` | Deprecated |
| `0.159.0..=0.161.0` | `codex.app-server.v2.protected-aws-workspace-roots` | Maintained |

The independent thread catalogue and lifecycle claims now qualify through
`0.161.0`; their existing behavior boundaries and operation shapes remain
unchanged. The previously supported app-server windows stay unchanged:
`0.80.0..=0.81.0`, `0.84.0..=0.99.0`, `0.100.0..=0.107.0`,
`0.110.0..=0.130.0`, and `0.131.0..=0.155.1`, with the already-recorded
feature-specific floors. The `0.131.0..=0.155.1` segment retains its prior
Maintained status; earlier status values remain unchanged.

The excluded points remain `0.82.0..=0.83.0`, `0.108.0..=0.109.0`,
`0.149.2`, `0.150.2`, `0.151.1`, `0.152.2`, `0.154.1`, and `0.155.2`.
The first synthetic later point is `0.161.1`, permitted only with explicit
`UnverifiedNewer` status. No new operation or capability was added.

## Selected-source hop ledger

Every hop and exact changed-file set is frozen in the selected-source review.
The route-level result for each hop is:

- `0.155.1 → 0.156.0`: accepted under decisions `d7dbacde-6381-47c5-8255-6729d309b147`
  and `d9a1fe6e-6481-433c-b643-ada177613e07`. Projectless startup skips
automatic persisted trust. Selected model/list and fresh or retained turn/start
  requests revalidate managed model-provider requirements and fail before
  provider work when requirements change or cannot load. Permission
  materialization fails closed for unresolved home-relative denials and invalid
  workspace globs. The default system-proxy fallback applies to safe-to-replay
  bootstrap GETs. Explicit ambient `windows.sandbox = "mxc"` remains subject to
  managed policy; the route sends no MxC selection or setup RPC.
- `0.156.0 → 0.156.1`: no selected route behavior change; complete source-tree
  inventory retained.
- `0.156.1 → 0.157.0`: accepted under decision
  `d9a1fe6e-6481-433c-b643-ada177613e07`. Host-managed `application.network`
  governs provider/API HTTP traffic independently of tool
  `networkAccess=false`; local policy load failure remains fail-closed.
- `0.157.0 → 0.157.1`: no selected route behavior change; complete inventory
  retained.
- `0.157.1 → 0.158.0`: accepted under decision
  `d7dbacde-6381-47c5-8255-6729d309b147`. Provider path aliases normalize
  within the approved root, and linked `.git` targets remain read-only; neither
  grants access outside the root.
- `0.158.0 → 0.159.0`: accepted under decision
  `cb18500a-499a-4cbe-ae1b-8daec34d5ebe`. An existing top-level `.aws`
  directory inside the approved root becomes read-only. The route adds no
  `.aws` write exception.
- `0.159.0 → 0.159.1`, `0.159.1 → 0.159.2`, and
  `0.159.2 → 0.159.3`: no selected compatibility change; exact file deltas stay
  in the complete inventory.
- `0.159.3 → 0.160.0`: incremental running-turn count bookkeeping preserves the
  selected status values; duplicate starts, removals, and emitted counts are
  covered by the named upstream regression.
- `0.160.0 → 0.160.1`: no selected route behavior change; complete inventory
  retained.
- `0.160.1 → 0.161.0`: history snapshot revisions are reused only when the
  exact rollout revision still matches after writer acquisition; otherwise
  authoritative history is reloaded. Selected `thread/resume` shape and
  outcomes remain compatible. Explicit MxC remains bounded by managed policy;
  Windows source evidence does not establish runtime isolation.

## Route boundary and regression proof

The exact selected-route fixtures retain one host-approved writable root.
`thread/start` sends `approvalPolicy=never`, `sandbox=workspace-write`, the
approved `cwd`, and only that root as `runtimeWorkspaceRoots`. `turn/start`
sends only that writable root, `networkAccess=false`, and the existing
read-only defaults. The route sends no trust, config, permission, proxy,
Windows sandbox, application-network, path-alias, linked-`.git`, or `.aws`
write override.

Fixtures prove an ordinary write within the approved root, `.aws` denial,
projectless and marked startup, path-alias and linked-`.git` root boundaries,
managed model/list and fresh or retained turn-start refusal, failed
home-relative permission projection, generic provider failure, cleanup, and
unchanged read-only session request shape. Provider-specific denial details
remain hidden from consumers. Upstream source identities and named regression
cases prove that the bootstrap fallback is limited to safe bootstrap GETs,
that application-network rules concern provider/API traffic, and that explicit
MxC honors managed restrictions. No live catalogue, provider session,
credential, prompt, installation, or host update was used.

The `.aws` limitation is a consumer-visible workspace-write narrowing. Its
acceptance qualifies this currentness point but does not waive the separate
Contract 036 patch-compatibility review required before v0.5.2. Planner must
classify that release implication. No other currentness stop remains open.

## Sources

- [Official npm package](https://www.npmjs.com/package/%40openai/codex)
- [GitHub release `rust-v0.161.0`](https://github.com/openai/codex/releases/tag/rust-v0.161.0)
- [Contract 023: Codex trust, workspace, managed network and policy rulings](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md#codex-app-server-managed-policy-qualification)
- [Contract 029 Upgrade Workflow](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md#upgrade-workflow)
- [Contract 036 source-only patch compatibility](../knowledge/contracts/036-crate-release-and-compatibility-boundary.md)
- [Currentness checkpoint procedure](../knowledge/operations/version-currentness-checkpoint.md)
- [Research 369 all-route checkpoint](./369-all-route-version-currentness-checkpoint.md)
