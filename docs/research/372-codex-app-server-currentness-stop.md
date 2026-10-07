# Research 372: Codex App-Server Currentness Stop at 0.161.0

Status: stopped for operator ruling; no compatibility claim changed

Owner: version-currentness lane
Date: 2026-10-07
Task: swallowtail#094

## Question

Can the `codex.app-server` claim extend from `0.155.1` through the official
stable `0.161.0` for the v0.5.2 version sweep?

Answer: the route identity and stable release chain were recorded, but
qualification stopped before changing the claim. Tom authorized bounded
adaptations for the projectless trust change at `0.156.0` and path-alias and
linked-Git handling at `0.158.0`. Further source review found that `0.159.0`
also makes an existing top-level `.aws` directory read-only by default within
a writable root. This narrows the selected workspace-write behavior and needs
a separate ruling before the claim can move.

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
runtime trust, permission, or lifecycle semantics described below. The
`.158.0` to `.159.0` hop changes exactly two protocol permission files; their
exact source hashes and path set are asserted by the route's stop fixture.

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
Tom's 2026-10-07 ruling authorizes this route to accept skipped automatic
persisted trust for projectless working directories. Deterministic regression
proof is still required before qualification.

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
the same preflight-approved working-resource root. Tom's 2026-10-07 ruling
allows path normalization and linked `.git` read-only protections inside that
approved root. Deterministic regression proof is still required.

### 0.159.0: `.aws` becomes read-only by default

The exact `.158.0` to `.159.0` hop changes
`codex-rs/protocol/src/permissions.rs` and
`codex-rs/protocol/src/permissions/target.rs`. In the frozen
[`permissions.rs` at `rust-v0.159.0`](https://github.com/openai/codex/blob/rust-v0.159.0/codex-rs/protocol/src/permissions.rs#L40), Codex adds `.aws` to `PROTECTED_METADATA_PATH_NAMES`; [`default_read_only_subpaths_for_writable_root`](https://github.com/openai/codex/blob/rust-v0.159.0/codex-rs/protocol/src/permissions.rs#L2384) adds an existing top-level `<writable-root>/.aws` directory as a read-only subpath. Its upstream [regression test](https://github.com/openai/codex/blob/rust-v0.159.0/codex-rs/protocol/src/permissions.rs#L3460) creates `.aws/config` under a writable root and confirms that writes are denied. The corresponding `.155.1` and `.158.0` sources do not include `.aws` in the protected names. Exact source hashes and the changed-file set are in the stop fixture.
[`permissions.rs` at `rust-v0.159.0`](https://github.com/openai/codex/blob/rust-v0.159.0/codex-rs/protocol/src/permissions.rs#L40), Codex adds `.aws` to `PROTECTED_METADATA_PATH_NAMES`; [`default_read_only_subpaths_for_writable_root`](https://github.com/openai/codex/blob/rust-v0.159.0/codex-rs/protocol/src/permissions.rs#L2384) adds an existing top-level `<writable-root>/.aws` directory as a read-only subpath. Its upstream [regression test](https://github.com/openai/codex/blob/rust-v0.159.0/codex-rs/protocol/src/permissions.rs#L3460) creates `.aws/config` under a writable root and confirms that writes are denied. The source explains that AWS profiles can select credential helpers Codex executes, so this is a security-sensitive default. The corresponding `.155.1` and `.158.0` sources do not include `.aws` in the protected names. Exact source hashes and the changed-file set are in the stop fixture.

Swallowtail sends `workspace-write` with one preflight-approved root in
`thread/start`, and its `turn/start` sandbox policy has one matching writable
root, network access disabled, and the existing temporary-directory
exclusions. It sends no `.aws` exception. Therefore an existing top-level
`.aws` directory in that approved root becomes read-only under the selected
route. This narrows earlier workspace-write behavior without expanding the
approved root.

## Claim and next decision

The `codex.app-server.cli-window-2` claim remains through `0.155.1`, preserving
its existing segments and exclusions. `codex.exec` and its claim remain
unchanged. No route guide, matrix, changelog, release note, or selection value
was edited. The prior ruling covers the `.156.0` and `.158.0` behaviors, but
they are not presented as qualified here; their required deterministic route
proofs are incomplete.

The next ruling is whether the bounded `codex.app-server` workspace-write
contract accepts this security-sensitive default read-only carveout for an
existing top-level `.aws` directory under the approved root, including the
resulting consumer-facing narrowing. If not, define a separate route adaptation
that preserves the `.aws` protection and approved-root boundary while resolving
the consumer guarantee; do not add an automatic write exception. Then complete
the authorized `.156.0` and `.158.0` regression proof and the full per-hop
semantic review before reconsidering the `.161.0` claim.

Sources: [official npm registry](https://registry.npmjs.org/@openai/codex),
[GitHub release `rust-v0.161.0`](https://github.com/openai/codex/releases/tag/rust-v0.161.0),
[Contract 029 upgrade workflow](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md#upgrade-workflow),
[version-currentness checkpoint procedure](../knowledge/operations/version-currentness-checkpoint.md),
[Research 369](./369-all-route-version-currentness-checkpoint.md).
