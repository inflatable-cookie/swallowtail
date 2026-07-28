# Claude Agent Structured Write Authority

Date: 2026-07-28

## Changed

- Split Claude Agent ACP access by operation.
- Kept interactive sessions on the exact ambient read-only tool set.
- Moved structured runs to an explicit ambient `ReadWrite` resource lease.
- Added only `Edit` and `Write` to the structured session tool set.
- Required advertised `acceptEdits` support and selected it before the prompt.
- Accepted command, configuration, and mode updates between session creation
  and the first turn while retaining fatal handling for unknown updates.
- Preserved permission rejection for every unexpected provider request.

## Current State

Claude Agent structured conversion runs can mutate their approved working
resource. The working resource selects the process directory and lease
authority. `AmbientHost` remains explicit: this is not a filesystem-
containment claim.

Exact package inspection confirmed `session/set_mode` and `acceptEdits` across
the maintained behavior milestones `0.53.0`, `0.54.1`, and `0.61.0`.
The fixture reproduces session-scoped notifications after `session/new`,
`session/set_mode`, and both configuration mutations.

## Validation

- focused Claude Agent ACP driver, structured-run, and prepared-facade suites
- maintained-range request-shape assertions
- strict all-target adapter Clippy
