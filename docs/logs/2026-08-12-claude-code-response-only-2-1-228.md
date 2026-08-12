# Claude Code Response-Only 2.1.228 Qualification

Date: 2026-08-12
Roadmap: g03.068
Contracts: 039, 044

Status: superseded the same day by
[Claude Code Response-Only Protocol Compatibility](./2026-08-12-claude-code-response-only-protocol-compatibility.md).
The exact-patch adoption below is retained only as historical evidence.

## Outcome

Exact Claude Code `2.1.228` replaces exact `2.1.227` as the sole maintained
release for `swallowtail.claude-code.response-only`. The implementation commit
is `6a3fe2aaeb0ccae8fc53598d90509b0280412182`.

The prepared API, route identity, command arguments, model selection,
host-service requirements, access posture, and output projection do not
change. Exact `2.1.227`, exact `2.1.229`, and every other version remain
incompatible; no range was added.

## Protocol Boundary

- the init envelope reports exact `2.1.228`, the selected model, default
  permission mode, `tools=[]`, and `mcp_servers=[]`
- medium effort admits only bounded cumulative thinking-token estimates and
  one empty private-thinking block with an opaque signature
- private thinking is validated and discarded; no thought content becomes
  output, reasoning, activity, or usage
- one assistant text record and one matching one-turn result become ordinary
  untrusted `OperationContent`
- unknown envelopes, version drift, authority drift, a second response,
  malformed bounds, or post-terminal data still fail closed

## Access And Live Evidence

`/Users/tom/.local/bin/claude` resolved to
`/Users/tom/.local/share/claude/versions/2.1.228` and reported
`2.1.228 (Claude Code)`. Before the exact binding changed, the gated live probe
stopped at discovery with `VersionParse`.

With `ANTHROPIC_API_KEY` removed, the updated prepared facade passed ordinary
text, medium thinking/progress, and local cancellation through Max/OAuth using
only approved `HOME`, `USER`, and `LOGNAME`. All runs cleaned; the repository
source-status snapshot stayed unchanged.

## Validation

- focused adapter validation: 80 tests passed
- affected-package archive, dependency closure, and extracted compilation passed
- guide coverage passed for 36 routes, 35 examples, and 44 portable features
- route, lifecycle, feature, and 70-operation activity matrices passed
- the full docs selector passed
- authenticated live probe: one test passed in 18.59 seconds

Historical `v0.3.2`, release, research, and exact `2.1.227` qualification
records were not rewritten. No release, tag, registry publication, or Figmatic
mutation was performed.

## Next Task

In Figmatic, check out Swallowtail implementation commit
`6a3fe2aaeb0ccae8fc53598d90509b0280412182`, link it through `effigy deps link`, and run
the packaged `g04.005` mutation-runway smoke under card 215.
