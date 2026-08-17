# DeepSeek Harness JSON-RPC Live Acceptance

Date: 2026-08-17
Roadmap: g03.069
Card: 221

## Outcome

Card 221 live smoke passed through the prepared facade. Exact runtime-bin
`0.1.0rc6` is now live-proven for one structured JSON-RPC run. The live model
was host-local Ollama. That does not qualify `deepseek-official` or change
`deepseek.continuation`.

ACP, Web `/api`, headless CLI, session-id continuity, version bump, tag, and
registry work remain outside this milestone.

## Live Fixes

Installed discovery no longer spawns the JSON-RPC binary. The packaged
`dsh-jsonrpc-agent` has no `--version` CLI; classification is basename plus
payload digest, then exact `0.1.0rc6`.

The live host forwards `HOME`, `DSH_CORDIS_CONFIG`, `DSH_CWD`, and any
Cordis-named provider key already present in the probe process. Swallowtail
does not lease that key.

The JSON-RPC parser now accepts the installed `0.1.0rc6` stdout shape as well
as the redacted corpus subset:

- `agent/inbox/spliced` at seq 0 and `session.status` running may arrive
  before the prompt `{ messageId }` result
- turn and step ids may be numbers
- deltas may use `text`; finish and `turn/end` may use `reason.kind`
- `session/title` and inbox splices are content-free progress
- reasoning bodies are ignored, not fail-closed

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek-harness` — 13 tests
- `python3 scripts/check-deepseek-harness-corpus.py` — 13 tests
- `effigy package:verify-affected swallowtail-adapter-deepseek-harness`
- `effigy qa:guides`
- `effigy probe:deepseek-harness-installed`
- `effigy probe:deepseek-harness-live`

No Cordis path, provider key, prompt, tool body, or reasoning text is
recorded here.

## Next

Reassess session-id continuity, ACP, and Web `/api` as separate later
surfaces. Keep `deepseek-official` unqualified.
