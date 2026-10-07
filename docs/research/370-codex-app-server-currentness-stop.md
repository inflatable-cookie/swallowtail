# Research 370: Codex App-Server Currentness Stop at 0.161.0

Status: stopped for operator ruling; no compatibility claim changed

Owner: version-currentness lane
Date: 2026-10-07
Task: swallowtail#094

## Question

Can the `codex.app-server` claim extend from `0.155.1` through the official
stable `0.161.0` for the v0.5.2 version sweep?

Answer: the route identity and stable release chain were recorded, but
qualification stopped before changing the claim. Codex changed projectless
trust handling at `0.156.0` and filesystem permission derivation at `0.158.0`.
The current route binds `thread/start` to its preflight-approved working
resource. Contract 029 and the task brief require a ruling for a security or
authority change.

## Method and limits

Re-probed the official npm registry and GitHub releases metadata on 2026-10-07.
They agreed on `@openai/codex` `0.161.0` / `rust-v0.161.0`. The npm `alpha`
dist-tag was `0.162.0-alpha.18` and was excluded. The 12 published stable
points after the current `0.155.1` ceiling are preserved in the artifact file.

Compared the exact GitHub release tags from `rust-v0.155.1` through
`rust-v0.161.0`, recording tag objects, peeled commits, complete tree file
counts, deterministic full-tree manifest hashes, and per-hop added, removed,
changed, and feeding-candidate paths. This is a source inventory, not a
semantic classification of every provider-internal change. The selected
schema check found no removed selected method or changed selected required
request field; optional additions and unmapped methods do not settle the
runtime permission changes described below.

No live app-server session, provider prompt, artifact execution, installation,
credential use, or host update occurred. The observed host was `codex-cli
0.159.0`; its resolved 55-byte launcher stub was hashed, so that digest is not
the Codex runtime binary identity.

The secret-free artifact and path inventories are frozen in the adapter
fixture: [npm and GitHub identities](../../crates/swallowtail-adapter-codex/tests/fixtures/codex-app-server-0.161.0/published-artifacts.json),
[complete tagged-source inventory](../../crates/swallowtail-adapter-codex/tests/fixtures/codex-app-server-0.161.0/source-inventory.json),
and [stop analysis](../../crates/swallowtail-adapter-codex/tests/fixtures/codex-app-server-0.161.0/stop-analysis.json).

## Observed authority changes

### 0.156.0: projectless trust handling

In the official
[`thread_processor.rs` at `rust-v0.156.0`](https://github.com/openai/codex/blob/rust-v0.156.0/codex-rs/app-server/src/request_processors/thread_processor.rs#L1361),
the condition for automatically persisting project trust adds
`!config.config_layer_stack.is_projectless()`. The prior condition checked a
caller-supplied `cwd`, absent active-project trust, and the effective
trust-project permission, then called `set_project_trust_level(...,
TrustLevel::Trusted)` and reloaded configuration. The loader classifies a
directory as projectless when project discovery finds no root marker, checkout
root, or project-local configuration. The new gate therefore skips that
automatic trust-state persistence for some caller-selected working directories.

Swallowtail sends the preflight-approved root as `cwd` on `thread/start`
([`session_access.rs`](../../crates/swallowtail-adapter-codex/src/session_access.rs),
[`session_role.rs`](../../crates/swallowtail-adapter-codex/src/app_server/session_role.rs)).
Whether this trust-state change fits the existing workspace-roots contract is
not settled by the current evidence.

### 0.158.0: path and writable-root calculation

The official
[`permissions.rs` at `rust-v0.158.0`](https://github.com/openai/codex/blob/rust-v0.158.0/codex-rs/protocol/src/permissions.rs#L1555)
changes system-alias normalization and the local policy matcher. For Linux and
macOS writable-root calculations, it also resolves a workspace `.git` pointer
target and adds that target as a read-mode entry in read-only carveout
calculations. If the resolved Git directory falls within a broader writable
root, the resulting write boundary can differ. This is not evidence that the
change grants reads outside the selected root.

Swallowtail binds `cwd`, `runtimeWorkspaceRoots`, and turn `writableRoots` to
the same preflight-approved working-resource root. The alias and linked-Git
directory behavior therefore needs a ruling against that authority boundary.

## Claim and next decision

The `codex.app-server.cli-window-2` claim remains through `0.155.1`, preserving
its existing segments and exclusions. `codex.exec` and its claim remain
unchanged. No route guide, matrix, changelog, release note, or selection value
was edited.

The next ruling is whether the existing app-server workspace-roots contract
accepts (a) the `0.156.0` projectless trust-state change on the selected
`thread/start` path and (b) the `0.158.0` path normalization and read-only
Git-directory carveout when the target is inside a broader writable root. If
either is outside that contract, the next adaptation must preserve earlier
qualified segments and define the narrow route behavior or boundary needed to
continue the Contract 029 currentness lane.

Sources: [official npm registry](https://registry.npmjs.org/@openai/codex),
[GitHub release `rust-v0.161.0`](https://github.com/openai/codex/releases/tag/rust-v0.161.0),
[Contract 029 upgrade workflow](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md#upgrade-workflow),
[version-currentness checkpoint procedure](../knowledge/operations/version-currentness-checkpoint.md),
[Research 369](./369-all-route-version-currentness-checkpoint.md).
