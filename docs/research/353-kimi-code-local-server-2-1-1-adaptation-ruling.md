# 353 Kimi Code Local Server 2.1.1 Adaptation Ruling

Status: promoted evidence. No production claim edit.

Date: 2026-09-26

Authority: Contracts 017, 023, 029, and 061; Research 282 and 326; the
`kimi-code.local-server` selection, prepared route, and disabled-tool
controls; the official npm and GitHub channels.

## Question

Does `kimi-code.local-server` qualify current official `@moonshot-ai/kimi-code`
with restored, route-controlled Bash `cwd` containment, or which exact
rulings does Tom need before any claim can move?

## Official latest

Re-probed npm `@moonshot-ai/kimi-code` `latest` and GitHub latest on
2026-09-26. Both name `2.1.1`. npm published `2026-09-24T07:27:15.480Z`.
GitHub published `2026-09-24T07:24:08Z`. Published stables after the
Research 326 observation are `0.43.1`, `2.0.0`, `2.0.1`, `2.0.2`, `2.1.0`,
and `2.1.1`. `0.43.1` is now published (Research 326 recorded it
unpublished). `2.1.2` is the first unpublished later stable. Not a
preview channel.

`2.0.0` is a major-line reset on the same package. Contract 029 treats that
as an identity investigation, not `UnverifiedNewer`, and the in-run movement
rule stops and asks. The selected local-server protocol files at `2.0.0`
are byte-identical to `0.43.1` except `CHANGELOG.md` and
`packages/kap-server/src/routes/prompts.ts`. The changelog "Major Changes"
entry is the `/desktop` slash command and `kimi install-app` subcommand.
That is not a new local-server driver. Same product, same axis, still a
major-line ask.

Host `kimi` remains `0.34.0` at
`sha256:9f4337e10da47843f6b550474012a53ba8b30dd665f83b176a5cd479c5f7e859`.
`--version` and the digest were observed. The host was not installed,
updated, or replaced. Downloaded npm tarballs were hashed and extracted
in `/tmp/kimi-066` and not executed. No prompt, login, catalogue, live
session, or local-server start.

Identity, blob ledger, and the four rulings are frozen in
`crates/swallowtail-adapter-kimi/tests/fixtures/kimi-local-server-2.1.1/`.

## Containment

`RuntimeWorkspaceView.resolve` at official `2.1.1` is still a pure path
map. `assertAllowed` exists as a separate method and is not called from
`resolve`. The `0.40.0` blob `01db1bbf…` is byte-identical at `0.43.0`,
`0.43.1`, and every `2.x` hop through `2.1.1`. `bashTool` still computes
`effectiveCwd` as `view.resolve(args.cwd ?? view.workDir)` and still
registers as `name: 'Bash'`. That blob `41090010…` is also unchanged
from `0.39.0` through `2.1.1`. Session PTY create still calls
`assertAllowed`; that is not the Bash tool path.

Changelog notes after `0.43.0` do not restore the assertion. `2.0.1`
removes a system-prompt rule against file access outside the working
directory. `2.0.2` says the agent no longer assumes cwd is the project
root. `2.1.0` blocks file-tool symlink escape. None of those change the
Bash `resolve` blob.

## Route-controlled mechanism

The route already sends optional `profile` and `disabled_tools` on the
REST prompt from `0.29.0`. At `2.1.1`:

- `POST …/prompts` applies `disabled_tools` through
  `setSessionDisabledTools` after `setModel`.
- `setModel` binds `DEFAULT_AGENT_PROFILE_NAME` (`agent`) when no profile
  is bound, so an explicit consumer profile is not required for the
  denylist to apply.
- `isToolActiveComposed` includes `sessionDisabledTools` as an exact-name
  builtin denylist. The name is `Bash`. `bash` is an unknown-tool typo.
- The tool-executor guard rejects a disabled call with
  `Tool "Bash" is disabled by the active tool policy`.
- `evaluate.ts` is byte-identical from `0.43.0` through `2.1.1`.
- Structured runs share the interactive prompt path, so a pinned
  configuration would reach both operations.

That hides the uncontained tool. Contract 023 still says a tool denylist
does not contain the `AmbientHost` process, its descendants, or
unmediated filesystem and network access. Loopback bind contains
network exposure, not process cwd. `ProcessRequest` still sets no cwd.
No pinned setting or profile restores `assertAllowed` on `resolve`.

Pinning `disabled_tools: ["Bash"]` for every point above `0.39.1` would
remove the default shell tool. That changes a consumer-visible
guarantee. The brief stops the claim on that narrowing.

## Decision

**Ruling request. No claim change.** Keep
`kimi.local-server.executable-window-5`, baseline `0.28.1`, ceiling
`0.39.1`, heartbeat-ping behavior, and `QualifiedOnly`. Every point
above `0.39.1` stays rejected, including newly published `0.43.1` and
`2.0.0..=2.1.1`. ACP and headless stay untouched. Decoder specimens stay
on `kimi-local-server-0.28.1-0.29.0`.

## Rulings

Each option is one Tom ruling. None is taken here.

| Id | What it authorizes | What it costs |
| --- | --- | --- |
| A | Pin `disabled_tools: ["Bash"]` on every local-server prompt and structured run above `0.39.1`, then qualify current official `2.1.1` as a same-axis milestone after a claim-step corpus | Removes the default Bash tool. Does not contain `AmbientHost`. Exact name `Bash` only. Treats the `2.0.0` major line as the same local-server axis |
| B | Treat the `0.40.0` assertion removal as outside Swallowtail isolation, because `AmbientHost` never claimed workspace containment, and qualify `0.40.0..=2.1.1` without restoring `assertAllowed` | Withdraws the Research 282/326 fail-closed reading and the A2 same-risk-class conclusion |
| C | Authorize a new `ProviderEnforced` or `HostEnforced` isolation route that contains Bash cwd | New public lifecycle. No such mechanism exists in the frozen `2.1.1` artifacts |
| D | Keep `QualifiedOnly` at `0.39.1` until the provider restores `assertAllowed` on `resolve` | Exception to Contract 029 No Terminal Stop. Current official `2.1.1` stays rejected |

A live turn that proves the REST denylist blocks Bash is not taken. The
brief stops if evidence needs a live turn. Source blobs already name the
guard. A live turn is only a follow-up if Tom picks A and wants runtime
proof beyond the frozen executor path.

## Sources

- npm `@moonshot-ai/kimi-code` `0.43.0` through `2.1.1`
- [GitHub `2.1.1`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai/kimi-code%402.1.1)
- `apps/kimi-code/CHANGELOG.md` at `0.40.0` through `2.1.1`
- git blobs at commits `ffa94fae`, `75ac010b`, `1b89e4b0`, `caf7d4e2`,
  `9d07f634`, `52437299`, and `f67e6398`
- frozen `crates/swallowtail-adapter-kimi/tests/fixtures/kimi-local-server-2.1.1/`
  and the historical `kimi-local-server-0.43.0/` corpus
- [Contract 017](../knowledge/contracts/017-provider-owned-session-load-replay-and-host-containment.md),
  [Contract 023](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md),
  [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md),
  and [Contract 061](../knowledge/contracts/061-consumer-route-feature-and-control-projection.md)
- [Research 282](./282-kimi-code-local-server-0-41-0-identity.md) and
  [Research 326](./326-kimi-code-local-server-0-43-0-containment.md)
