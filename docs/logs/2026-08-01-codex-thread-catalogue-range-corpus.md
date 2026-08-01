# 2026-08-01 Codex Thread Catalogue Range Corpus

## Result

Card 052 is complete. Exact tagged source selects Codex CLI `0.105.0` as the
first release where cursor listing, explicit source and cwd filtering,
history-bearing read, runtime status, and resume coexist.

## Range Truth

Catalogue/import is evidence-ready for `0.105.0..=0.107.0` and
`0.110.0..=0.146.0`, split at existing `excludeTurns` and workspace-root
behavior milestones. The existing `0.108.0..=0.109.0` gap remains excluded.
Later versions remain visible as unverified newer.

Qualified `0.80.0..=0.104.0` points keep their existing app-server support but
do not advertise catalogue/import. The operation floor does not become a new
general Codex baseline.

## Selected Profile

The frozen profile lists non-archived `cli`, `vscode`, and `appServer` threads
under one exact cwd, follows only opaque bounded cursors, and revalidates one
selection with `thread/read` plus `includeTurns: true`. Provider paths, Git
metadata, ancestry, pin state, one-shot exec sessions, and subagent sessions
remain outside portable candidate content.

Current app-server documentation corroborates feasibility only. Every
historical boundary uses an exact NPM publication and peeled upstream tag
commit in the deterministic corpus.

## Validation

- `effigy validate:focused swallowtail-adapter-codex` passed 149 tests
- no production capability, live provider, broad workspace, package, or
  consumer validation changed

## Next

Execute card 053. Implement the corpus-qualified Codex catalogue/import driver
and prepared facade mapping without widening legacy support.
