# Cursor Agent Headless 2026.10.01 Identity and Qualification

## Decision

Qualify only `cursor-agent.headless` through official stable
`2026.10.01-14929f9`. Keep its baseline `2026.07.01-41b2de7`, claim id
`cursor-agent.headless.release-window-3`, behavior revision
`cursor-agent.stream-json.structured-v1`, prior exact points, gaps, and
`AllowUnverified` posture. Add the exact `2026.09.26-dd393fe`,
`2026.09.28-64d2043`, and `2026.10.01-14929f9` points. At this identity
capture, the separate ACP claim was qualified through `2026.09.18-9a7762b`;
its later qualification through `2026.10.01-14929f9` is recorded by Research
395 and is not evidence for this headless claim. The catalogue claim is
unchanged.

This is an independently reviewed compatible extension of the existing
headless wire and driver. It adds no public operation, capability, authority,
or behavior revision. It does not transfer catalogue or ACP evidence.

## Official channel and hops

The selected channel is the Cursor entry in the
[official ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
and the archive URLs published by that entry. The registry reported
`2026.10.01-14929f9` as latest stable on 2026-10-07. Its published registry
commits record every hop after the headless ceiling:

| Version | Registry commit | Commit date |
| --- | --- | --- |
| `2026.09.26-dd393fe` | `b62398f8201ca95926c16c93d0aaf7d7e8cf9d94` | 2026-09-28 |
| `2026.09.28-64d2043` | `3a6bcd28d931a154338602f966418d00f8c71229` | 2026-09-30 |
| `2026.10.01-14929f9` | `310074e4c18c5ff39ec178e84d0f461f8ff73fe0` | 2026-10-02 |

The installed host reported `2026.09.18-9a7762b` through the isolated
`CURSOR_HOME` version probe. No update or installation was performed. The
separately published `2026.08.25-3e8eec8` and `2026.09.08-6caf4ff` points stay
unqualified. Calendar dates between published points remain gaps. npm
`cursor-agent` is a separate axis.

## Artifact identity and complete trees

Both official Darwin ARM64 and Linux x64 archives were downloaded and their
SHA-256 digests matched the published package identities. Each archive was
inspected offline; none was executed. The table records the complete-tree
manifest digest, runtime index, and selected headless writer chunk for each
artifact. Archive URLs use Cursor's versioned
[`downloads.cursor.com/lab`](https://downloads.cursor.com/lab/) release path.

| Version / platform | Archive SHA-256 | Complete tree | Runtime index SHA-256 | Headless writer chunk SHA-256 |
| --- | --- | --- | --- | --- |
| `2026.09.18-9a7762b` Darwin ARM64 | `4e67b9ac80cc4a56e0a91b3b437894e0ba489ef7ec37f120d8084d2bfd02095d` | 445 files; `ee5d619e7026b0b36b199a2fa242fe5ff3edf4369a8b330df6892f49ce390a5c` | `19af19274b07dcb8ad9bd86915320b7b49a62de763af10c6857ab45257c8a14b` | `1218.index.js` `585414d66d3759f374326ea903a3e117f09e0c4a6d9f55d2d407e3882d2a94e6` |
| `2026.09.18-9a7762b` Linux x64 | `b1308f5a2fc05458b9d8966752986bb23a971bbcc67c842c1df94c4b8132bad9` | 453 files; `a645783d7179ee0943f1523378603789e914bae0eb857dec19a172dceea1dcf1` | `c4f1fb0ebae82b89261f6beffb9167ce03df95c7b114c2c0ce066f0271529c03` | `7021.index.js` `e92538763e1bde7afff7be24eb504a51d2ff0afbcc6ad3c94d32b1f0ad4feb18` |
| `2026.09.26-dd393fe` Darwin ARM64 | `538827d96a779bab854a865c8e42859e8e87db34b5f69770261b90fc8cfff191` | 446 files; `1cc6d2ae66858469d31c4ef76247ab511dc6d01723365cb76f174df0c2d81e5e` | `f8bd1c549f844859f8aeb9f06c01420f299bee22fe136695fe891eaf18674b12` | `9352.index.js` `188134bf2f3a4f17e47dc20ad0f295b77a59b8c825d16594b45b5744fd3cebb9` |
| `2026.09.26-dd393fe` Linux x64 | `8085fd120f5c71f4eae7fea26a043718e5644e3071e4fab3220a0e58c51f9593` | 454 files; `6e82b78b0924c63ce5c03a8c563fb826d76bba896d94a6ffd181017053cfac62` | `abc8b0878cc059942b6038cef0c631b755d7911dcb8eade3855166d6575cb4b7` | `7021.index.js` `8372a83c3853a0ccbf1f1c02d34be64258fa75ae1d29d9fdccb293f773061c0f` |
| `2026.09.28-64d2043` Darwin ARM64 | `c0d7e9cd2e62438610b886d3439907dc1f98c2923b07b3a41416cc919aaf53c7` | 447 files; `848402ca013e37a33da602ca1902d7e0953e743a115a0e6ce6dbcad6360be778` | `0d0c83f5478c3dcd806cb9697123cee3f548005fbcd3acafe31f102d659f5a7b` | `6949.index.js` `638d60026e23ff5993daa1fd911bdb807a0e5ea70037297909456cb5cfa7d4f5` |
| `2026.09.28-64d2043` Linux x64 | `6e4cd936a4866b8a77c50ff51a564460d715772fabc477a01aa0f0455d9559f0` | 455 files; `f71e8f6dc552b10d034d55810a6f2b1bf43c8164b35b1e0e0966e38585692116` | `45d9b1df85d0165cb2e690f96fa5fbe4b59e8a71ddd22176a301ba7e5b18a0b9` | `7000.index.js` `749a10b62a2094860d219683b5c5bb36e12c1fb036fcd31e08789f0085b2f5bd` |
| `2026.10.01-14929f9` Darwin ARM64 | `778d04e542adc5c8b6760fda3ebe0757f903b1764f2792c232ef9a35e6e2151b` | 447 files; `c21440a7188f442e6e3c98207937b8f916aa68eafdc9e56870411c7c2ba5f5e6` | `ad1d9d915946a57ff1bf0d8364a82b91870035a09f19c502ffac9bb5b95edc5d` | `1962.index.js` `ecc9ccc254e97606503b982e724dd7948fa2e94ee055bb1c4b1111bd73c3d2a5` |
| `2026.10.01-14929f9` Linux x64 | `ba9a855f8f813c91b9f2707127572d2dc9ae5a62818e1c36719625d0fb8bd452` | 455 files; `908061cf1d0d0e2d18714dfe115c1321696adf9a12bd2fdc3513ad7690852a34` | `7e14af745050e80afc473c8b8fa2b7bba64197ca96f0181b4b5c2d359193bee1` | `7000.index.js` `e25d67ad6b2c9a78e4480098b5e161cf5c51f8b4b23e24aee132fdd180b6b3a1` |

The exact archive URLs, full file lists, per-file digests, complete tree
manifests, and hop ledgers are frozen in
[`cursor-agent-headless-2026.10.01/identity.json`](../../crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-headless-2026.10.01/identity.json)
and the referenced
[`dist-inventory.json`](../../crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-2026.10.01/dist-inventory.json).
The artifact identity fixture SHA-256 is
`55a51a3d3192285fe07ff115150a2008aba94f66fe2f705e398b353a902af231`; the
complete inventory SHA-256 is
`84473ea98c85ed1474b9d9ea14921030a48821ac9ae99d8343d6dd259994d4fc`. Research
393 supplied package identities and complete trees for its catalogue task;
this record independently re-inspects the headless behavior and uses those
files only as immutable artifact identity evidence.

## Selected headless surface

The exact runtime `dist-package/index.js` and headless writer chunk were
inspected on both platforms at all four points. In every artifact, selected
`--print` and `stream-json` strings occur only in those two files. The selected
option registration and mode dispatch for `--print`, `--output-format
stream-json`, `--model`, `--trust`, and `--mode plan|ask` remain present with
the existing meaning. No selected operation or option was added or removed.

The headless chunks change at each published hop. Their output remains
newline-delimited JSON and retains the event shapes consumed by the current
adapter:

- One initial system/init event carries `session_id`, selected `model`, and
  `permissionMode: default`. Subsequent events must match that session.
- Assistant text blocks continue to produce output deltas. Thinking delta and
  completed events retain their existing reasoning projection.
- Tool-call start and completion retain `call_id` and the selected tool-case
  mapping. Raw tool payload remains private.
- A successful result retains `is_error: false`, durations, nonempty request
  id, result text, and optional usage. Input, output, cache-read, and
  cache-write token values retain their existing mapping. Malformed selected
  fields still fail closed.
- Process nonzero exit, incomplete terminal streams, cancellation, deadline,
  force-stop, wait, join, and cleanup continue through the existing typed
  failure and lifecycle paths.

The selected runtime index and writer chunk changed byte-for-byte on every
hop. Each exact old/new path, file digest, package delta category, and full
added/removed/changed/identical path-set digest is recorded in the fixture and
checked by `cursor_agent_headless_2026_10_01_identity` and the existing
complete-tree identity test. The other package changes, including provider
internals and SEA/worker binaries, are enumerated in the same full ledgers and
remain unmapped. The `cursor-agent` launcher and `package.json` stayed
byte-identical. No archive or packaged runtime was executed.

## Claim shape and remaining limits

The change adds only the three exact published headless points above. It
preserves all earlier exact points and exclusions, including the independently
unqualified older releases and gaps. `2026.10.02` and any later date remain
visible as `UnverifiedNewer` until separately qualified. Build revisions are
validated exactly; matching a date alone does not qualify a different build.

No authenticated catalogue, provider session, prompt, credential, host
installation, or update was used. The claim does not transfer to Cursor ACP,
the catalogue, npm `cursor-agent`, other platforms, private provider behavior,
or package internals. No release or tag authority follows from this research.
