# Cline ACP 3.0.70 currentness evidence

Secret-free identity and source evidence for extending only `cline.acp` from
its 3.0.55 ceiling through official npm `latest` 3.0.70. The published npm
sequence includes 3.0.56–3.0.58 and 3.0.60–3.0.70; 3.0.59 is absent from the
selected channel and remains excluded.

- `release-identity.json` freezes the current npm channel observation, selected
  package axis, source commits, and runtime platform scope.
- `dist-inventory.json` contains complete per-file SHA-256 trees and tarball
  digests for every published wrapper hop and the Darwin ARM64 runtime package.
  Its changed, added, removed, and identical path sets are reproducible from
  those trees.
- `provenance.json` freezes the wrapper and six platform runtime SLSA
  attestations for each published hop. The registry SHA-512 values match each
  attestation subject; all seven artifacts at each hop name the wrapper's
  source commit.
- `source-tree-inventory.json` contains every ACP source file and the reviewed
  adjacent runtime support files, package manifests, and `bun.lock` at each
  exact provenance-bound source commit. `apps/cli/package.json` declares
  `@agentclientprotocol/sdk` `^0.16.1`; `bun.lock` resolves `0.16.1` at every
  published hop. `protocol.json` classifies every changed path for every
  published source hop.

The qualification preserves the text-only ACP output boundary, permission
handling, selected methods, one-session lifecycle, failure mapping, and the
unmapped provider model catalogue and `session/load` operations. It adds no
operation or authority. No package was installed or executed, and no provider
prompt, account, credential, catalogue, or ACP session was used.
