# Research 422: Claude Code Headless 2.1.294 Stream and Hook-Safety Adaptation

Status: qualified through the frozen official stable `2.1.294` identity.

Owner: swallowtail#144

Date: 2026-10-09

Axis: `claude-code.headless-stream-json`

Route: `claude-code.headless`

## Decision

Extend only the headless route from its preserved `2.1.281` ceiling through
official stable `2.1.294`. Keep behavior v1 on `2.1.220..=2.1.286` and add
private behavior v2 on `2.1.287..=2.1.294`. Preserve unpublished gaps,
`AllowUnverified`, the baseline, claim id, SDK/ACP/response-only routes, and
the public operation lifecycle. The new point does not change the read-only
Plan policy, admitted `Read,Glob,Grep` tools, approved working resource, or
ambient `user,project,local` settings.
The optional maximum-turn feature remains restricted to its exact Research 226
set; this qualification does not widen it to `2.1.282..=2.1.294`.

## Frozen artifacts and selected path

Research 421 retains the complete npm wrapper, Darwin arm64, and Linux x64
package trees and every published hop after `2.1.281` through `2.1.294`.
Research 374 retains the per-hop selected-path classification through
`2.1.293`; this record reuses its unchanged hop findings and resolves the
`2.1.287` stream and `2.1.290` safety stops against the selected mapping.
The exact `2.1.294` platform executable SHA-256 values are:

| Artifact | SHA-256 |
| --- | --- |
| Darwin arm64 `claude` | `def0d15e64dd7d89621f88d28214f885b1c38b0ddd69762fb8593e34915d6d53` |
| Linux x64 `claude` | `27122ca7b624f537546fbef35b80c66370d974ff258f3d9b10ac50bb8771f262` |

Static inspection of both exact published executables found the following
selected-path behavior in their embedded control flow:

| Behavior | Frozen behavior |
| --- | --- |
| Instruction-form prompt and agent hooks | The decision requires `ok` and `reason`; `ok: false` chooses a blocking outcome. |
| `PreToolUse` denial | `permissionDecision: "deny"` chooses a blocking error before tool dispatch. |
| `PreToolUse` rewrite | The rewritten input goes through another safety and permission decision; a resulting deny blocks the call. |
| Stop and SubagentStop | `stop_hook_active: true` is treated as success by the stop-condition check. A repeated-block cap defaults to 8 and a positive `CLAUDE_CODE_STOP_HOOK_BLOCK_CAP` value overrides after the configured count. |

The selected command continues to use `--permission-mode plan`,
`--tools Read,Glob,Grep`, and `--setting-sources user,project,local`. It does
not add a hook-disable flag, `--bare`, or a permission-bypass argument. The
cap behavior is provider-owned: the adapter neither chooses a host's hook cap
nor disables hooks.

The evidence is static analysis of the exact published binaries, correlated
to their recursive package inventories. It is not source-to-runtime
correlation. No Claude Code binary was executed, and no hook prompt, user
settings, login, credential, or provider session was used.

## Adapter mapping and fake-process proof

The stream decoder now accepts `message.stop_reason: null` only when that
field is present. It treats a forked assistant frame as provider-unspecified,
does not mark it final, and continues consuming the bounded JSONL stream. Text
from later `end_turn` frames remains output, terminal `result` remains the
operation boundary, and usage is emitted once from that result.

Deterministic fake-process fixtures cover two selected cases:

- A synthetic forked-skill frame has `parent_tool_use_id` and a null stop
  reason, followed by stop-hook reentry-style assistant frames and one
  terminal result. The adapter completes only at the terminal result, keeps
  the forked activity non-final, preserves bounded output and usage, and joins
  the child.
- A synthetic denied Read result carries an error and no file bytes. The
  adapter reports the correlated provider tool result as failed and does not
  leak the fixture path. The fixture proves the adapter's projection at its
  process boundary; it is not a provider transcript or evidence of execution.

The first fake transcript is synthetic and does not claim that this route
receives hook-event records. The platform-binary control flow establishes the
provider's hook decisions. The fake process only proves the selected adapter
projection when the provider reports a denial. The existing structured-run
fake-process regressions continue to cover joined cancellation and cleanup;
the new targeted selector includes that regression binary.

## Currentness movement

The `2.1.294` identity was frozen before GitHub published stable `v2.1.295` at
`2026-10-08T19:48:38Z`; the frozen task commit is
`a9c841815a1d7c165f9e86b884b968de3b01bf77` (`2026-10-08T19:06:32Z`). On
2026-10-09, npm `latest` and `next` both report `2.1.295`, npm `stable` remains
the delayed `2.1.286` channel, and GitHub latest non-prerelease is `v2.1.295`.
The channels agree on the official current stable under the existing
operator ruling. `2.1.295` remains visible as `UnverifiedNewer`; this
adaptation does not qualify that post-identity point or infer behavior from
its metadata.

## Limits

No vendor artifact execution, live provider session, provider spend, credential
access, host installation, or persistent settings change was performed. No
change is made to SDK, ACP, response-only, public API, or public lifecycle
claims. Contract 036 release compatibility remains a separate gate.

## Sources

- [Research 421: original 2.1.294 currentness stop](./421-claude-code-2-1-294-headless-currentness-stop.md)
- [Research 374: selected hop classifications through 2.1.293](./374-claude-code-2-1-293-headless-currentness-stop.md)
- [Exact 2.1.294 package inventories and static semantics](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.294/)
- [Claude Code 2.1.294 release](https://github.com/anthropics/claude-code/releases/tag/v2.1.294)
- [Claude Code hooks documentation](https://code.claude.com/docs/en/hooks)
- [Claude Code 2.1.295 release](https://github.com/anthropics/claude-code/releases/tag/v2.1.295)
- [npm registry metadata](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 023, Claude Code Headless Safety And Stream Adaptation](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md#claude-code-headless-safety-and-stream-adaptation)
