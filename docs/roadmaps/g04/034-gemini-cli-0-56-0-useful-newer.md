# g04.034 Gemini CLI 0.56.0 Useful Newer

Status: completed
Owner: Tom
Created: 2026-08-22

## Purpose

Qualify official npm `@google/gemini-cli` `0.56.0` (released
2026-08-19) on the separate `gemini-cli.acp-agent` and
`gemini-cli.headless-stream-json` axes.

This is one Contract 029 family run. It does not flatten ACP and headless,
does not reopen consumer-account login, and does not keep g04 open. The
selected access posture is an enterprise-owned Gemini API key supplied through
the existing provider-supported API-key profile. Code Assist browser login and
individual-account service stay outside both routes.

## Acceptance

Identity:

- Freeze host `0.53.0` and official npm/GitHub `0.56.0` evidence
- Compare every selected ACP and headless surface across published stable
  points from the existing ceilings through `0.56.0`
- Name the segment shape separately for ACP and headless
- Do not send a prompt, authenticate, install, update the host, or edit a
  production claim in the identity card

Claim:

- If both axes remain compatible extensions, raise ACP and headless
  latest-qualified ceilings to `0.56.0`
- Keep their separate axes, claim ids, baselines, behavior revisions, and
  `AllowUnverified` posture
- Update tests, Gemini guide, architecture ceiling statements, and both route
  matrix surfaces
- Leave Gemini Live and Gemini Models unchanged
- Pass the exact focused, affected-package, route, Northstar, and docs-index
  gates named by card 094

If either selected surface needs a provider prompt, live authentication, a new
public operation, or a materially changed lifecycle, stop before claim edits
and return for an explicit keep-or-remove decision on the Gemini CLI family.

## Out Of Scope

- Code Assist browser login or individual Google-account access
- Gemini Live, Gemini Models, or another route family
- Provider prompt, live catalogue, live session, install, or host update
- Mapping new flags, auth modes, nested fields, or provider features merely
  observed in `0.56.0`
- Gemini transcript import, history lookup, or management
- Per-route feature completion work
- Workspace `qa`, broad `qa:docs`, live probes, MSRV, or consumer checks

## Batch Cards

- [093-gemini-cli-0-56-0-identity.md](batch-cards/093-gemini-cli-0-56-0-identity.md) — completed
- [094-gemini-cli-0-56-0-claim.md](batch-cards/094-gemini-cli-0-56-0-claim.md) — completed

## Result

Official npm/GitHub Gemini CLI `0.56.0` is a compatible extension on both
selected axes. ACP and headless retain separate claims, baselines, behavior
revisions, and `AllowUnverified` posture; both are maintained through
`0.56.0`. Published intermediates are qualified and `0.56.1` remains visible
`UnverifiedNewer`. Transcript management, browser login, individual-account
access, Gemini Live, and Gemini Models remain outside the claim.

## References

- [Research 159 Post-Harness-Expansion Version Currentness Checkpoint](../../research/159-post-harness-expansion-version-currentness-checkpoint.md)
- [Research 182 Gemini CLI 0.56.0 Identity](../../research/182-gemini-cli-0-56-0-identity.md)
- [Research 045 Gemini CLI Headless Currentness And Corpus](../../research/045-gemini-cli-headless-currentness-and-corpus.md)
- [Research 059 Gemini ACP Bounded Write Contract Fit And Corpus](../../research/059-gemini-acp-bounded-write-contract-fit-and-corpus.md)
- [Contract 029 Interface Version Qualification And Compatibility](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Version Currentness Checkpoint](../../guides/version-currentness-checkpoint.md)
- [Gemini CLI Prepared Integration](../../guides/gemini-cli-prepared-integration.md)
- [Standing Lanes](../standing-lanes.md)
