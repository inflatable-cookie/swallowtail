# 020 Claude Code Watcher Live Acceptance

Status: complete; pre-contact evidence stop; the authorized turn is unconsumed
Owner: Tom
Created: 2026-08-30
Updated: 2026-08-30
Milestone: `../007-claude-watcher-live-acceptance.md`
Depends on: merged g05.006 card 019; Contracts 044, 059, and 060

## Goal

Run the repaired Claude watcher acceptance selector exactly once against exact
Claude Code `2.1.251` and exact model `claude-haiku-4-5`. Close with either one
exact live-proved watcher claim or one sanitized evidence stop. Never rerun,
fall back, or weaken the same-turn oracle.

## Outcome

Stopped before provider contact on 2026-08-30. Three envelope and validation
gates failed, so the selector never ran and no request reached Claude. The
authorized turn remains available. See
[the pre-contact stop log](../../../logs/2026-08-30-g05-007-card-020-pre-contact-stop.md).

- installed `claude --version` is exact `2.1.251`, `ANTHROPIC_API_KEY` is
  absent, and the source tree is clean and unchanged
- the frozen native SHA-256 in this card is Research 261's `darwin-arm64`
  value; the worker host is `linux-x86_64`, whose official `2.1.251` digest is
  `fd5f10ff0eb58daec04900466b143ea98aab50abf208a422bc008eaec13f61f7`. The
  probe hard-codes the `darwin-arm64` constant and asserts it before
  preparation
- `effigy package:verify-affected` fails on unchanged `main`: `Cargo.lock`
  pins yanked `chacha20 0.10.1`
- `effigy package:api` fails because `cargo-public-api 0.52.0` is absent

Widening the digest constant is a probe change and installing the API tool is
a setup need. Both are out of scope before contact, so all three return to
planning.

## Authorization Envelope

The operator authorized one fresh provider turn on 2026-08-30 with all of the
following bounds:

- exact installed Claude Code `2.1.251`
- frozen native SHA-256
  `625869b01e0050f260b2980fac248fd9cef9e462612bded4ec9d3d49ff8969a5`
- exact model `claude-haiku-4-5`, not a moving alias
- existing local subscription state; no API-key billing, credential inspection,
  login, installation, update, or ambient settings mutation
- one `90`-second operation deadline
- one provider turn through `effigy probe:claude-code-watcher-live`
- no model fallback and no automatic or review-driven rerun

Version/digest checks and credential-free validation do not consume the turn.
Any request that reaches Claude consumes it regardless of the result.

## Scope

1. Start from the pushed handoff commit in a clean dedicated worker worktree.
   Do not modify the live probe or production code before contact.
2. Validate the unchanged card 019 implementation and compile the ignored live
   probe without running it. Confirm the source tree remains clean.
3. Re-probe the installed path, exact `2.1.251` version, and frozen digest.
   Confirm `ANTHROPIC_API_KEY` is absent. Stop before contact on any drift or
   setup need.
4. Run only `effigy probe:claude-code-watcher-live`, once. Do not run any other
   Claude prompt, response-only probe, direct `claude -p`, or substitute test.
5. Retain only bounded sanitized proof facts. Never retain or publish prompt
   text, raw provider/HTTP payload, endpoint, bearer, credential, path, command,
   argument, environment, PID, watcher output, or source artifact.
6. After the consumed attempt, update this card, g05.007, one outcome log,
   indexes, and the sole Next Task. Return one PR whether the result proves the
   exact claim or stops honestly.
7. Only after every live oracle row passes may the worker update the Claude
   integration guide, route/activity/feature matrices, and other existing
   claim surfaces for the exact proved point. A failed or ambiguous turn keeps
   all watcher claims absent.

No implementation repair belongs to this card. If the unchanged probe, runtime,
or contract surface needs a change, record the stop and return to planning
without contacting Claude again.

## Review Oracle

- **Invariant:** one opted-in exact Claude turn cannot complete successfully
  while its watcher is active or unjoined. Native Stop must query the active
  completion gate, block completion, and return control to the same provider
  session before the model explicitly waits or stops.
- **Smallest counterexample:** the model starts one approved watcher and emits
  final text while it remains active, but Claude exits successfully; or the
  selector passes after proactive wait, direct gate use, or adapter-only
  terminal rejection without native Stop re-entry.
- **Expected stop:** consume the attempt, preserve bounded sanitized facts,
  withhold the capability and all matrix/guide claims, open the evidence PR,
  and require fresh operator authority for any later provider request.
- **Required proof:** one ordered trace contains MCP initialization and reserved
  tool discovery, watcher start, an active completion-gate response attributed
  to Stop, same-session post-hook activity, explicit wait or stop, zero active
  or unjoined watchers, clean provider success, complete watcher activity, and
  joined provider/bridge/process cleanup. Every fact must belong to the exact
  turn and session.

## Acceptance Criteria

- [ ] pre-contact validation and exact identity/digest checks pass on a clean
      unchanged source tree — failed; digest, `package:verify-affected`, and
      `package:api` are red
- [ ] exactly one provider turn is run with exact `claude-haiku-4-5`; no other
      live selector, prompt, fallback, or retry runs — no turn ran
- [ ] the live proof either satisfies the full ordered oracle or records the
      exact bounded missing/reordered fact as a sanitized stop — no live trace
      exists; the stop is pre-contact
- [ ] successful proof includes complete HostWatcher lifecycle activity and
      clean joined cleanup; terminal text alone is insufficient — not reached
- [x] no private or raw material enters logs, diagnostics, fixtures, docs, PR
      text, or consumer events
- [x] route claims remain exact to the live-proved version/model point and are
      published only after the full oracle passes — no claim published
- [x] one outcome PR reconciles card, milestone, logs, indexes, and Next Task
      without authorizing merge or another provider turn

## Validation

Before provider contact:

- `cargo fmt -p swallowtail-runtime -p swallowtail-host-local -p swallowtail-adapter-claude-agent -p swallowtail-testkit -- --check`
- `effigy validate:focused swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent swallowtail-testkit`
- `effigy package:verify-affected swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent swallowtail-testkit`
- `effigy package:api`
- `cargo test -p swallowtail-adapter-claude-agent --features live-probes --test live_watcher_probe --no-run`
- `git diff --check`

Authorized live evidence, once and only once after the prior checks pass:

- `effigy probe:claude-code-watcher-live`

After the outcome edits:

- `effigy qa:routes` only if route or feature matrices change
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

Do not run broad workspace `qa`, another live probe, or another Claude command
that supplies a prompt.

## Stop Conditions

- any authorization-envelope precondition fails before contact
- source or probe changes are required before a meaningful live attempt
- any provider request has already occurred in this handoff
- the trace lacks watcher start, active Stop attribution, same-session re-entry,
  explicit wait/stop, joined zero, complete activity, clean terminal, or joined
  cleanup
- proof requires raw/private evidence or a wider model/version statement
- validation exposes a contract, product, or architecture change

## Auto-Continuation

No. One provider turn is the complete budget. Return one reviewable evidence PR
and stop. No rerun, merge, new card, route-feature promotion, or second provider
session follows automatically.

The stop does not authorize a worker-side repair. The digest platform question
and the two environment validation failures return to the orchestrator as
planning findings.
