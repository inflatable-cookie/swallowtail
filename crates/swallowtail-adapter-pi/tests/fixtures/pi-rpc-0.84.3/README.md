# Pi RPC 0.84.3 currentness corpus

This secret-free identity corpus freezes official npm
`@earendil-works/pi-coding-agent` `0.84.3` before Swallowtail widens the
`pi.package` claim. Host `pi` was not on PATH. Missing install is not a
gap.

Exact npm tarball and selected git-blob identities live in `identity.json`.
`rpc-types.ts`, `rpc-mode.ts`, and strict-LF `jsonl.ts` stay byte-identical
from the previous ceiling. `session-cwd.ts` is unchanged. `0.84.3` adds
unused `id` and `toolName` on `toolcall_start`; Swallowtail still classifies
that event as Progress. `--`, `powershell`, bundled bin path, and
streaming `usage` stay unmapped.

Unpublished `0.83.1` stays a gap. Unpublished `0.84.4` stays the synthetic
later-stable point. Oh My Pi stays a separate axis. No provider prompt. No
live RPC session. The host install was not replaced.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
