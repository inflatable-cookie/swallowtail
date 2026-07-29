# 2026-07-28 Provider Retention Implementation And Closeout

## Scope

Complete roadmap g02.030 cards 101-102: realize the four selected runtime
cells, correct the stale OpenCode cell, close the 75-cell inventory, and select
the next matrix family.

## Production Changes

- Gemini CLI headless now has a separate installed-executable
  provider-session management role across exact `0.51.0..=0.52.0`.
- A successful durable Gemini run can return one take-once management binding
  after terminal completion. Raw ids, list output, ACP bindings, failed runs,
  and temporary runs grant no authority.
- Gemini deletion sends only the exact bound id, joins the child, then runs one
  bounded `--list-sessions` reconciliation. Exact absence yields only
  `HistoryRemoved`.
- Gemini and Claude Agent expose separate opt-in temporary structured
  profiles. Their durable defaults remain unchanged.
- Claude Agent temporary cleanup sends native close then provider-data delete
  for only the operation-private session.
- OpenAI background Responses send at most one terminal response-delete
  request before credential release. Inference status and response cleanup
  truth remain independent.
- `OwnedRemoteResourceKind::Response` represents the new exact cleanup target.
- OpenCode already deleted its private structured-run session; only its matrix
  cell was stale.

## Matrix Closeout

The 75 starting provider-retention `No` cells now have final dispositions:

| Disposition | Cells |
| --- | ---: |
| `Not applicable` | 58 |
| realized `Yes` | 5 |
| retained exact `No` | 12 |
| **Total** | **75** |

The 12 retained absences are:

- archive and restore for Claude Agent, Gemini CLI, Kimi Code installed
  harnesses, and OpenCode
- delete and owned cleanup for Kimi Code installed harnesses
- delete and owned cleanup for Kimi local server

The full audited matrix falls from 432 to 369 `No` cells. The route gate
machine-locks both the historical classification and final disposition.

## Validation

- `cargo fmt --all -- --check`
- `cargo check --workspace --lib -j2`
- focused tests for `swallowtail-core`, `swallowtail-runtime`,
  `swallowtail-adapter-gemini`, `swallowtail-adapter-claude-agent`,
  `swallowtail-adapter-openai`, and `swallowtail-adapter-opencode`
- `effigy qa`
- `effigy check:examples`
- `effigy package:check`

Full docs, Northstar, route, formatting, all-target, Clippy, workspace-test,
locked-example, metadata, public-API, generated-doc, MSRV/current-stable, and
23-crate local package gates pass. Extracted package tests include the selected
Gemini, Claude Agent, and OpenCode prepared paths.

## Retained Risks

- Gemini proves local provider-history removal, not provider-data deletion or
  secure erasure. Management is guaranteed only for exact `0.51.0..=0.52.0`;
  unverified-newer execution is rejected.
- Claude Agent reports provider-data deletion, not secure erasure.
- OpenAI confirms only the exact response-delete acknowledgement. Disconnect,
  mismatch, false acknowledgement, 404, active state, or missing terminal
  truth remains unconfirmed and cleanup-degraded.
- Kimi exposes no selected-route delete or owned-cleanup authority. Its four
  cells remain honest `No`.
- No live authentication, provider request, paid operation, container, model
  server, publication, or consumer repository was used.

## Continuation

Roadmap g02.031 and card 103 begin the 59-cell retained-execution and recovery
currentness audit. The family keeps retained execution, retrieval, stream
reattachment, transport reconnect, and provider-managed recovery separate.
