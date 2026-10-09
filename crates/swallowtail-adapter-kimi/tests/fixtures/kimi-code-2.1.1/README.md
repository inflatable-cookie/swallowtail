# Kimi Code 2.1.1 Currentness Evidence

Secret-free static evidence for the official npm and GitHub stable chain from
the previously qualified `0.43.0` ceiling through `2.1.1`. It covers only the
`kimi-code.headless` route. ACP and local-server claims are separate families
and are not changed here.

`identity.json` freezes npm registry identity and the matching GitHub release
tag, commit, and tree for `0.43.0`, `0.43.1`, `2.0.0`, `2.0.1`, `2.0.2`,
`2.1.0`, and `2.1.1`. The npm points include registry integrity, shasum,
tarball SHA-256, package size, runtime entry digest, Node floor, and GitHub
release asset digests. The npm package has no `gitHead`; the tagged source
identity is recorded independently and no source parity is inferred from the
package version.

`artifact-tree.json` is a sorted path, SHA-256, size, and mode inventory of
every file in each downloaded npm tarball. Its hop deltas classify every added,
removed, and changed path. `source-tree.json` is the complete path, mode, and
Git blob inventory of each tagged source tree, with the same per-hop path
classification. The full maps make identical paths derivable without storing
them twice.

`protocol.json` records the selected headless source ledger, per-hop review of
CLI, loop, tool, permission, and path-access changes, and static marker counts
from each shipped `dist/main.mjs`. It also records the before and after claim
shapes. Under the operator ruling, qualify `2.1.1` and leave exact `2.1.0`
unsupported; unpublished `0.43.2`, the `1.x` line, and `2.0.3` stay outside the
qualified segments. The bundle oracles in `bundle-oracles.json` freeze the
selected stream writers, dispatch, version preamble, and print background
policy. Artifacts were inspected statically and never executed.

The `2.0.2 → 2.1.0` hop adds symlink-aware workspace checks to Kimi's built-in
file tools. `2.1.1` removes them. Because the checks change effective security
and filesystem authority in the selected headless runtime, only `2.1.1`
qualifies; `2.1.0` remains an exact unsupported gap.

The installed `kimi 0.34.0` identity is recorded for continuity. No host path,
credential, provider prompt, authentication, live session, host update, or
binary execution is included.
