# 174 Headless And Codex Exec Debug Emissions

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../056-cross-route-debug-observation-emissions.md`
Depends on: card 173

## Goal

Thread `HostServices` into headless process pumps and emit on parse/process
failures; cover Codex exec failure paths beyond app-server.

## Closeout

Headless pumps (qwen, muse, antigravity, cursor, claude-code, gemini, kimi)
and Codex exec emit `ProtocolParse` / `HostProcess` on decode and process
failures. Qwen and Codex exec fixtures prove observer capture.

## Validation

- qwen + codex: 202 passed
- muse + antigravity: 50 passed
- cursor + claude-agent + gemini + kimi: 273 passed
