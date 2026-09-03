# 2026-09-03 Claude Agent SDK 0.3.259 Identity

Card 055 stopped when official npm stable moved off the qualified point. The
operator selected the refresh exit, so this record freezes official
`@anthropic-ai/claude-agent-sdk` `0.3.259` and its coupled native Claude Code
`2.1.259` identity and rebinds the two exact axes on the preserved PR 188.

Research 280 holds the evidence. Both tarballs were downloaded to `/tmp`,
hashed, and extracted; nothing was executed, no platform package was fetched,
no host changed, and no provider session, login, or token read occurred. The
native binaries are identified from the shipped `manifest.json` digests rather
than by pulling 200 MB artifacts.

The `0.3.258` tarball re-hashed to the exact digest Research 278 froze, so this
is a clean hop from the previously frozen point with no published stable in
between. `0.3.259` is `latest` and `next`, published `2026-09-02T21:22:40.857Z`,
tarball SHA-256 `0c5740e4…3f7e`.

The deterministic package-tree inventory is the decisive evidence: 15 files in
both versions, 7 identical, 8 changed, none added or removed. `package.json`
moved only its version, `claudeCodeVersion`, and the eight platform pins, and
the `exports` map is byte-identical, so the `.` entry point and the separate
`/bridge` and `/browser` subpaths are unchanged.

Every changed shipped file is classified and none is mapped. The new
`Options.permissionPrompts` selector is forwarded only when set, so an unset
option keeps `canUseTool` governing admission — the route never sets it, and a
mutation-sensitive test asserts the shipped asset never names it, because
`'none'` would silently remove the host's approval surface. The four
`user_message_uuids` additions are correlation siblings the route does not read.
The task `summary` change is documentation over an existing field. The
`managedMcpServers` settings key and its neighbouring prose are managed-settings
tier, which `settingSources: []` never loads. The `sdk-tools.d.ts` additions
belong to skill publishing, which this route prohibits.

The lifecycle premise survived the hop unchanged: `canUseTool` still pushes
`--permission-prompt-tool stdio`, `spawnClaudeCodeProcess` still exists, and the
bounded `waitForExit()` race with its `unref()`'d `SIGKILL` escalation is still
there. The SDK still supplies no joined stop, which is exactly why this route
owns its own close.

Credential non-custody re-verified identically: three prose hits across the
ten-pattern search of the `.` entry declarations, 17 exported functions, no
login or OAuth export, and byte-identical `bridge.d.ts` and `browser-sdk.d.ts`.

All eight native platform binaries rotated with the version and commit, but
`sdkCompat.harnessSchema` stays `1`, so the wrapper-to-native protocol surface
did not move and no behavior revision is warranted.
`sdkCompat.testedWrapperVersions` still tops out at `0.3.227`, so the shipped
declaration-versus-artifact discrepancy Research 278 recorded persists.

Rebound: `claude-agent.sdk.package` to exact `0.3.259` and
`claude-agent.sdk.native` to exact `2.1.259`. Unchanged: the Node, wire, and
sidecar source-tag axes, every claim id, the `claude-agent.sdk-v1` behavior
revision, and the QualifiedOnly posture. `0.3.258` becomes unqualified rather
than a second supported point.
