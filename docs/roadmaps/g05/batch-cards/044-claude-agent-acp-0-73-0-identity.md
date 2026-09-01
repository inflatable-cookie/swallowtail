# 044 Claude Agent ACP 0.73.0 Identity

Status: completed
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../018-claude-agent-acp-0-73-0-useful-newer.md`
Depends on: Contract 029; Research 271; operator restart; official stable `0.73.0`

## Goal

Freeze exact official Claude Agent ACP `0.73.0` identity and classify its
selected mapped ACP surfaces without changing a claim.

## Scope

1. Recheck npm, GitHub tag/commit, tarball, extracted dist, and selected
   git blobs for `0.70.0`, `0.71.0`, `0.72.0`, and `0.73.0`. Do not infer
   identity from registry `latest` alone.
2. Keep host `0.63.0` observation-only. Do not install or update it.
3. Compare only the selected mapped ACP subset with frozen `0.70.0`, plus
   the complete `0.72.0`→`0.73.0` dist inventory.
4. Classify changelog extras as unmapped unless they change a selected
   mapped route. Keep the repaired `0.71.0`/`0.72.0` ledger as intermediate
   evidence, not a standalone ceiling.
5. Retarget Research 272 and one secret-free `0.73.0` identity/protocol
   corpus. Record the operator restart and `0.73.0` publication timing.
6. Commit identity evidence before any selection, matrix, guide, changelog,
   or standing-lane claim edit.
7. Record compatible extension, private milestone, new revision, or stop.

## Out Of Scope

Production claim edits, Claude Code, watcher, Gemini, another family,
provider contact, login, install, host update, live probe, projection,
skill, papercut, g05.009 card 034, release, or execution of downloaded
official binaries.

## Acceptance Criteria

- official identity is corroborated through independent official channels
- mapped and material unmapped additions are explicit
- current production claims are byte-for-byte unchanged in this commit
- fixture provenance, digests, and negative boundaries are load-bearing
- Claude Code and the watcher are not raised
- card 045 continues only for an admitted segment

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, to card 045 only after an admitted segment is recorded.

## Result

Operator restart after official latest moved to `0.73.0` during the
unmerged `0.72.0` family. Official stable is exact `0.73.0` published
2026-09-01T20:27:53.428Z; GitHub target
`ea7076c0bc324603e65d8c124b7573f158749969`. Host `0.63.0` matches the
frozen `0.70.0` host digest. Published intermediates after `0.70.0` are
exactly `0.71.0`, `0.72.0`, and `0.73.0`. Research 272 and the frozen
corpus, including the complete dist inventory and exact 0.71/0.72/0.73
delta classification, land in identity-only commit `af9ddfd4`; production
claims remain `0.70.0` in that commit. Mapped `dist/index.js`, `dist/elicitation.js`,
`dist/lib.js`, `dist/settings.js`, and `dist/utils.js` are byte-identical
to `0.70.0`. Every `dist/**` file is byte-identical `0.72.0` to `0.73.0`;
the only package change is `package.json` version plus Agent SDK pin
`0.3.252`→`0.3.257`. `#1004`/`#1045` and `0.72.0` effort / result
attribution / model-switch hooks stay classified as intermediate evidence.
Five new emitted update kinds stay unmapped. Exclusion `0.58.0` stays.
Classification: compatible extension of
`claude-agent.acp.initialize-meta-extensions-v7`, so card 045 continues.
