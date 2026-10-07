# Research 373: Codex App-Server Currentness Stop at 0.161.0

Status: stopped for operator rulings; no compatibility claim changed

Owner: version-currentness lane  
Date: 2026-10-07  
Task: swallowtail#094

## Question

Can the `codex.app-server` claim extend from `0.155.1` through the current
official stable `0.161.0` for the v0.5.2 version sweep?

Answer: no claim moved. Source review found selected-path policy and security
changes in `0.156.0` and `0.157.0` that have no app-server ruling. The earlier
projectless-trust, path-alias, linked-`.git`, and `.aws` boundaries have exact
operator rulings, but their route proof does not qualify the unresolved later
behavior.

## Identity and method

On 2026-10-07, the official npm package and GitHub release channels both
reported `@openai/codex` `0.161.0` / `rust-v0.161.0`. npm's
`0.162.0-alpha.18` prerelease was excluded. The exact base and darwin-arm64
package integrities, release time, source tag/commit identities, and every
published hop after `0.155.1` are in the fixtures linked below. The selected
runtime artifact is `@openai/codex-darwin-arm64@0.161.0`, `bin/codex`,
245,096,256 bytes, SHA-256
`12ac11d2c7eee27cfae34393986d7b7c9ed0dea537cb749831cdd7033893e6de`; it was
identified offline and never executed.

The source inventory freezes 13 release tags, all 12 published stable hops,
and a deterministic full-tree inventory for each tag. A separate selected
source ledger records exact blob and SHA-256 identities, per-hop source
classifications, selected and unselected schema files, and upstream regression
names. Schema review found no removed selected method or changed selected
required request field. The adapter's exact request fixture sends one approved
root, retains read-only defaults, and sends no trust, home, config, path-alias,
linked-`.git`, or `.aws` override.

The deterministic route fixture also covers the approved root and denied
network policy, read-only sessions, observable approval and user-input
requests, generic failed-turn projection, and cleanup. No Codex artifact was
executed. There was no provider prompt, live session, credential use,
installation, host update, release, or `codex.exec` claim change.

## Stops

### 0.156.0: managed model-provider revalidation

Codex now checks managed provider requirements on selected `model/list` and
`turn/start` paths, including existing threads. A changed provider selection
or required provider definition, and a requirement-load failure, can reject
the retained request with `PermissionDenied` before provider work. Frozen
provider tests exercise both cases. The route does not write this config, but
the selected app-server path consumes it. Contract 023 has no ruling for this
app-server behavior.

### 0.156.0: fail-closed permission materialization

`materialize_project_roots_with_path_uris` now returns a policy with no write
grants when a home-relative denial cannot be safely expanded without a home
fact. Invalid workspace deny globs also take a deny-root fallback. The source
explains that expanding an old denial incompletely could weaken it. The
selected session and turn permission paths call this materializer. The frozen
cross-path regression covers invalid interior-dot globs. This can narrow
writes within the approved root and needs an app-server ruling or a scoped
adaptation that preserves the denial and root boundary.

### 0.156.0: default system-proxy fallback

`SystemProxyFallback` is default-enabled. Codex can retry safe-to-replay
bootstrap GETs through the host system proxy after direct connection or
timeout failure. The source tests bind this to bootstrap requests and feature
requirements; the ordinary request path is not described as retrying. This is
a new transport path for the selected app-server runtime. Contract 023's
network ruling for `codex.exec` does not cover it. Decide whether this exact
app-server bootstrap fallback is accepted or whether the route needs an
adaptation that keeps the existing transport boundary.

### 0.156.0 and 0.161.0: Windows MxC tool sandbox selection

Codex adds an explicit ambient `windows.sandbox = "mxc"` selection that routes
tool commands and patches through its MxC Windows sandbox backend. Swallowtail
sends no such config and does not select the Windows sandbox setup RPC, but
the app-server loads provider configuration while running selected tools. In
`0.161.0`, MxC selection is additionally constrained by managed
`windows.sandbox` requirements and local `prefer_mxc` is gated by that policy.
The source regression confirms configured backend selection and managed
vetoes. This changes the available tool sandbox authority; decide whether the
selected route may inherit this explicit provider setting, or name an
adaptation that prevents unqualified backend selection without bypassing
managed policy.

### 0.157.0: app-server application-network policy

Codex adds local, device-management, and cloud `application.network` policy to
app-server provider/API HTTP traffic. Local rules are applied before cloud
requirements, and local policy load failures stop the request. This is
separate from the adapter's `turn/start sandboxPolicy.networkAccess=false`,
which controls tool sandbox network access. No app-server ruling covers this
provider/API transport policy. The `codex.exec` ruling is independent.

## Ruled boundaries and later hops

Tom's decision `d7dbacde-6381-47c5-8255-6729d309b147` accepts skipped automatic
persisted trust for projectless directories and path-alias normalization plus
linked-`.git` read-only protection within the approved root. Decision
`cb18500a-499a-4cbe-ae1b-8daec34d5ebe` accepts the `0.159.0` read-only
protection for an existing top-level `<approved-root>/.aws` directory. No
`.aws` write exception is sent or allowed. These rulings preserve the root
boundary and read-only defaults; the `.aws` behavior is a version-specific
workspace-write narrowing. Planner still needs to classify that implication
under Contract 036 before release.

`0.158.0` path alias and linked-`.git` sources and `0.159.0` `.aws` sources
include upstream restrictive-policy tests. Adapter fixtures now assert the
exact selected request key set and sole approved root. This proves the adapter
does not add root or permission overrides; it does not promote the accepted
provider rules to a qualified compatibility claim while the other stops
remain.

The `0.160.0` running-turn counter is maintained incrementally; the frozen
regression covers duplicate starts, removals, status updates, and emitted
counts. `ContentFilter` remains a failed turn and is projected as a generic
provider failure. In `0.161.0`, resume history snapshots carry a file revision
and are reused only if the exact rollout revision still matches after writer
acquisition; otherwise the store reloads authoritative history. This preserves
the existing selected `thread/resume` surface. New thread-goal methods,
prediction notifications, and other unselected methods remain unmapped.

## Claim and next decision

The `codex.app-server.cli-window-2` claim remains through `0.155.1`, preserving
all supported segments, holes, exclusions, claim IDs, and behavior revisions.
The `codex.exec` claim is separate and unchanged. No adapter operation or
provider capability was added.

Before reconsidering the ceiling, obtain rulings or scoped adaptation briefs
for the exact `0.156.0` managed provider checks, fail-closed permission
materialization, default bootstrap proxy fallback, explicit MxC tool backend,
and `0.157.0` app-server application-network policy. Then finish the approved
projectless-trust, alias/linked-`.git`, and `.aws` route regressions and the
complete per-hop review at the current official stable. Do not restore trust,
add a `.aws` write exception, bypass managed policy, expand the approved root,
or infer a release compatibility waiver.

Sources: [official npm package](https://www.npmjs.com/package/%40openai/codex),
[GitHub release `rust-v0.161.0`](https://github.com/openai/codex/releases/tag/rust-v0.161.0),
[official package and release identities](../../crates/swallowtail-adapter-codex/tests/fixtures/codex-app-server-0.161.0/published-artifacts.json),
[complete tagged-source inventory](../../crates/swallowtail-adapter-codex/tests/fixtures/codex-app-server-0.161.0/source-inventory.json),
[selected-source review and exact identities](../../crates/swallowtail-adapter-codex/tests/fixtures/codex-app-server-0.161.0/selected-source-review.json),
[Contract 023 Codex trust and workspace changes](../knowledge/contracts/023-codex-trust-workspace-and-managed-network-changes.md),
[Contract 029 upgrade workflow](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md#upgrade-workflow),
[version-currentness checkpoint procedure](../knowledge/operations/version-currentness-checkpoint.md),
[Research 369 all-route checkpoint](./369-all-route-version-currentness-checkpoint.md).
