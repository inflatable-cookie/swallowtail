# 2026-09-21 Claude Agent ACP 0.79.0 Claim

## Result

Raised `claude-agent.acp-adapter` through exact official `0.79.0`
(`0.66.0..=0.79.0`) as a compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7`. No new milestone.
Published hops `0.77.0`, `0.78.0`, and `0.79.0` are qualified. Unpublished
`0.58.0`, `0.73.1`, `0.74.1`, `0.75.2`, `0.76.1`, `0.77.1`, and `0.78.1`
stay incompatible; unpublished `0.80.0` is the synthetic later
`UnverifiedNewer` point. The ACP SDK pin stays `1.4.0`. `dist/index.js`,
`dist/settings.js`, `dist/utils.js`, and `dist/lib.js` are byte-identical
across every hop, so mode ids/categories, `plan`/`acceptEdits`, permission
option kinds, and the effort config id stay. Already-mapped form
elicitation accepts the two new Other description strings. The removed
`agent` config option, capability-gated `compaction_update`, Agent SDK pin
(`0.3.257` → `0.3.270` → `0.3.274`), AIR file-change / diffStats,
defaultToNo option order, and shell permission titles stay unmapped with
reasons. Baseline `0.53.0`, claim id `claude-agent.acp.window-2`, exclusion
`0.58.0`, every historical segment, and `AllowUnverified` survive. Claude
Code stream-JSON and the Claude Agent SDK sidecar are untouched. Research
335 froze the identity evidence in the same batch.

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check` passed
- `scripts/validate-focused-packages.sh swallowtail-adapter-claude-agent` passed: 542 tests and `-D warnings` clippy
- `scripts/verify-affected-packages.sh swallowtail-adapter-claude-agent` passed from a checkout whose path does not collide with the existing SDK fixture substring `rule/workspace types`; the `/workspace` checkout fails that host-path audit on pre-existing Claude Agent SDK text, not this ACP change
- official latest was rechecked immediately before push: npm `latest` `0.79.0`, GitHub `v0.79.0` target `d421f56a6c43cde16d9a7531d08a750a5ef2f04a`, ACP registry `claude-acp` `0.79.0`. No in-run movement

One material observation: the first focused nextest run failed three Claude Agent SDK sidecar asset tests with `node_runtime_unsupported` on host Node `22.14.0`. Those tests passed after switching to the CI pin `22.23.2`. Unrelated to this Claude Agent ACP claim.

No workspace `qa`. No provider prompt, live ACP initialize, authentication, install, host update, downloaded-artifact execution, release, tag, publication, or consumer mutation.

## Next

Family claim is done. No Next Task, roadmap, generation, or handoff
mutation.
