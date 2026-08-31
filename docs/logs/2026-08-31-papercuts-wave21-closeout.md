# Papercuts wave 21 closeout

Date: 2026-08-31
Handoff: `docs/handoffs/20260830-223400-papercuts-wave21-skill-closeout.md`
PR: [#132](https://github.com/inflatable-cookie/swallowtail/pull/132), merged
as `811db499c2b59e42f1a290923b64ceac1468b237`

## Outcome

- Closed `Effigy validation materializes an untracked repo skill` in
  `PAPERCUTS.md`.
- No code or ignore-rule change. The committed 11-file
  `.agents/skills/effigy/` tree remains managed by PR 125; Effigy
  `f3057b9bb554f1a54b4c2d4cab2df27d5f6da202` supplies the sync behavior.
- Review found no blocking issue. The operator authorised the merge.

## Validation

- Worker proof: `effigy v0.12.1+local.f3057b9` → `effigy qa:docs`, exit 0.
- `git status --porcelain` stayed empty; no untracked
  `.agents/skills/effigy/` paths appeared.
- PR 132 CI passed: dependency security, external Git-source consumer,
  documentation/semantic API, pinned MSRV, and stable format/lint/test/guides.

## Scope and next

- Live-probe cleanup, scoped-task execution, and the other listed papercuts
  remain open.
- `docs/roadmaps/README.md` remains unchanged; continue its existing planning
  `Next Task`.
