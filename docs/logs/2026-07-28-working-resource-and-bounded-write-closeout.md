# 2026-07-28 Working Resource And Bounded Write Closeout

Roadmap: `../roadmaps/g02/032-working-resource-and-workspace-authority-feature-closure.md`
Cards: 107-110

## Changed

- Research 058 classifies all 31 starting cells: 24 operation-shape
  non-applicabilities, six selected-surface absences, and one Gemini ACP
  candidate.
- Research 059 freezes the exact Gemini CLI `0.51.0` contract and offline
  corpus. Existing shared contracts are sufficient.
- `GeminiSessionProfileInput::bounded_write` adds one explicit prepared
  `ReadWrite` profile. The existing constructor remains read-only.
- Capability, access policy, lease resolution, ACP negotiation, process mode,
  returned mode, callback dispatch, and cleanup now agree before effects.
- Unnegotiated write callbacks fail before host mutation.
- The package gate now includes the Gemini prepared facade from extracted
  crate artifacts.
- The matrix converts 24 cells to `Not applicable`, Gemini bounded write to
  `Yes`, and retains six honest `No` cells.

## Current State

- the 31-cell family closes at 24 `Not applicable`, one `Yes`, and six `No`
- the full matrix has 309 `No` and 143 `Not applicable` cells
- Gemini bounded write is a host-mediated UTF-8 create/replace callback
- Gemini remains `AmbientHost` with ambient configuration agreement
- no sandbox, containment, shell, provider-tool, approval, or fallback claim
  was added
- no release was published and the retained final candidate was not replaced

## Validation

- focused Gemini library, ACP, and prepared-facade tests: 29 passed
- negative unnegotiated-write fixture: passed
- `effigy qa`: passed in 456 seconds
- workspace examples: passed
- route and docs gates: passed
- public API baseline: intentionally refreshed for the new constructor
- final `effigy package:check`: passed in 135 seconds from the strengthened
  script state, including extracted Gemini prepared-facade tests
- retained-candidate verification was not run because the cumulative
  worktree is intentionally dirty; its clean-source gate rejected before
  packaging and no candidate evidence changed

## Remaining Risks

- the Gemini profile's callback is bounded; the ambient harness process is
  not contained
- Gemini ACP support remains guaranteed only at exact `0.51.0`
- Qwen, Claude Agent, Claude Code, Pi, Kimi local server, and OpenCode retain
  honest bounded-write `No` cells on their selected surfaces

## Next

Card 111 audits the 40 owned-runtime-lifecycle and
planned-connection-rollover `No` cells. Cards 112-114 remain in bounds.
