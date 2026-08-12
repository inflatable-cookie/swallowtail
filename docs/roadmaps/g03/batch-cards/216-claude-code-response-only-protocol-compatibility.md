# 216 Claude Code Response-Only Protocol Compatibility

Status: completed
Owner: Tom
Created: 2026-08-12
Milestone: `../068-claude-code-response-only-protocol-compatibility.md`

## Goal

Replace patch-version equality with a stable response-only protocol identity
whose safety comes from exact preparation binding and fail-closed runtime
validation.

## Acceptance

- [x] `2.1.227` remains the minimum qualified baseline and `2.1.228` remains a
      qualified live-evidenced point
- [x] stable newer releases are permitted only as unverified newer; below-
      baseline, prerelease, build-qualified, malformed, and denied versions fail
- [x] a static route deny-list feeds the compatibility claim exclusions
- [x] init must report the exact executable version frozen by preparation
- [x] arguments, empty tools/MCP, init, thinking, cumulative estimates,
      assistant, terminal, ordering, and exactly-one-text invariants fail closed
- [x] preparation and run-start debug observations expose the exact version and
      compatibility posture
- [x] `2.1.227`, `2.1.228`, provisional-newer, deny-list, and protocol-drift
      fixtures pass deterministic validation
- [x] the existing prepared Figmatic API and command shape remain unchanged
- [x] focused, affected-package, guide, route, docs, and gated `2.1.228`
      Max/OAuth validation pass without `ANTHROPIC_API_KEY`

## Evidence

- implementation commit: `IMPLEMENTATION_COMMIT_PENDING`
- focused validation: 166 tests passed across the adapter and testkit
- affected-package archives, dependency closures, and extracted compilation
  passed for both packages
- guide coverage passed for 36 routes, 35 examples, and 44 portable features
- route, lifecycle, feature, and 70-operation activity matrices passed
- the full docs selector passed
- live `2.1.228` Max/OAuth probe passed with `ANTHROPIC_API_KEY` removed in
  19.08 seconds

## Stop Conditions

- stop on a required prepared-API change, weakened stream invariant, visible
  private thought, non-empty tools or MCP, version mismatch tolerance, silent
  qualified-range widening, or API-key dependence
