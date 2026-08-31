# Codex Exec Output-Token Census Source

Status: open
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

## Boundary

This note is planning evidence only. It changes no census row, contract,
roadmap, compatibility claim, or execution authority.
