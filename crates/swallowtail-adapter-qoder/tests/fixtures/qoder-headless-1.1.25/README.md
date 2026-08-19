# Qoder headless 1.1.25 identity corpus

Secret-free source identity for `qoder.headless` before any Swallowtail
package or claim exists.

Official npm `@qoder-ai/qodercli@1.1.25` freezes the selected print
wire: `qodercli --print --output-format stream-json --permission-mode
dont_ask --max-turns N --no-session-persistence --cwd DIR PROMPT`.
Stdout is stream-json NDJSON (`system` / `assistant` / `result`).
`--acp`, SDK stdio, TUI, and `--yolo` / `bypass_permissions` are not
this corpus.

No live `--print`. No provider prompt. No install. No host `qoder` or
`qodercli`.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
