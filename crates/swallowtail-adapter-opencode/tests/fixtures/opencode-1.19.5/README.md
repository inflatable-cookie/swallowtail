# OpenCode HTTP 1.19.5 identity

Secret-free identity evidence for official npm `opencode-ai` and GitHub stable
`v1.19.5`, compared with the qualified `1.19.4` ceiling, observed 2026-10-28.
The npm launcher and Linux x64 runtime tarballs, and the standard GitHub tag
archives, were downloaded and SHA-256 hashed before comparison. No downloaded
executable was run. `opencode` was not on `PATH`. The host was not changed.

npm `latest` and GitHub latest stable agree at `1.19.5`. The only published
stable after `1.19.4` is `1.19.5`. No stable `1.18.36` exists. `1.19.6` was
not published. Snapshot, dev, next, beta, and other non-latest dist-tags were
ignored. npm metadata has no `gitHead`.

The ACP registry `opencode` and `opencode-v2` entries both advertise `2.2.0`
and name OpenCode 2 release assets. npm `opencode-ai` has no `2.x` version.
That line is not this family's official stable and is not qualified here.

`dist-inventory.json` freezes npm file hashes, both repository tree digests,
and every added, removed, and changed path. Selected HTTP route declaration
and handler files stay byte-identical. `packages/sdk/openapi.json` stays
byte-identical: 162 paths, `info.version` `1.0.0`. The source edits are a TUI
worker error reply, a PTY connect route match, unused-import and dead-file
removal, and the `gitlab-ai-provider` pin `6.19.0` to `6.23.1`.

Decision: stop. The qualified ceiling remains `1.19.4`; `1.19.5` is
`UnverifiedNewer`. The byte comparisons describe tag trees only. The native
payload has not been correlated to the tag: npm `gitHead` is absent, the
release target is a branch, and no attestation was observed. Only Linux x64
native contents were inventoried; the wrapper selects twelve native packages.
The shared GitLab dependency effects remain a qualification gate. See
Research 464 and the owning adaptation lead `5b445586-3c98-47c5-aa88-7950aace016b`.
No downloaded artifact was executed and no provider was contacted.
