# g05.081 Goose 1.50.1 Failure-Binding Reopen

Status: ready
Owner: Tom
Created: 2026-09-17
Governing refs: Contract 029; Contracts 017 and 023
Depends on: completed g05.071 (typed stop and Research 319); the recorded failure-binding ruling in `docs/roadmaps/standing-lanes.md`

## Outcome

Reopen the exact `goose.release` point under the operator's recorded
failure-binding ruling and land one honest `goose.acp` claim at the
then-current published stable, on a behavior revision that covers the typed
provider-authentication failure mapping.

## Ready-State Rubric

- [x] g05.071 froze the exact `1.46.0` baseline and all four published stable
  successors through `1.50.0` with Research 319 and a mutation-sensitive
  ledger.
- [x] The failure-binding ruling is recorded: the route binds the typed
  provider-authentication semantics, and the reopen needs a new behavior
  revision plus the typed shape in the prepared guide.
- [x] No prompt, configure, login, credential, installation, or host update is
  needed for the identity and claim decision.
- [x] Scope, acceptance, validation, evidence, and stop conditions are explicit.
- [x] Review oracle is present because the claim is exact, version-bound, and
  partly negative.
- [x] Official stable is re-probed immediately before the identity commit.

## Decisions

None open. The ruling settles whether the typed arm is admissible: it is,
because `1.46.0`'s untyped error text plus `end_turn` gave a consumer nothing
to branch on. This task settles the behavior revision and the recorded shape.

## Dispatch manifest

- **State:** ready; provider-free; one lane; serial against other currentness
  families on shared matrices; no automatic successor.
- **Completion:** one exact `QualifiedOnly` point lands at the then-current
  published stable on a new behavior revision covering the typed
  provider-authentication mapping; the guide records that a chained ACP
  `AuthRequired` on `session/new` or `session/prompt` surfaces as
  `auth_required`; focused validation, route and matrix QA, and named docs
  gates pass; independent exact-head review accepts the head.
- **Owned mutable paths:** Goose selection, prepared route, ACP connection and
  decoder, and fixtures under `crates/swallowtail-adapter-goose/**`; the new
  Research record and one research-index line; the Goose prepared guide,
  route/activity/feature matrix rows, architecture ceiling, standing-lane
  entry, `CHANGELOG.md` `[Unreleased]`, identity log, claim log, and this
  task's result lines; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Worker:** evidence-first Rust currentness worker; exact tagged-tree
  retrieval and ACP boundary reconciliation; no provider credentials.
- **Excluded:** `goose serve`, HTTP/WebSocket/TLS, desktop/TUI, recipes, ACP
  provider adapters, persistent permission, load/list/close, fork/steer, and
  client MCP servers; any live ACP session; release, tag, or publication.
- **Escalation:** operator via Chatterbox for any new route-policy choice;
  Queue coordinator for mechanical blockers.

## Work

1. Re-probe the canonical release channel. Freeze identity for the
   then-current published stable and re-confirm the `1.46.0`..`1.47.0` hop
   that introduced the typed arm.
2. Confirm the mapped failure surface across every hop to that stable, with a
   mutation-sensitive fixture that fails if the typed mapping is removed or
   further reshaped.
3. Mint the behavior revision that covers the typed mapping. Keep the exact
   point single and `QualifiedOnly`; do not infer a range.
4. Record the typed failure shape in the Goose prepared guide and reconcile the
   decoder, matrix, and architecture surfaces it touches.
5. Stop with the smallest exact counterexample if a mapped wire, lifecycle,
   permission, builtin, mode, or process-authority boundary changed beyond the
   failure binding.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Provider-auth failure is typed on the selected route | A failed `session/new` still completes with error text and `end_turn` | fixture over the injected ACP failure |
| The claim carries the changed failure mapping | The old behavior revision is reused although the mapping moved | selection table shows the new revision and its rationale |
| The claim stays one exact point | A range is inferred across the four published successors | selection table shows one exact point |
| Advertised siblings stay independently gated | Advertised `session/delete` or effort menus become live operations | census rows remain unmapped with reasons |

## Stop conditions

- Stop on official-channel disagreement or an unreachable channel.
- Stop if a mapped wire, lifecycle, builtin, mode, or permission boundary
  changed materially beyond the failure binding.
- Stop and escalate if the typed mapping cannot be expressed without widening
  the public lifecycle or a shared-vocabulary surface.

## Evidence

On completion, record the identity ledger, the failure-mapping fixture result,
the new behavior revision, validation actually run, PR link, reviewed exact
head, merge commit, and any material limit.

## Acceptance

- [x] Official `v1.50.1` identity and the `1.50.0..1.50.1` patch hop are
  frozen in Research 328 and the currentness fixture.
- [x] Typed `auth_required` is covered on both `session/new` and
  `session/prompt`; the adapter diagnostic is mutation-tested.
- [x] The claim is one exact `QualifiedOnly` point at `1.50.1` on the new
  `goose.acp.stdio-v2.auth-required` behavior revision.
- [x] Builtin, mode, lifecycle, permission, process, and advertised sibling
  surfaces remain independently gated.

## Result

Research 328 re-probed official Goose GitHub `v1.50.1`, froze its tag and
Darwin ARM64 archive identity, and compared the selected ACP closure through
the patch hop from `1.50.0`. Only the MCP protocol-version default changes in
the mapped source closure, and that change is unmapped because the selected
route sends an empty `mcpServers` list; the ACP server, dispatch, new-session,
prompt, and conversation-message sources are byte-identical.

The exact `goose.release` claim now binds `1.50.1` under
`goose.acp.stdio-v2.auth-required`. Typed provider-authentication failures on
`session/new` and `session/prompt` map to
`swallowtail.goose.acp.auth_required`; other provider/model resolution errors
retain their existing diagnostic. No provider prompt, login, configure,
installation, downloaded-artifact execution, release, or tag action occurred.
Validation and PR review remain queue-owned after this worker opens the PR.

## Next task

Nothing is auto-started. Chatterbox resumes planning on the returned result and
reconciles the standing lane.
