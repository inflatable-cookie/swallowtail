# DeepSeek Harness Web `/api` Planning

Date: 2026-08-17
Roadmap: g03.070

## Outcome

Promoted DeepSeek Harness Web `/api` into Swallowtail planning without
collapsing it onto JSON-RPC or Open Platform continuation:

- Research 125
- Spec 009
- Milestone g03.070 with completed cards 222-224 and active card 225; its
  deterministic acceptance is complete and its operator-gated Web live smoke
  remains pending

JSON-RPC `deepseek-harness.jsonrpc` stays the live-proven one-shot stdio run.
The second route is `deepseek-harness.local-server` on the same package,
spawned as owned `dsh web` on loopback.

First subset: exact `@deepseek-ai/dsh@0.1.0-rc.6`, method allowlist covering
list/search/create, history, models, prompt, native cancel, fork, and
archive. Credentials, settings, llm configuration, directory picker, and ZIP
export stay denied. No `dsh web` process was booted in this record.

Live proof may use host-local Ollama; that does not qualify
`deepseek-official`. Contract 054 stays unsupported until history proof.

ACP, JSON-RPC session-id continuity, headless CLI, and the browser UI stay
outside.

## Validation

- `python3 scripts/check-deepseek-harness-web-corpus.py -v` — 15 tests passed
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g03`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy qa:northstar:docs-front-door`

## Next

Card 223 completed at `01004db0`: the owned-process Web `/api` driver now has
loopback admission, allowlisted/bounded unary and WebSocket decode, catalogue
pagination, control-free history, structured prompt events and usage,
native cancel, fork, archive, deadline, and joined cleanup. Focused package
validation passed without network credentials or a live process.

Next: run the operator-gated Web installed classification and one prepared
facade prompt smoke with exact host inputs, then reassess the Contract 054 /
ACP / JSON-RPC continuity checkpoint.
