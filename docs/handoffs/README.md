# Handoffs

Dispatch and continuation artifacts. Each file is one seven-section Northstar
handoff. A worker lane is active only when the coordinator dispatches a
handoff whose frontmatter declares `handoff_mode: worker-pr-loop`,
`worker_mode: implementation`, and `dispatch_authority: orchestrator`; the
absolute path of that file is the only dispatch artifact. An
`orchestrator-continuation` handoff starts a successor coordinator.

Handoffs are evidence of how a lane was launched. They do not replace roadmap
tasks, dispatch manifests, closeout logs, or the Next Task pointer.

A handoff must not grant a worker write access to a task card's pinned prose,
including a result or evidence section, and must not carry a phrase such as
"this task's result lines" among the worker-owned paths. The only machine-owned
region of a task file is its generated lifecycle block, and the lifecycle hook
owns it. Worker evidence goes in the worker report and in the lane's designated
evidence files.
