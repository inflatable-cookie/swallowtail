# 2026-08-28 g04.090b Kiro ACP Agent-Profile Evidence

Status: complete
Generation: g04
Card: 257
Research: 254
Branch: `t3code/review-agent-profile-evidence`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-cc2d9c73`
PR: [#110](https://github.com/inflatable-cookie/swallowtail/pull/110)

## Scope

Exact `kiro.acp` `2.18.1` `--agent` profile membership, authority, application,
confirmation, failure, lifecycle, and omission evidence only.

## Boundary

Evidence only. This lane owns card 257, Research 254, this log, and optional
new Kiro-local frozen evidence. Shared planning and production stay unchanged.

## Result

Honest empty deliver-now set. No Kiro ACP agent-profile row ships from this
lane.

Official ACP page documents `kiro-cli acp` and optional
`kiro-cli acp --agent my-agent`. The name is a placeholder, not closed
membership. Built-in slash ids and custom files under `.kiro/agents/` /
`~/.kiro/agents/` are interactive/ambient surfaces. Official troubleshooting
documents missing-agent fallback to the default without warning. Initialize
`agentInfo` is product identity `kiro-cli`, not applied-profile confirmation.
`session/set_mode` remains unmapped. Production argv remains `["acp"]`.

Exact `2.18.1` platform archives returned HTTP 403 on ranged GET; current
stable manifest tip is `2.20.1`. Package parser bytes were not frozen.
Chat `/agent`, ambient host profiles, and unsupported `session/set_mode`
were not promoted onto ACP. No host profile was created or mutated.

## Changed Surfaces

- `docs/research/254-kiro-acp-agent-profile-evidence.md` — promoted empty set
- `docs/roadmaps/g04/batch-cards/257-kiro-acp-agent-profile-evidence.md` —
  complete
- `crates/swallowtail-adapter-kiro/tests/fixtures/kiro-acp-2.18.1-agent-profile-evidence/`
  — sources + disposition

Unchanged: adapter production code, guide, identity corpus, shared milestone,
inventory, programme, triage, matrices, indexes, Next Task.

## Validation

Passed:

- `effigy validate:focused swallowtail-adapter-kiro` (30 tests)
- `effigy qa:northstar`
- `git diff --check`

Inherited doctor baseline: `scan.god-files` 380 findings (334 warnings, 46
errors); generated-in-src warning; graph index stale.

## Review Fix

Card 257 acceptance no longer checks the positive fail-closed claim. It
records the pre-prompt reject gate as failed (official silent default
fallback), matching Research 254's empty-set disposition.

## Unresolved

Reopen only with recoverable qualified-version package/source parser, closed
portable membership without ambient host files, fail-closed invalid-name
behavior on ACP, and confirmable applied-profile return that is not generic
`agentInfo`. No production binding from this PR. Merge not authorised.
