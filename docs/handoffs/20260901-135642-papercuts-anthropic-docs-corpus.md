---
title: Anthropic platform docs corpus papercut worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: research
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-01
updated: 2026-09-01
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260901-135642-papercuts-anthropic-docs-corpus.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, papercuts, research]
---

## Objective

Close the Anthropic platform-docs corpus papercut by preserving Research 209's
historical HTML/redirect evidence and adding deterministic Markdown corpus
identities for every official source. Do not change the adaptive-thinking
capability conclusion.

## State And Scope

- **Repository:** `/Users/tom/Dev/projects/swallowtail`
- **Required ancestor:** `803f5e7e1e62764e4097853fd65fcbb83af51808`
- **Worker branch:** `worker/papercuts-anthropic-docs-corpus`
- **Worker worktree:** Paseo-managed from pushed `origin/main`, titled with the
  capitalized `Papercuts` label.
- **Worker class:** mechanical, token-heavy evidence reconciliation. The nine
  source rows and stop oracle are fixed; no product design is delegated.
- **Authority:** the exact PAPERCUTS entry, Research 209, the nine official
  Anthropic source URLs it freezes, current official `.md` exports, and this
  handoff.
- **Observed lead:** appending `.md` to the thinking page returns HTTP 200,
  `content-type: text/markdown`, with a rewrite to
  `/docs/as-markdown/en/build-with-claude/thinking`.
- **In scope:** `PAPERCUTS.md` and Research 209 only.
- **Out of scope:** new Anthropic capability/currentness research, runtime or
  fixture changes, claims, matrices, contracts, roadmaps, other research,
  other SPA papercuts, provider calls, or any log/index edit.
- **Parallel partition:** the Codex currentness lane owns Codex surfaces and
  shared research/log indexes. This lane owns only the two paths above.
- **Serial edge:** no later papercut starts before this lane merges or stops.

## Acceptance And Review Oracle

For each of Research 209's nine official source rows, retrieve the historical
HTML URL and its `.md` form without authentication. Record retrieval time,
requested URL, effective URL, HTTP status, body kind, byte count, and SHA-256.
Follow redirects for corpus identity, but retain the original response-hop
status/effective target so `adaptive-thinking` and any other redirect cannot
masquerade as a distinct binding page.

Preserve the 2026-08-25 HTML hashes exactly. They remain historical response
bodies, not converted text. Add the smallest clear Research 209 corpus section
or table that binds the 2026-09-01 Markdown bodies and explains which rows
resolve to the same effective page. Do not call moving Markdown immutable or
use it to widen the delivered row.

Reconcile the Markdown text against every decisive Research 209 statement:
adaptive/disabled/enabled shape, Opus 4.7 applicability and default-off rule,
explicit omitted display, streaming sequence, tool-result replay, effort
independence, model identity, and steering. Close only when the deterministic
Markdown corpus supports the existing conclusion. Stop and report if any
export disappears, a body is not Markdown, redirects are ambiguous, the
corpus contradicts a decisive statement, or a capability/fixture/claim change
would be needed.

## Validation And Completion

Confirm a clean non-`main` worktree, `HEAD == origin/main`, and this handoff
from `HEAD`. Use public official documentation only. No account, credential,
catalogue call, prompt, provider request, package download, or implementation
command.

Run `effigy qa:docs:links`, `effigy qa:docs:index:research`,
`effigy qa:northstar`, and `git diff --check`. Close the exact PAPERCUTS entry
with the source count, corpus method, and conclusion-preservation result. Do
not add a log or touch an index.

Commit the bounded correction, push, and open one PR. Report exact head/base,
all nine requested/effective source identities, redirect equivalences,
changed files, disposition, validation, and PR URL. Do not merge.
