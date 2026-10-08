# Research 377: OpenCode ACP 1.18.35 Identity and Currentness

Swallowtail#104; evidence checked 2026-10-08 for `opencode.acp` only.

## Official current point

The official [`opencode-ai` npm package](https://www.npmjs.com/package/opencode-ai)
reported `latest=1.18.35`. The official [OpenCode GitHub releases](https://github.com/anomalyco/opencode/releases)
reported latest stable tag `v1.18.35`, published 2026-10-06. The selected
channels agree. Stable releases published after the prior `1.18.32` ceiling
are `1.18.33`, `1.18.34`, and `1.18.35`, in order, with no stable gap or
withdrawn hop. `1.18.36` was not published at observation and remains the
next visible `UnverifiedNewer` point.

For the current point, npm `latest` and GitHub latest stable were re-probed
before artifact selection and immediately before push. The exact npm package
integrities matched their downloaded tarballs. npm metadata did not include
`gitHead`, so this evidence does not claim that a launcher tarball is
source-commit-attested by npm. GitHub tag commits and archive hashes are
recorded independently in the fixture.

| Stable | GitHub tag commit | Source archive SHA-256 | npm launcher tarball SHA-256 | Darwin arm64 tarball SHA-256 |
| --- | --- | --- | --- | --- |
| `1.18.32` | `545f51d26cc39a907d2867492d498d9607ea5fa4` | `65e95c9a6666ca65bbd17de1e7cecddac1504e66eeebbcfaf5ac68f97e6f392b` | `454fbb032ade95a21323891138d4b425573f67631e7e888e516da893dc4be8ba` | `a1707bb6cc9deaccca501c4097f1580b8af70dfc227f194ae0f576f32abbdebe` |
| `1.18.33` | `51ef4be1d3c122f18fefb510dca8d778571f4f18` | `34a4b810f4e839f2c4ac62206bbc64d736006f3c96712b903cda83ca60271301` | `6fed32445d04df1d9d56aef0f15ae200618663ef532f7ec4241432024f84097c` | `cd2c704ad653137b62992bca2b32383f8daa31b3c9462e566278e1efba07b31d` |
| `1.18.34` | `aec0b9a6d8898f68f923aaf08b7306d931fd9d76` | `c2c60efde22639b64c7bfa740da39b9c8079391a7540e2f67bf91b36e5797f17` | `279923bb5754e810b173a8a7401cf17699fccad3c1159500c10c477887f0b53b` | `9a336191ee84c8f54364b5d1990ec87abb2acbb00dea189f11dda5e968d7bf05` |
| `1.18.35` | `53d1eabb61e21162157817bf677da0a4ad3332e3` | `3092a7b9f55d80c42a9c1bb2e2cc0a9961316673faf92b741577257a1576dd59` | `4d3408d0950d70cf870efe3f86e8bd0271d8149bea0767008d42de14f0986a04` | `b626543f4427cbd7a59756c24045f6f32dbc4cf7347ab5a8613f5c9fd6b0eeb3` |

The exact npm publication times, SHA-512 integrities, every file path, byte
length, and SHA-256 are frozen in
[`opencode-acp-1.18.35`](../../crates/swallowtail-adapter-opencode/tests/fixtures/opencode-acp-1.18.35/README.md).
The `opencode-ai` launcher tarball contains four files at each hop; only
`package.json` changes, while `LICENSE`, `bin/opencode.exe`, and
`postinstall.mjs` remain byte-identical. The `opencode-darwin-arm64` tarball
contains `bin/opencode` and `package.json`; both change at each published hop.
The installed `1.18.32` runtime SHA-256 matches the exact official Darwin
arm64 binary artifact. No artifact was executed.

The full tagged source archive inventory records all directories, regular
files, and symlinks for all four versions. Regular files carry exact size and
SHA-256; symlinks carry their targets. The canonical TSV digest is
`6ccadbecb9d91183cb79e236b046cf89ffe031602e96a1451c3c88e64102faff`.
`dist-inventory.json` also records each complete source-hop added, removed,
and content-changed path with its before and after identity.

## Selected ACP v1 review

The selected source slice covers OpenCode's ACP v1 entrypoint, JSON-RPC wire,
session lifecycle, error and event mapping, permission handling, usage,
configuration, tool updates, MCP configuration, and the selected HTTP MCP
backing configuration. Eighteen behavior files are byte-identical over all
three new stable hops. The only selected source file that changes is
`packages/opencode/package.json`:

- `1.18.32 → 1.18.33` updates `gitlab-ai-provider` from `6.15.0` to `6.18.0`
  and removes the unrelated `open` dependency.
- `1.18.33 → 1.18.34` changes the package version only in the selected
  manifest.
- `1.18.34 → 1.18.35` updates `@ai-sdk/xai` from `3.0.102` to `3.0.139` and
  `gitlab-ai-provider` from `6.18.0` to `6.19.0`.

The package pins `@agentclientprotocol/sdk` at `0.21.0` at every selected
point. The ACP protocol version stays provider-fixed at `1`; the selected
entrypoint, methods, notifications, error mapping, session close behavior,
permission callbacks, usage fields, configuration keys, and tool update
decoding do not change. The route still exposes one bounded interactive
session, a single consumer-supplied MCP entry under the existing name, and no
registered-tool operation. No operation or capability is added.

Other changed implementation sources are bounded separately in
`protocol.json` and the complete hop ledger. `1.18.33` tightens schemes for
internal browser and OAuth redirects, changes provider request-timeout and
Gemini/Gemma model transforms, and updates CLI/provider integration code.
`1.18.34` adds `x-opencode-session-id` and optional
`x-opencode-parent-session-id` to provider HTTP requests; these duplicate the
existing session identifiers. `1.18.35` filters unsupported image formats
from xAI tool-result attachments. These are host or provider internals; no
ACP v1 wire, lifecycle, permission, usage, configuration, or tool-update
contract changes. They do not establish model-specific support or registered
tools. No live provider call or session was used.

The previously admitted live HTTP MCP honouring evidence remains bound to
exact OpenCode `1.18.18` (Research 349, g06.028). It does not transfer to
`1.18.33`–`1.18.35`; this research makes no HTTP route claim.

## Decision and limits

This is a compatible extension of
`opencode.acp-v1.client-mcp-servers-v2`. Extend the existing maintained
segment `1.18.31..=1.18.32` through `1.18.35`. Keep deprecated baseline
`1.18.18..=1.18.30`, claim ID `opencode.acp.executable-window-1`, both
behavior revisions, no exclusions, and `AllowUnverified`. The next stable
point `1.18.36` remains `UnverifiedNewer`; prereleases remain outside the
stable qualification. The exact `1.18.18` HTTP MCP live gate and the separate
`opencode.server` family remain unchanged.

No package was installed, no provider prompt or session was sent, no
credentials were used, and no host state was changed.
