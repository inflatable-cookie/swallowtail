# Research 421: Claude Code 2.1.294 headless currentness stop

Status: currentness stop; no claim changed.

Owner: swallowtail#144
Date: 2026-10-08
Axis: `claude-code.headless-stream-json`
Route: `claude-code.headless`
Target: exact current npm and GitHub stable `2.1.294`

Question: can `claude-code.headless` extend its `2.1.281` ceiling through the
current official stable after adapting the `2.1.287` forked-skill stream and
`2.1.290` post-`PreToolUse` permission checks?

## Method

Re-probed npm dist tags and GitHub's latest non-prerelease release on
2026-10-08. npm `latest` and GitHub agree on `2.1.294`. npm `stable` is the
intentionally delayed `2.1.286` channel. npm `next` points to `2.1.295`; that
point is outside the stable target and remains an unverified newer version.
The first later version absent from the registry and GitHub tag is `2.1.296`.

Downloaded the npm wrapper, Darwin arm64 package, and Linux x64 package for
`2.1.294`. Verified each tarball's SHA-512 integrity against current npm
metadata, extracted into task-owned temporary storage, and added full
per-file SHA-256 and size inventories for all three package trees. The
inventory retains every prior point and adjacent delta from `2.1.281` through
`2.1.293`; it adds `2.1.294` and its exact file deltas. Static string checks
on both platform executables found the selected print, input/output,
permission, tool, model, effort, turn-limit, settings, and strict MCP options
and the stream-JSON input/output help text. Downloaded binaries were not run.

## Identity

| Surface | Identity |
| --- | --- |
| npm `latest` | `@anthropic-ai/claude-code` `2.1.294`, published `2026-10-08T03:42:57.084Z` |
| GitHub latest stable | `v2.1.294`, published `2026-10-08T05:03:54Z`, commit `71cdddec623889d38af14b7a489670a03186f659` |
| npm `stable` | `2.1.286`, delayed channel |
| npm `next` | `2.1.295`, published `2026-10-08T18:22:58.621Z`, outside the stable target |
| Published stable hops | `2.1.282` through `2.1.294`, no unpublished point between the qualified ceiling and latest |
| First absent later point | `2.1.296`; npm returned 404 and the GitHub tag was absent |

The three exact package trees, tarball integrity values, SHA-256 digests,
per-file hashes and sizes, and every adjacent delta are frozen in
[`claude-code-2.1.294`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.294/).
No source-to-runtime correlation is claimed: the public tag identifies a
commit, but the published executables are compiled provider runtime code.

## Hop review

The wrapper tree changes only `package.json` from `2.1.293`. Each mapped
platform package changes `claude` and `package.json`; the exact runtime and
package identities are recorded in the fixture.

The `2.1.294` release notes say instruction-form `prompt` and `agent` hooks
can now block actions as intended. They also change how instruction-form
`prompt` hooks on Stop and SubagentStop are judged. The selected headless
command keeps user, project, and local settings enabled, so these hook paths
remain selected. This is a separate safety and stop-behavior change from the
approved `2.1.287` forked-skill stream and `2.1.290` post-`PreToolUse`
rechecks. Release notes are discovery evidence; no provider prompt, runtime
execution, or proof of the exact hook outcomes was attempted.

## Decision

This record stops before a claim change. The approved task covers the
`2.1.287` and `2.1.290` adaptation only. The newly current `2.1.294` adds
selected hook behavior outside that scope, so the task must return for an
operator-approved follow-up adaptation before it can qualify the current
stable. The production claim, behavior revision, unpublished exclusions, and
`AllowUnverified` posture remain unchanged at `2.1.281`.

The `2.1.295` npm `next` point remains outside the official stable target. If
observed by a configured instance, it remains `UnverifiedNewer` under the
existing claim. Research 374 remains unchanged.

## Sources

- [npm registry metadata](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code)
- [GitHub `v2.1.294` release](https://github.com/anthropics/claude-code/releases/tag/v2.1.294)
- [Claude Code release channels](https://code.claude.com/docs/en/setup#configure-release-channel)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 023, Claude Code Headless Safety And Stream Adaptation](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md#claude-code-headless-safety-and-stream-adaptation)
- [Research 374](./374-claude-code-2-1-293-headless-currentness-stop.md)
