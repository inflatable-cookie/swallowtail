# Research 374: Qwen Code 0.25.0 Identity And Headless Qualification

Observed 2026-10-07. This record qualifies only `qwen.headless` on the npm
package `@qwen-code/qwen-code`. It follows Research 334's `0.24.2` ceiling
and does not borrow evidence from a sibling Qwen route.

## Decision

Raise the `qwen-code.package` ceiling from `0.24.2` to `0.25.0`. Keep the
existing `qwen-code.headless.package-window-2` claim, `0.19.11` baseline,
segment statuses, and behavior revisions. The maintained segment
`0.22.0..=0.25.0` keeps
`qwen-code.headless.v0.21.15-reasoning-control`. The older deprecated segments
remain `0.19.11..=0.20.1` and `0.21.0..=0.21.14`; exact `0.21.15` remains
maintained. Exclude unpublished `0.20.2`, `0.21.16`, `0.22.4`, and `0.23.5`.
Plan remains exact through `0.22.3`; reasoning and budget selections remain
exact at `0.21.15`.

The qualification depends on a private mapping guard. Qwen Code 0.24.4 added
automatic SSH workspace selection from the selected `cwd`. Operator decision
`977ff17f-4b95-48fc-84ff-ff3e438d41ea`, recorded in Contract 013 § “Qwen
Implicit SSH Workspace Selection”, authorizes blocking that path while
preserving the local route and delegated authentication. The adapter resolves
the exact host-approved read-only resource before provider work, canonicalizes
its filesystem path, and rejects the provider-reserved
`ssh-workspaces/<64-hex-connection-hash>/workspace` shape. Canonicalization
covers normalized and symlink aliases. No credential state is read, moved, or
deleted. The original `WorkingResourceRef` and delegated `EnvironmentRef`
remain bound to the child. Ordinary local resources continue to work.

This route has no SSH workspace support. It makes no filesystem containment or
remote-host claim. No public operation, request signature, or lifecycle method
was added. The route guide already required the read-only working-resource
service; the driver now declares and enforces that host service requirement so
the guard can inspect the host-resolved resource. The reserved Qwen SSH path
was not a guaranteed Swallowtail behavior. Under Contract 036 this is a
patch-compatible currentness extension plus the operator-ratified rejection
of an implicit remote selection. No release or tag authority follows.

## Official Channel And Artifact Identity

The selected channel is npm `dist-tags.latest` for
[`@qwen-code/qwen-code`](https://www.npmjs.com/package/%40qwen-code/qwen-code).
The final channel probe on 2026-10-07 returned `latest=0.25.0`,
`preview=0.25.1-preview.0`, and
`nightly=0.25.0-nightly.20261007.8003d28042`. Preview and nightly are not
stable releases. The GitHub TypeScript SDK is a separate product and does not
identify this CLI package. GitHub source tags below identify the source
snapshots; npm `gitHead` is present through `0.24.3` and absent from the
published npm metadata at `0.24.4` and later, so the npm package and tagged
source identities remain separately recorded.

The downloaded npm tarballs were inspected offline and never executed. Each
tarball has an exact npm SRI and SHA-1 shasum plus a locally calculated
SHA-256. The table records the stable package points after the old ceiling.
Exact `dist.integrity`, `dist.shasum`, file count, unpacked size, complete
inventory digest, package metadata and source identity are frozen in
`crates/swallowtail-adapter-qwen/tests/fixtures/qwen-code-0.25.0/identity.json`.

| npm version | Published (UTC) | Files | npm shasum | Tarball SHA-256 | Official source tag commit |
| --- | --- | ---: | --- | --- | --- |
| `0.24.3` | 2026-09-21 14:36:03.483 | 1,276 | `450fc47746eee14b67a13b5077c6dd6d6f961876` | `116e7c65b851ff8d318c06b1eef1648815deac0c204e16977791de017c0165f1` | [`v0.24.3`](https://github.com/QwenLM/qwen-code/tree/v0.24.3) `557f36ba83bcf98bc031eefc94e60991289dc4f2` |
| `0.24.4` | 2026-09-22 15:32:25.511 | 1,286 | `0e3eacbf406c2c291649cc3a49671ca9a4451a43` | `7840ff5c70010be9bd7c903d1ec0893893f5429fde5b4e8a189f61450269d8f4` | [`v0.24.4`](https://github.com/QwenLM/qwen-code/tree/v0.24.4) `9674d6efdc8ac16583270bd2a55cfa49e3ec254e` |
| `0.24.5` | 2026-09-24 17:16:56.822 | 1,289 | `a7cb1fdeeb28e289c63861fde356d876034e47c9` | `539b8deb2cbc2108df3d1d763f420566acfc83d2d3afa202f7da585f1c2e4f2d` | [`v0.24.5`](https://github.com/QwenLM/qwen-code/tree/v0.24.5) `59c8e93c0e9b34db77beb1b087e4076916532513` |
| `0.24.6` | 2026-09-26 00:57:41.243 | 1,302 | `39568c78af703d4e04d382d03c374884dfc7912b` | `9f0e7ba70009cc0a5133208cf8edc21f26eef211e9d14d0d3f497cd42a9e31f2` | [`v0.24.6`](https://github.com/QwenLM/qwen-code/tree/v0.24.6) `753bd075a24ca886b7d0119ca53d97bd65c09e4a` |
| `0.24.7` | 2026-09-29 14:23:39.669 | 1,327 | `4069abba916a257afab21dedca17d4fe7919c217` | `64430248ab6e996fc0c6b9a0789361f868c3b97f83d16210e0424e51dffc5fa9` | [`v0.24.7`](https://github.com/QwenLM/qwen-code/tree/v0.24.7) `b12edec1401a28fc53cd9e714d5928b285071fc8` |
| `0.25.0` | 2026-10-05 09:55:55.980 | 1,358 | `759806b6028175c794afabe10713e2f05f1026cd` | `afeab0c85d682f201101319ce05decf3302d7ed7fb6319baa6a592051ea1a1ba` | [`v0.25.0`](https://github.com/QwenLM/qwen-code/tree/v0.25.0) `6788c035698a0ada471c958d1e789e96c6cddd9b` |

The `0.24.2` starting artifact and all seven artifact tree inventories are
retained beside the new identity. The old artifact's tarball SHA-256 is
`ea8b977eac79977603f52c9a3f4d279f3f70e230910a087372a51d70cd2759fa` and its
source tag commit is `1026c4a50f4a32f77da98bdacfba2e5faa8cc70a`. The exact
npm SRI for each artifact is in the identity JSON.

## Complete Published-Hop Ledger

Each `dist-inventory-<version>.json` lists the SHA-256 of every file extracted
from that exact official npm tarball, including `LICENSE`, `README.md`,
`package.json`, and package contents. Each `dist-diff-<from>-to-<to>.json`
records the complete added, removed, changed, and byte-identical path sets.
The identity test recomputes those path sets from the inventories and checks
the exact ledger hashes and counts. The source snapshots under the same
fixture preserve all eleven selected mapped files for each version, plus the
SSH workspace store and the extracted approval-mode helper. Their byte counts
and SHA-256 digests are checked in a targeted identity test.

| Hop | Added | Removed | Changed | Identical | Changed selected source files |
| --- | ---: | ---: | ---: | ---: | --- |
| `0.24.2 → 0.24.3` | 361 | 352 | 15 | 900 | `config.ts` |
| `0.24.3 → 0.24.4` | 435 | 425 | 15 | 836 | `config.ts` |
| `0.24.4 → 0.24.5` | 312 | 309 | 34 | 943 | `config.ts` |
| `0.24.5 → 0.24.6` | 673 | 660 | 22 | 607 | `config.ts`, `top-level-options.ts`, `dashscope.ts` |
| `0.24.6 → 0.24.7` | 502 | 477 | 40 | 785 | `config.ts`, `types.ts`, `systemController.ts` |
| `0.24.7 → 0.25.0` | 522 | 491 | 28 | 808 | `config.ts`, `top-level-options.ts` |

All package paths are in the complete diffs. The named source files below
are the selected mapping surface; unchanged mapped files keep their frozen
hashes in `selected-source-identities.json`. The official source tree's
`ssh-workspace-store.ts` and its source test are unchanged from `0.24.4`
through `0.25.0`. `approval-mode-value.ts` appears at `0.24.7` and is
byte-identical at `0.25.0`.

## Selected-Behavior Review

- **`0.24.2 → 0.24.3`:** `config.ts` adds a host-policy shell execution
  sandbox, masks review-lease paths for that policy, and changes the default
  PTY choice for an explicit one-shot prompt. This route sends no host shell
  sandbox policy and does not select a shell tool; the mapped read-only tool
  and permission set is unchanged.
- **`0.24.3 → 0.24.4`:** `config.ts` calls `readSshWorkspace(cwd)` before
  deriving `safeMode`. `ssh-workspace-store.ts` resolves
  `QWEN_HOME/ssh-workspaces/<64-lowercase-hex>/workspace`, reads its
  `connection.json`, and returns a connection used to construct
  `SshExecutionEnvironment`. Qwen then routes file, search, and shell tools
  to the remote project. `--safe-mode` does not prevent the selection. The
  path is derived from the approved cwd, but the adapter does not choose that
  remote host. This is an authority change; it is the sole newly qualified
  compatibility stop and is addressed by the authorized private guard.
- **`0.24.4 → 0.24.5`:** `config.ts` stops appending the review-lease path to
  a configured shell sandbox's masked paths. The selected route supplies no
  shell sandbox policy. The SSH selection introduced at `0.24.4` remains.
- **`0.24.5 → 0.24.6`:** `config.ts` adds the `batch` command and advisor
  model configuration and loads the review command only on explicit request;
  `top-level-options.ts` adds `--advisor`. The route selects none of these,
  and `--safe-mode` leaves the advisor unset. `dashscope.ts` changes proxy
  selection from a module-load constant to a getter. The child environment is
  fixed for this process and Swallowtail does not mutate it after launch, so
  that timing change does not alter the selected route's authentication or
  proxy input. The selected CLI/session wire remains unchanged.
- **`0.24.6 → 0.24.7`:** `config.ts` extracts `parseApprovalModeValue` into
  `approval-mode-value.ts`; its implementation is byte-identical, including
  accepted aliases and normalization. It adds a session execution-engine
  field and structured-memory-recall setting; safe mode keeps structured
  memory recall off. `types.ts` and `systemController.ts` pass optional MCP
  app-resource byte and timeout settings. The route declares no MCP app
  resources. `session.ts` is byte-identical, so session creation, resume and
  turn wire behavior are unchanged.
- **`0.24.7 → 0.25.0`:** `top-level-options.ts` changes `--sandbox` from a
  boolean to a string selector. The exact adapter argv does not contain
  `--sandbox`. `config.ts` adds an internal `agentHostReadOnly` policy that
  suppresses SSH selection, but it also forces safe mode and disables MCP and
  other configured tools; it is not a compatible local-only switch for this
  route and is not selected. The existing SSH cwd selection still runs when
  that internal policy is absent. Memory/Mem0 additions remain disabled by
  safe mode. No new operation or permission is mapped.

The complete artifact diffs include rebuilt distribution chunks, bundled
assets, documentation, translations, and provider-internal modules. They
remain fully enumerated by exact path and digest. Selected interface source
files and exact tag identities were reviewed separately; no source evidence
is inferred from a changelog or file count. No unrelated provider operation,
ACP interface, shell feature, remote host, or new permission is qualified.

## Deterministic Mapping Proof

The new `validate:current-qwen-headless` selector freezes exact npm identity,
the full seven-version inventories and all six stable-hop deltas. It runs the
current `0.25.0` claim on a deterministic fake process and asserts the exact
selected argv, unchanged delegated environment reference, and ordinary local
read-only resource binding. The same driver test resolves a reserved path,
a normalized path alias, and a symlink alias; each is rejected before the
fake process service starts and releases its lease. A prepared-session test
also proves that a reserved workspace is rejected before a turn child starts.
The guard only resolves host-approved resource leases and inspects their
canonical paths. It does not read `QWEN_HOME`, `connection.json`, credential
state, or provider configuration.

The adapter's `WorkingResourceRef`, execution host, access, and filesystem
lease remain exact. `EnvironmentRef` is passed unchanged. Lease release follows
joined child cleanup for runs and turns. Failure to resolve or canonicalize
the approved resource fails closed before process work. No provider prompt,
SSH connection, live catalogue, credential use, installation, host update,
or artifact execution occurred.

The fixture snapshots are evidence only; the downloaded tarballs were not
added to the repository. Their digests and complete extracted path manifests
are retained for later independent inspection.
