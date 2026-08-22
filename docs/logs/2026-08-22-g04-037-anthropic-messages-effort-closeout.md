# 2026-08-22 g04.037 Anthropic Messages Effort Closeout

Status: completed
Owner: Tom
Branch: `t3code/anthropic-messages-effort`
PR: [#37](https://github.com/inflatable-cookie/swallowtail/pull/37)
Implementation head: `185204f2b8413c88ec882141d517239752cce29e`

This is the route-local worker closeout. The orchestrator records merge and
shared-surface closeout separately. No merge, release, live provider request,
account inspection, credential use, or effective-effort claim was made.

## Delivered Row

Research 185 promotes one exact route-local row:

- facade: `anthropic.messages` / `anthropic-2023-06-01`
- model: `claude-opus-4-7`
- values: `low`, `medium`, `high`, `xhigh`, `max`
- profiles: one-attempt structured inference and resource-free direct
  continuation sessions
- session rule: select once during preparation; reuse on every attempt, later
  turn, and fresh restoration
- wire: additive `output_config: {"effort": "<exact>"}` only
- exclusions: fixture model ids, other unqualified Anthropic model ids,
  `thinking`, Claude Code, Ultracode, Fast mode, Managed Agents, newer search,
  and all live-provider/effective-effort claims

## Changed Route-Local Surfaces

- `docs/research/185-anthropic-messages-effort-evidence.md`: promoted official
  evidence, source URLs and retrieval digests, exact dispositions, and wire
  specimen
- `crates/swallowtail-adapter-anthropic`: typed optional `ReasoningMode` input,
  exact model/value validation, immutable plan/evidence binding, structured and
  session driver agreement checks, fixed-session propagation, and additive
  Messages wire encoding
- `crates/swallowtail-adapter-anthropic/src/protocol/tests.rs` and prepared
  facade fixtures: all five value dispatch, absent-body, unsupported-input,
  plan/request mismatch, continuation, later-turn, and restoration coverage
- `docs/guides/anthropic-direct-prepared-integration.md`: exact effort usage,
  fixed-session behavior, and requested/planned/dispatched/accepted/effective
  claim boundaries
- `docs/roadmaps/g04/037-anthropic-messages-effort.md` and cards 101-103:
  completed statuses and acceptance evidence
- `release-baselines/public-api-unreleased/swallowtail-adapter-anthropic.txt`:
  new public effort builders and evidence accessor

## Required Shared-Surface Delta For Orchestrator

Do not apply these edits from this worker branch while the parallel wave is
open:

- `docs/architecture/system-architecture.md`: extend the realized
  `swallowtail-adapter-anthropic` direct-facade paragraphs with the exact
  `claude-opus-4-7` effort row, portable `ReasoningSelection`, fixed-session
  dispatch, and no-thinking/no-effective claim posture.
- `docs/guides/provider-route-matrix.md`: add the structured/session effort
  qualification to the `anthropic.messages` prepared-path row.
- `docs/guides/provider-solution-feature-matrix.csv`: change only the
  `anthropic.messages` `reasoning_selection` cell from `No` to `Yes` and add
  the exact model/value/profile note; do not change Managed Agents or sibling
  Anthropic rows.
- `CHANGELOG.md`: add the Unreleased g04.037 route-local effort entry with
  exact values/model and dispatch-versus-effective wording.
- `docs/roadmaps/g04/per-route-feature-completion.md`: mark g04.037 complete;
  retain Ollama → Anthropic → DeepSeek integration order.
- `docs/roadmaps/g04/README.md`: update the g04.037 planned/current checkpoint
  from ready to complete, without changing the generation shape.
- `docs/roadmaps/README.md`: advance the active Next Task pointer only after
  the orchestrator integrates the fixed-order wave; the next route is
  g04.038 / cards 104-106 once the shared closeout is accepted.
- `docs/roadmaps/g04/batch-cards/README.md`, `docs/research/README.md`, and
  `docs/logs/README.md`: refresh shared status/index text only in the
  orchestrator closeout. Research 185 and this log were pre-indexed and their
  links remain valid.
- `release-baselines/public-api-0.3.3/packages.txt`: no change expected.

## Validation

Passed route-local and named acceptance checks:

- `cargo fmt -p swallowtail-adapter-anthropic`
- `effigy validate:focused swallowtail-adapter-anthropic`
- `effigy package:verify-affected swallowtail-adapter-anthropic`
- `effigy check:examples`
- `effigy qa:routes`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy package:api`
- `git diff --check`

The focused suite covers 79 Anthropic package tests, including all five
deliver-now values on structured requests and sessions. `effigy doctor` still
reports the inherited repository health context from dispatch: 42 existing
god-file findings, stale graph state, and the generated-in-src PAPERCUTS
warning. Those findings are unrelated to this route-local change.

The repository-wide route and documentation checks passed with no new
findings. `effigy package:api` passed at v0.3.3 with the Anthropic baseline
delta recorded in the allowed unreleased package baseline.
