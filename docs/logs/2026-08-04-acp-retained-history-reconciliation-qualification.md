# 2026-08-04 ACP Retained History Reconciliation Qualification

Roadmap: `../roadmaps/g03/031-acp-retained-history-reconciliation-qualification.md`
Card: 079

## Changed

- qualified Claude Agent ACP and Kimi ACP retained-history behavior against
  Contract 048
- recorded that stable ACP `session/load` restores resumable context, connects
  requested MCP servers, replays history, and returns a live session
- confirmed both production drivers preserve that load-and-attach shape
- kept ordinary load/replay support unchanged and added no reconciliation mapping
- compiled g03.032 across Gemini durable transcripts and Anthropic managed operations

## Evidence

- stable ACP v1 schema distinguishes stateful load from resume-without-replay
- Claude Agent continuity evidence through `0.61.0` and installed exact `0.63.0`
  source show `getOrCreateSession` before `getSessionMessages`; the qualified
  `0.62.0..=0.64.0` artifacts expose no separate history method
- Kimi ACP `0.28.1..=0.31.1` exposes metadata through `session/list`, history
  through `session/load`, and continuation without replay through `session/resume`
- Swallowtail's Claude and Kimi load drivers return `LoadedSession` with a live handle

No authenticated provider work, executable launch, prompt, or paid inference ran.

## Next

Execute card 080. Qualify Gemini CLI's durable transcript as an exact read-only
reconciliation candidate before considering runtime implementation.
