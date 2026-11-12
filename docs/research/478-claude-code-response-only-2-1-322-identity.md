# Research 478: Claude Code response-only 2.1.322 identity

Status: official latest is `2.1.322`; response-only remains qualified through
`2.1.319` on its existing `stream-json.v3` claim. The selected behavior of the
changed native binaries at `2.1.321` and `2.1.322` is unresolved on both
platforms, so both remain `UnverifiedNewer`. Headless stays qualified through
`2.1.317`; this task does not adapt its separate permission-flow stop. Claude
Agent ACP and SDK claims remain independent.

Route: `claude-code.response-only`
Axis: `claude-code.response-only-stream-json`
Claim: `claude-code.response-only.window-1`
Previous ceiling: `2.1.319` on maintained
`claude-code.response-only.stream-json.v3`

## Official identity

Re-probed on 2026-11-12. npm `latest` and `next` and GitHub latest stable agree
on `2.1.322`; npm `stable` remains delayed at `2.1.311`. The published npm
hops after `2.1.319` are `2.1.321` and `2.1.322`. `2.1.320` is absent from
npm and has no GitHub tag; it remains an explicit gap. `2.1.323` is the first
unpublished point above the new ceiling.

| Version | npm publish time | Wrapper tarball SHA-256 | GitHub tag | GitHub release time | Commit |
| --- | --- | --- | --- | --- | --- |
| `2.1.321` | `2026-11-09T19:02:47.641Z` | `00ae37b8e165be25ffc4068ba698317b07fa2d3bf56146b21fb82ac950dbf161` | `v2.1.321` | `2026-11-09T19:22:30Z` | `a4fcccc70a89df6607e66cc30682d551269d0680` |
| `2.1.322` | `2026-11-10T01:58:41.811Z` | `39f9d22aa2938cf276494d46b0c2bb593b4ef923323d02bf3cc5805b9d9a71c9` | `v2.1.322` | `2026-11-10T02:29:43Z` | `6e1e44f6433bfa17d198e932bff53373aa229eea` |

The [frozen 2.1.322 corpus](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.322/README.md)
contains npm SHA-512 integrity and SHA-256 for each wrapper and selected
Darwin arm64 and Linux x64 platform tarball, extracted executable digest and
size, complete per-file tree hashes, and the prior `2.1.319` tree rows reused
from Research 477. `2.1.320` and `2.1.323` both returned npm HTTP 404 and have
no GitHub tag. No downloaded executable was run; there was no prompt, login,
install, host change, or live session.

## Artifact identity and selected-evidence limit

The npm launcher binary, `cli-wrapper.cjs`, and `install.cjs` are byte-identical
from `2.1.319` through both hops. Wrapper `package.json` changes are version
and platform dependency metadata; `sdk-tools.d.ts` adds an optional
`CronDeleteOutput.endedLoopWakeup` declaration at `2.1.321`, unrelated to
these CLI routes. Both native platform packages retain the same four-file
shape; only `claude` and version metadata change at each hop. The complete
inventories and digests are frozen in `dist-inventory.json`.

The native binary identity and static-string observations are exact, but they
do not prove selected behavior. The package contains the executable and
metadata, not source-level route logic. No vendor binary or help command was
run. Both changed executables contain the same printable input/output-format
help lines as Research 477, but native code differs. Between `2.1.319` and
`2.1.321`, marker counts on both platforms change for `--tools` (85 to 83),
`--mcp-config` (93 to 94), `permissionDecision` (177 to 181),
`parent_tool_use_id` (130 to 131), `PreToolUse` (350 to 352), and
`updatedInput` (562 to 580). Linux `onFailure` remains 151; Darwin remains 159.
Between `2.1.321` and `2.1.322`, the executable hashes and sizes change again
while these measured marker counts stay the same. Counts establish neither
which branch owns a marker nor whether it is reachable from the selected argv.

The unchanged Swallowtail wrapper establishes only argument construction. Its
fake-process tests establish adapter parsing and joined cleanup, not how the
new native executables consume the flags, serialize stream-json init/result or
failure frames, reach hooks/settings, or shut down after cancellation. Release
notes do not close those gaps: the `2.1.321` Linux NFS note names text/JSON
output but does not settle the `-p` stream-json path; the hook-counter note
does not prove the relevant paths unreachable; and the `2.1.322` Vertex
fast-mode note describes a server/API path without proving anything about the
other changed native code. Therefore the available static evidence cannot
qualify either hop on Darwin arm64 or Linux x64.

The `permissions.denyInsteadOfPrompting` setting remains in the `2.1.319`
baseline. This task leaves the headless claim at `2.1.317`, with its separate
permission-flow stop; it changes no maximum-turn point, ACP claim, SDK pin, or
provider-wide activity record.

## Decision

Preserve the response-only v3 ceiling at `2.1.319` because static inspection
does not settle selected native behavior in either published hop. Keep
`2.1.321` and `2.1.322` `UnverifiedNewer`; keep the `2.1.227` baseline,
deprecated v1/v2 segments, `AllowUnverified`, and previous holes. Deny
unpublished `2.1.320` so it remains an explicit gap; `2.1.323` is also
`UnverifiedNewer` as the first unpublished point above official latest. No
public operation, authority, or lifecycle changes.

## Sources

- [npm package metadata](https://registry.npmjs.org/%40anthropic-ai%2Fclaude-code)
- [npm `2.1.321` wrapper](https://registry.npmjs.org/%40anthropic-ai/claude-code/-/claude-code-2.1.321.tgz)
- [npm `2.1.322` wrapper](https://registry.npmjs.org/%40anthropic-ai/claude-code/-/claude-code-2.1.322.tgz)
- [GitHub `v2.1.321` release](https://github.com/anthropics/claude-code/releases/tag/v2.1.321)
- [GitHub `v2.1.322` release](https://github.com/anthropics/claude-code/releases/tag/v2.1.322)
- [Research 477](./477-claude-code-headless-and-response-only-2-1-319-identity.md)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
