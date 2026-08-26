# 2026-08-26 g04.071 Copilot CLI ACP Built-In Tool Allowlist Closeout

Status: stopped after evidence
Owner: Tom
Milestone: g04.071
Cards: 195 complete; 196-197 blocked
Branch: `t3code/copilot-acp-tool-allowlist`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-156ca066`
Base: `a971f30ed025459dbec463a136fdfcdc6f672569` (`origin/main` at dispatch)
Planning base ancestor: `08d24df25dc242b50be75d0c7ebd97bf63fbb182`
PR: pending

## Result

Card 195 completed an exact `1.0.80` package, parser, ACP-startup, registry,
permission, and production-seam audit. Research 218 admits no deliver-now
built-in tool-allowlist row. Cards 196 and 197 are blocked and were not
executed. The Copilot CLI ACP adapter, prepared facade, child argv, fixtures,
guide, matrices, and API baseline are unchanged. No install, native-binary
execution, login, account inspection, initialize, prompt, tool invocation, or
paid operation was used.

## Evidence Stop

Exact `1.0.80` commander registers `--available-tools [tools...]`. `T5` comma-
splits at paren depth 0 and trims; `xW` treats `undefined`/`true`/empty as
omitted. ACP `session/new` stores that list on the session. Unknown names warn
via `session.info` rather than fail spawn. Native `sessionFilterEnabledToolIndexesJson`
does the membership filter and was not executed.

Documented identifiers are not a closed JS table (`list_bash` is absent from
`app.js`). CLI emits bare names; SDK types say bare names match any source.
ACP still loads host MCP config and `github-mcp-server` when the client sends
`mcpServers: []`. Official docs say available-tools wins over excluded-tools;
SDK empty-mode and types say excluded wins. CLI ACP does not set
`toolFilterPrecedence`.

The filter is provider-native behavior, not permission and not isolation.
Current `copilot --acp --stdio` argv, unmapped fixtures, observe-and-stop
permission, and `AmbientHost` stay unchanged.

## Changed Surfaces

- `docs/research/218-copilot-cli-acp-built-in-tool-allowlist-evidence.md`:
  promoted exact package, parser, ACP startup, registry, permission, production
  audit, claim strength, and empty deliver-now table
- cards 195-197, g04.071, programme, triage, indexes, this closeout

No production code, public API, shared contract/runtime, guide capability,
matrix, or changelog edit.

## Validation

Passed:

- `effigy validate:focused swallowtail-adapter-copilot-cli` — 28 tests passed
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`

No production code changed. Doctor was not re-run; the inherited 378 god-file
baseline is unchanged by docs-only edits.

## Continuation

Keep g04 open. Reassess the remaining per-route feature inventory for the next
serial lane unless the operator supplies a different direction. Contract 029
currentness remains standing. Do not compile the next family from this closeout.
