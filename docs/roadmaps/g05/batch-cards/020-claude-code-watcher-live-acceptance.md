# 020 Claude Code Watcher Live Acceptance

Status: complete; live evidence stop after one authorized Linux Haiku turn
Owner: Tom
Created: 2026-08-30
Updated: 2026-08-31
Milestone: `../007-claude-watcher-live-acceptance.md`
Depends on: merged g05.006 card 019; Contracts 044, 059, and 060

## Goal

Repair the live probe's platform-specific native identity check, then run the
repaired Claude watcher acceptance selector exactly once on the selected
`linux-x86_64` host against exact Claude Code `2.1.251` and exact model
`claude-haiku-4-5`. Close with either one exact live-proved watcher claim or one
sanitized evidence stop. Never rerun, fall back, dispatch Darwin, or weaken the
same-turn oracle.

## Authorization Envelope

The operator authorized one fresh provider turn on 2026-08-30, then selected
the Linux envelope on 2026-08-31 after the first worker stopped before contact.
The turn is now consumed. The complete bounds were:

- exact installed Claude Code `2.1.251`
- exact `linux-x86_64` host and official `linux-x64` native SHA-256
  `fd5f10ff0eb58daec04900466b143ea98aab50abf208a422bc008eaec13f61f7`
- exact model `claude-haiku-4-5`, not a moving alias
- existing local subscription state; no API-key billing, credential inspection,
  login, installation, update, or ambient settings mutation
- one `90`-second operation deadline
- one provider turn through `effigy probe:claude-code-watcher-live`
- no model fallback and no automatic or review-driven rerun
- no Darwin dispatch; the former `darwin-arm64` digest is not the Linux
  envelope and cannot satisfy this authorization

Version/digest checks and credential-free validation do not consume the turn.
Any request that reaches Claude consumes it regardless of the result.

## Scope

1. Start from the pushed handoff commit in a clean dedicated worker worktree.
   Repair only the live probe's native digest selection before contact: retain
   the frozen `darwin-arm64` value, add the frozen `linux-x64` value, select by
   the actual target platform, and fail closed on unsupported targets.
2. Add credential-free proof that Linux selects only the `linux-x64` digest,
   Darwin ARM64 selects only the existing digest, and an unsupported target
   cannot fall through to either value. Do not change production code, the
   prompt, model, deadline, lifecycle oracle, or claim surfaces in this repair.
3. Commit the bounded probe repair. Validate that repair and the card 019
   implementation without running a provider request. The live run must start
   from that clean committed worker head.
4. Re-probe the installed path, exact `2.1.251` version, selected
   `linux-x86_64` host, and exact `linux-x64` digest. Confirm
   `ANTHROPIC_API_KEY` is absent. Stop before contact on any drift or setup need.
5. Run only `effigy probe:claude-code-watcher-live`, once. Do not run any other
   Claude prompt, response-only probe, direct `claude -p`, or substitute test.
6. Retain only bounded sanitized proof facts. Never retain or publish prompt
   text, raw provider/HTTP payload, endpoint, bearer, credential, path, command,
   argument, environment, PID, watcher output, or source artifact.
7. After the consumed attempt, update this card, g05.007, one outcome log,
   indexes, and the sole Next Task. Return one PR whether the result proves the
   exact claim or stops honestly.
8. Only after every live oracle row passes may the worker update the Claude
   integration guide, route/activity/feature matrices, and other existing
   claim surfaces for the exact proved point. A failed or ambiguous turn keeps
   all watcher claims absent.

No production repair belongs to this card. The exact platform-selection repair
above is the only pre-contact code authority. If the runtime, contract surface,
prompt, lifecycle oracle, or any other behavior needs a change, stop and return
to planning without contacting Claude.

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
- **Platform identity invariant:** the live probe selects one frozen digest from
  the actual target platform before preparation. On this authorized Linux host,
  only the official `linux-x64` digest is accepted.
- **Platform counterexample:** Linux accepts the Darwin digest, either supported
  platform accepts both values, or an unsupported platform silently falls back
  to one. Each must fail in credential-free proof before contact.

## Live Evidence Stop

The per-platform digest repair committed at `adb04f17`. Credential-free proof
selects only the frozen `linux-x64` digest on rustc `linux`/`x86_64` and only
the existing `darwin-arm64` digest on rustc `macos`/`aarch64`. Unsupported
targets, including npm-style `darwin`/`arm64`, do not fall through.

Pre-contact validation and the exact Linux envelope passed on that clean head.
One `effigy probe:claude-code-watcher-live` turn then ran with exact
`claude-haiku-4-5`. The ordered recorder kept one fact for turn
`claude-code-headless:live-claude-code-watcher`: `JoinedZero`. It did not
record MCP initialize, reserved tool discovery, watcher start, Stop-hook start,
Stop-attributed completion-gate, Stop-hook response, same-session continuation,
explicit wait or stop, or provider success. No session correlation was
observed. Terminal cleanup produced `JoinedZero`; provider success did not.

No watcher capability, matrix, or integration-guide claim follows. The attempt
is consumed. Card 011, g05.003, and PR 127 stay unchanged. No second provider
turn, Darwin dispatch, prompt change, or merge is authorized.

## Acceptance Criteria

- [x] the bounded per-platform probe repair is committed and credential-free
      proof rejects cross-platform or unsupported-target fallback
- [x] pre-contact validation and exact Linux identity/digest checks pass on a
      clean committed worker head
- [x] exactly one provider turn is run with exact `claude-haiku-4-5`; no other
      live selector, prompt, fallback, or retry runs
- [x] the live proof either satisfies the full ordered oracle or records the
      exact bounded missing/reordered fact as a sanitized stop
- [ ] successful proof includes complete HostWatcher lifecycle activity and
      clean joined cleanup; terminal text alone is insufficient
- [x] no private or raw material enters logs, diagnostics, fixtures, docs, PR
      text, or consumer events
- [x] route claims remain exact to the live-proved version/model point and are
      published only after the full oracle passes
- [x] one outcome PR reconciles card, milestone, logs, indexes, and Next Task
      without authorizing merge or another provider turn

## Validation

Before provider contact:

- `cargo fmt -p swallowtail-runtime -p swallowtail-host-local -p swallowtail-adapter-claude-agent -p swallowtail-testkit -- --check`
- `effigy validate:focused swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent swallowtail-testkit`
- `effigy package:verify-affected swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent swallowtail-testkit`
- `effigy package:api`
- `cargo test -p swallowtail-adapter-claude-agent --features live-probes --test live_watcher_probe`
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

- any Linux authorization-envelope precondition fails before contact
- source or probe changes beyond the bounded platform-selection repair are
  required before a meaningful live attempt
- the repair head is uncommitted, dirty, or not the exact head validated before
  contact
- any provider request has already occurred in this handoff
- the trace lacks watcher start, active Stop attribution, same-session re-entry,
  explicit wait/stop, joined zero, complete activity, clean terminal, or joined
  cleanup
- proof requires raw/private evidence or a wider model/version statement
- validation exposes a contract, product, or architecture change

## Auto-Continuation

No. One provider turn is the complete budget. Return one reviewable evidence PR
and stop. No rerun, Darwin dispatch, merge, new card, route-feature promotion,
or second provider session follows automatically.
