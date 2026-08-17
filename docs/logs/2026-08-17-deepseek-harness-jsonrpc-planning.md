# DeepSeek Harness JSON-RPC Planning

Date: 2026-08-17
Roadmap: g03.069

## Outcome

Promoted DeepSeek Harness JSON-RPC into Swallowtail planning without collapsing
it onto Open Platform continuation:

- Research 124
- Spec 008
- Milestone g03.069 with ready card 218 and planned cards 219-221

First subset: exact runtime-bin `0.1.0rc6`, owned-process NDJSON JSON-RPC,
structured run, content-free reasoning progress, harness-owned tool activity,
usage, process-kill cancel. ACP, Web `/api`, headless CLI, and
`deepseek.continuation` stay outside.

Live proof may use host-local Ollama; that does not qualify
`deepseek-official`.

## Validation

- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g03`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy qa:northstar:docs-front-door`

## Next

Card 218 is ready for a worker. Cards 219-221 continue only through their
auto-continuation after the predecessor lands.
