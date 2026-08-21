# 2026-08-22 Gemini CLI 0.56.0 Claim

## Result

Card 094 raised both separate Gemini CLI qualified ceilings through official
`0.56.0`:

- ACP: `0.51.0..=0.56.0` on `gemini-cli.acp-agent`
- headless: `0.51.0..=0.56.0` on `gemini-cli.headless-stream-json`

Both retain their claim ids, baselines, behavior revisions, and
`AllowUnverified` posture. Published intermediates are qualified. The first
unpublished later stable, `0.56.1`, remains visible `UnverifiedNewer`.

The historical ACP activity and headless stream-json corpora remain in place.
Transcript management remains unsupported: cleanup makes one exact delete
attempt and performs no stateful list confirmation. Browser login,
individual-account access, Gemini Live, and Gemini Models remain outside this
claim.

## Validation

The card's focused, affected-package, route, Northstar, docs-index, and
roadmap next-action gates passed. `git diff --check` passed.

## Next

Compile the first numbered per-route feature milestone, starting with Cursor
headless model parameters.
