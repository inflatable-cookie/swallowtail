# Research 478: Goose ACP 1.59.0 Identity and Qualification

Status: candidate for independent review  
Date: 2026-11-12  
Route: `goose.acp`  
Axis: `goose.release`

## Finding

The official `aaif-goose/goose` GitHub stable channel was re-probed on
2026-11-12. Its latest stable is `v1.59.0`, published 2026-11-10. The
published hops after Swallowtail's `1.57.0` ceiling are `1.58.0`, `1.58.1`,
and `1.59.0`; the matching stable tag refs point to each release's recorded
source commit. The old `block/goose` repository identity resolves to
`aaif-goose/goose`. Exact release, source tree, and Darwin ARM64 asset
identities are frozen in the
[`goose-acp-1.59.0` fixture](../../crates/swallowtail-adapter-goose/tests/fixtures/goose-acp-1.59.0/README.md).

These points are a compatible extension of the selected ACP route. The
`1.58.0` state-machine service rehome preserves the selected method boundary.
The extracted compaction threshold preserves its prior calculations. Prompt
contributions retain their text and now sort deterministically by extension
name. In `1.59.0`, interrupted tool-call recovery returns the same error
response for every unanswered unique request: the prior implementation used
unique request IDs and an unanswered-response set; the new state ledger treats
every request without a `ToolResponse` as open and deduplicates request IDs.
Active advertised one-shot permission replies remain accepted. The additional
advertised-request validation rejects stale or unmatched history, which the
selected path does not submit.

## Identity and method

The fixture freezes exact GitHub release publication, stable tag commit, root
tree, Darwin ARM64 asset name, byte count, GitHub asset SHA-256, and complete
recursive tag-tree inventories for `1.53.0` through `1.59.0`. The earlier
`1.53.0` through `1.57.0` manifests reproduce the accepted 1.57.0 fixture
byte-for-byte. Each adjacent published hop has complete added, removed,
changed, and identical path sets. The selected closure follows the 1.58
`StateMachineServices` rehome and the 1.59 `goose-agent` tool-call and
permission implementation. Every changed selected path and residual path is
classified in `protocol.json`; the test reconstructs these sets from the
manifests and rejects missing, added, or reclassified paths.

The fixture preserves earlier selected paths including model configuration,
prompt management, reply parts, state-machine inference preparation,
compaction, and exit-on-error. It also includes the added agent and service
files that participate in the current method pipeline, permission replies,
errors, cancellation, and cleanup. Optional context reporting, client tags,
and live voice metadata remain unselected. Static release asset digests use
GitHub's published digest metadata; source tags and recursive tree identities
bind each point. Release binaries were not downloaded or executed.

## Claim and boundaries

- Extend the existing `goose.acp.release-window-1` claim through exact points
  `1.58.0`, `1.58.1`, and `1.59.0`; retain baseline `1.50.1`, all eight
  earlier points, `goose.acp.stdio-v2.auth-required`, and `AllowUnverified`.
- Keep unpublished interstitials `1.57.1` and `1.58.2` incompatible. A later
  stable `1.59.1` is `UnverifiedNewer` in the regression fixture.
- Preserve the typed `auth_required` mapping, generic-error/render-failure
  handling, permission authority, cleanup ownership, and emission-only HTTP
  MCP boundary. Static qualification does not claim live HTTP MCP honouring.
- The release and source evidence qualifies these exact published points. It
  does not authorize install, provider prompts, credentials, or live sessions.

No provider prompt, live session, credential, Goose installation, host
configuration change, or provider permission was used. No release binary was
downloaded or executed. The previous qualified claim is extended without a
new behavior revision or a change to the selected route's authorities.

## Sources

- [Goose v1.58.0 release](https://github.com/aaif-goose/goose/releases/tag/v1.58.0)
- [Goose v1.58.1 release](https://github.com/aaif-goose/goose/releases/tag/v1.58.1)
- [Goose v1.59.0 release](https://github.com/aaif-goose/goose/releases/tag/v1.59.0)
- [Canonical `aaif-goose/goose` repository](https://github.com/aaif-goose/goose)
- Official GitHub stable tag refs and complete source manifests in the fixture.
