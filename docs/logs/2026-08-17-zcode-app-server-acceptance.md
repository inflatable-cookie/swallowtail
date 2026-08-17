# ZCode App-Server Live Acceptance

Date: 2026-08-17
Roadmap: g03.071
Card: 229

## Outcome

Card 229 live smoke passed through the prepared facade. Exact runtime
`0.16.3` is now live-proven for one structured app-server run. The live model
was host-local Ollama through custom provider id `zai`. That does not qualify
Z.AI official, Coding Plan, or OAuth.

OpenCode, hosted GLM HTTP, ACP, `--print`, native `session/stop`, history,
version bump, tag, and registry work remain outside this milestone.

## Live Fixes

The packaged payload rejects `--settings`. Host-approved settings are read
from `$HOME/.zcode/cli/config.json`. The live probe copies the host file into
an isolated HOME and wipes that HOME before each run. Swallowtail does not
mint the file or default `yolo`.

The live `0.16.3` create snapshot is admitted alongside the reconstructed
corpus subset:

- protocol name `ZCode Protocol` version 1, or corpus `zcode-app-server`
- host mode on `settings.mode.current` and `settings.permission.mode` when
  present; `session.mode` is a different field
- session model as `{modelId, providerId}` or a string
- runtime `cliVersion` checked when present; payload digest remains the pin

Create still answers `session/requestRuntimePreferences`. Later same-session
requests, including scope `user-execution`, reuse the fail-closed defaults.
Unscoped unknown session events and non-session notifications
(`state.updated`, `v4/telemetry/event`) are content-free progress. Namespaced
`zcode/` events stay namespaced observations. Telemetry bodies and request
headers are not projected.

## Validation

- `effigy validate:focused swallowtail-adapter-zcode` — 23 tests
- `python3 scripts/check-zcode-app-server-corpus.py` — 13 tests
- `effigy package:verify-affected swallowtail-adapter-zcode`
- `effigy qa:guides`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy probe:zcode-installed`
- `effigy probe:zcode-live`

No settings path, provider key, prompt, tool body, session id, or reasoning
text is recorded here.

## Next

Reassess native `session/stop`, `--print`, history, ACP, and Z.AI official as
separate later gates. Keep OpenCode and hosted GLM HTTP outside the first
ZCode route.
