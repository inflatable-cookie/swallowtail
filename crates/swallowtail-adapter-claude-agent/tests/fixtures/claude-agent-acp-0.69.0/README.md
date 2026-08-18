# Claude Agent ACP 0.69.0 currentness corpus

This secret-free identity corpus freezes host Claude Agent ACP `0.63.0` and
official npm/GitHub `0.69.0` before Swallowtail widens the
`claude-agent.acp-adapter` claim.

Exact npm tarball and selected `dist` identities live in `identity.json`.
`dist/elicitation.js` stays byte-identical from `0.64.0` through `0.69.0`.
Selected initialize session capabilities stay the same. Initialize `_meta`
gains unmapped `goal` from `0.66.0`, Air session-failure from `0.67.0`, and
file-change report from `0.69.0`. Swallowtail maps none of those extras.

`0.58.0` remains unpublished. Claude Code stays a separate axis. No provider
prompt. No live ACP initialize. The host install was not replaced.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
