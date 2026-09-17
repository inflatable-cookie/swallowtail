# g06.006 Command Code 1.54.0 Paid-Model Live Requalification

Status: ready
Owner: Tom
Created: 2026-09-17
Depends on: Contract 029; Contract 043; Research 116, 118, 317; completed g05.068 and g05.069
Vision tags: route currentness, Command Code, live evidence, exact pin

## Outcome

Run one operator-authorized live gate for the exact `command-code.headless`
`1.54.0` point on the paid probe model `deepseek/deepseek-v4-flash`, and settle
the feature and activity cells that depend on live evidence. The exact
`1.54.0` point is unchanged; this lane proves or disproves its live surface.

## Why It Matters

g05.069 closed as a typed stop twice over. First the account refused every
request, including the advertised-free models, with an account-level
`insufficient credits` response. Tom then subscribed to a Command Code tier,
which cleared that, but the free model `meituan/LongCat-2.0:free` reached the
provider and failed the frozen route's decoder with
`swallowtail.command_code.headless.malformed_stream`. That points at the free
backend's event grammar rather than the route. `deepseek/deepseek-v4-flash` is
the CLI's own default and the model Research 116 and 118 actually probed on
`1.15.1`, so it is the right model to test whether the malformed stream is a
free-backend artefact.

## Ready-State Rubric

- [x] g05.069 recorded both typed stops with the exact model named and no
      claim change.
- [x] Tom authorized a paid-model attempt on 2026-09-17 after subscribing.
- [x] The frozen executable is installed and the selected route is proven to
      start on this host.
- [x] Scope, acceptance, validation, evidence, and stop conditions are explicit.
- [x] Review oracle is present because the claim is exact, version-bound, and
      partly negative.
- [x] Official stable is re-probed at the boundary before the claim decision (`latest` `1.55.1`; observation only).

## Dispatch manifest

- **State:** ready; provider-operation lane; one lane; no automatic successor.
- **Completion:** under this one authorized gate, the exact `1.54.0` point on
  `deepseek/deepseek-v4-flash` establishes structured authenticated completion,
  tool lifecycle, usage accounting, and the Contract 043 two-turn private
  exact-id continuation, and the version-bound live feature and activity cells
  are reconciled to that exact tuple; or the gate records a typed stop naming
  the exact model and the version-bound gate is preserved. Focused Command Code
  tests, route and activity/feature matrix QA, and the named docs gates pass;
  independent exact-head review accepts the head.
- **Owned mutable paths:** the Command Code live-evidence record, the Command
  Code feature and activity matrix rows, the prepared-integration guide, and
  this task's result lines; the new Research record and one research-index
  line; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Worker:** live-evidence worker with provider-operation authority limited to
  the one authorized gate below.
- **Excluded:** any second attempt, rerun, fallback, or model substitution;
  install, update, login, host update, consumer mutation, public catalogue,
  import, export, or resume operation outside the gate; and any compatibility
  claim for a version other than exact `1.54.0`.
- **Escalation:** operator via Chatterbox for any provider budget or policy
  question; Queue coordinator for mechanical blockers.

### Authorized gate

One authenticated `command-code.headless` `1.54.0` session on
`deepseek/deepseek-v4-flash`, with the stored `~/.commandcode` account state,
establishing structured completion, tool lifecycle, usage accounting, and the
Contract 043 two-turn private exact-id continuation.

Frozen-executable hazard, learned the hard way in g05.069: this CLI
self-updates, and a model-listing call performed an update that moved the host
from the frozen `1.54.0` to `1.54.2`. Before any provider operation, verify
that `command-code --version` reports exactly `1.54.0` and that `dist/index.mjs`
SHA-256 is
`157feefa0140e78f060ef2c1f9c50d10de702196ea229dc73db6bbcc39a0bcbb`. Do not run
`command-code update`, `/update`, or `--list-models`. The adapter already passes
`--no-auto-update`. If either check drifted, stop and report rather than
repairing it.

The typed credit-failure path was already observed on this exact point and is
excluded from this gate. Official stable has moved to `1.54.2`; stay on exact
`1.54.0`, and record the `1.54.2` availability as an observation note only,
never as a claim change.

## Work

1. Re-probe official npm stable and re-freeze the exact `1.54.0` identity
   boundary before the claim decision.
2. Run the one authorized gate exactly as scoped above; record the exact model
   string in the observation identity and retain no raw provider stream,
   account identifier, prompt, session id, or private path.
3. On acceptance, move only the live-derived feature and activity cells that
   the accepted evidence covers, and reconcile the prepared-integration guide.
   On a typed stop, preserve the version-bound gate and record the reason.
4. Keep Research 116 and 118 immutable and bound to `1.15.1`; they are
   comparison evidence, not authority to rewrite.
5. Keep `QualifiedOnly`, the exact tuple, and private exact-id resume unchanged.

## Boundaries

One exact version, one model, one attempt. No range, release, tag, publication,
or consumer mutation.

## Validation

Focused Command Code tests, route and activity/feature matrix QA, the named
docs gates, and `git diff --check`. The gate supplies its own exact provider
evidence.

## Acceptance

- [x] the exact-`1.54.0` paid-model gate runs or records a typed stop naming the model
- [x] Research 116/118 remain immutable and version-bound
- [x] live feature and activity cells agree with the accepted live outcome
- [x] no second attempt, model substitution, or executable update occurred

## Result

Accepted 2026-09-17. Pre-gate checks held exact `1.54.0`, the frozen
`dist/index.mjs` digest, and disabled auto-update. The one authorized gate ran
once on `deepseek/deepseek-v4-flash`: the structured one-turn probe completed
with the exact reply, clean cleanup, and every streamed event valid, and the
Contract 043 two-turn probe completed both turns with exact replies, private
exact-id resume, and a clean session close. The g05.069 `malformed_stream`
stays a free-backend artefact. Research 330 records the gate; the feature and
activity rows and the prepared-integration guide now rest their `1.54.0` live
basis on it, with credit failure still bound to `1.15.1`. Official stable
`1.55.1` is an observation only. No second attempt, substitution, or update
occurred.

## Next Task

The queue dispatches g06.006 as one bounded live gate. On return, Chatterbox
reconciles the version-bound live cells from the accepted or stopped outcome.
