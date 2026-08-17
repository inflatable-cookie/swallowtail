# DeepSeek Harness Web `/api` Planning

Date: 2026-08-17
Roadmap: g03.070

## Outcome

Promoted DeepSeek Harness Web `/api` into Swallowtail planning without
collapsing it onto JSON-RPC or Open Platform continuation:

- Research 125
- Spec 009
- Milestone g03.070 with ready card 222 and planned cards 223-225

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

- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g03`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy qa:northstar:docs-front-door`

## Next

Implement card 222: freeze the Web `/api` artifact, loopback fence, method
allowlist, and redacted corpus.
