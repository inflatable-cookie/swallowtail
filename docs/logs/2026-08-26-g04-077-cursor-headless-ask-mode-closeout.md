# 2026-08-26 g04.077 Cursor Headless Ask Mode Closeout

Status: complete
Owner: Tom
Milestone: g04.077
Cards: 213, 214, and 215 complete
Branch: `t3code/review-cursor-ask-mode-handoff`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-dcac6f7e`
Base: `4319e7ce1a7b5154226c5ffb405f6b2db079f38f` (`origin/main` at dispatch)
Planning base ancestor: `c12eeaf3ac041d66b31bd4cd26dd569efc1e6efd`
PR: https://github.com/inflatable-cookie/swallowtail/pull/76

## Result

Card 213 completed an exact four-build artifact, parser, precedence,
application, read-only, observation, model-parameter, and production-seam
audit. Research 224 admits four deliver-now rows: `--mode ask` with
`ResourceAccess::Read` on `2026.07.01-41b2de7`, `2026.07.23-e383d2b`,
`2026.08.04-aaa8809`, and `2026.08.11-e8db854`, at qualified dispatch and
application only. Cards 214 and 215 bound and proved one closed Cursor-local
selection.

The lane first closed as an evidence stop. The orchestrator requested changes
on PR 76: the exclusion was stronger than the lane and existing route-local
precedent require, since the route already binds `--mode plan` and Research
183 model parameters at the same tier without effective-value confirmation.
Research 224, the cards, and this closeout were revised accordingly, and the
binding was delivered.

## Evidence

Selection is exact and closed. All four qualified builds register
`--mode <mode>` with commander `.choices(["plan","ask"])` and no default.
Isolated parser cases are identical on every binary: `ask`, `plan`,
`--mode=ask`, `--mode=plan`, repeats, and every placement around the exact
production argv parse; `agent`, `ASK`, `Ask`, empty, `--help`-as-value, and
`ask,plan` reject. Persisted configuration holds no agent-mode key, no
environment variable selects the mode, and a headless session without
`--resume`/`--continue` starts fresh with no inherited mode metadata. Headless
refuses model-initiated switch-mode requests, so the value is immutable for
the run. `--plan` beats `--mode ask` in `chat.ts`; Swallowtail sends neither
by default.

Application is exact. `run-agent.tsx` stores Ask as agent-store metadata
`"search"`, and `headless.ts` attaches `AgentMode.ASK` to the outbound
`UserMessage`.

Effective and observed mode are withheld. `getIsAskMode` has one consumer:
`shared/resources.ts` picks `workspace_readonly` instead of
`workspace_readwrite` for the shell-exec sandbox policy, gated on
`sandboxFeatureGateEnabled && isSandboxSupported() && "enabled" === resolved
sandbox mode`. This route sends no `--sandbox` and both the default and host
configs hold `sandbox.mode: "disabled"`, so that branch is inert here; where
it is not inert, ambient config, team, and feature-gate state decide it. No
tool registry, approval path, or write refusal keys on Ask, and the qualified
stream emits a constant `permissionMode: "default"` with no mode field. No
locally enforced read-only boundary is claimed anywhere.

Every Research 183 deliver-now model tuple parses with `--mode ask` on all
four builds; `--mode` and `--model` stay independent.

## Binding

`CursorHeadlessReadMode` carries `Plan` and `Ask`, no raw string, and is not
portable `HarnessMode`.

- `CursorHeadlessRunProfileInput::new` is unchanged: `Read` resolves to
  `--mode plan`, `ReadWrite` resolves to no mode.
- `with_read_mode` is the only selection path and rejects any selection on
  `ReadWrite` with `swallowtail.cursor.headless.read_mode_access_rejected`.
- `prepare_run` rejects Ask on a release that is not exactly qualified with
  `swallowtail.cursor.headless.ask_mode_unqualified`. `UnverifiedNewer` does
  not inherit Ask. The behavior revision is unchanged because all four
  qualified builds share identical Ask semantics.
- The resolved mode is frozen on `CursorPreparedHeadlessRun`, readable through
  `read_mode`, and passed to `CursorHeadlessDriver::with_read_mode`.
- The low-level driver re-validates access and exact release before process
  work and never falls back to Plan or Agent after a rejected selection.
- `AmbientHost`, `Ambient`, `DurableAllowed`, working-resource authority,
  `--trust`, model rendering, activity, usage, cancellation, deadline,
  terminal, and joined cleanup are untouched.

## Changed Surfaces

- `crates/swallowtail-adapter-cursor/src/headless_command.rs`: closed
  `CursorHeadlessReadMode`, mode resolution, canonical argv rendering
- `crates/swallowtail-adapter-cursor/src/headless.rs`: driver read-mode
  binding and accessor
- `crates/swallowtail-adapter-cursor/src/headless_validation.rs`: fail-closed
  read-mode validation
- `crates/swallowtail-adapter-cursor/src/selection.rs`: exact-qualified
  release gate for Ask
- `crates/swallowtail-adapter-cursor/src/prepared/headless.rs`: typed
  selection, preparation gate, immutable prepared state, driver hand-off
- `crates/swallowtail-adapter-cursor/src/lib.rs`: public export
- `crates/swallowtail-adapter-cursor/tests/headless_ask_suite.rs` (new,
  registered in `Cargo.toml`), `tests/prepared_suite.rs`, `tests/plan.rs`, and
  `tests/headless_suite.rs`: driver, prepared, and argv acceptance for
  dispatch, defaults, rejection, and model composition
- `release-baselines/public-api-unreleased/swallowtail-adapter-cursor.txt`
- `docs/research/224-cursor-headless-ask-mode-evidence.md`, cards 213-215,
  g04.077, programme, triage, indexes, Next Task, guide, route and feature
  matrices, architecture, changelog, this closeout

No shared contract or runtime change. No sibling-route change. No currentness
movement.

## Validation

Passed:

- `cargo fmt -p swallowtail-adapter-cursor`
- `effigy validate:focused swallowtail-adapter-cursor`
- `effigy package:verify-affected swallowtail-adapter-cursor`
- `effigy check:examples`
- `effigy package:api`
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `effigy doctor`
- `git diff --check`

`effigy doctor` matches the inherited baseline: `scan.god-files` 380 findings
(334 warnings, 46 errors) and `scan.generated-in-src` one warning. Adding the
Ask coverage first pushed `tests/headless_suite.rs` past the 400-code-line
error threshold; splitting it into `tests/headless_ask_suite.rs` returned the
scan to baseline rather than recording new drift. `tests/prepared_suite.rs`
was already an error finding before this lane and remains one.

Evidence used no install, host-binary replacement, login, account inspection,
catalogue, provider prompt, tool execution, paid work, ambient config
mutation, or live model run. Archives were extracted under `mktemp -d`, and
parser cases used an isolated `HOME` and `CURSOR_CONFIG_DIR` that left
`~/.cursor/cli-config.json` unmodified.

## Continuation

Keep g04 open. No ready lane remains. Reassess the remaining per-route feature
inventory for the next serial lane unless the operator supplies a different
direction. Contract 029 currentness remains standing. Do not compile the next
family from this closeout. Merge remains a separate operator-authorised action.
