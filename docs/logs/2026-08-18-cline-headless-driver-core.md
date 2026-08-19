# 2026-08-18 Cline Headless Driver Core

## Result

Card 305 added the smallest `cline.headless` driver on
`swallowtail-adapter-cline`. Discovery is exact `cline.package` `3.0.55`
with claim `cline.headless.package-window-1` and behavior
`cline.headless.stdio-json-v1`. Spawn is `cline --json --auto-approve
false -c <cwd> <prompt>`. Stdin closes without writing the prompt. ACP
spawn stays `["--acp"]`. `--id`, `--auto-approve true`, docs `ask`/`say`,
and ACP JSON-RPC fail closed. CLI `--timeout` stays unselected; the host
process deadline is required.

`effigy validate:focused swallowtail-adapter-cline` passed (46 tests,
Clippy). No live install, login, or `--json` prompt.

## Next

Implement the Cline headless prepared facade (card 306).
