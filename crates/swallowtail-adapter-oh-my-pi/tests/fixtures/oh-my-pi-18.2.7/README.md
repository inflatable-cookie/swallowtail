# Oh My Pi 18.2.7 currentness corpus

Secret-free identity, selected-source, and shipped-tree freeze for official
npm `@oh-my-pi/pi-coding-agent` from the previous qualified ceiling `18.1.22`
through official latest `18.2.7`, taken before any claim edit.

Official npm `latest` and the GitHub latest release/tag both name `18.2.7`
(published 2026-09-21). `omp` is not on `PATH`; missing install is not a gap.
The host was not installed, updated, replaced, or executed.

Every published npm stable from `18.1.22` through `18.2.7` was retrieved from
the registry into `/tmp` and verified against the registry `integrity` and
`shasum` before extraction. Downloaded artifacts were never executed. The
registry records no source commit (`gitHead` is null). Unpublished GitHub
tags `v18.0.2` and `v18.1.7` stay incompatible; unpublished `18.2.8` is the
synthetic later-stable point. `18.1.23` was only the previous synthetic
later-stable and was never published.

`identity.json` freezes the 9-row published ledger (previous ceiling plus
eight hops) plus host, authority, and decision. `dist-inventory.json` freezes
the complete shipped `package/` tree delta for every hop and the 1683 files
byte-identical across all nine compared versions. `protocol.json` freezes the
selected `--mode rpc` flags, RPC v2 commands, wire invariants, mapped RPC
source digest groups, and one classification per changed mapped hop.

Research 327's `18.1.22` identity reproduces exactly (integrity, shasum,
tarball `6eac4319…80ac`, `dist/cli.js` `b8bfd4f1…2499` at size 22437945,
file count 3156, GitHub `23a5b9ae`). `rpc-frame.ts`, `rpc-input.ts`,
`rpc-messages.ts`, and `host-uris.ts` are byte-identical across every
compared hop. `docs/rpc.md` is blob `310b4470` through `v18.2.0`, then
documents the stdout spool (`b3b5d8aa`) and login secret rejection
(`b91b20a8` through `v18.2.7`).

The decision is a compatible extension of the existing adapter-private `18.x`
segment from `18.0.0..=18.1.22` to `18.0.0..=18.2.7` on the unchanged
`oh-my-pi.rpc-v2-v18.0.0` revision. Not a major-line reset, new public
operation, new driver/facade, or family flatten. `pi.package` (`0.86.1`)
stays a separate axis.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
