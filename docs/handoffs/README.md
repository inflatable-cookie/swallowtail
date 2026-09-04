# Handoffs

Dispatch and continuation artifacts. Each file is one seven-section Northstar
handoff. A worker lane is active only when the coordinator dispatches a
handoff whose frontmatter declares `handoff_mode: worker-pr-loop`,
`worker_mode: implementation`, and `dispatch_authority: orchestrator`; the
absolute path of that file is the only dispatch artifact. An
`orchestrator-continuation` handoff starts a successor coordinator.

Handoffs are evidence of how a lane was launched. They do not replace roadmap
cards, dispatch manifests, closeout logs, or the Next Task pointer.
