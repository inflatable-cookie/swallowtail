# Research 396: Pi SDK Sidecar Node 22.23.3 Identity and Qualification

Status: qualified through the current official Node 22 stable `22.23.3`

Observed 2026-10-08 on the official Node 22 distribution channel. This task
qualifies only `pi.sdk-sidecar.node`; the existing Pi SDK tuple, source tag,
wire, behavior revision, platform boundary, and every prior route point remain
unchanged. The official Node 22 channel still reports `22.23.3` as current,
after the previously qualified `22.23.2`. Re-probe it immediately before push.

## Outcome and segment

The exact `22.23.2` point remains qualified, and the maintained Node segment is
now `22.23.2..=22.23.3` on `pi.sdk-sidecar.node`, claim
`pi.sdk-sidecar.node-window-1`, behavior `pi.sdk-sidecar-v1`, with
`QualifiedOnly` posture. This is a compatible extension of the same Node axis.
No SDK, sidecar, wire, public operation, tool, permission, usage, lifecycle,
baseline, or release claim changed. Older `22.23.1`, prereleases, Node 26, and
later points remain outside this qualified segment; at observation, `22.23.4`
was not published.

The exact other axes remain `@earendil-works/pi-coding-agent@0.84.2`,
`swallowtail-pi-sdk-jsonl-v1`, `pi.sdk-sidecar-v1`, and
`swallowtail-pi-sdk-sidecar@0.5.1`. The package/source baseline remains
[Research 181](./181-pi-sdk-sidecar-route-qualification.md). Pi SDK source
review used official tag `v0.84.2`, commit
`914cf1472e715297caa30db4b9535d534a9eb718`; exact selected source path hashes
and call-path review are in
[`396-pi-sdk-sidecar-node-22-23-3-hop-review.json`](./396-pi-sdk-sidecar-node-22-23-3-hop-review.json).
No evidence is transferred from the sibling Claude SDK Node qualification.

## Official hop and artifact identity

The only published stable hop after the existing ceiling is `22.23.2` to
`22.23.3`. Both Darwin ARM64 tarballs were checked against Node's signed
`SHASUMS256.txt` using the official release keyring. Their exact source tag,
binary and bundle identities are retained in the
[identity record](./396-pi-sdk-sidecar-node-22-23-3-identity.json).
The host `node` observation was `v22.23.2`, Darwin ARM64, digest
`18e387c90ab8a8400183e8bdd396376e1e875b91b4c874b894dcade7b35bf572`; it
matches the official `.2` binary and was not the qualification input.

| Node | Official Darwin ARM64 tarball SHA-256 | `bin/node` SHA-256 | Source tag peeled commit |
| --- | --- | --- | --- |
| `22.23.2` | `61130f394c1630d211dd50aecc4353d379480f36d3ac913cd85dbba1aed585c6` | `18e387c90ab8a8400183e8bdd396376e1e875b91b4c874b894dcade7b35bf572` | `aa4c77582be995286fc6e00aaf530dc7ade102a9` |
| `22.23.3` | `23b25245dcfb9af7262f8ff142e9e2e0af025368117329e7a7458a51e5922f53` | `68f4d07ca49e0500cc135c7e0a445093e228e42e126ac22306d045f0a8c2636b` | `80dc632040e6bada37aac1220dde9c79581c9c22` |

The `.2` checksum signature was dated 2026-07-29 and verified under
`CC68F5A3106FF448322E48ED27F5E38D5B0A215F` (Marco Ippolito). The `.3`
signature was dated 2026-09-23 and verified under
`5BE8A3F6C8A5C01D106C0AD820B1A390B168D356` (Antoine du Hamel). The official
release keyring SHA-256 is
`610b8d249da3d5733f5a128def2dd0294dbbf5b5713e6ca2529db8db419dee00`.

Each extracted distribution has 5,865 entries (4,750 regular files, 1,112
directories, and 3 symlinks). The complete hop has zero added or removed paths,
393 changed paths, and 5,472 identical paths. The complete inventories and path
sets remain in the two numbered inventory JSON files and
[`tree-diff.json`](./396-pi-sdk-sidecar-node-22-23-3-tree-diff.json). The
separate hop review classifies every changed distribution path into the Node
executable, Node documentation, native-addon headers/build metadata, bundled
OpenSSL headers, embedded Corepack, or embedded npm groups. Those classifications
are mutation checked against the exact changed path set.

## Node TLS and HTTP path review

The release updates OpenSSL from `3.5.7` to `3.5.8` and the embedded Undici
runtime from `6.28.0` to `6.28.1`. Source and runtime evidence confirms the
bundled TLS root snapshot changed: `tls.rootCertificates` contains 145 roots
in `.2` and 119 in `.3`; 26 prior DER identities are absent or changed, and no
target identity is added. The exact source-header hashes and all removed names
and DER fingerprints are preserved in the identity record and
[`root-certificate-diff.json`](./396-pi-sdk-sidecar-node-22-23-3-root-certificate-diff.json).
This is accepted as a Node-version-specific provider trust boundary under the
approved continuation. It does not establish whether a particular provider
endpoint needs a removed root.

The frozen Pi `ModelRuntime.create` path uses `modelsPath: null` and
`allowModelNetwork: false` for both catalogue and session. Provider HTTP
defaults to `globalThis.fetch`; the sidecar supplies no fetch, dispatcher,
CA, HTTPS agent, proxy, or certificate-verification override. Pi Codex's
automatic WebSocket transport uses Node's global `WebSocket`. The `.3` Undici
changes feeding those selected transports were classified individually:

- The HTTP/1 idle keep-alive path now checks pending peer bytes, FIN, or RST in
  a refed `setImmediate` before reusing a socket. The transport scheduling and
  stale-socket failure path changed; Pi's request/response mapping did not.
- WebSocket protocol validation now fails closed when a server offers a
  subprotocol that was not requested. WebSocket inflate cleanup now destroys
  the decoder after an over-limit message, preventing a later unhandled error.
  Valid session wire and sidecar event projection stay the same.
- The changed Undici retry handler is not selected: the sidecar injects no
  `RetryAgent` or dispatcher, and Pi Codex owns its fetch retries. Undici's
  EventSource parser is not selected: Pi reads its SSE responses through Fetch.

The distribution inventory also freezes package-manager, header, and build
changes. The sidecar does not invoke the embedded npm or Corepack; native
headers and build metadata are not runtime inputs. No new Node API or upstream
operation is added to the Pi route. Exact Node and Pi paths, blob identities,
classification rationale, and SDK call path are in the hop-review JSON.

## Offline trust proof and limits

Both exact Node binaries were run only against local synthetic inputs in an
isolated temporary directory. Each runtime reported the bundled root-set
fingerprint digest recorded in
[`tls-offline-proof.json`](./396-pi-sdk-sidecar-node-22-23-3-tls-offline-proof.json).
With an empty inherited environment except for path and scratch-home settings,
default `fetch` rejected the self-signed loopback HTTPS certificate with
`DEPTH_ZERO_SELF_SIGNED_CERT` on both versions. The normal hostname check
returned `ERR_TLS_CERT_ALTNAME_INVALID` for the deliberately mismatched test
name on both versions. No CA was installed or supplied, and no remote endpoint,
credential, catalogue, or session was used.

Node's normal certificate-chain and hostname verification remain enabled.
This proof qualifies the runtime default and its fail-closed path only. It does
not claim an endpoint trusts the new root set, that a required endpoint avoids
the removed roots, or that custom/system CA trust is supported. Any required
endpoint that depends on a removed root, custom CA, or system trust needs a
separate evidence task before that compatibility can be claimed.

## Sources and method

- [Official Node 22 distribution index](https://nodejs.org/dist/index.json)
- [Node 22.23.3 release notes](https://nodejs.org/en/blog/release/v22.23.3)
- [Node 22.23.2 signed checksums](https://nodejs.org/dist/v22.23.2/SHASUMS256.txt)
- [Node 22.23.3 signed checksums](https://nodejs.org/dist/v22.23.3/SHASUMS256.txt)
- [Node 22 TLS documentation](https://nodejs.org/download/release/v22.23.3/docs/api/tls.html)
- [Node v22.23.2 source tag](https://github.com/nodejs/node/tree/v22.23.2)
- [Node v22.23.3 source tag](https://github.com/nodejs/node/tree/v22.23.3)
- [Pi v0.84.2 source tag](https://github.com/earendil-works/pi/tree/914cf1472e715297caa30db4b9535d534a9eb718)

Historical artifact identities, the exact hop tree inventory, source headers,
release checksums, and the root comparison were preserved; the prior artifact
proofs and canonical sibling number repairs were not repeated. No package was
installed, no host runtime was updated, and no provider traffic or credential
was used.
