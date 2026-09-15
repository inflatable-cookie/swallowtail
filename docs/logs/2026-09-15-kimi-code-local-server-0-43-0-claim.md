# 2026-09-15 Kimi Code Local Server 0.43.0 Claim

g05.078 requalified the separate `kimi-code.local-server` claim through
official npm and GitHub `0.43.0` from Research 326 under the Research 308
containment-or-fail-closed rule.

`kimi.local-server.executable-window-2` extends from maintained
`0.35.0..=0.38.0` to maintained `0.35.0..=0.39.1` on the unchanged
`kimi.local-server.rest-ws-v2-heartbeat-ping` behavior revision with baseline
`0.28.1` and the claim id unchanged, and the newer-version posture changes
from `AllowUnverified` to `QualifiedOnly`. `0.39.0` and `0.39.1` preserve the
Bash workspace assertion, while `0.40.0` removes it and the uncontained
`resolve` persists byte-identical through `0.43.0`, so every point above
`0.39.1` fails closed, including the published `0.40.0..=0.43.0` gap. No new
exclusion is added, and no new behavior revision, public operation, or shared
type. Decoder specimens stay on `kimi-local-server-0.28.1-0.29.0`.

The local-server selection tests, the `0.41.0` identity suite, the `0.39.1`
and `0.43.0` installed suites, the executable-identity windows, the
local-server corpus, the binding-import authority proof, and the interactive
detachment proof now assert the `0.39.1` `QualifiedOnly` ceiling; the new
`kimi-local-server-0.43.0` identity suite ties the frozen ledger to the live
claim. The route and activity matrices, the feature matrix, and
`[Unreleased]` agree. `kimi-code.acp` and `kimi-code.headless` are untouched.

Official latest was rechecked at npm and GitHub `0.43.0` at the push
boundary. No provider prompt, login, credential, installation, host update,
downloaded-binary execution, local-server start, release, tag, publication,
or consumer mutation occurred. Research 326, g05.078.
