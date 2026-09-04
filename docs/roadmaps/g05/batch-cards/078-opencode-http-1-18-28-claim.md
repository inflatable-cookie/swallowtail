# 078 OpenCode HTTP 1.18.28 Claim

Status: ready; serially unblocked after card 077 admitted compatible `surface-19`
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../028-opencode-http-1-18-28-useful-newer.md`
Depends on: completed card 077 with an admitted segment

## Goal

Apply only the OpenCode HTTP compatibility segment proved by card 077, update
matching tests and canonical route truth, validate, and stop for exact-head
review.

## Scope

1. Extend only the claim axis and operation-specific windows admitted by card
   077. Preserve baseline `1.14.48`, existing claim IDs, historical segment
   boundaries, gaps, behavior revisions unless the evidence requires a private
   milestone, and `AllowUnverified`.
2. Set `OPENCODE_LATEST_QUALIFIED_VERSION` only to the admitted ceiling and
   test every new published hop plus the first later unverified point.
3. Update exact claim fixtures, route/feature matrices, prepared integration
   guide, architecture ceilings, `CHANGELOG.md` `[Unreleased]`, standing-lane
   truth, and one claim log.
4. Keep Contract 061 projection, web-search work, provider behavior, public API,
   and unrelated families unchanged.
5. Re-probe official latest immediately before push and apply Contract 029's
   post-identity rule without reopening the frozen claim segment.

## Out Of Scope

Another family; new route operations; provider contact; host update; Contract
061 Candidate L; Gemini; release work.

## Acceptance Criteria

- only card 077's admitted segment moves
- every historical boundary and negative point survives
- selection, fixtures, guides, matrices, architecture, changelog, and standing
  lane agree
- public API change is zero

## Validation

- `cargo fmt -p swallowtail-adapter-opencode -- --check`
- `effigy validate:focused swallowtail-adapter-opencode`
- `effigy package:verify-affected swallowtail-adapter-opencode`
- `effigy qa:routes`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g05`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:roadmaps:numbers`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`

## Auto-Continuation

No. Stop for exact-head review.
