# 2026-07-23 Codex Multi-Release Compatibility Corpora

## Changed

- Froze exact exec evidence at `0.121.0`, `0.122.0`, `0.130.0`, `0.140.0`,
  `0.144.6`, `0.145.0`, and `0.146.0-alpha.4`.
- Froze exact app-server evidence at `0.110.0`, `0.120.0`, `0.131.0`,
  `0.140.0`, `0.144.6`, and `0.145.0`.
- Recorded tagged commits, npm integrity, archive shasums, selected CLI help
  and source digests, generated stable schemas, generated experimental
  schemas, and model-list schema digests.
- Added offline exec JSONL and app-server JSON-RPC transcripts for selected
  events, catalogue, sessions, callbacks, interruption, failures, disconnect,
  and close.
- Added explicit malformed, prerelease, unpublished, below-floor, and unknown
  newer rejection cases.
- Added a gate-enforcing mock app-server and frozen experimental-field cases.

## Findings

- Exec first satisfies the current bounded invocation at `0.122.0`.
  `0.121.0` has JSON and ephemeral execution but lacks both
  `--ignore-user-config` and `--ignore-rules`.
- App-server v2 has a viable first floor at `0.110.0`.
- `runtimeWorkspaceRoots` appears in the experimental schema at `0.131.0`.
- `allowProviderModelFallback` appears in the experimental schema at
  `0.144.6`. Sending it as `false` still requires `experimentalApi`.
- Stable and experimental generated schema bundles differ at every candidate.
  They cannot substitute for one another.
- Existing additive unknown events remain safe progress. Missing required
  fields on a known event remain malformed.

## Evidence Method

Evidence came from official Codex release tags, official platform npm
packages, selected tagged source, and each executable's app-server schema
generator. Several older macOS archives carried a now-revoked Developer ID
certificate. Temporary extracted copies were ad-hoc signed only to execute
offline version, help, and schema-generation commands. Nothing was installed,
no credentials or provider request were used, and no repaired binary entered
the repository.

- [Codex app-server schema documentation](https://developers.openai.com/codex/app-server/#message-schema)
- [Codex releases](https://github.com/openai/codex/releases)
- [Codex npm package](https://www.npmjs.com/package/@openai/codex)

## Validation

- 52 Codex adapter tests pass
- focused warnings-denied clippy passes
- corpus tests use no installed Codex, credential, network, or container
- `git diff --check` passes

## Next

Card 113 binds both production drivers to exact observed versions, publishes
only these qualified windows, dispatches at the `0.131.0` milestone, and
removes ungated default-false experimental emission.
