# 2026-09-15 Oh My Pi 18.1.22 Identity

g05.079 re-probed npm, GitHub, and the installed host on 2026-09-15 before any
claim moved. npm `latest` is `@oh-my-pi/pi-coding-agent@18.1.22`, published
`2026-09-14T19:41:55.446Z`. GitHub latest release is `v18.1.22`, published
`2026-09-14T19:29:59Z`, tag commit
`23a5b9ae38864d3f785dc6cbc96eb6d674a1d32d`. The installed host
`/Users/tom/.local/bin/omp` is `omp/18.1.16` at
`sha256:99eed6d45d984d2f13d76832f78782b9aa07862921ab4e98e2d8e31ca8129795`,
observed only through `--version` and a digest.

Research 327 freezes the complete published ledger from `17.4.0` through
`18.1.22` in
`crates/swallowtail-adapter-oh-my-pi/tests/fixtures/oh-my-pi-18.1.22/`.
All 36 registry `integrity` and `shasum` values reproduce, and the extracted
tarball, `dist/cli.js`, package version, shipped file count, and GitHub tag
commit for every point are frozen before any claim edit. The registry records
no source commit (`gitHead` is null), so the shipped tree plus the tag commit
and `docs/rpc.md` are the evidence. GitHub-only tags `v17.4.3`, `v17.4.4`,
`v18.0.2`, and `v18.1.7` have no npm package and stay incompatible; `18.1.23`
is the synthetic later stable.

Research 217 reproduces exactly (npm `18.0.5` integrity and shasum, extracted
`dist/cli.js` `3edd2768…9651` at size `19316793`, GitHub `eab72e88`) with one
correction: the `optionDetails` type and the `agentInvoked` /
`runCommandInBackground` mode changes land in published `17.4.2`, not at the
`18.0.0` tag. `rpc-messages.ts`, `rpc-input.ts`, `host-uris.ts`, and
`message-framing.ts` are byte-identical across all 36 compared versions;
`rpc-types.ts` is byte-identical from `17.4.2`; `rpc-mode.ts` is byte-identical
`17.4.2..=18.0.6` and `18.1.18..=18.1.22`; and `docs/rpc.md` is blob
`310b4470` from `v17.4.2` through `v18.1.22`.

Every changed mapped hop is classified. The `17.x` hops are additive or
advisory (an unmapped background-command callback, an accurate `agentInvoked`
field, optional select `optionDetails`). The `18.x` mapped hops are an
unmapped `available_commands_update` extension-root plumbing change, an
unmapped embedded-client launcher and spawn transport with parameter renames,
an append-only `/restart` argv helper, comment-only `rpc-frame.ts` /
`rpc-mode.ts` / `host-tools.ts` edits, a `--no-skills`-disabled skill-prompt
timing change, an additive client forwarded-event name, a barrel re-export,
and a queue-mode command persistence change whose session-local setting is
unchanged. No mapped wire, authority, failure, retention, cancellation,
ownership, or cleanup change was found.

No downloaded artifact was executed. No provider prompt, login, credential,
host install or update, live session, release, tag, publication, or consumer
mutation occurred. Research 327, g05.079.
