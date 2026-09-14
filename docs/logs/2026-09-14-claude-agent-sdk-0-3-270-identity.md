# 2026-09-14 Claude Agent SDK 0.3.270 Identity

## Result

Froze official npm `@anthropic-ai/claude-agent-sdk` `0.3.270`
(`latest`/`next`, published 2026-09-12T18:53:13.002Z) against the `0.3.259`
claim. All nine published hops after the ceiling were retrieved, hashed, and
classified without executing anything: `0.3.260`, `0.3.261`, `0.3.263`, and
`0.3.265..=0.3.270`; `0.3.262` and `0.3.264` are unpublished gaps. Every
wrapper `0.3.X` carries its coupled native `2.1.X`; the `0.3.259` digests
reproduce Research 280 exactly.

All ten points ship the same 15 files with zero additions or removals; six
files are identical through every hop including the credential-bearing
`bridge.d.ts`. Three hops carry zero declaration changes (pure native
rotation and metadata). The `exports` map is identical on all ten points.
42 declaration surfaces are classified unmapped with the mapped
query/session/permission/model/tool-admission subset unchanged: the
`pluginDelivery` default stays `argv`, the ask-dialog hints never reach the
programmatic `canUseTool` path, the `systemPromptSnapshot` default prose
cannot move the route's constant digest-pinned prompt, the new controls and
additive result/usage/thinking/Settings fields sit on uncalled, unread, or
unloaded surfaces, and the two new assistant error strings never reach the
sidecar projection. Implementation probes hold: the `canUseTool` conflict
and stdio push, the single-object spawn hook, the discarded `waitForExit`
race with `unref()`'d `SIGKILL`, the stderr-drain remap, and the identical
bundled MCP literal (`2025-11-25` first, same five versions) on every hop.
Ten-pattern credential search returns the same three prose hits; 17 exported
functions, no login/OAuth; `harnessSchema` stays `1` on all ten points.

Host Node is `22.23.2` and installed Claude Code is `2.1.258`
(observation-only). Research 301 live evidence stays bound to
`0.3.259`/`2.1.259`; identical MCP constants on `0.3.270` are not live
evidence. Production claims stayed at `0.3.259`/`2.1.259` in this record.
Research 315. g05.065.

## Next

Apply the exact one-point rebind with the honest live-feature gating in the
g05.065 claim commit.
