# Installed Harness Maintenance Inventory

Date: 2026-07-31

## Outcome

Research 074 reconciles 13 production installed/attached harness route ids
across ten solutions. Each row records driver, transport, version axis,
qualified segments, milestones, gaps, evidence owner/date, deterministic
corpus, repeatable probe posture, and consumer exposure.

## Findings

- all ordered claims already permit visible unverified-newer stable execution
- Codex, Kimi, and Gemini require route-specific claims despite shared
  executables
- Claude Agent wrapper and Claude Code headless are separate compatibility
  surfaces
- ACP wire v1 does not qualify any harness executable release
- the OpenCode live selector still hardcodes `1.14.48` while the production
  claim ends at `1.18.10`
- Gemini's live selector observes installation but does not classify its ACP
  or headless claim
- Pi and Qwen retain one-point guarantees despite later source comparisons
- the route validator retained pre-elicitation aggregate counts; its
  validation-only invariant now matches the unchanged 713-cell matrix
- the Grok matrix and guide now name the implemented
  `grok-build.executable` version axis instead of repeating the route id

No durable contract or architecture delta is required before currentness
research.

## Card 002 Boundary

The first external source set is Codex, stable ACP plus Claude Agent, Gemini
CLI, Pi, and Qwen. OpenCode receives only a bounded current publication check
to classify its stale probe. Kimi, Grok, OpenCode range code, and Claude Code
headless remain outside the full comparison after 2026-07-30 refreshes.

## Validation

- `effigy qa:routes` — 27 routes, 23 solutions, 713 audited feature cells,
  and 57 activity operations passed
- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `git diff --check` — passed

## Next

Execute g03 card 002.
