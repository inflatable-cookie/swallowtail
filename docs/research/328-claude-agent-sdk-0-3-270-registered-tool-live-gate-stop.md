# 328 Claude Agent SDK 0.3.270 Registered-Tool Live Gate Stop

Status: typed stop; no provider work started
Owner: Swallowtail worker
Created: 2026-09-17
Task: g05.066; planning `5e88c4d7f2acc4d63ac8f7b761bb8c8ff75d164`

## Question

Can the existing Desktop Card 318-shaped runner execute the one authorized
registered-tool gate against the rebound SDK `0.3.270` / native `2.1.270`
tuple?

## Stop

No. The existing consumer runner is not bound to the rebound tuple. Its
provider-free test `tests::frozen_static_tuple_matches_linked_swallowtail`
stopped with the typed code `setup.static_tuple_mismatch`: the runner still
expects SDK `0.3.259` and native `2.1.259`, while the linked Swallowtail route
declares SDK `0.3.270` and native `2.1.270`.

This is a setup stop before provider work. No Claude session, prompt, tool
dispatch, credential use, live capsule, retry, or fallback occurred. The
existing runner cannot be changed within this Swallowtail task, and adapting
or inventing a new consumer runner would exceed the handoff boundary.

## Preflight evidence

The Swallowtail-side provider-free preflight passed:

- `cargo build --offline --locked -p swallowtail-host-local --features
  mediated-stdio-proxy --bin swallowtail-registered-tool-courier`
- `effigy validate:card116-mediated-stdio`: 15/15 tests passed
- Desktop runner `--check --source-root <this checkout>`: 35/36 runner tests
  passed; the one tuple-binding test stopped the check as above

The host has the required Node `22.23.2`, but the installed Claude executable
is native `2.1.258`. The exact SDK `0.3.270` tarball is present only in the
package-manager cache; using it would require an install or a new runner,
both excluded by the handoff. No provider operation was attempted with either
non-matching artifact.

## Disposition

The compiled route remains exactly SDK `0.3.270` / native `2.1.270`, while the
frozen live point remains SDK `0.3.259` / native `2.1.259`. The
`registered_tools` and `consumer_tool_exchange` cells remain producer-gap
`live_tuple_not_compiled`. No qualification, matrix change, or route-guide
change follows from this stop.

## Next move

A separately authorized consumer update must bind the existing runner to the
exact rebound tuple and return a fresh redacted capsule. That is a new
provider-operation prerequisite; this task performs no second attempt.
