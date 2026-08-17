# DeepSeek Harness Web `/api` Live Acceptance

Date: 2026-08-17
Roadmap: g03.070
Card: 225

## Outcome

Card 225 live smoke passed through the prepared facade. Exact npm
`@deepseek-ai/dsh@0.1.0-rc.6` is now live-proven for one structured Web `/api`
run. The live model was host-local Ollama. That does not qualify
`deepseek-official` or change `deepseek.continuation`.

JSON-RPC remains the live-proven one-shot stdio run. ACP, session-id
continuity, headless CLI, version bump, tag, and registry work remain outside
this milestone. Contract 054 stays corpus-qualified only; this smoke did not
page history.

## Live Fixes

Owned spawn now passes `dsh web --patch <overlay>`. The JSON-RPC
`DSH_CORDIS_CONFIG` plugin tree is not the Web bind. The live probe approves
an interpreted Node launch because the npm `dsh` launcher uses
`#!/usr/bin/env` and ambient PATH is cleared.

A fresh Web host has no matching `workspace.list` row. Structured create
keeps `workspaceId` when a row matches the working resource, otherwise
creates with `cwd` and the pinned `standard` preset. It still does not send
both.

Mux `session/queue`, `session/jobs`, and `session/projection` snapshots are
ignored without reading bodies. Unknown session event types are content-free
progress; assistant text is still taken only from known `assistant/chunk`
shapes. Runtime event sequence is local and monotonic; provider seq is not
reused as the runtime ordinal.

The live host adds BlockingWork. Isolated `DSH_HOME` stays host-owned.

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek-harness`
- `effigy package:verify-affected swallowtail-adapter-deepseek-harness`
- `effigy probe:deepseek-harness-web-installed`
- `effigy probe:deepseek-harness-web-live`

No Cordis path, provider key, prompt, tool body, or reasoning text is
recorded here.

## Next

Reassess Contract 054 live promotion, ACP, JSON-RPC session-id continuity,
and DeepSeek-official as separate later gates. Keep `deepseek-official`
unqualified.
