# 285 OpenCode HTTP 1.18.28 Identity

Status: promoted
Owner: Tom
Date: 2026-09-04
Card: g05 batch 077

## Question

Is official npm/GitHub OpenCode `1.18.28` a compatible extension of the
qualified `opencode.server` ceiling `1.18.20`, a private milestone, a new
facade, or a stop?

## Remaining Rank

This run covers only OpenCode HTTP. At observation time it remained the first
unblocked material candidate from Research 284.

| Surface | Host | Official | Swallowtail boundary | Classification |
| --- | --- | --- | --- | --- |
| `opencode.http` / `opencode.server` | `1.18.18` | npm and GitHub `1.18.28` | qualified through `1.18.20`; AllowUnverified | official-newer |

Gemini remains deferred. Kimi local server and Antigravity remain stopped.
OpenCode ACP and web search are separate surfaces and were not reopened.

## Method

Re-probed npm `opencode-ai@latest` and GitHub latest release, then downloaded
every npm tarball and GitHub tag source archive from `1.18.20` through
`1.18.28` into `/tmp`. Each archive was SHA-256 hashed before extraction.
Downloaded executables were never run.

For every hop:

- hashed all four npm package files;
- built a deterministic SHA-256 inventory of `packages/opencode/src`;
- compared exact added, removed, and changed implementation-source paths;
- hashed `packages/sdk/openapi.json` and inspected its full delta;
- inspected every changed `packages/opencode/src` file that could feed mapped
  wire shape, lifecycle, failure, permissions, usage, capability, session,
  config, or provider execution behavior; and
- bounded UI, console, docs, provider-specific, login, upgrade, and tool extras
  that cannot change the selected adapter protocol.

The npm package contains four files at each hop. `LICENSE`, the platform
bootstrap `bin/opencode.exe`, and `postinstall.mjs` are byte-identical across
all nine points; only `package.json` changes. The correlated GitHub tag source
contains 406 `packages/opencode/src` files through `1.18.23` and 407 thereafter,
when `config/v2-compat.ts` is added. npm metadata provides no `gitHead`, so the
npm artifact remains release authority; GitHub tags are independently frozen
correlation and implementation evidence.

Host `opencode --version` returned `1.18.18`. The installed executable is
143182562 bytes with SHA-256
`4f5979c2dadb06fbff1335335afaaea274e58f92e79aa43cf2ed98618d555422`
and an ad-hoc linker signature. It was observed only and not replaced.

No prompt, login, live server, catalogue, session, installation, provider
contact, or host change occurred.

## Identity

npm and GitHub latest agree on `1.18.28`. npm published it at
2026-09-04T15:40:40.661Z; GitHub published `v1.18.28` at
2026-09-04T15:38:23Z.

| Version | npm tarball SHA-256 | GitHub tag commit | Source archive SHA-256 |
| --- | --- | --- | --- |
| `1.18.20` | `d7af626824cab417d9c5c12e5c0187e506f1c903ea93bd8e4b1615be16305d2a` | `7248bc1964b13fa67e601733f89ee9dc6dfa0563` | `10129b7a233d8ea227fe8a65c158d3df4adc3d1296e3af5a136d94080b25a630` |
| `1.18.21` | `62eae64938cfc9ae1ee74222c6967ab081a2b5037bce947b0a3ba0b35694cd38` | `826d9ad46a22bef0294998e08daa3c4904fea28f` | `387ad0d4ef4364c00100fafaf90f80cff6858d7055bddd920aeacbab9b34a49d` |
| `1.18.22` | `920ce17f8d9f24865d161e26d7e3e5121aa386b5a221374109cc118d52cce4e7` | `47b6b6f5f4f9b42d2bce7af1c4e5bf6efaf22ba7` | `b777d4f92268168b9386b79eca0faa72a92367773fb6d81197cccf886901a3b9` |
| `1.18.23` | `4ba5929a9bf726bde96c9566b6902a58dcb2b338503a7b380c2fbf503cb1f0fe` | `ef2880f379129aa048be9e9353e30aa168d42c17` | `7b621e56e9e9162464f2524d88810f8e0e0036cf29cdadb35c574384eba3e6cf` |
| `1.18.24` | `b2adda2c2ba0bb7cf5e6a12cc090c56c969613acc9a464e75691cc5b316fa166` | `57e80556975fca613a116447ddecc8dcbc1f33b2` | `5acb57c53b39221ea8b8c8f55acd82c0a402a556a5da164c94b4d879756bb0ac` |
| `1.18.25` | `9be29b0858b3c9bb1214569f1d8e48a783956c8f5093cc6dcd86717e2cd8c5a3` | `cb7d8b2f5e44876ef98b661dc10590c915af3a9f` | `44e9530d7be172005c7d60aef317440eecb85d557d94cce7fa35c5a7b9d9da0b` |
| `1.18.26` | `d3eabbc23b5ef7e9383697c689b3b919f504d2cba36dcabe1ccc8de67380acb5` | `774cc7c1914e4329eefde5a669f938b0cf566661` | `a2ff47601072064f04263a97cce014c5b8d0692f7beaa7c7427ac02362d6c3d0` |
| `1.18.27` | `d1746d5dab3997f971fa643c0e1e5e553e9e18be9530f27a2a562bc19f610300` | `4b7e19e315cca414121ba1d61523fef74bb3ae8b` | `3d3851762d41da2dafe3be39d3b17a222426747e9b49e5e87d2a88b46b0866f1` |
| `1.18.28` | `ae46e3653cb85edb4eab36127f289ba71833d70c0efb56992f99eca2940117c4` | `22006d97652839999596a34a48ff6be7dbb40c6e` | `8eea501a6a00cbebe524af7c3248c0bfc56290f444671903e32aa6b799ee6616` |

All npm integrity values, SHA-1 sums, publication times, release times, and
per-hop archive identities are frozen in `identity.json`. Published stables
after the previous ceiling are exactly `1.18.21..=1.18.28`, with no gap. First
unpublished later stable at observation: `1.18.29`.

## Selected Protocol

Selected route declaration and handler files for health, provider catalogue,
session create/list/status/get/messages/prompt/abort/delete, and global events
are byte-identical from `1.18.20` through `1.18.28`. Session identity,
import/history/reconciliation, callback, usage, and detachment closures retain
their existing public wire shapes.

OpenAPI changes are limited to:

- `1.18.22`: unselected `global.upgrade` now requires an explicit semver
  `target`; and
- `1.18.27`: provider config describes finite 300000 ms header and SSE-chunk
  timeout defaults and permits `false` for the chunk timeout.

The provider timeout default changes upstream model-request failure timing,
but not the selected OpenCode HTTP/SSE schema or Swallowtail process authority.
It is finite, configurable upstream, and terminates through already-mapped
provider error, abort, disconnect, and detachment handling. No new public
operation or adapter control is required.

Changed run-path implementation is also bounded:

- `1.18.21` continues on an upstream `unknown` finish reason rather than
  treating it as terminal;
- `1.18.23` sends the parent-session header consistently across providers;
- `1.18.26` logs dropped Anthropic thinking blocks, preserves tool start time
  across repeated running updates, and omits empty apply-patch move paths; and
- provider/model transforms across the range affect upstream-specific request
  options, not selected OpenCode server wire or lifecycle authority.

Login URL validation, Azure and Cloudflare routing, config-v2 lowering,
GitHub/Copilot integration, model transforms, pricing copy, global upgrade,
and apply-patch extras remain unmapped. The exact per-hop source path sets,
OpenAPI hashes, and npm invariant hashes are frozen in `dist-inventory.json`
and enforced by the delta-ledger test.

## Decision

**Compatible-extension.**

- keep axis `opencode.server`, baseline `1.14.48`, AllowUnverified, existing
  historical segments and gaps, claim IDs, and `surface-19` behavior revision;
- qualify each published point `1.18.21..=1.18.28` in serial Card 078;
- raise all existing OpenCode HTTP closures on this axis through `1.18.28`;
- keep decoder specimen `opencode-1.14.48`; and
- use synthetic `1.18.29` as the later `UnverifiedNewer` point after claim.

This identity card changes no production claim. Card 078 is admitted but
remains serial and outside this PR.

## Sources

- npm registry: `https://registry.npmjs.org/opencode-ai`
- npm tarballs: registry `dist.tarball` URLs for `1.18.20..=1.18.28`
- GitHub releases: `https://github.com/anomalyco/opencode/releases`
- GitHub tag archives and refs: `v1.18.20..=v1.18.28`
- Frozen corpus: `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.28/`

