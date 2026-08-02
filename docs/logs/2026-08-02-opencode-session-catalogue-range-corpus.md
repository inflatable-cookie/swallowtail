# OpenCode Session Catalogue Range Corpus

Date: 2026-08-02
Roadmap: g03.022
Card: 058

## Evidence

All 51 exact qualified OpenCode releases from `1.14.48` through `1.18.10`
contain `session.list`, `session.status`, `session.get`, `session.messages`, and
`session.prompt_async`. Recursive local-reference closure produces seven exact
surface digests and the same 12 published-version segments as existing
continuity. Semantic gaps and `1.18.11+` remain unavailable for import.

The new deterministic corpus freezes exact directory and attached-endpoint
binding, Basic-auth lease authority, positive offset pagination, root versus
child sessions, idle/busy/retry status, exact lookup, ordered bounded history,
and stale or malformed no-binding outcomes. Child import, foreign-directory
import, project/account scans, and server ownership remain excluded.

## Validation

- `effigy validate:focused swallowtail-adapter-opencode` passed 86 tests
- `git diff --check` passed
- no attached server, authentication, live prompt, consumer edit, or broad
  suite

## Next

Card 059 implements the selected HTTP catalogue/import route and reuses the
existing load/replay/resume path after binding issuance.
