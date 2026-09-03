# 055 Claude Agent SDK Provider-Free Foundation

Status: complete; restored on card 061 reservations with one enclosing cleanup guardian and merged through PR 196 at `493f8194`
Owner: Tom
Created: 2026-09-02
Updated: 2026-09-03
Milestone: `../022-claude-agent-dual-route-parity.md`
Depends on: completed cards 053-054 and 060; reviewed and merged g05.025 card 061;
  Research 278-279; Contracts 009, 010, 017, 019, 029, and 047

## Goal

Restore the withdrawn provider-free `claude-agent.sdk` route foundation through
a bounded Node sidecar, exact official SDK identity, subscription credential
non-custody, read-only streaming, and a caller-bounded process lifecycle with
honest descendant-completion truth.

## Scope

1. Start from canonical main only after card 061 merges and passes independent
   exact-head review. Recheck the Anthropic subscription-use article and
   official npm stable immediately before implementation. Refresh the frozen
   identity evidence rather than copying PR 188's now-historical version point
   or bulk-bumping from `latest`. Both were rechecked on `2026-09-03`: the
   article still leads with the paused-changes notice at its stated
   `June 16, 2026` update, and official `latest`/`next` is still exactly
   `0.3.259`, so the frozen ledger is current rather than carried forward.
2. Add the distinct `claude-agent.sdk` route. Bind the newly frozen exact SDK,
   native wrapper, Node runtime, source-tagged sidecar, private wire, and
   behavior revision independently. Start with one exact QualifiedOnly point;
   do not inherit ACP or Claude Code claims.
3. Ship a small source-tagged JavaScript sidecar asset. The application
   provisions Node, the `.` SDK entry point, and platform package. Swallowtail
   never installs, vendors, updates, repairs, or redistributes them.
4. Use explicit runtime, native binary, cwd, environment, settings sources,
   skills, persistence, model, permission, and read-only tool inputs. Prohibit
   `/bridge`, `/browser`, raw token fields, API-key helpers, cloud auth refresh,
   ambient settings, Bash, terminal, write tools, MCP, plugins, hooks,
   subagents, and experimental surfaces.
5. Implement strict bounded correlated framing for open/query input, streamed
   system/output/activity, runtime capabilities, first-party account readiness,
   `canUseTool`, interrupt, close, and terminal failure. Keep raw SDK values,
   credentials, paths, and payloads out of public records and diagnostics.
6. Before any credential, resource, process, task, or provider work, obtain
   card 061's operation-scoped reap reservation from the exact selected task
   service. A boolean capability probe is insufficient. Launch one enclosing
   guardian task through the host's declared task authority. The guardian owns
   the pump, native-process handle, working-resource lease, and credential lease
   and preserves this cleanup order: interrupt → native close → force-stop →
   root/process observation → pump completion/join → resource release →
   credential release. At the caller deadline, transfer the enclosing guardian,
   not the pump, through the held exact-host/exact-scope reservation. Sidecar
   close joins within the caller cleanup deadline, escalates through host
   authority, and rejoins.
   `OwnedTreeEmpty` is the only basis for `Clean`. On ordinary macOS,
   `RootOnly` plus confirmed sidecar/root exit after the descendant termination
   attempt is an exact `Degraded` cleanup; unconfirmed root exit or an observed
   survivor is `Failed`. Do not claim descendant-tree completion there.
   `AcceptedForReap` is never join or cleanup evidence. The transferred
   guardian keeps process and leases until eventual ordered cleanup; the caller
   reports honest unresolved/failed or route-qualified degraded cleanup without
   waiting past its deadline.
7. Preserve cwd and first-party account binding through the session. Resume,
   fork, model/effort/thinking mutation, commands, checkpoints, usage detail,
   and broader permissions remain later layers.
8. Add provider-free fake-sidecar and fake-native-child fixtures proving exact
   identity, framing, backpressure, ordering, readiness, capability negatives,
   interruption, crash/disconnect, redaction, descendant survival
   counterexamples, escalation, rejoin, and cleanup ordering. The integrated
   deadline proof must use real `LocalHostServices` so the worker is retained,
   transferred, reaped, and joined by the outer host owner. No fixture may
   discard its worker handle. No login or provider turn.
9. Update route inventory, matrices, guide, architecture, changelog, milestone,
   card, and log only for behavior actually delivered. Hold public API and
   god-file baselines unless the card's exact facade requires a reviewed
   additive surface. That restoration passed independent exact-head review and
   merged through PR 196 at `493f8194`.

## Out Of Scope

Authenticated/live proof; token custody; package installation; Bash or terminal;
write tools; MCP; hooks; plugins; skills; subagents; background tasks;
checkpoints; resume/fork; session management; other Claude routes; release
preparation; tags; g05.009.

## Acceptance Criteria

- [x] policy and official artifact are rechecked without executing downloaded code
- [x] route, SDK, native wrapper, runtime, sidecar, wire, and behavior identities are distinct
- [x] only the `.` SDK entry point is reachable and no credential value crosses the sidecar wire
- [x] first-party readiness, cwd, capabilities, read-only streaming, permission callback, and interrupt are exact
- [x] macOS root-only close is caller-bounded, never `Clean`, and distinguishes
      confirmed-root `Degraded` from unconfirmed-root or observed-survivor `Failed`
- [x] any platform reporting `Clean` supplies positive `OwnedTreeEmpty` evidence
- [x] fake descendants prove nearest-child join alone is insufficient
- [x] unsupported or closing selected hosts reject the reap reservation before
      credential, resource, process, task, or provider work
- [x] caller expiry transfers the enclosing guardian to the exact host and
      scope without blocking, global parking, lease release around unfinished
      work, or strengthened cleanup truth
- [x] a shutdown race after reservation grant cannot refuse the valid handoff
- [x] real `LocalHostServices` proves retained worker transfer, eventual reap,
      outer reaper join, and the synchronous-drop counterexample
- [x] provider-free fixtures cover bounds, ordering, failure, redaction, cancellation, and cleanup
- [x] existing ACP and Claude Code behavior and claims are unchanged
- [x] route/docs/API/god-file gates hold

## Validation

```sh
cargo fmt -p swallowtail-adapter-claude-agent
effigy validate:focused swallowtail-adapter-claude-agent swallowtail-host-local
effigy package:verify-affected swallowtail-adapter-claude-agent swallowtail-host-local
effigy package:api
effigy qa:routes
effigy qa:docs
effigy qa:northstar
effigy --json scan god-files
git diff --check
```

Do not run live probes, provider sessions, release commands, or broad workspace
tests. Add another package only if the implementation necessarily changes it
and report why.

## Review Oracle

Invariant: Rust obtains guaranteed host-reap authority before effects, keeps the
whole sidecar lifecycle and both leases inside one enclosing guardian, and never
reports stronger cleanup truth than the host observes while never possessing
the user's subscription credential.

Smallest counterexample: shutdown starts after a boolean support probe and late
guardian transfer is refused, so dropping the real local handle blocks past the
caller deadline; or the pump transfers while resource and credential leases are
released around the still-live process.

## Stop Conditions

Stop on changed subscription policy, unavailable fresh identity evidence,
required token custody, an unbounded or unconfirmed sidecar/root join, `Clean`
from root-only evidence, an SDK-only type leaking into shared API, provider
contact need, or a required shared public vocabulary decision. Stop if card 061
is not on canonical main; reservation occurs after effects; deadline transfers
only the pump; leases release before guardian terminal cleanup; the integrated
proof does not use real `LocalHostServices`; or accepted-for-reap is treated as
joined cleanup.

## Auto-Continuation

No. Exact-head frontier review accepted in PR 196. Release readiness is
unpaused with card 050 ready as the sole Next Task for a fresh g05.021 audit.

## Containment And Restoration

PR 188 merged at `ff7ec3d8` despite rejected exact-head review of
`6f102f83`. The review found that refused relinquishment can synchronously join
on drop past the caller deadline, leases can be released while transferred work
still runs, and the integrated fixture does not retain or reap its worker. The
containment change forward-reverted PR 188's tree delta and withdrew
`claude-agent.sdk`.

This restoration re-enters from canonical main after g05.025 card 061 merged at
`53153af1`, and repairs all three findings rather than reinstating the reverted
head:

- Cleanup authority is taken first. `open_session` reserves the open-guardian,
  pump, and close-guardian lanes from the exact selected task service *and
  starts both guardian tasks* before any credential, resource, process, or
  provider effect. The later transfer is therefore non-fallible while the work
  is unfinished, and the later activation cannot fail at all, so no refusal path
  can drop a live handle and block the caller.
- One enclosing guardian owns the whole ordered continuation. Close hands the
  connection, process, pump, remaining turn-deadline task, and both leases to
  that guardian *before the public cleanup future exists*, so a runtime that
  refuses the future without polling it cannot strand live state. The guardian
  runs interrupt, native close, force-stop, root observation, pump join,
  resource release, and credential release in that order. Caller expiry, caller
  cancellation, and a dropped or rejected cleanup future all transfer that
  guardian — never the pump alone — through its `Drop`, and no lease is released
  around still-live transferred work. Cancelling a pending open additionally
  releases the open guard's cleanup signal before the handoff, so its credential
  and working resource are released at once rather than at the abandoned open
  deadline.
- The integrated deadline proof runs on real `LocalHostServices`. The
  provider-free fixture also retains every worker handle, joins on drop, and
  reaps only through an outer owner.

`AcceptedForReap` remains transfer only, never join or cleanup success. PR 196
merged at `493f8194` after independent exact-head review, unpausing the `v0.4.0`
release lane and g05.021/card 050 for a fresh audit.
