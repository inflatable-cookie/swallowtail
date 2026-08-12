# Claude Code Response-Only Protocol Compatibility

Date: 2026-08-12
Roadmap: g03.068
Contracts: 029, 039, 044

## Outcome

`swallowtail.claude-code.response-only` now selects by validated protocol
compatibility instead of exact patch equality. The prepared Figmatic-facing API
is unchanged. The implementation commit is `IMPLEMENTATION_COMMIT`.

The policy is:

- `2.1.227` is the minimum proven baseline
- `2.1.227..=2.1.228` is the qualified segment
- later stable releases may execute provisionally as `UnverifiedNewer`
- prerelease, build-qualified, malformed, below-baseline, and explicitly denied
  releases remain incompatible
- the static known-bad deny-list is empty at this closeout

Qualification never transfers from the version alone. Each run must pass the
same complete protocol validator before it can return text.

## Fail-Closed Boundary

The driver still constructs one exact argument vector with empty tools and an
empty strict MCP configuration. Preparation binds the observed executable
version into the selected plan. Run-start validation checks that binding again;
the init frame must echo the same version, selected model, default permission
mode, and empty tool/MCP surfaces.

The event parser admits only the bounded system, cumulative thinking estimate,
empty private-thinking, one text-only assistant, and one matching successful
terminal sequence. It rejects missing or reordered required frames, unknown
frames, tool or user content, readable private thought, non-cumulative or
inconsistent token accounting, extra assistant/result content, model or version
drift, malformed terminal state, and post-terminal data. Private thinking is
validated and discarded. Exactly one ordinary text result survives.

Opt-in preparation and run-start debug observations report the exact observed
executable version plus `Qualified` or `UnverifiedNewer` posture. Public
prepared evidence remains version-bound.

## Evidence

Committed `2.1.227` and `2.1.228` complete and medium-thinking fixtures exercise
the proven segment. A synthetic stable-newer fixture proves provisional
selection and the same runtime rejection boundary without claiming that patch
qualified. Mutation tests cover command arguments, tool/MCP authority, init,
thinking envelopes, cumulative token accounting, assistant frames, terminal
state, and exactly-one-text projection.

With `ANTHROPIC_API_KEY` removed, the live `2.1.228` probe used the approved
Claude executable and local Max/OAuth state. `HOME`, `USER`, and `LOGNAME`
remained the only auth-supporting environment additions.

## Validation

- focused validation: 166 tests passed across the adapter and testkit
- affected-package archive, dependency closure, and extracted compilation
  passed for both packages
- guide coverage passed for 36 routes, 35 examples, and 44 portable features
- route, lifecycle, feature, and 70-operation activity matrices passed
- the full docs selector passed
- live `2.1.228` Max/OAuth probe: one test passed in 19.08 seconds with
  `ANTHROPIC_API_KEY` removed

Historical release, research, and exact `2.1.227` records were not rewritten.
The superseded same-day exact-`2.1.228` log remains as historical evidence. No
release, tag, registry publication, or Figmatic mutation was performed.

## Next Task

In Figmatic, check out Swallowtail implementation commit
`IMPLEMENTATION_COMMIT`, link it through `effigy deps link`, and run packaged
`g04.005` under card 217. The route must report the installed Claude version in
diagnostics and fail closed if the live transcript departs from Contract 039.
