# 085 Grok ACP Answerable Permissions

Status: planned; gated behind current Grok permission observation in `turn.rs`
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: current Grok permission observation in `turn.rs`; Contracts 012 and 015

## Goal

Answer `grok-build.acp` `session/request_permission` requests from the consumer, or publish an explicit labelled activity-only posture if the ACP path cannot answer.

## Readiness

Planned in the consumer's priority order under g05.029. Chatterbox makes
this card ready with a full scope, manifest, oracle, and validation tier
when its gate is satisfied. It carries no execution authority now.
