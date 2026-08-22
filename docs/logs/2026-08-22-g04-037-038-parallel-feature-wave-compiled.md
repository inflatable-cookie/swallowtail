# 2026-08-22 g04.037-038 Parallel Feature Wave Compiled

## Change

- assessed parallel safety across the active per-route feature programme
- kept g04.036 Ollama internally serial because evidence defines its binding
- compiled g04.037 and cards 101-103 for Anthropic Messages effort
- compiled g04.038 and cards 104-106 for DeepSeek reasoning controls
- reserved Research 185 and 186 before dispatch

## Decision

The three route families may execute concurrently in isolated worktrees because
their implementation, fixtures, and focused package validation are independent.
Each route remains internally serial: exact evidence, then binding, then
acceptance.

The worker branches have disjoint mutable scope. Research and closeout files are
pre-reserved and indexed. Anthropic and DeepSeek defer shared architecture,
matrix, changelog, programme, front-door, index, and `packages.txt` changes to
orchestrator closeout. The integration order is g04.036 Ollama, then g04.037
Anthropic, then g04.038 DeepSeek. Workers may develop and review concurrently;
the orchestrator owns restacking, shared-surface reconciliation, and merge
truth.

Anthropic effort maps only exact Research 185 model/value/profile combinations
to portable reasoning selection. It is not Messages thinking, Claude Code
effort, Ultracode, Fast mode, or Managed Agents configuration.

DeepSeek Research 186 must separate effort from thinking mode and preserve the
private continuation contract. Provider aliases cannot silently map to `high`.
A required facade revision or Contract 030 change stops the worker.

## Next

Execute g04.036, g04.037, and g04.038 in isolated route-family workers. Review
and restack them in fixed order. Compile xAI only after this wave is assessed.
