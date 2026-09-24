# 341 Claude Code 2.1.281 Built-in Hooks and Qualification Stop

Status: promoted evidence; g06.018 branch 3. No production claim edit.

Date: 2026-09-24

## Question

Can Claude Code headless and response-only move from qualified `2.1.278` to
current official stable `2.1.281` without weakening response-only isolation?
Research 338 identified the `2.1.280` safe-mode hook change. This record
tests its actual built-in inventory and the next published hop.

## Method and identity

npm `@anthropic-ai/claude-code` `latest` and GitHub latest release both named
`2.1.281`. npm published `2.1.280` on 2026-09-22T15:44:39.443Z and
`2.1.281` on 2026-09-23T17:01:17.780Z. GitHub published the corresponding
stable releases on 2026-09-22T16:38:14Z and 2026-09-23T19:19:15Z. Their
lightweight tag commits are `56f36532530f88b572854538d685fcf781141e8c`
and `d78be9481b889e11186ec4578b4f5e9301396e25`. `2.1.279` is not
published; `2.1.282` was the first unpublished later stable at observation.

The npm wrapper and both platform tarballs for `2.1.278`, `2.1.280`, and
`2.1.281` were downloaded to `/tmp/g06-018`. Every tarball and shipped file
was SHA-256 hashed. The `2.1.278` and `2.1.280` wrapper and binary digests
reproduce Research 331 and 338. The exact per-hop file inventory is frozen in
`tests/fixtures/claude-code-2.1.281/dist-inventory.json`. Each platform
package still contains four files; the wrapper contains seven. Wrapper
executable support files are unchanged; `2.1.281` changes its package pin and
SDK declaration file. The platform `claude` binaries change at both hops.

The host has a `claude` command on `PATH`, but it was not executed. None of
the downloaded binaries were executed. No install, prompt, login, live
session, watcher operation, or provider mutation occurred. Evidence comes
from embedded `// @bun @bytecode` source in the official binaries. GitHub
release notes were discovery only.

| Version | Darwin arm64 binary SHA-256 | Linux x64 binary SHA-256 |
| --- | --- | --- |
| `2.1.278` | `bd245662fb8a0e321b3bf133e930371d6563c387527885f30b2613aef3ba14d6` | `5c4735937844e84f8a93306e841a5b0e12252909b07870f789b190468da147ab` |
| `2.1.280` | `387a5c5dcdbb815085edf0baf79591f9d8894efe922bceaf3d75b1b08055229d` | `1e08503dbdf3c2cb0d706d32f3408277388d1c76ef108673e8fe42c1b322925b` |
| `2.1.281` | `a922981f6f3b55a251ef9f9dbaa0621a5f99cbcb5ca67f8a797476ccfc83f626` | `56fe3da88458465fb27d7e9299dddb3fead55750fb9c2de795f233b5eea6dce1` |

## Built-in hook inventory

Both `2.1.280` and `2.1.281` register the same nine named built-ins in both
platform builds: `sec-default`, `agents-md`, `telemetry`, `plugin-authoring`,
`tips`, `mermaid`, `responsive-mode`, `diff`, and `claude-test`. The frozen
`builtin-hook-ledger.json` gives each plugin's hook events, effect, Linux
embedded-module hash, and both platform registration checks. The selected
response-only route passes `--tools ""`, an empty strict MCP config,
`--disable-slash-commands`, and `--safe-mode`.

- `sec-default` is policy-only. Its hooks can preserve managed instructions,
  tool policy, and settings before user plugins. It is seated only under the
  provider's policy conditions.
- `agents-md` hooks `session.start`, `prompt.context`, `agent.spawn`, and
  `tool.call`. Its `prompt.context` handler reads ancestor `AGENTS.md` files
  and appends their contents as `instructionFiles`; its `Read` hook may append
  nested instructions. Its embedded default changes from **off** at
  `2.1.280` to **on** at `2.1.281` on both platform builds. The selected
  empty tool list prevents `Read`, but does not prevent `prompt.context`.
- `telemetry` hooks `engine.create`, `session.start`, and `session.end` and
  sends provider analytics under its settings gate. This is a network effect,
  although it is not a consumer tool call.
- `plugin-authoring` supplies an invocable skill and has no hook event.
  `tips` hooks `session.start` for UI tips; `mermaid` hooks terminal
  `ui.render`. They do not rewrite the selected stream-JSON response.
- `responsive-mode` hooks `prompt.section`, `prompt.submit`, and terminal
  `ui.render`. If its rollout gate is on, it adds prompt text and can abort a
  turn after a quick reply. `diff` and `claude-test` register UI, command,
  skill, and tool hooks. Their command paths are blocked by the selected
  disabled slash commands and empty tool list, but their hooks still load.

## Switch search and branch decision

Research 338's safe-mode filter is reproduced on both `2.1.280` builds and
persists on both `2.1.281` builds: it keeps plugins whose source ends in
`@builtin`. The registration path then loads their hook modules. The
`enabledPlugins` parser discards entries naming a built-in from writable
settings, including `--settings`; only administrator-controlled managed
settings can name those IDs. The non-managed `disableAllHooks` setting is
treated as managed-hooks-only: the internal all-disabled predicate reads
`policySettings.disableAllHooks` alone. An `instructionFiles` setting can
change `agents-md`'s instruction policy, but it does not disable the plugin
hook set or the other built-ins. No supported selected-surface flag or
setting disables all the kept hooks.

The `2.1.281` default-enabled `agents-md` prompt-context hook can change the
response-only turn by adding project instructions. Thus branch 1 (inert)
fails. No selected switch disables the kept hook set, so branch 2
(switchable) fails. **Branch 3 applies.** A narrowed response-only claim
must explicitly allow provider built-in hooks, project instruction injection,
and first-party telemetry under provider settings, while retaining the
adapter's empty tools, empty strict MCP, disabled slash commands, and no
session persistence. That changes the consumer-visible isolation guarantee;
the operator must rule before the claim or bound moves. The present claim
remains at `2.1.278` for both axes. `2.1.280` and `2.1.281` remain
`UnverifiedNewer`; unpublished `2.1.279` remains `UnverifiedNewer` until a
future claim extends the window, when it must become an explicit exclusion.

Headless does not pass `--safe-mode`. Its `2.1.280` hop has no new selected
flag or stream-JSON shape in Research 338. The `2.1.281` `agents-md` default
is also a provider instruction-source change for headless, which already
permits ambient project instructions. It does not justify moving the
response-only claim or watcher `2.1.251` authorization. The two axes remain
separate in the prepared claim change.

## Sources

- [npm package metadata](https://registry.npmjs.org/@anthropic-ai%2Fclaude-code)
- [GitHub v2.1.280 release](https://github.com/anthropics/claude-code/releases/tag/v2.1.280)
- [GitHub v2.1.281 release](https://github.com/anthropics/claude-code/releases/tag/v2.1.281)
- [Research 338](./338-claude-code-2-1-280-identity-stop.md)
- Frozen corpus: `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.281/`
