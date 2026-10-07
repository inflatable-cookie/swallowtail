# Gemini CLI 0.63.0 ACP currentness stop

Secret-free identity and source corpus for official npm
`@google/gemini-cli` `0.61.0` through `0.63.0` and GitHub tags `v0.61.0`
through `v0.63.0`. Research 371 records the decision.

On 2026-10-07 npm `latest` and the latest GitHub stable release agree on
`0.63.0`. The published stable hops after the qualified `0.61.0` ceiling are
`0.62.0` and `0.63.0`. Research 358's exact `0.61.0` identities reproduce.
No downloaded artifact was executed. The installed host was read for
`--version` and its bundle digest only; it remains exact `0.61.0`.

`identity.json` freezes the npm integrity, shasum, tarball, package and bin
entry identities, GitHub source archive and tag identities, and release
runtime asset digests. `npm-package-inventory.json` hashes every regular file
in each npm package and freezes each hop's added, removed, changed and
identical file sets. `surface-ledger.json` hashes every regular file in each
tagged source archive and freezes each hop's complete changed-path set plus
selected-source classifications. The only non-regular source path in each
archive is the `docs/CONTRIBUTING.md` symlink.

The `0.62.0` ACP session adds existing ACP v1 pending and failed tool-call
updates. The `0.63.0` selected permission and file paths change: `.gemini`
configuration writes and some redirected shell commands now require user
confirmation, which this route rejects and cancels; file reads also revalidate
the resolved real path. The current claim remains through `0.61.0`, with
`0.62.0` and `0.63.0` visible as `UnverifiedNewer` pending the adaptation
described in Research 371. No headless claim or live HTTP MCP evidence moves.

No fixture contains credentials, a private host path, account identity,
provider payload, prompt, or session id.
