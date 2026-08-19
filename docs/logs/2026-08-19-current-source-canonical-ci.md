# Current Source Canonical CI

Date: 2026-08-19
Roadmap: `../roadmaps/g04/003-current-source-tag-before-readiness.md`
Card: `../roadmaps/g04/batch-cards/008-current-source-canonical-ci.md`

## Result

The accepted `v0.3.3` candidate is on canonical `main`. Local `HEAD`, remote
`main`, and the dispatched `CI` workflow head resolve to
`51d186208e75dca4c04f077dd7179ec3c2fafae9`. All five jobs passed: stable,
Rust `1.95.0` floor, documentation and semantic API, supply-chain, and the
external Git-source consumer.

`CI` triggers automatically only for `v*` tags. The pre-tag run used
`workflow_dispatch` against `main`. Run
https://github.com/inflatable-cookie/swallowtail/actions/runs/32308431817

PR 3 merged by fast-forward. No annotated tag exists.

## Authority

No local or remote `v0.3.3` tag, GitHub Release, registry publication,
consumer mutation, or provider call exists. The candidate is not a supported
release identity until card 009 receives separate exact authorization.

A later closeout commit on `main` is not the tag identity. Tag `v0.3.3` at
`51d186208e75dca4c04f077dd7179ec3c2fafae9`.

## Next

If authorized, create one annotated `v0.3.3` tag at that exact CI-green
commit, push it, and require the tag-triggered workflow to pass at the same
commit.
