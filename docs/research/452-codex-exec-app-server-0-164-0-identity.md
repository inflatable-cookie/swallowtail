# Codex exec and app-server 0.164.0 identity

Status: promoted.
Date: 2026-10-26.
Scope: `codex.exec` and `codex.app-server` on `codex.cli`. Research 452.

## Question

Can the existing AllowUnverified ceilings move from `0.161.0` to official
stable `0.164.0` without a new behavior revision?

## Decision

Compatible extension. Raise both ceilings to `0.164.0`. Keep the claim ids,
baselines, older segments, exclusions, and the separate exec and app-server
behavior revisions. The lifecycle claim follows the app-server ceiling on
`codex.app-server.lifecycle.v1.strict-descendant-hard-delete`. No new public
operation.

## Method

Re-probed npm `@openai/codex` `latest` and the GitHub latest non-prerelease
release on 2026-10-26. Both are `0.164.0` / `rust-v0.164.0`. Ignored
`0.165.0-alpha.1`. Walked every published stable after `0.161.0`: `0.162.0`,
`0.162.1`, `0.163.0`, `0.164.0`. Hashed the npm wrapper tarballs and matched
registry SHA-512 integrity. Did not download or execute platform binaries.
Compared the selected source files on the public tags. No host `codex` binary
was required. No prompt, login, or live session.

## Identity

| Version | npm published | GitHub tag | Commit | Source files |
| --- | --- | --- | --- | --- |
| `0.161.0` | 2026-10-07T16:04:02.844Z | `rust-v0.161.0` | `979011409de0a60b52f179721948e65531d26144` | 8899 |
| `0.162.0` | 2026-10-08T19:01:18.070Z | `rust-v0.162.0` | `c1382380de69521303b416720a52f42d51af6248` | 9017 |
| `0.162.1` | 2026-10-09T19:50:30.512Z | `rust-v0.162.1` | `092d3acd6bec3e3a14bdc7e7a2810ab628ab759d` | 9018 |
| `0.163.0` | 2026-10-15T21:37:11.083Z | `rust-v0.163.0` | `83086188ae0614bd882818ff1aa23f3194962012` | 9176 |
| `0.164.0` | 2026-10-23T04:52:29.240Z | `rust-v0.164.0` | `6aaac694803004a63c6f393d153c96d22521a6b9` | 9438 |

The `0.161.0` tag object `7e21416b38834816c224ea0dfd135c3de94b2f15` matches
Research 410. Wrapper SHA-256 values are in the adapter fixture.

## Selected protocol

Byte-identical from `0.161.0` through `0.164.0`:

- exec argv definitions, JSONL processor, and exec event types
- `thread/delete` processor
- app-server `SandboxPolicy`, `AskForApproval`, and v1 initialize
- managed application loader and `application.network`

Additive and unselected:

- optional `parentTurnId`, `rootTurnId`, and responses metadata on `turn/start`
- optional `excludedThreadIds` and read-state fields on thread list and read
- optional model and reasoning effort on sub-agent activity
- `imageGeneration` payload reshape, which the adapter already treats as an unknown item
- `with_expanded_deny_globs` for unselected sandbox backends; `.aws` protection remains
- removal of `namespace_tools` from the unselected provider-capabilities read
- optional Desktop storage requirements
- listener, unload, and v2-child eviction changes that do not alter archive, restore, or delete requests

`0.162.0` to `0.162.1` does not change those selected files. The adapter does
not send the new optional fields. Unknown notifications stay progress
snapshots. Unpublished gaps, including `0.155.2`, stay incompatible.
`0.164.1` is the synthetic `UnverifiedNewer` point.

## Claim shape

- `codex.exec.cli-window-2` maintained `0.156.0..=0.164.0` on `codex.exec.jsonl-v1`
- `codex.app-server.cli-window-2` maintained `0.159.0..=0.164.0` on `codex.app-server.v2.protected-aws-workspace-roots`
- lifecycle maintained `0.140.0..=0.164.0` on the existing hard-delete revision

## Sources

- [Official npm package](https://www.npmjs.com/package/@openai/codex)
- [Codex `0.164.0` source tag](https://github.com/openai/codex/tree/rust-v0.164.0)
- [Research 410](./410-codex-exec-0-161-0-identity-and-policy-qualification.md)
- [Research 388](./388-codex-app-server-currentness-qualification.md)
