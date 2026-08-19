# 2026-08-18 Cline Headless 3.0.55 Identity

## Result

Card 304 froze official Cline headless identity at npm `cline@3.0.55` /
GitHub `cli-v3.0.55` without installing, logging in, or sending a prompt.
The selected wire is `cline --json --auto-approve false` plus one prompt
operand: stdout envelope NDJSON (`run_start`, `agent_event`,
`run_result`), not `--acp` and not the docs `ask`/`say` schema. Named
fixtures live under the existing adapter tree. No headless claim yet.

## Next

Implement the Cline headless driver core (card 305).
