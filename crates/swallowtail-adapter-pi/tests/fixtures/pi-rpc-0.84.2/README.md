# Pi RPC 0.84.2 currentness corpus

This secret-free identity corpus freezes host Pi `0.83.0` and official npm
`@earendil-works/pi-coding-agent` `0.84.2` before Swallowtail widens the
`pi.package` claim.

Exact npm tarball and selected git-blob identities live in `identity.json`.
`rpc-types.ts` and strict-LF `jsonl.ts` stay byte-identical from `0.83.0`
through `0.84.2`. `session-cwd.ts` is unchanged. `0.84.0` drops cumulative
`message_update` snapshots; Swallowtail already maps only
`assistantMessageEvent` deltas. `0.84.2` adds unused streaming `usage`.

Unpublished `0.83.1` stays a gap. Oh My Pi stays a separate axis. No
provider prompt. No live RPC session. The host install was not replaced.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
