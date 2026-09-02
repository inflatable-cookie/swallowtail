# 2026-09-02 g05.021 v0.4.0 Release Readiness Compiled

## Result

g05.021 is the bounded `v0.4.0` release-readiness milestone. Contract 036
requires a minor because current source removed the previously guaranteed
OpenAI Background `minimal` reasoning value. The runway stays inside g05 and
uses cards 050-052: exhaustive compatibility/freeze audit, coordinated
candidate plus 11 local gates and exact-SHA CI, then current source-consumer and
one operator-selected working-application proof.

Card 050 is ready. Cards 051-052 are planned and serial. Card 052 also waits on
the operator to name the application, route, command, candidate-consumption
shape, credentials/provider authority, mutation allowance, success evidence,
and cleanup. No application is inferred from prior consumer work.

## Boundary

The milestone plans one source-only annotated Git tag on canonical GitHub. No
card authorizes tag creation or push. The final card must stop at the exact
candidate SHA and request explicit authority for source commit, branch, remote,
tag name, tag message, creation, and push while confirming no crates.io or
GitHub Release, binary, sidecar, or installer.

The feature/currentness freeze keeps Kimi local server `0.40.1` and closed PR
182 parked post-release. Gemini, watcher live readiness, bounded skill-
inventory implementation, Kimi ACP above its A2 `0.38.0` cap, and g05.009/card
034 remain deferred. Contract 061 projection stays 249/767 proved with 518
rows remaining. Non-gating papercuts stay outside the milestone.

## Evidence Context

At compilation, `v0.3.3..main` is 773 commits and 2,405 changed files. Current
source remains 40 packages and has 48 routes versus the immutable tag's 47;
`pi.sdk-sidecar` is additive. The current exploratory format, two-Clippy-shape,
and 2,825-test pass stopped at a checkpoint-held `qa:docs` lock. It is not
release evidence; card 051 reruns all 11 gates on the frozen candidate.

## Next

Execute card 050. Do not change versions, requirements, changelog release
state, candidate baselines, code, claims, or fixtures in that audit card.
