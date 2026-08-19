# Cline headless 3.0.55 identity corpus

Secret-free source identity for `cline.headless` on the existing
`swallowtail-adapter-cline` package. This is a distinct route from
`cline.acp`.

Official npm `cline@3.0.55` plus GitHub `cli-v3.0.55` freeze the selected
print wire: `cline --json --auto-approve false`, one prompt operand, optional
`-c` working resource, stdout envelope NDJSON, join/kill the child.

The tagged encoder emits `{ts, type, ...}` envelopes (`run_start`,
`agent_event`, `run_result`, abort/error). It does not emit the docs.cline.bot
`ask`/`say` schema. `--acp`, TUI, hub, `--id`, `--yolo`, `--zen`, and
`--auto-approve true` are not this corpus.

No live `--json` run. No provider prompt. No install. No host `cline`.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
