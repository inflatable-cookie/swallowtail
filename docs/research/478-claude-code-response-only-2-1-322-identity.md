# Research 478: Claude Code response-only 2.1.322 identity

Status: response-only extends through official stable `2.1.322` on its existing
`stream-json.v3` claim. Headless stays qualified through `2.1.317` with
`2.1.318..=2.1.322` `UnverifiedNewer`; this task does not adapt its separate
permission-flow stop. Claude Agent ACP and SDK claims remain independent.

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

## Selected surface and hop classification

The npm launcher binary, `cli-wrapper.cjs`, and `install.cjs` are byte-identical
from `2.1.319` through both hops. Wrapper `package.json` changes are version
and platform dependency metadata; `sdk-tools.d.ts` adds an optional
`CronDeleteOutput.endedLoopWakeup` declaration at `2.1.321`, unrelated to
these CLI routes. Both native platform packages retain the same four-file
shape; only `claude` and version metadata change at each hop. Complete added,
removed, changed, identical, and per-file digest sets are frozen and tested in
`dist-inventory.json`.

Static printable-string inspection of both exact native artifacts finds the
same selected input/output-format help lines as Research 477. The ledger records
the selected flags and stream, result/failure, permission, hook and lifecycle
markers for each binary. No executable or help command was run. Swallowtail's
selected argv remains `-p --input-format text --output-format stream-json`
with `--tools ""`, `--safe-mode`, disabled slash commands and Chrome, prompt
suggestions disabled, an empty MCP server map, and strict MCP configuration.
The existing adapter still validates the same init and text result shape,
requires one terminal result, rejects tools and additional messages, maps
provider `is_error` and nonzero child exit to the existing typed failures, and
cancels and joins the child process using unchanged host code.

- `2.1.321`: compatible extension. Release notes include an NFS transcript-loss
  fix for Linux `-p`, now surfaced as a process error instead of silent exit 0.
  The note names text and JSON output, not a changed stream-json frame; this
  failure correction reaches the existing nonzero-exit process-failure class.
  The `num_hooks` / `num_success` change requires Bash or PowerShell calls, which
  the empty tool set cannot make. The other noted MCP, plugin, hook, settings,
  filesystem-write, model-selection, remote-control and workspace changes are
  outside the selected tool-free route or its configured command.
- `2.1.322`: compatible extension. Vertex fast-mode fallback concerns a remote
  Vertex/API path, not this locally launched Claude Code CLI route.

The `permissions.denyInsteadOfPrompting` setting introduced at `2.1.319`
remains in the binaries. Empty tools and safe mode leave no permissioned tool
call for it to change in response-only. Headless retains its separate stop;
this task changes no headless claim, maximum-turn point, ACP claim, SDK pin, or
provider-wide activity record.

## Decision

Extend only response-only v3 from `2.1.282..=2.1.319` through
`2.1.322`; keep the `2.1.227` baseline, deprecated v1/v2 segments, deny-list,
`AllowUnverified`, and previous holes. Add unpublished `2.1.320` to the explicit
deny-list so it stays a gap; classify `2.1.323` as `UnverifiedNewer`. No public
operation, authority, or lifecycle changes.

## Sources

- [npm package metadata](https://registry.npmjs.org/%40anthropic-ai%2Fclaude-code)
- [npm `2.1.321` wrapper](https://registry.npmjs.org/%40anthropic-ai/claude-code/-/claude-code-2.1.321.tgz)
- [npm `2.1.322` wrapper](https://registry.npmjs.org/%40anthropic-ai/claude-code/-/claude-code-2.1.322.tgz)
- [GitHub `v2.1.321` release](https://github.com/anthropics/claude-code/releases/tag/v2.1.321)
- [GitHub `v2.1.322` release](https://github.com/anthropics/claude-code/releases/tag/v2.1.322)
- [Research 477](./477-claude-code-headless-and-response-only-2-1-319-identity.md)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
