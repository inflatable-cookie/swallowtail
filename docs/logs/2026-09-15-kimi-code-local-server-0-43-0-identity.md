# 2026-09-15 Kimi Code Local Server 0.43.0 Identity

g05.078 re-probed npm, GitHub, and the installed host on 2026-09-15 before
any claim moved. npm `latest` is `@moonshot-ai/kimi-code@0.43.0`, published
`2026-09-14T12:10:41.073Z`. GitHub latest is
`@moonshot-ai/kimi-code@0.43.0`, published `2026-09-14T12:03:30Z`. The
installed host remains `kimi 0.34.0` at
`sha256:9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859`,
observed only through `--version` and its digest.

Research 326 freezes the local-server selected-file ledger spanning
`0.38.0..=0.43.0` in
`crates/swallowtail-adapter-kimi/tests/fixtures/kimi-local-server-0.43.0/`.
Every `0.39.1` through `0.41.0` npm/GitHub identity value reproduces
Research 270, Research 282, and Research 325; the `0.42.0`/`0.43.0` tarball
SHA-256, file counts (547 and 543), `dist/main.mjs` digests, annotated tags,
commits, and trees reproduce Research 325 and were re-verified from the
official channels this run. All eight tagged-source trees resolve to the
frozen commits.

The safe prefix is `0.39.0..=0.39.1`: `RuntimeWorkspaceView.resolve` still
maps then calls `assertAllowed` at both points, and `bashTool` still computes
`effectiveCwd` through `view.resolve`. The `0.40.0` removal of that assertion
persists byte-identical through `0.43.0` with no restored containment, so the
Bash `cwd` widening is uncontained under the `AmbientHost` local-server
launch. The two newly classified hops add no containment: `0.41.0→0.42.0`
wire deltas (model-catalog import-source move, `watch_fs` removal,
delete-response reshape, terminal compat-schema removal, Remote Control
de-experimentalization) are all unmapped or inert on the selected route, and
`0.42.0→0.43.0` is one added field in the unmapped Remote Control QR output.

No downloaded artifact was executed, installed, or authenticated. No prompt,
catalogue call, login, credential, host update, live session, or
local-server start occurred. Research 326, g05.078.
