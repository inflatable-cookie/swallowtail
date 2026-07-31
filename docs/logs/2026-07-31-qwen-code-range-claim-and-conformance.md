# 2026-07-31 Qwen Code Range Claim And Conformance

## Changed

- extended the maintained Qwen package window through `0.21.2`
- retained baseline behavior through `0.20.1`
- added the `0.21.0` catalogue-filter behavior segment
- replaced the runtime pinned-version check with the exact preflight-bound
  executable version for structured and restarted-session turns
- added latest-boundary runtime and later-unverified coverage

## Current State

All 35 focused Qwen tests pass. The extracted 60-file package compiles. Exact
`0.19.11..=0.20.1` and `0.21.0..=0.21.2` stable points are maintained;
`0.20.2` remains an unsupported unpublished gap and later stable points remain
visible unverified newer.

## Next

Execute card 023. Prove installed `0.21.2`, gate authenticated catalogue and
read-only prompt evidence separately, reconcile public route truth, and close
roadmap g03.009.
