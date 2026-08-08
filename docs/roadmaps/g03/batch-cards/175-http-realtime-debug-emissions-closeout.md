# 175 HTTP Realtime Debug Emissions And Guide Closeout

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../056-cross-route-debug-observation-emissions.md`
Depends on: card 174

## Goal

Emit on hosted/attached HTTP/SSE/WS and remote ACP failure paths; close the
guide emitter inventory for realized routes.

## Closeout

HTTP/SSE/WS pumps and remote ACP emit failure-path observations. Anthropic
disconnect fixture proves observer capture. Guide Current Emitters table
lists shared discovery, ACP/RPC, headless, hosted, realtime, and remote
emitters. Milestone g03.056 complete.

## Validation

- focused batches for opencode/anthropic/openai/deepseek, alibaba/kimi-platform/
  kimi/bedrock, gemini/xai/transport-acp-remote: passed
- `effigy package:api`: passed
- `effigy qa:docs`: run at milestone closeout
