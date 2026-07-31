# 2026-07-31 Claude Agent 0.64 Range Acceptance

## Changed

- superseded the coupled Claude/Gemini roadmap
- moved future Gemini CLI range qualification to deferred backlog while
  retaining existing production support
- froze exact Claude Agent package and selected-source evidence for
  `0.61.0..=0.64.0`
- extended the maintained ACP range through exact `0.64.0`
- added private milestones for `0.63.0` tool/subagent correlation and `0.64.0`
  host-steering/form metadata without selecting either optional capability
- retained separate local-subscription and public-API-key access profiles

## Evidence

The managed wrapper reports exact `0.63.0`. The separately inspected signed
release and npm artifact identify exact `0.64.0`. Focused Claude/ACP validation
passed 157 tests. Both affected packages compiled from extracted archives.
Route, docs, Northstar, and diff checks passed the final acceptance gate.

No provider prompt, authentication change, session mutation, consumer edit, or
publication ran.

## Next

Return to the g03 compatibility-maintenance checkpoint. Gemini CLI range
requalification remains deferred until its explicit backlog gate is met.
