# g06.028 OpenCode ACP HTTP MCP Live Gate

Owner: Tom
Created: 2026-09-24
Depends on: Contract 063 (Consumer-Supplied HTTP MCP Placement); Research 337; completed g06.019
Vision tags: OpenCode ACP, MCP placement, live evidence

## Outcome

Run one operator-authorized live gate proving that `opencode.acp` honours a
consumer-supplied streamable-HTTP MCP entry: OpenCode connects, lists the
tool, and completes one tool call. Settle the gated MCP cells for the exact
tuple.

## Why It Matters

g06.019 wired the entry; emission is proven, honouring is not. This gate is
what turns `opencode.acp` into a harness that reaches Longhorn's Contract 022
`agent-control` MCP directly, with no stdio carrier. Tom authorized it on
2026-09-24.

## Ready-State Rubric

- [x] g06.019 merged the production encoder and prepared facade.
- [x] Host `opencode` is installed at `1.18.18`, byte-identical to the
      official artifact (Research 337) and inside the qualified window.
- [x] Tom authorized one live attempt on 2026-09-24.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- **Harness proof first.** Before the live attempt, drive the exact probe
  harness end to end against a fake ACP agent and the disposable MCP server,
  asserting the full record shape (declaration sent, connect, tool list, call,
  result, terminal status, cleanup) in a committed test. A harness without
  that proof is not ready and the live attempt does not run.
- The MCP server is a disposable, test-only loopback streamable-HTTP server
  with a per-run bearer and one deterministic tool. It is gate
  infrastructure behind the live-probe feature, never a production listener.
- Tuple: installed exact `1.18.18`, the `http` form, loopback URL, one bearer
  header. Do not update, reinstall or log in.
- Model: the cheapest model the host's existing OpenCode configuration can
  already use. Record the exact model string. If none is usable without a
  login or new credential, stop and ask.
- One attempt. No rerun, model substitution or tuple change without new
  authority.
- Success moves only the `opencode.acp` MCP cells the evidence covers, bound
  to exact `1.18.18`. Other points in the window stay unqualified for
  honouring until evidence covers them.

## Dispatch manifest

- **State:** ready; provider-operation lane with one authorized gate.
- **Completion:** the harness proof test passes and the gate runs once,
  either accepting the tuple or recording a typed stop naming the model and
  failure; matrices and guide agree; independent exact-head review accepts
  the head.
- **Owned mutable paths:** `crates/swallowtail-adapter-opencode/**` for the
  feature-gated probe, disposable MCP server, and harness proof test; the
  OpenCode ACP prepared guide; the `opencode.acp` feature and activity matrix
  rows; one new research record and its index line; `CHANGELOG.md`
  `[Unreleased]`; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Excluded:** this card's prose; contracts; production listener code;
  `opencode.http`; other routes; any second attempt; install, update, login;
  release, tag, publication.
- **Worker evidence:** the authenticated run result and the research record.
  Retain no raw provider stream, bearer, account identifier, session id or
  private path.
- **Escalation:** operator via Chatterbox for budget or model questions;
  Queue coordinator for mechanical blockers.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Harness proven before spend | The live attempt runs with an untested harness | committed fake-agent test on the same code path |
| Honouring, not emission | Success claimed from the declaration alone | server-side log of one authenticated tool call and its result reaching the turn |
| No production listener | The test server is reachable from a production path | feature-gated, test-only module |
| Secrets stay private | The bearer or URL appears in evidence | redacted research record |
| One attempt | A retry or model swap | run result shows one attempt |

## Stop conditions

Stop before the attempt if the harness proof fails, if host `opencode` is not
exactly `1.18.18`, or if no configured model is usable without a login.

## Evidence

Research 337; the new research record. Focused and affected-package validation
for `swallowtail-adapter-opencode`, route and docs QA.

## Next Task

Chatterbox reconciles the cells and reports to Longhorn whether `opencode.acp`
is a `direct-http` route.
