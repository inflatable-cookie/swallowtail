# Oh My Pi 18.1.22 currentness corpus

Secret-free identity, selected-source, and shipped-tree freeze for official
npm `@oh-my-pi/pi-coding-agent` from the previous qualified ceiling `17.4.0`
through official latest `18.1.22`, taken before any claim edit.

Official npm `latest` and the GitHub latest release/tag both name `18.1.22`
(published 2026-09-14). Installed `omp` reports `omp/18.1.16` at
`sha256:99eed6d4…9795`; it is an observation, was not installed, updated,
replaced, or executed beyond `--version`.

Every published npm stable from `17.4.0` through `18.1.22` was retrieved from
the registry into `/tmp`, and each tarball was verified against the registry
`integrity` and `shasum` before extraction. Downloaded artifacts were never
executed. The registry records no source commit (`gitHead` is null), so the
shipped package tree plus the GitHub tag commit and `docs/rpc.md` are the
evidence. Unpublished GitHub tags `v17.4.3`, `v17.4.4`, `v18.0.2`, and
`v18.1.7` have no npm package and stay incompatible; unpublished `18.1.23`
is the synthetic later-stable point.

`identity.json` freezes the 36-row published ledger (published time, npm
integrity, shasum, tarball digest, `dist/cli.js` digest and size, shipped
file count, package version, GitHub tag commit) plus the host, authority, and
decision. `dist-inventory.json` freezes the complete shipped `package/` tree
delta for every published hop and the 1773 files byte-identical across all 36
compared versions. `protocol.json` freezes the selected `--mode rpc` flags,
RPC v2 commands, wire invariants, the mapped RPC source digest groups, and one
classification per changed mapped hop.

Research 217's retained identity reproduces exactly (npm `18.0.5` integrity
and shasum, `dist/cli.js` `3edd2768…9651` at size 19316793, GitHub
`eab72e88`), with one correction: the `optionDetails` type and the
`agentInvoked` / `runCommandInBackground` mode changes land in published
`17.4.2`, not at the `18.0.0` tag. `rpc-types.ts` is byte-identical from
`17.4.2` through `18.1.22`, `rpc-messages.ts`, `rpc-input.ts`,
`host-uris.ts`, and `message-framing.ts` are byte-identical across all 36
versions, and `docs/rpc.md` is blob `310b4470` from `17.4.2` through
`18.1.22`.

The decision is a compatible extension of the 17.x segment through `17.4.2`
on the unchanged `oh-my-pi.rpc-v2-v17.2.9` revision, plus an admitted distinct
adapter-private `18.x` segment `18.0.0..=18.1.22` on the new
`oh-my-pi.rpc-v2-v18.0.0` revision. `18.0.2` and `18.1.7` are explicit
exclusions. `pi.package` (`0.85.1`) stays a separate axis and is never
flattened onto `oh-my-pi.package`.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
