# Kimi Negotiated Reasoning Records And Corpus

Date: 2026-07-24
Card: `../roadmaps/g01/batch-cards/129-negotiated-session-option-records-and-kimi-range-corpus.md`

## Outcome

Contract 034 is realized as typed runtime records.

`NegotiatedReasoningSetup` binds one portable `ReasoningMode` to an exact
interactive-harness preflight requirement and new-session lifecycle.
`EffectiveReasoningSetup` exists only after the harness confirms the same
portable value. Missing or ambiguous capability constraints, request-plan
drift, load or resume mutation, and effective-value drift reject explicitly.

No string configuration bag was added. Provider option ids, categories,
labels, ordering, values, and raw snapshots remain Kimi-private evidence.

## Frozen Kimi Evidence

Two behavior milestones are independent exact points:

- `0.28.1`: annotated tag
  `0032545b65f95c139ecba5a48ba1b911844e1ffe`, peeled commit
  `efacf0452d46f5dbd67499eabc053869495d5213`, ACP adapter `0.3.4`,
  legacy `off`/`on`
- `0.29.0`: annotated tag
  `03c34eefa49513e6216390a9773326077a37f414`, peeled commit
  `8bf5bacba9e524c38fb808c0122070037ead25a8`, ACP adapter `0.3.5`,
  declared effort levels plus legacy aliases

Both lock ACP SDK `0.23.0`, Zod `4.3.6`, and wire version 1. Source-file
digests bind the option builder, session application, server dispatch, and
lockfile evidence.

The corpus covers legacy, declared-effort, boolean-fallback, always-thinking,
missing, duplicate, unsupported, provider-rejected, missing-confirmation, and
effective-drift cases. Compatibility evidence uses two singleton segments.
`0.28.0`, `0.28.2`, prerelease, and malformed points reject. `0.30.0` is the
frozen unverified-newer example using the latest qualified private behavior.

## Boundaries

- no installation, authentication, provider request, container, or sandbox
- no continuous inferred version interval
- no load or resume reasoning mutation
- no fallback to `on`, a default, another model, route, driver, or provider
- existing Kimi new, load, resume, replay, write, cancellation, and cleanup
  fixtures remain unchanged

## Validation

- `cargo test -p swallowtail-core -p swallowtail-runtime -p swallowtail-testkit -p swallowtail-protocol-acp`
- `effigy check:rust`
- `effigy lint:rust`
- `git diff --check`

All passed. `effigy doctor` remains at the inherited 19 oversized-file
findings: 12 warnings and seven errors.

## Continuation

Card 130 is ready: add installed Kimi discovery, publish the exact two-segment
claim, select the private behavior revision, and dispatch one confirmed
new-session reasoning option.
