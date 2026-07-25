# Second Installed Harness Range Selection

Date: 2026-07-23
Card: `../roadmaps/g01/batch-cards/120-second-installed-harness-range-selection.md`

## Outcome

OpenCode HTTP is selected for the second installed-harness compatibility
retrofit. The candidate envelope is `1.14.48..=1.18.4`.

The baseline is the existing frozen and locally observed server release. The
latest boundary is current stable. This preserves the current support floor
and creates no open-ended latest claim.

## Evidence

- OpenCode has 45 maintained stable releases in the candidate envelope
- all six selected operation ids and paths remain present at every point
- exact server release is exposed by `GET /global/health`
- full generated schemas change throughout the envelope
- card 121 later corrects the first-pass projection to 18 recursively closed
  selected surfaces and 20 contiguous published spans
- the full schema changes at `1.18.1` / `1.18.2`, but the selected closure
  remains stable

Gemini `0.52.0`, Kimi Code `0.29.0`, Qwen Code `0.20.1`, and Pi `0.81.1`
remain outside their current exact-only or one-point claims. Kimi is the
leading later ACP range because `0.29.0` changes thinking capability
negotiation.

## Contract Posture

Contract 029 already governs exact bindings, closed ranges, milestones,
exclusions, and fail-closed behavior. Contract 032 remains specific to
host-approved executables. OpenCode observes an attached server through an
approved endpoint, so no new shared contract is required.

Provider authentication, entitlement, model availability, catalogue
freshness, endpoint authority, and external-server ownership remain separate
from compatibility.

## Continuation

- card 121 is complete: all 45 releases, 18 surfaces, 20 spans, and safe health
  observation are frozen
- card 122 remains in bounds: publish the claim and add private exact-version
  dispatch
- card 123 remains in bounds: run cross-topology range conformance and close
  roadmap 040
- no production support range is published yet

## Sources

- [Research 027](../research/027-second-installed-harness-range-selection.md)
- [OpenCode releases](https://github.com/anomalyco/opencode/releases)
- [OpenCode server documentation](https://opencode.ai/docs/server/)
- [Kimi Code `0.29.0`](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%400.29.0)
