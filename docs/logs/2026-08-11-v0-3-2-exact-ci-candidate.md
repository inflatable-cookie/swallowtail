# v0.3.2 Exact CI Candidate

Date: 2026-08-11
Roadmap: `../roadmaps/g03/067-v0-3-2-source-patch-release.md`
Card: `../roadmaps/g03/batch-cards/212-v0-3-2-exact-ci-candidate.md`

## Result

The accepted `v0.3.2` candidate is committed and pushed to canonical `main`.
Local `HEAD`, remote `main`, and the canonical workflow head resolve to the
same source identity. All five workflow jobs pass: stable, Rust `1.95.0`
floor, documentation and semantic API, supply-chain, and the external
Git-source consumer.

The `CI` workflow triggers automatically only for `v*` tags. The pre-tag run
used its existing `workflow_dispatch` entry point against canonical `main`; no
workflow file changed.

## Authority

No local or remote `v0.3.2` tag, GitHub Release, registry publication,
consumer mutation, or provider call exists. The candidate is not a supported
release identity until card 213 receives separate exact authorization.

## Next Move

If authorized, create one annotated `v0.3.2` tag at this exact CI-green commit,
push it, and require the tag-triggered workflow to pass at the same commit.
