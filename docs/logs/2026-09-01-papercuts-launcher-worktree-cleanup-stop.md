# Papercuts launcher worktree-cleanup ownership stop

Date: 2026-09-01
Branch: `worker/papercuts-launcher-worktree-cleanup`
Base: `d8e5b49c6effc71ed8f459c02b6c18661cc1c1a0`
Host Paseo: `0.6.1`
Papercut: Launcher cleanup leaves stale Git worktree registrations
  (2026-08-26)

## Outcome

Evidence-backed stop. No honest Swallowtail code, `paseo.json`, script, or
agent-direct cleanup can close the papercut. Entry stays open.

## Ownership

| Claimed fix piece | Owner | Swallowtail lever |
| --- | --- | --- |
| Create review worktree | T3 / Paseo launcher (`git worktree add`) | none |
| Remove owned directory after merge/archive | T3 / Paseo `deletePaseoWorktree` / thread archive | none |
| Deregister Git metadata | `git worktree remove` then `git worktree prune` in the launcher, against the source checkout | none; repo-wide prune is forbidden here |
| `paseo.json` teardown | Swallowtail hook, invoked **before** directory removal | Effigy `deps unlink` only |

Swallowtail surfaces checked and empty for Git worktree add/remove/prune:

- `crates/`, `scripts/`, `effigy.toml`: no `git worktree` invocation
- `paseo.json` `worktree.teardown`: Northstar `paseo:worktree -- unlink` (Effigy
  dependency-link ledger only; see `paseo-worktree.rhai`)
- Architecture authority map: Swallowtail does not own launcher lifecycle

Paseo public source at `860fcb2e` already sequences `git worktree remove
--force`, recursive directory delete, then `git worktree prune` inside
`deletePaseoWorktree`. Docs state teardown runs during archive before the
directory is removed. That is the owning repair path. This lane did not patch
Paseo or T3.

## Frozen PR 67 evidence

g04.067 closeout recorded worktree
`/Users/tom/.t3/worktrees/swallowtail/t3code-cfab66d3` for
[PR 67](https://github.com/inflatable-cookie/swallowtail/pull/67). On this
host that directory is absent and Swallowtail `.git/worktrees/` has no
`cfab66d3` admin dir. Read-only `git worktree list` on the primary checkout
showed nine live trees, all present; this lane did not prune or remove any of
them.

Absence of the original stale row is not load-bearing proof that current
T3 Nightly / auto-archive-after-merge always deregisters. The entry stays
open.

## Throwaway reproduction (owned fixture only)

Isolated repo under `/tmp/swallowtail-papercut-wt-*`, then deleted:

1. `git worktree add` a linked tree on a throwaway branch.
2. `rm -rf` that linked directory **without** `git worktree remove`.
3. `git worktree list --porcelain` still named the path and branch, with
   `prunable gitdir file points to non-existent location`.
4. `git -C <removed-path> status` exited 128:
   `fatal: cannot change to '…/linked': No such file or directory`.
5. `git worktree prune` **in that throwaway primary only** dropped the stale
   row. The throwaway tree was then removed.

Falsification: directory delete alone leaves the registration; prune (or
`git worktree remove` before delete) is required. `paseo.json` teardown cannot
supply that prune for the tree being deleted, because the directory still
exists during teardown. Adding a repo-wide prune there would also touch
unrelated missing worktrees.

No Swallowtail checkout was pruned. No provider contact.

## Residuals

- Papercut remains open until T3/Paseo launcher cleanup is proved to
  deregister owned worktrees on the merge/archive path Swallowtail actually
  uses (including T3 Nightly `.t3/worktrees/` if that launcher still ships).
- Actionable handoff: keep `deletePaseoWorktree` (remove → delete → prune) on
  every auto-archive-after-merge and T3 thread-delete path; do not rm an owned
  worktree directory without Git deregistration.
- Next open Swallowtail papercut after this one:
  Antigravity invalid-`--agent` probes crossed card 161's no-prompt boundary
  (2026-08-24). The Effigy graph-explore timeout entry stays an open Effigy
  handoff.

## Validation

- Docs-only stop record: `effigy qa:docs:index:logs` and `effigy qa:docs:links`
  after the log and PAPERCUTS serial edits.
