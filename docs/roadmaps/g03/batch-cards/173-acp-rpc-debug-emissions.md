# 173 ACP And Harness-RPC Debug Emissions

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../056-cross-route-debug-observation-emissions.md`
Depends on: card 172

## Goal

Plumb `HostServices` into ACP and harness-RPC connections and emit
malformed-inbound wire/parse observations using the Codex pattern.

## Closeout

`HostServices` plumbed into ACP connections (claude-agent, cursor, gemini,
grok, kimi) and RPC connections (pi, oh-my-pi). Pumps emit `ProtocolParse` on
decode/dispatch/read/finish failures. Claude Agent also emits from
`ActiveTurn::fail` for turn-local malformed usage. Fixture proves observer
capture on malformed usage.

## Validation

- focused claude-agent: 70 passed (including debug observer fixture)
- focused pi: passed (with claude-agent batch)
- focused cursor/gemini/grok/kimi: 230 passed
- focused oh-my-pi: 42 passed
- `effigy package:api`: unchanged baseline
