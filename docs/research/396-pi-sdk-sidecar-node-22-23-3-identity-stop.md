# Research 396: Pi SDK Sidecar Node 22.23.3 Identity Stop

Status: the official Node 22.23.2→22.23.3 identity is frozen. Qualification is
stopped pending a security/authority ruling; no Pi claim changed.

Observed 2026-10-08. The official Node distribution index lists `22.23.3`
(2026-09-23) as the current Node 22 stable, immediately after `22.23.2`
(2026-07-28). The Pi SDK tuple remains package
`@earendil-works/pi-coding-agent@0.84.2`, sidecar behavior
`pi.sdk-sidecar-v1`, wire `swallowtail-pi-sdk-jsonl-v1`, and source tag
`swallowtail-pi-sdk-sidecar@0.5.1`. Research 181 retains the SDK source and
package identity; this record changes only the Node runtime identity review.
Its Node claim `pi.sdk-sidecar.node-window-1` still qualifies only exact
`22.23.2`.

## Official hop and artifact identity

The only published selected-channel hop after the existing ceiling is
`22.23.2`→`22.23.3`; the current index shows no later Node 22 stable. Both
Darwin ARM64 tarballs were checked against Node's signature-verified
`SHASUMS256.txt` using the official release keyring. No downloaded binary was
executed or installed.

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

The exact tuple and artifact/source identities are also recorded in
`396-pi-sdk-sidecar-node-22-23-3-identity.json`.

Complete extracted Darwin ARM64 distribution inventories and the exact path
delta are frozen alongside this record. Each tree has 5,865 entries (4,750
regular files, 1,112 directories, 3 symlinks). The hop has 0 added, 0 removed,
393 changed, and 5,472 identical paths. The full path/content inventories are
`396-pi-sdk-sidecar-node-22-23-3-inventory-22.23.2.json` and
`396-pi-sdk-sidecar-node-22-23-3-inventory-22.23.3.json`; the complete path
sets are in `396-pi-sdk-sidecar-node-22-23-3-tree-diff.json`. This freezes
artifact identity, not a classification of all changed files against Pi
behavior; that review stopped when the security boundary change was found.

The official Node source tags are annotated tag objects
`490a9fef8f8adcda5a95bd6f96035b05cb43fe5b` (`v22.23.2`) and
`9ff018de597f54877b72bde69eb37de4e47c15c4` (`v22.23.3`), peeled to the
commits above. Their `src/node_root_certs.h` files have SHA-256
`b8381bf64c65982dc9e93e87489e98f25cc660cfe46bd0f53ffaf99154ac6aac` and
`0a8a329be474650b59a08c0d0bdc20965c1591ea42983e1a7204983cdb5aaba0`.

## Security/authority stop

The official `22.23.3` release notes identify “crypto: update root
certificates to NSS 3.125.” Parsing the bundled certificates from both exact
source-tag headers and comparing DER SHA-256 identities found 145 roots in
`22.23.2` and 119 in `22.23.3`: 26 prior root identities are absent or
changed, with no new or changed identity on the target side. The names and
DER digests are frozen in
`396-pi-sdk-sidecar-node-22-23-3-root-certificate-diff.json`.

Node v22 TLS documentation describes `tls.rootCertificates` as the versioned
bundled Mozilla root snapshot; the release notes and source diff show that
this trust input changed. This is a Node TLS trust-authority change. The
static evidence does not establish whether a particular provider endpoint
uses any affected root, and no live provider request was made.

The task brief requires a separate ruling for a security/authority change.
Therefore retain the existing exact `22.23.2` qualified point and its
exclusions, keep `22.23.3` unqualified, and leave package, wire, behavior,
guide, route matrix, and `CHANGELOG.md` claims unchanged. The adapter's fake
sidecar proofs do not resolve the Node trust policy. No qualification or
consumer-visible compatibility conclusion is inferred from this record.

## Next decision

Rule whether the changed bundled trust roots are within the authorized Pi Node
axis extension. If they are, define the required trust-store treatment and
proof boundary for the one-family adaptation; if they are not, identify the
approved runtime/trust identity that the Pi SDK must retain. Any live provider
proof remains separately gated. This is a currentness stop requiring an
adaptation, not a terminal unsupported result.

Official sources: [Node distribution index](https://nodejs.org/dist/index.json),
[Node 22.23.3 release notes](https://nodejs.org/en/blog/release/v22.23.3),
[Node v22 TLS documentation](https://nodejs.org/download/release/latest-v22.x/docs/api/tls.html),
[Node v22.23.2 artifact checksums](https://nodejs.org/dist/v22.23.2/SHASUMS256.txt),
[Node v22.23.3 artifact checksums](https://nodejs.org/dist/v22.23.3/SHASUMS256.txt),
[Node v22.23.2 source tag](https://github.com/nodejs/node/tree/v22.23.2), and
[Node v22.23.3 source tag](https://github.com/nodejs/node/tree/v22.23.3).
Existing Pi SDK package/source baseline: [Research 181](./181-pi-sdk-sidecar-route-qualification.md).
