# 359 Antigravity Headless Retry-Pin Evidence (`AGY_CLI_MODEL_API_MAX_RETRIES`)

Status: current. This record proves the `1.2.11` retry-control semantics and
qualifies exact `1.2.11` headless on a retry-disabled behaviour revision.
No Contract 023 exception is taken.

Owner: Tom
Date: 2026-09-26
Card: swallowtail#069 (`antigravity.headless` retry-pin evidence, plan item
`version-currentness`, Q-003 option 2)
Authority: Contracts 023 and 029; Research 283, 323, and 353; the frozen
`1.2.11` artifacts. Queue approval: Tom approved Q-003 option 2 evidence
first on 2026-09-26.

## Outcome

`AGY_CLI_MODEL_API_MAX_RETRIES` in Antigravity `1.2.11` bounds the
provider-managed model-request retry, and `0` disables it: exactly one
model-request attempt, then an immediate typed exit-3 `AGY_ERROR` failure.
Finite `N` allows exactly `N+1` attempts on every retryable failure class
Research 353 lists (502, 503, 504, per-minute 429, mid-stream interruption).
Invalid values warn and keep the compiled default (8 retries, 9 attempts).

`antigravity.headless` therefore qualifies exact `1.2.11` on the new
behaviour revision
`antigravity.stream-json.cli-1.1.8-artifact-1.2.11-retry-disabled-v1`, whose
approved environment pins `AGY_CLI_MODEL_API_MAX_RETRIES=0`. Provider-managed
retry is disabled on that segment, so no Contract 023 exception is needed.
`1.1.9..=1.1.17` stay on the existing revision unchanged. `1.1.18..=1.2.10`
stay unqualified: pin semantics are proven on `1.2.11` only, and per-point
backfill is a successor task. Later stables stay visibly unverified newer.

## Proven semantics

Static analysis of the digest-verified `1.2.11` binary plus loopback
execution against a fake model endpoint (disposable home, no provider auth)
prove:

| Value | Model-request attempts | Evidence |
| --- | --- | --- |
| unset | 9 (default max 8 retries) | dynamic, loopback 503 |
| empty / whitespace-only | 9, silent | static early return before parse |
| `0` | exactly 1, immediate typed exit 3 | dynamic, loopback 503 |
| `1` | exactly 2 | dynamic, loopback 503 |
| `2` | exactly 3 | dynamic, loopback 502, 503, 504, 429, mid-stream EOF |
| invalid (`abc`, `-1`, `3.5`, `2^32`) | 9 with a log warning, same as unset | dynamic `abc` + static shared `ParseUint` error branch |
| `4294967295` | accepted by the parser (not executed; would allow ~4bn retries) | static `ParseUint(_, 10, 32)` domain |

Accepted domain is base-10 `uint32` (`0..=4294967295`, leading `+`
tolerated): the loader calls `strconv.ParseUint(value, 10, 32)`. Anything
else reaches the single error branch, which logs

```text
invalid AGY_CLI_MODEL_API_MAX_RETRIES="abc": must be a non-negative integer
within uint32: strconv.ParseUint: parsing "abc": invalid syntax; falling back
to the default retry budget
```

and keeps the compiled default. The warning goes to the CLI log file, not
to stdout/stderr. An invalid value therefore never widens retry beyond the
default, and never selects the pin: the approved environment must set
exactly `0`.

Retryable classification is orthogonal to the pin. Terminal failures
(probed: HTTP 400) fail once with no retry at any budget. Retryable
failures retry up to the bound, with `run.go` logging
`attempt N failed (...), retrying in Ns`: observed backoffs start at 4 s
and grow toward the published 30 s per-attempt cap (Research 353). The
`AGY_ERROR` `retryable` flag describes the error class, not remaining
budget: mid-stream EOF reports `retryable:true` and still stops after
`N+1` attempts, with the partial responses preserved in the run output
(the `1.2.10` partial-output shape).

## Method

Static analysis stayed bounded to the one control: the `1.2.11` binary
carries the string `AGY_CLI_MODEL_API_MAX_RETRIES` exactly once (rodata
offset `0x5c8c5c8`), with exactly one code reference (text `0x8d45903`)
inside `backend.applyModelAPIMaxRetriesOverride` (entry `0x8d458e0`,
length `29` = `0x1d` passed beside the pointer, single caller
`0x8d44883` in shared backend construction). The loader uses env lookup
with a presence check (unset keeps the default), returns early on an empty
value, parses with base 10 and bit size 32, and on success stores a boxed
`uint32` into the backend `ModelAPIRetryConfig.max_retries` proto field.
No `AGY_CLI_MODEL_OUTPUT_MAX_RETRIES` string exists in the binary: the
separate `ModelOutputRetryConfig.max_retries` has no environment override
and is outside this evidence (no 353 failure class exercised it; observed
runs show no attempt outside the counted model-request budget).

Dynamic proof drove the digest-verified mac-arm64 `1.2.11` binary
(tarball `437a813c…`, extracted `42e76bed…`, both matching the frozen
identity) from a disposable home against a loopback HTTP endpoint selected
with `AGY_GATEWAY_URL=http://127.0.0.1:18731` and a dummy gateway key.
The gateway override is measurement apparatus only; the pin loader sits in
shared backend construction independent of transport, so the result
transfers to the personal-subscription headless path, whose approved
environment carries no gateway variables. Every run was connection-audited
(`lsof` polling plus a dead-port proxy trap for non-loopback egress):
only `127.0.0.1:18731` was ever contacted. No provider call, prompt,
login, credential use, host install, or host update occurred.

Attempt counts come from the loopback request log (full-size
`streamGenerateContent` POSTs; the 795-byte conversation-title sidecar is
ancillary, 1–3 per run, loopback-bound, and outside the counted budget).
Runs used `--model gemini-3.8-flash --effort medium`, which records the
`1.2.11` `--effort` mapping this milestone needs: an explicit `--model`
requires `--effort`, admitted as `low`, `medium`, or `high` (the CLI
rejects a bare `--model` naming exactly that set, and rejects `--effort
max` for that model). Enforcement in dispatch is covered above; the
value gate itself is unchanged.

## Decision

- **Headless: qualify exact `1.2.11` on a retry-disabled milestone.**
  New segment `1.2.11..=1.2.11` on
  `antigravity.stream-json.cli-1.1.8-artifact-1.2.11-retry-disabled-v1`,
  approved environment pins `AGY_CLI_MODEL_API_MAX_RETRIES=0`. The
  milestone bumps the claim to `antigravity.headless.release-window-2`.
- **Keep `1.1.9..=1.1.17`** on
  `antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1`, relabeled
  `Deprecated` per the Contract 029 segment rule (older revision retained
  for existing harnesses, not targeted for new integrations).
- **`1.1.18..=1.2.10` stay unqualified** (interior to the new latest, so
  assessed incompatible, not unverified newer). Pin backfill per point is a
  successor task under the standing currentness lane.
- **No Contract 023 exception.** Retry is disabled on the new segment, not
  accepted; the host deadline, cancellation, and typed exit-3 failure
  behaviour are unchanged.
- **No option 1 taken.** Accepting provider-managed retry remains the
  Q-003 fallback and was not used.

## Effort enforcement

The `1.2.11` `--effort` mapping is enforced in dispatch, not just
recorded. Adapter dispatch always passes an explicit `--model`, and
`1.2.11` refuses that without `--effort`: `--model gemini-3.8-flash`
alone fails with `requires --effort (available: low, medium, high)`, and
`--effort max` fails with `has no "max" effort` for that model. The
adapter therefore rejects effort-less runs and continuation opens bound
to the retry-disabled segment instead of spawning a child the CLI
rejects; omission stays a rejection, never an invented default. The
admitted set stays `low`, `medium`, or `high` with a planned
`ReasoningSelection` constraint. Continuation carries a session-scoped
effort from its profile input into every turn. (`gemini-3.1-flash-lite`
is not a recognized gateway-mode model slug; the title sidecar that
names it travels its own path.)

## Validation

Provider-free apart from the loopback apparatus above.
`effigy validate:focused swallowtail-adapter-antigravity`,
`effigy package:verify-affected swallowtail-adapter-antigravity`,
`effigy qa:docs`, and `effigy qa:routes` pass.

## Sources

- official `agy_cli_mac_arm64.tar.gz` and `agy_cli_linux_x64.tar.gz` for
  `1.2.11` (digests as frozen in Research 353), executed only against the
  loopback endpoint above
- frozen `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.2.11/`
- new pin-evidence fixture
  `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.2.11-retry-pin/pin-evidence.json`
- loopback request log and per-run CLI logs/outputs (disposable; method
  retained here, raw logs not frozen)
- [Research 283](./283-antigravity-1-1-26-identity.md),
  [Research 323](./323-antigravity-1-2-2-identity.md), and
  [Research 353](./353-antigravity-headless-adaptation-to-current-official.md)
- [Contract 023](../knowledge/contracts/023-harness-operation-isolation-and-native-boundary.md)
  and [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
