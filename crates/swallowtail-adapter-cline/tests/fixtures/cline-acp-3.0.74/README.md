# Cline ACP 3.0.74 currentness evidence

Secret-free identity for extending only `cline.acp` from its `3.0.70` ceiling
through official npm `latest` `3.0.74`. The published hops are `3.0.71`,
`3.0.72`, `3.0.73`, and `3.0.74`. Unpublished `3.0.59` stays excluded.
`3.0.75` is the first unpublished later stable.

- `release-identity.json` freezes the npm channel, source commits, CLI tags,
  and six-platform attestations for `3.0.70` through `3.0.74`.
- `dist-inventory.json` contains complete per-file SHA-256 trees for the
  wrapper and the Darwin ARM64 runtime. The `3.0.70` trees match the Research
  397 corpus. At `3.0.74` every platform package's registry file count drops
  from 142 to 2; the selected Darwin tree keeps `bin/cline` and `package.json`
  and removes the hub webview plus `extensions/plugin-sandbox-bootstrap.js`.
- `provenance.json` freezes SLSA publish attestations. Registry SHA-512 values
  match the attestation subjects, and every platform artifact names the
  wrapper source commit. Each `cli-v3.0.x` tag commit matches that commit.
- `source-tree-inventory.json` contains every `apps/cli/src/acp` file and the
  reviewed adjacent slice, including the `apps/cli/src/main.ts` ACP entry
  gate, at each provenance-bound commit. Its per-version blob identities
  freeze the explicit-only reasoning change at `3.0.71`, the hub-only
  telemetry guard at `3.0.72`, and byte identity from `3.0.72` through
  `3.0.74`. `release-identity.json` binds the reasoning introduction's
  before/after blobs. `protocol.json`
  classifies every changed path. `@agentclientprotocol/sdk` stays `^0.16.1`,
  resolved to `0.16.1`.

The selected command stays `cline --acp`. Mode remains `id=mode`,
`category=mode`, values `plan` and `act`. The model option remains `id=model`,
`category=model`. Reasoning stays off unless `--thinking` is explicit.
`session/load`, provider catalogues, media output, and client MCP stay
unmapped. No package was installed or executed, and no provider prompt,
account, credential, catalogue, or ACP session was used.
