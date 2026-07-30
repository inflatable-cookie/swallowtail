# Claude Agent Form Elicitation Closeout

Date: 2026-07-30

## Outcome

Claude Agent ACP now advertises form elicitation and maps the exact
losslessly representable `AskUserQuestion` form subset through
`HarnessUserInputRequest`. Typed consumer answers return through the original
ACP request as accepted form content. Consumer failure returns decline.

## Evidence And Boundary

Current official ACP and claude-agent-acp sources establish
`clientCapabilities.elicitation.form` and `elicitation/create`. Tagged bridge
comparison covers `0.53.0` through `0.64.0`; the maintained Swallowtail
guarantee remains `0.53.0..=0.61.0`, with later releases visible as
unverified-newer.

The bridge constructs form fields from selected Claude question properties.
It does not preserve arbitrary question or request context. Swallowtail
therefore adds no invented runtime context slot. Option preview survives only
inside Claude-private option metadata, so preview-bearing and otherwise richer
forms are declined instead of flattened.

Research 073 records the source comparison. Contracts 015 and 041 own the
negotiation, typed-subset, and no-context rules.

## Implementation

- initialization advertises ACP form elicitation
- `elicitation/create` validates session and active-turn authority
- current and historical Claude choice schemas map to typed questions
- single, multiple, Other, and skipped answers round-trip exactly
- provider request ids correlate responses exactly once
- cancellation and terminal completion abandon pending callbacks
- unsupported forms receive decline without entering consumer callbacks
- diagnostics expose no raw question, answer, or form payload

## Validation

- `effigy validate:focused swallowtail-protocol-acp
  swallowtail-adapter-claude-agent` — 147 tests passed; clippy passed
- affected protocol and adapter package archives assembled and compiled
- `effigy doctor` — 147 warnings, zero errors
- provider-solution CSV shape, provider sort, and Claude question-exchange
  value checked
- docs, formatting, and diff checks passed

No consumer repository, live provider state, release candidate, or publication
state changed.

## Remaining Risk

- ACP form elicitation remains unstable within the ACP v1 wire family.
- claude-agent-acp `0.62.0..=0.64.0` remains unverified-newer.
- arbitrary context and option-preview presentation need upstream bridge
  preservation before a shared runtime contract would be honest.
- consumer UI adoption remains separately authorized.

## Next

Reassess Nucleus adoption authorization for the completed child-work and
Claude typed user-input handoffs.
