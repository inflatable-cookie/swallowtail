# 2026-07-19 Integration Surface Inventory

Status: complete
Owner: Tom

## Change

Inventoried current harness, direct API, shared protocol, and self-hosted model
surfaces before runtime trait design.

The inventory includes Codex, Claude Code, OpenCode, Cursor, Pi, Kimi, Qwen
Code, Gemini CLI, xAI, GLM, Qwen, DeepSeek, ACP, Ollama, llama.cpp, vLLM, and
SGLang.

## Decision

Swallowtail will not identify an open-weight model family as a transport.
Model artifacts, serving drivers, configured deployments, protocol facades,
and model routes remain distinct under Contract 007.

At least seven materially different integration shapes now constrain runtime
work. Compatible request schemas remain wire-level evidence only.

## Consequence

Card 007 is complete. Card 008 may now settle the smallest host-neutral runtime
boundary without assuming Codex, CLI subprocesses, hosted APIs, or open-weight
serving is the default shape.
