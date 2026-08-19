# g04.005 Kernel Worker Dispatch

Date: 2026-08-20
Roadmap: `../roadmaps/g04/005-connection-lifecycle-kernel.md`
Handoff: `../handoffs/20260820-000805-g04-005-connection-lifecycle-kernel.md`

## Result

One serial worker lane is dispatched for the connection-lifecycle kernel.
Cards 013-015 are the assigned runway. Planning base is
`2ca191252a275dee177da54b4a88454c39facf61`. Catalog, admission, and sign-in
stay planned.

`.agents.local.env` is absent on the planning machine. The worker should use
the launcher-provided worktree, or ask for `AGENTS_WORKTREE_CONTAINER_DIR`
if it has to create a fallback.

## Next

Operator starts one worker thread with only:

```text
Read and follow `docs/handoffs/20260820-000805-g04-005-connection-lifecycle-kernel.md`.
```
