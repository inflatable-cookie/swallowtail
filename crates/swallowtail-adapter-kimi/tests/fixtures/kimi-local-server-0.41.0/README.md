# Kimi Code local-server 0.41.0 identity corpus

Secret-free identity freeze for official `@moonshot-ai/kimi-code`
`0.41.0` on the `kimi-code.local-server` family. Host `0.34.0` was
observed by hashing the installed binary and was not run, installed, or
replaced. Downloaded archives were hashed and extracted, never executed.

The first run targeted `0.40.1`. Official latest moved to `0.41.0` during
that run. The operator retargeted this card to `0.41.0` on 2026-09-04;
`0.40.1` is published adjacency.

Exact npm, GitHub, and platform-archive identities live in `identity.json`.
Selected REST/WebSocket v2 protocol files are comment-only from `0.38.0` to
`0.39.0` and byte-identical through `0.41.0`. Approval, question, and
`terminals.ts` blobs are unchanged from `0.38.0`. Application `ping`/`pong`
remains.

The `0.40.0` Bash tool `cwd` change is the authority outcome: 
`RuntimeWorkspaceView.resolve` stopped asserting workspace membership, so
`bashTool` `view.resolve(args.cwd ?? view.workDir)` accepts a path outside
the workspace roots. Swallowtail launches `kimi web` under `AmbientHost`
with no process cwd. Nothing contains that escape for a local-server
client. Outcome is stop. Qualified ceiling stays `0.38.0`. Decoder
specimens stay on `kimi-local-server-0.28.1-0.29.0`.

No fixture contains a credential, bearer token, host path, account
identity, provider payload, real session id, or model observation.
