# Research 387: Claude Agent SDK Node 22.23.3 Identity and Qualification

Observed 2026-10-08. Scope is the `claude-agent.sdk.node` runtime axis only.
The official [Node 22 distribution index](https://nodejs.org/dist/index.json)
reported `22.23.3` as the newest stable point after the qualified `22.23.2`
ceiling at `2026-10-08T07:02:20Z`. The only published stable hop after the
ceiling was `22.23.3`, published September 23; `22.23.4` was absent. This is
not an npm or SDK package channel. Recheck the Node index immediately before
push; a new published point must be assessed under Contract 029 before the
claim moves.

The selected tuple preserves the existing SDK, native, wire, sidecar, and
behavior identities: `@anthropic-ai/claude-agent-sdk@0.3.284`, native
`2.1.284`, `swallowtail-claude-agent-sdk-jsonl-v1`, sidecar source tag
`swallowtail-claude-agent-sdk-sidecar@0.5.1`, and behavior
`claude-agent.sdk-v1`. Only Node changes. The old Node `22.23.2` point remains
qualified; the new maintained segment is `22.23.2..=22.23.3` on claim
`claude-agent.sdk.node-window-1`, with `QualifiedOnly` posture, no exclusions,
and no new public operation or lifecycle behavior. `22.23.4` and later points,
other Node major lines, and all SDK/native version movements remain outside
this claim.

## Official artifact and source identity

Both selected archives are official Darwin arm64 artifacts. Their detached
`SHASUMS256.txt` signatures verified against the official Node release
keyring. The archive, manifest, and signature identities are:

| Point | Archive SHA-256 | SHASUMS SHA-256 | Signature SHA-256 | Verified signer |
| --- | --- | --- | --- | --- |
| `22.23.2` | `5eff7a9011895aae3f29d06f167b84a62b028a591370c7cafb59103559fd26e1` | `778ac5b2fcdbd68d9c0ae9f4310674faa3af0910bd0d18e7f6597787c40a3e39` | `169f1452c14cd653247408352f1534b9f31e3d13f9c6399c3977368095e11eda` | `CC68F5A3106FF448322E48ED27F5E38D5B0A215F` |
| `22.23.3` | `72d5d8832b41c9d9646197af614ffd751406ea4d215060eb91b98864e1919a3e` | `4fe99a2ba9d552a6f51c13ed68fb11104cfa5df601aec616be689253a8139e7a` | `0aafa311177794108f9f15cf7baebc747ba21983e999b20eee94738c651469e0` | `5BE8A3F6C8A5C01D106C0AD820B1A390B168D356` |

The `22.23.3` archive's `bin/node` SHA-256 is
`68f4d07ca49e0500cc135c7e0a445093e228e42e126ac22306d045f0a8c2636b`; the
prior point's is
`18e387c90ab8a8400183e8bdd396376e1e875b91b4c874b894dcade7b35bf572`. The
candidate binary reported `v22.23.3` when invoked with `--version` under
isolated `HOME` and `TMPDIR`; it was not installed or used to update the host.

The complete extracted distribution trees each contain 5,865 entries. Their
canonical tree digests are `13f3f6716b51144605fc2deff341d8d76cfc49896b81d41ac175119ef276bce5`
for `22.23.2` and
`50e45afc486c881797754ffe363ad2d6a13b86bd43ad064ec180406499f13d9d` for
`22.23.3`. The hop has 393 changed paths, no added or removed paths, and only
`bin/node` as a changed selected runtime executable. The exhaustive path,
kind, mode, size, and digest ledger is frozen in
[`dist-inventory.json`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-node-22.23.3/dist-inventory.json),
SHA-256 `dcf2aa0d6d313587bdcb6ccbe4b404ec0922683068fd11702184d11dfa6e1d84`.
Its changed paths are classified as 1 selected runtime executable, 2
documentation/license entries, 176 unselected development headers, and 214
unselected npm/Corepack entries. The regression test checks exact category
keys, counts, uniqueness, and equality between the classified and changed path
sets.

The upstream source comparison is between Node tag `v22.23.2` (tag object
`490a9fef8f8adcda5a95bd6f96035b05cb43fe5b`, peeled commit
`aa4c77582be995286fc6e00aaf530dc7ade102a9`) and `v22.23.3` (tag object
`9ff018de597f54877b72bde69eb37de4e47c15c4`, peeled commit
`80dc632040e6bada37aac1220dde9c79581c9c22`). The full source hop has 755
changed paths: 750 modified, 4 added, and 1 deleted. Its complete path ledger
contains both source blob ids, modes, status, and one explicit classification
for every path in
[`source-delta.json`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-node-22.23.3/source-delta.json),
SHA-256 `91173b432f0589721cd8d9f68d37bcbdb8e608aca1d7360c38115c25dc5dc180`.
The classifier covers these exact path counts: TLS trust store 401; npm 210;
DNS 65; unshipped CI/tests 32; unshipped docs/build tooling 16; global HTTP
client 9; locale/time data 5; Corepack 4; Node API 3; selected ESM loader 3;
HTTP/2 2; and one each for call-site formatting, JavaScript engine, URL setter,
version metadata, and Windows task runner. The identity test checks the exact
classification and reason keys, counts, unique path sets, and one-to-one
coverage of all 755 changes.

The selected source change is in
`lib/internal/modules/esm/load.js`, `lib/internal/modules/esm/resolve.js`, and
`lib/internal/modules/esm/translators.js`: Node's ESM internals use the `fs`
module object so explicit filesystem patches remain observable. The sidecar
does not patch `fs`. This loader change is exercised by the fake-backed
sidecar-open proof under the extracted `22.23.3` binary. N-API, V8,
TLS/trust-store, DNS, global HTTP/2/client, locale, package-manager, and
Windows-specific changes remain classified in the frozen ledgers; they do not
add a Swallowtail operation or provider/live evidence.

## Qualification bounds

The fake-backed route proof binds the candidate to the existing SDK `0.3.284`,
native `2.1.284`, wire, behavior, and sidecar identity. It covers sidecar
option/callback handling, identity and open readiness, wire decoding and
lifecycle, usage projection, and registered-tool mediation. The SDK package
and native executable are fakes in this proof. No provider prompt, live
catalogue/session, credential, or provider HTTP request was used. The
sidecar's Node runtime performs no provider HTTP request, so Node's TLS, DNS,
HTTP, or HTTP/2 changes do not transfer the separate live HTTP MCP gate.

This artifact proof is Darwin arm64 only. It makes no new Linux environment,
consumer, or provider claim. Research 301's live registered-tool acceptance
remains tied to SDK `0.3.259`, native `2.1.259`, Node `22.23.2`, and its exact
Darwin arm64 evidence. Research 303's native-read mediation evidence remains
on its historical SDK `0.3.284`, native `2.1.284`, Node `22.23.2` tuple. Neither
live acceptance nor any sibling route evidence transfers to `22.23.3`.

No SDK/native, wire, sidecar-source, platform, capability, permission,
credential, lifecycle, or public API claim changes here. The exact retained
route tuple is summarized in
[`identity.json`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-node-22.23.3/identity.json),
and the claim and selected proof boundary in
[`protocol.json`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-node-22.23.3/protocol.json).
