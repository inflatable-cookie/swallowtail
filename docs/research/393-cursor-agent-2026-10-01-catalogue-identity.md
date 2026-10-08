# Research 393: Cursor Agent 2026.10.01 Catalogue Identity and Qualification

Status: complete identity evidence for `cursor-agent.catalogue`; production
claim update is recorded separately in this change.

Observed 2026-10-08 on `cursor-agent.release-date`. The official ACP registry
current entry reports `2026.10.01` and publishes Cursor archive `2026.10.01-14929f9`
for Darwin ARM64 and Linux x64. The official registry main at
`a1c535174151467335d39083cc2167b210b7e3c1` contains the same current entry.
The downloaded artifact identities and complete inventories are frozen in
the [identity fixture](../../crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-2026.10.01/README.md).

## Official hop ledger

The selected channel is the official ACP registry Cursor entry and its
versioned archive URLs. All published hops after the catalogue ceiling
`2026.09.18-9a7762b` are included: registry commits
`b62398f8201ca95926c16c93d0aaf7d7e8cf9d94` (`2026.09.26-dd393fe`),
`3a6bcd28d931a154338602f966418d00f8c71229` (`2026.09.28-64d2043`), and
`310074e4c18c5ff39ec178e84d0f461f8ff73fe0`
(`2026.10.01-14929f9`). Registry dates were used for discovery; every
selected point is bound to its exact archive digest and build revision.
There is no later Cursor update in the registry history at observation. No
calendar gaps are inferred.

| Version | Darwin ARM64 archive SHA-256 | Linux x64 archive SHA-256 | Darwin files | Linux files |
| --- | --- | --- | ---: | ---: |
| `2026.09.26-dd393fe` | `538827d96a779bab854a865c8e42859e8e87db34b5f69770261b90fc8cfff191` | `8085fd120f5c71f4eae7fea26a043718e5644e3071e4fab3220a0e58c51f9593` | 446 | 454 |
| `2026.09.28-64d2043` | `c0d7e9cd2e62438610b886d3439907dc1f98c2923b07b3a41416cc919aaf53c7` | `6e4cd936a4866b8a77c50ff51a564460d715772fabc477a01aa0f0455d9559f0` | 447 | 455 |
| `2026.10.01-14929f9` | `778d04e542adc5c8b6760fda3ebe0757f903b1764f2792c232ef9a35e6e2151b` | `ba9a855f8f813c91b9f2707127572d2dc9ae5a62818e1c36719625d0fb8bd452` | 447 | 455 |

The prior qualified ceiling archive is retained as the first tree in the
inventory, so each of the three adjacent hops is classified independently.
Darwin ARM64 deltas are `+63/-62/~54`, `+65/-64/~82`, and `+60/-60/~80`
(added/removed/changed); Linux x64 deltas are `+5/-4/~44`, `+64/-63/~23`,
and `+0/-0/~20`. Each delta also records the exact byte-identical path set.
The complete inventory contains every path, size and SHA-256 in all eight
trees, plus path-set digests and exact per-hop classifications.

## Selected catalogue surface

The command definition remains
`.command("models").description("List available models for this account")`.
The selected handler remains `handleModelsList` and keeps the account options
`endpoint`, `apiKey`, `authToken`, and `insecure`. It builds the available
model set with `useModelParameters=true` and `doNotUseMarkdown=true`, then
prints the same `Available models` heading and rows. The adapter's existing
parser maps model identity and visible display text. It extracts default status
only from the exact `(current, default)` suffix and exposes no separate current
field. It does not claim invocability or parameter support. A nonzero process
exit still fails as `catalogue_exit_failed`; malformed or empty output fails
closed through the existing plain-text parser. New parameterized-model help
in an upstream `Tip` line is ignored by the adapter. The added `team` command,
local-provider branch, interactive model picker, and internal feature changes
are unmapped.

Complete selected file identities are listed for both architectures and all
hops in `dist-inventory.json`. The entrypoint and selected command/service
chunks changed with builds; the bundled Node runtime, shell launcher, and
package metadata remain byte-identical per architecture. Every other
packaged path remains visible in the exhaustive delta ledger and is not
projected into the selected catalogue behavior.

## Contract 029 decision

This is a compatible extension of the existing
`cursor-agent.catalogue.calendar-release-v1` behavior and
`cursor-agent.catalogue.release-window-3` claim. Keep baseline
`2026.07.01-41b2de7`, the nine previously qualified points, and older
independently unqualified published gaps `2026.08.25-3e8eec8` and
`2026.09.08-6caf4ff`. Add only the three exact points above. No new public
operation, consumer-visible narrowing, behavior revision, or authority is
introduced. The ACP and headless claims retain their separate `2026.09.18`
ceilings; no evidence transfers to those axes. npm `cursor-agent@1.0.3` is
a different identity.

The host version was observed as the previous ceiling using an isolated
`cursor-agent --version` probe. Archives were hashed and inspected statically,
not executed. No authenticated catalogue, provider session, prompt, credential,
installation, or host update was used.

Official sources: [ACP Registry current metadata](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json), [Cursor ACP registry entry](https://github.com/agentclientprotocol/registry/blob/main/cursor/agent.json), [registry commit history](https://github.com/agentclientprotocol/registry/commits/main/cursor/agent.json).
