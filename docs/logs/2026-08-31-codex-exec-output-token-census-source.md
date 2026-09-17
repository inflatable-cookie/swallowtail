# Codex Exec Output-Token Census Source

Status: resolved; census row corrected as route-rejected descriptor evidence
Owner: Tom
Source: PR 133 exact-head review at `a787b942dd64ff9dea5c0b93455bba350abf087b`

## Observation

The Contract 061 census row for `codex.exec` /
`control.maximum-output-tokens` names
`CodexExecProfileInput::new (maximum_output_tokens)` as its public source.
`CodexExecProfileInput` has no such constructor input or prepared request
value. Card 023 therefore withholds the row rather than inventing prepared
truth.

The adapter does expose output-token handling on lower-level structured-run
request validation. That does not establish the census row's current prepared
facade source or requested/prepared state.

## Question

After card 023 closes, re-audit this one census cell against the exact
`codex.exec` low-level and prepared surfaces. Decide whether to correct the
named public source and state-support description, record a prepared-facade
gap, or remove the row. Do not widen card 023 or add a prepared input from this
note.

## Resolution

Keep the row as negative census coverage, but correct its authority. The public
source is the generic
`StructuredRunRequest::with_maximum_output_tokens(NonZeroU64)` surface followed
by `codex.exec` route validation. Every present value is rejected as
unsupported. `CodexExecProfileInput` cannot construct the value, so the row
has no requested, prepared, effective, or acknowledged state on this route.

The corrected row is descriptor-only evidence of a generic runtime control
that must be withheld from the Codex Exec contribution. This matches card
023's construction-time disposition and preserves the 767-row census
partition. It does not create a Codex control, change runtime behavior, or
authorize a later prepared-facade input.

## Boundary

This resolution changes only the census source and state description. It
changes no contract, compatibility claim, runtime behavior, or execution
authority.
