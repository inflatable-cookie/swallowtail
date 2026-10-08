# Copilot CLI ACP 1.0.80, 1.0.81, and 1.0.93 Authentication Stop

Date: 2026-10-08

## Scope

This record tests whether the exact official Darwin ARM64 Copilot CLI ACP
artifacts for retained `1.0.80`, first affected stable `1.0.81`, and observed
current stable `1.0.93` can reach permission handling without network access or
real credentials. It is evidence for the blocked qualification continuation
tracked by issue #119. It does not change a route claim.

## Frozen identity

Unauthenticated npm registry metadata observed `@github/copilot` and
`@github/copilot-darwin-arm64` `latest` at `1.0.93`, with prerelease
`1.0.94-4`. GitHub Releases identified `v1.0.93` as the latest non-prerelease
release. Wrapper and Darwin ARM64 native archives for `1.0.80`, `1.0.81`, and
`1.0.93` were fetched by exact version, checked against npm integrity metadata,
safely extracted in task scratch, and inventoried in full. Their complete
file inventories and digests are in
[`artifact-inventory.json`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/artifact-inventory.json).

| Version | Wrapper archive SHA-256 | Darwin ARM64 archive SHA-256 | Native executable SHA-256 |
| --- | --- | --- | --- |
| 1.0.80 | `799457937f8f87de6fdc95599380de5f5a0f761ab2fdfbba7f8d1c82d2988892` | `98640ca0de6576807f369c533c839b5742b038f105a970bdd7cb0d7efc8a7a71` | `fe779da7dd2342c1d23f0744873fa27d0251eaaee4dc6637fa53093639c0f3c9` |
| 1.0.81 | `0b0205dbb634579e3edc876d2f6facc5fde67c041dedc4eb037ef6f531fda08d` | `be324ba249b5744cb07fb7cfc275d52a0694136bc476d0299a9391bb3c0aa299` | `0f2ba6429dbee9f5adcdc2ad09ded7f5a0511f5da9af10c3a0dbc6ed070f004f` |
| 1.0.93 | `a8e704fb6874364af1b268aed2170bb597e0ca8086f3182b8fe5cb86ca3e43e1` | `f254651a3195e125b91d723c800e71e6541f8db3832d269854ae982254263eeb` | `df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1` |

The committed artifact inventory SHA-256 is
`2d122117ccbb52dd547a783117ea3b1699df8e15357bca86a4d27a65416c8b0f`. It
contains the complete wrapper and native trees, including file modes and
sizes. No vendor binaries are stored in the repository. The execution record
SHA-256 is
`55193aaaa5a7e015738a33f59bfd0f2ba0f290e75a0d882e260054819cdd40ee`; it records
the harness SHA-256 `65e8357d2942c8dd554b8973d35e065ed7a4e7a9596bf1c103298396338acb2d`.

## Containment and execution

Before exact artifact execution, the harness persisted a secret-free
task-owned record and passed its fake ACP and macOS sandbox checks. The fake
peer completed initialize, session creation, a permission request answered as
cancelled, and a cancelled prompt with no tool effect. The sandbox denied
loopback and reserved external connections, host-home reads, and keychain
service access; writes were confined to task scratch. The record was durable
before each exact binary process started. Exact invocations used
`copilot --acp --stdio`, isolated task homes, a synthetic token placeholder,
and a profile denying all network access.

The harness allows twenty seconds for each ACP response. All three exact
artifacts initialized and matched their frozen versions. Each advertised the
`copilot-login` auth method. Each then returned a JSON-RPC error for
`session/new`: code `-32000`, message `Authentication required`. The harness
retains only that known message and error code; provider-supplied error data is
omitted.

| Version | Initialize | Advertised auth method | `session/new` | Permission / tool effect |
| --- | --- | --- | --- | --- |
| 1.0.80 | Success; version matched | `copilot-login` | `-32000 Authentication required` | Not reached / absent |
| 1.0.81 | Success; version matched | `copilot-login` | `-32000 Authentication required` | Not reached / absent |
| 1.0.93 | Success; version matched | `copilot-login` | `-32000 Authentication required` | Not reached / absent |

No authentication flow was attempted. The task forbids real credentials and
network access, so the exact path stopped at the explicit authentication
error. No session or prompt was created, and no permission request, cancel
reply, or tool call occurred. The absent tool effect does not prove a
permission boundary. GitHub issue #4537 concerns prereleases and is not used to
infer behavior for these final stable artifacts.

## Result

The selected exact stable path requires authentication to create a session.
This run therefore does not establish stable permission, cancellation, or
no-effect behavior. The existing `copilot-cli.acp` qualification remains
exactly at `1.0.80`; no mapping adaptation, claim increase, or inference about
live MCP honoring follows. Additional permission evidence requires an approved
authentication path outside this synthetic-only, no-network run. The
secret-free execution record and reproduction notes are in the
[`offline proof fixture`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/README.md).
