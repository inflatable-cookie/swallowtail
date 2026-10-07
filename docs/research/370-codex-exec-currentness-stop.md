# Codex exec currentness stop at 0.156.0

Status: currentness identity sweep complete; no compatibility claim changed.
Date: 2026-10-07.
Scope: `codex.exec` on `codex.cli` only.

## Decision

Keep `codex.exec.cli-window-2` qualified through `0.155.1`. The first
security and authority change is in official `0.156.0`: the in-process
`thread/start` path no longer automatically persists project trust when the
working directory is classified as projectless. Codex exec supplies an
explicit `cwd` to that path. This is a change to trust behavior for the
selected route, so this sweep does not qualify `0.156.0` or any later stable.

Official `0.157.0` adds a separate host-managed network policy to ordinary
Codex exec. It loads system and macOS MDM application network requirements and
binds an allowlist to the exec client's network requests. That policy can
restrict model or search connectivity. Do not bypass it by setting
`ignore_managed_requirements`.

Both stops need their own provider-free adaptation and operator ruling before
the ceiling moves:

1. Exercise `codex exec --json --ephemeral` thread startup with explicit cwd
   in projectless and marked directories. Decide whether the projectless
   trust change preserves the `codex.exec` contract or needs a distinct
   behavior milestone. Do not restore upstream trust persistence around its
   gate.
2. Exercise managed application network policy with allowed and denied hosts
   through the prepared exec route. Decide how host-managed egress authority
   belongs in the route contract. Do not disable or weaken the policy.

Until those decisions and regression proofs land, the published stable points
`0.156.0`, `0.156.1`, `0.157.0`, `0.157.1`, `0.158.0`, `0.159.0`, `0.159.1`,
`0.159.2`, `0.159.3`, `0.160.0`, `0.160.1`, and `0.161.0` remain visible
`UnverifiedNewer`. The claim ID, behavior revisions, supported segments, and
exclusions remain unchanged. The independent `codex.app-server` claim was not
reviewed or changed by this route task.

## Official identity

The npm package `@openai/codex` reported `0.161.0` as `latest`. The official
GitHub repository published stable tag `rust-v0.161.0` at
`2026-10-07T15:58:45Z`; npm published its package at `2026-10-07T16:04:02.844Z`.
The observed stable sequence after the prior `0.155.1` ceiling is the twelve
points listed above. Both channels agree. Prerelease `0.162.0-alpha.18` is not
a stable point and was not considered.

The local host observation was `codex-cli 0.159.0`, SHA-256
`e89718aa1969bfc4a471277bdc4679a3a3529293de0a309909822dfd67ddb77a`, size
`240166592`, signed by OpenAI OpCo, LLC (TeamIdentifier `2DC432GLL2`). Its
local path is not retained. No host install or update occurred.

`370-codex-exec-currentness/identity.json` freezes the observed channel points,
publish times, source tag object and commit identities, package metadata,
artifact hashes, host observation, and the stop decision. `artifacts.tsv`
records npm tarball URLs, SRI integrity values, SHA-256 digests, file counts,
and unpacked sizes for the wrapper, Darwin arm64, and Linux x64 packages at the
baseline and all twelve stable hops. `artifact-file-inventory.tsv` and its
per-artifact manifests freeze the SHA-256 digest of every extracted path.
No downloaded archive or executable was run.

`source-name-status.tsv` records the complete changed-path set for all twelve
tag-to-tag hops; the matching files under `source-delta-name-status/` preserve
the per-hop name-status output. `source-tags.tsv` freezes the annotated tag
objects and commits. `selected-source-map.tsv` classifies each changed
`codex-rs/exec` source/test file and the projectless-trust and network-policy
paths that feed this selected route. Changelogs were used for discovery only.

## Hop findings

| Hop | Selected `codex.exec` finding |
| --- | --- |
| `0.155.1` → `0.156.0` | Stop: `thread/start` skips automatic project trust persistence for projectless directories. Codex exec supplies cwd. This hop also adds optional search `results` to JSONL; Swallowtail keeps query progress and ignores the additive results field. |
| `0.156.0` → `0.156.1` | No additional selected exec implementation change was found. The prior stop remains. |
| `0.156.1` → `0.157.0` | Second stop: system and macOS MDM `application.network` requirements bind a host allowlist to regular exec network clients. |
| `0.157.0` → `0.157.1` | No selected exec implementation change was found. |
| `0.157.1` → `0.158.0` | The changed exec file is a test; no selected exec implementation change was found. |
| `0.158.0` → `0.159.0` | Exec replaces internal app-server runtime path options. External argv and selected JSONL framing are unchanged. |
| `0.159.0` → `0.159.1` | No changed exec implementation file. |
| `0.159.1` → `0.159.2` | No changed exec implementation file. |
| `0.159.2` → `0.159.3` | No changed exec implementation file. |
| `0.159.3` → `0.160.0` | No changed exec implementation file; the config crate exposes a project-root discovery helper without changing its prior fallback result. |
| `0.160.0` → `0.160.1` | No changed exec implementation file. |
| `0.160.1` → `0.161.0` | Network-policy refresh keeps the active managed policy bound. Optional Daybreak/Cyber access program code is not selected by Swallowtail; the adapter adds no flag or feature. |

The selected decoder regression for the `0.156.0` additive search field verifies
that the query remains the only projected search content. It does not qualify
the authority changes in that same release.

## Evidence and validation

Official channels: [Codex npm package](https://www.npmjs.com/package/@openai/codex),
[Codex GitHub releases](https://github.com/openai/codex/releases), and the
[0.161.0 source tag](https://github.com/openai/codex/tree/rust-v0.161.0).

The task adds exact claim and artifact identity regression tests and the
additive search-results decoder test. Effigy selectors `validate:current-codex-exec`
and `check:current-codex-exec` define the focused proofs; results are recorded
in the task PR. No provider prompt, credential, live session, package install,
or host mutation was used.
