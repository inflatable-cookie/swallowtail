# 2026-08-05 Portable Failure Classification

Roadmap: `../roadmaps/g03/041-portable-failure-classification.md`
Cards: `../roadmaps/g03/batch-cards/115-portable-failure-contract-and-kernel.md`
through `../roadmaps/g03/batch-cards/117-portable-failure-provider-wide-acceptance.md`

## Changed

- accepted Contract 051 with provider-neutral failure origin, kind, and
  bounded recovery evidence
- kept exact safe diagnostic codes and messages as the route-specific support
  boundary
- made an all-unknown classification the safe default for every existing and
  future route
- added classified diagnostic construction, terminal failure views, cleanup
  diagnostic access, and warning-or-error activity diagnostics
- mapped typed direct-provider evidence for Anthropic, OpenAI, Kimi Platform,
  Bedrock, DeepSeek, llama.cpp, Ollama, xAI, and Alibaba Model Studio
- mapped qualified harness evidence for Gemini, Claude Code, Qwen, Kimi local
  server, Pi, Oh My Pi, Cursor, and Antigravity
- retained honest unknowns for opaque failures instead of parsing provider
  prose, stderr, output, or raw bodies
- published one consumer path through `TerminalOutcome::failure` with exact
  diagnostic and cleanup escape hatches

## Validation

- focused validation passed for common crates and every changed adapter
  package
- affected-package verification passed for common crates and every changed
  adapter package
- Kimi Platform's frozen failure corpus compiles from its extracted package
- `effigy qa:docs` — passed
- `effigy qa:routes` — passed
- `cargo fmt --all -- --check` — passed
- `git diff --check` — passed
- `effigy package:api` — expected additive held-candidate diff; the release
  baseline was not changed outside a candidate lane

No live or authenticated provider work ran.

## Next Move

Hold at the g03 evidence gate. A consumer can adopt the portable branch through
`TerminalOutcome::failure().diagnostic().failure_classification()` and retain
the exact diagnostic code for support evidence.
