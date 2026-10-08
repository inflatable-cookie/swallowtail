# Copilot CLI ACP 1.0.80, 1.0.81, and 1.0.93 Offline Permission Boundary Stop

Date: 2026-10-08

## Scope

This record tests whether the exact official Darwin ARM64 Copilot CLI ACP
artifacts for the retained `1.0.80` point, first affected stable `1.0.81`, and
observed current stable `1.0.93` can reach permission handling without network
or real credentials. It is evidence for the blocked qualification continuation
tracked by issue #119; it does not change a route claim.

## Frozen identity

Unauthenticated npm registry metadata observed `@github/copilot` and
`@github/copilot-darwin-arm64` `latest` at `1.0.93`, with prerelease
`1.0.94-4`. GitHub Releases identified `v1.0.93` as the latest non-prerelease
release. The selected exact stable points were `1.0.80`, `1.0.81`, and
`1.0.93`. Wrapper and Darwin ARM64 native archives were fetched by exact
version, checked against npm integrity metadata, safely extracted in task
scratch, and inventoried in full. The frozen registry metadata and per-file
tree hashes are in
[`artifact-inventory.json`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/artifact-inventory.json).

| Version | Wrapper archive SHA-256 | Darwin ARM64 archive SHA-256 | Native executable SHA-256 |
| --- | --- | --- | --- |
| 1.0.80 | `799457937f8f87de6fdc95599380de5f5a0f761ab2fdfbba7f8d1c82d2988892` | `98640ca0de6576807f369c533c839b5742b038f105a970bdd7cb0d7efc8a7a71` | `fe779da7dd2342c1d23f0744873fa27d0251eaaee4dc6637fa53093639c0f3c9` |
| 1.0.81 | `0b0205dbb634579e3edc876d2f6facc5fde67c041dedc4eb037ef6f531fda08d` | `be324ba249b5744cb07fb7cfc275d52a0694136bc476d0299a9391bb3c0aa299` | `0f2ba6429dbee9f5adcdc2ad09ded7f5a0511f5da9af10c3a0dbc6ed070f004f` |
| 1.0.93 | `a8e704fb6874364af1b268aed2170bb597e0ca8086f3182b8fe5cb86ca3e43e1` | `f254651a3195e125b91d723d800e71e6541f8db3832d269854ae982254263eeb` | `df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1` |

The frozen inventory's committed digest is
`2d122117ccbb52dd547a783117ea3b1699df8e15357bca86a4d27a65416c8b0f`, which
matches the inventory digest recorded at execution time.
It captures the complete wrapper and native trees, including file modes and
sizes. No vendor binaries are stored in the repository.

## Containment and execution

Before exact artifact execution, the harness persisted a task-owned record and
passed its fake ACP and macOS sandbox checks. The fake peer completed
initialize, session creation, a permission request answered as cancelled, and
a cancelled prompt with no tool effect. Sandbox checks denied loopback and
reserved external connections, host-home reads, and keychain service access;
writes were confined to task scratch. The record was durable before each exact
binary process started. Exact invocations used `copilot --acp --stdio`,
isolated task homes and only a synthetic authentication placeholder. The
network profile denied all network access, including loopback.

| Version | Exact binary outcome before permission path | Tool effect |
| --- | --- | --- |
| 1.0.80 | No initialize result before bounded timeout; process group terminated | Absent |
| 1.0.81 | Initialize succeeded and version matched; no result for `session/new` | Absent |
| 1.0.93 | Initialize succeeded and version matched; no result for `session/new` | Absent |

None reached `session/prompt` or `session/request_permission`; no permission
reply, cancellation exchange, or tool-call update was observed. The no-effect
observations therefore do not prove permission-denial or cancellation
semantics. The prerelease reports in GitHub issue #4537 are not evidence for
these final stable artifacts and are not used to infer their behavior.

## Result

The exact no-network path stopped before permission handling. The harness
proved its own containment and fake permission/cancellation behavior, but did
not prove shipped stable binary behavior. The existing `copilot-cli.acp`
qualification remains exactly at `1.0.80`; this evidence makes no claim
increase, mapping adaptation, or inference about live MCP honoring. Further
proof requires a separately authorized path if the exact stable binaries need
authentication or a model/network service to create a session. The execution
record and reproduction notes are in the
[`offline proof fixture`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/README.md).
