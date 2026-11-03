# Claude Code 2.1.317 headless and response-only identity

`dist-inventory.json` freezes the npm wrapper, Darwin arm64 package, and Linux x64
package for the previous ceiling `2.1.316` and official stable `2.1.317`. The
re-downloaded `2.1.316` tarball and both `claude` digests match Research 466 and the
frozen `claude-code-2.1.316` fixture. Every tarball was checked against current npm
SHA-512 integrity and hashed per file. No downloaded executable was run. The wrapper
changes only `package.json` and `sdk-tools.d.ts`; each platform package changes only
`claude` and `package.json`; the launcher files stay byte-identical.

`marker-ledger.json` records the exact marker and stream-json help-line arrays on
both compared points. `selected-control-flow.json` binds the measured selected JS on
`2.1.316` and `2.1.317` to the executable hashes: instruction-form prompt-hook
blocking, PreToolUse deny before dispatch, rewrite collection and the ask-to-deny
recheck, the Stop block cap default 8, and Stop/SubagentStop `stop_hook_active`
reentry. The extractor was calibrated to reproduce every frozen
`claude-code-2.1.316` window byte-for-byte before reading `2.1.317`; the differing
windows keep every decision token and change only minified identifiers, per-build
chunk file names, and the version banner. Three surfaces are byte-identical across
the hop.

`response-only-isolation.json` freezes `--safe-mode`, the builtin-only plugin filter,
init fields, the permission-mode default mapper, the interactivity gate, and the
`@builtin` plugin ids. `2.1.317` adds `cc-plugin-privacy` to the unconditional
builtin initializer; it attaches `tool.check` hooks to web and shell tools that
neither route selects and contributes no MCP server. `cc-plugin-claude-platform`
still requires `isInteractive()`, which the print detector makes false for `-p`.

`forked-stream-surface.json` re-checks the surface Research 374 stopped on: the
assistant-frame description is byte-identical on both platforms across the hop, the
nullable `parent_tool_use_id` schema field stays, and the `context:"fork"` command
definitions stay.

`headless-identity.json` and `headless-protocol.json` are the headless claim.
`response-only-identity.json` and `response-only-protocol.json` are the response-only
claim. At observation, npm `latest`, npm `next`, and GitHub latest non-prerelease all
point at `2.1.317`; npm `stable` is the delayed channel at `2.1.311`. `2.1.318` is
the first unpublished point. The host `claude` at `2.1.314` stays byte-identical to
the official Darwin arm64 `2.1.314` artifact; its `--version` was read and no
provider work ran. Research 473 records the qualification. Claude Agent ACP and the
Claude Agent SDK stay separate claims.
