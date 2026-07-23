# 2026-07-23 Installed Harness Compatibility And Codex Range Selection

## Changed

- revalidated the installed Codex, OpenCode, Gemini, Kimi Code, Qwen Code, and
  Pi release points
- corrected Codex latest stable from `0.144.6` to `0.145.0`
- recorded current upstream movement beyond every non-Codex adapter pin
- separated configured-only, post-effect runtime, and absent version
  observation paths
- selected Codex exec and app-server for the first installed-harness retrofit
- compiled cards 111-115 for observation records, corpora, production claims,
  conformance, and the older six-month span

## Current State

No Codex compatibility range is published yet.

The first exec candidate is `0.122.0..=0.145.0`. `0.122.0` introduced the
`--ignore-user-config` and `--ignore-rules` flags used by Swallowtail's current
isolated invocation. `0.121.0` is the lower rejection neighbor.

The first app-server candidate is v2 over `0.110.0..=0.145.0`.
`0.110.0` is the first published v2-only point. `0.131.0` begins the
runtime-workspace-root behavior segment. Version-specific stable and
experimental schemas must pass before the claim is published.

The app-server audit found that existing mocks do not enforce
`experimentalApi`. Production code can emit experimental runtime workspace
roots without enabling the capability and emits a later default-false
experimental model-fallback field. Card 112 must freeze a gate-enforcing
corpus; card 113 owns the version-aware repair.

## Discovery Boundary

Codex and Pi lack a production exact-version observation before execution.
Existing `DiscoveryRequest` and `DiscoveryOutcome` cannot express one explicit
host-approved executable candidate, deadline, exact interface binding, or
compatibility classification.

Card 111 will promote that narrow boundary. It grants no ambient executable
search, installation, update, login, configured-instance promotion, or raw
version output.

## Continuation

- card 111 is ready
- cards 112-114 remain in the first Codex retrofit tranche
- card 115 keeps exec `0.80.0..=0.121.0` and pre-v2 app-server feasibility in
  bounds
- OpenCode `1.18.4`, Gemini `0.52.0`, Kimi Code `0.29.0`, Qwen `0.20.1`, and
  Pi `0.81.1` remain unqualified

## Evidence

- [Research 025](../research/025-installed-harness-compatibility-and-codex-range-selection.md)
- [Codex CLI reference](https://developers.openai.com/codex/cli/reference/)
- [Codex app-server protocol](https://developers.openai.com/codex/app-server/)
- [Codex `0.145.0`](https://github.com/openai/codex/releases/tag/rust-v0.145.0)
