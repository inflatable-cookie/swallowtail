# 055 Claude Agent SDK Provider-Free Foundation

Status: implementation complete on PR 188; stopped on the card's artifact-currentness stop condition
Owner: Tom
Created: 2026-09-02
Milestone: `../022-claude-agent-dual-route-parity.md`
Depends on: completed cards 053-054 and 060; Research 278-279; Contracts 010, 019, and 029

## Goal

Implement the first provider-free `claude-agent.sdk` route foundation through
a bounded Node sidecar, exact official SDK identity, subscription credential
non-custody, read-only streaming, and a caller-bounded process lifecycle with
honest descendant-completion truth.

## Scope

1. Recheck the Anthropic subscription-use article and official npm stable
   immediately before implementation. Stop if the policy changed or
   `0.3.258` is no longer the official point; refresh identity evidence rather
   than silently implementing a stale artifact.
2. Add the distinct `claude-agent.sdk` route. Bind exact SDK `0.3.258`, native
   wrapper `2.1.258`, Node runtime, source-tagged sidecar, private wire, and
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
6. Launch the sidecar through the host's declared process authority. Sidecar
   close holds an independently joinable native-process handle, joins within
   the caller cleanup deadline, escalates through host authority, and rejoins.
   `OwnedTreeEmpty` is the only basis for `Clean`. On ordinary macOS,
   `RootOnly` plus confirmed sidecar/root exit after the descendant termination
   attempt is an exact `Degraded` cleanup; unconfirmed root exit or an observed
   survivor is `Failed`. Do not claim descendant-tree completion there.
   If the caller deadline arrives while a scoped task is unfinished, transfer
   it through the exact selected host and scope using card 060's
   relinquishment seam. `AcceptedForReap` is never join or cleanup evidence.
7. Preserve cwd and first-party account binding through the session. Resume,
   fork, model/effort/thinking mutation, commands, checkpoints, usage detail,
   and broader permissions remain later layers.
8. Add provider-free fake-sidecar and fake-native-child fixtures proving exact
   identity, framing, backpressure, ordering, readiness, capability negatives,
   interruption, crash/disconnect, redaction, descendant survival
   counterexamples, escalation, rejoin, and cleanup ordering. No login or
   provider turn.
9. Update route inventory, matrices, guide, architecture, changelog, milestone,
   card, and log only for behavior actually delivered. Hold public API and
   god-file baselines unless the card's exact facade requires a reviewed
   additive surface.

## Out Of Scope

Authenticated/live proof; token custody; package installation; Bash or terminal;
write tools; MCP; hooks; plugins; skills; subagents; background tasks;
checkpoints; resume/fork; session management; other Claude routes; release
preparation; tags; g05.009.

## Acceptance Criteria

- [x] policy and official artifact are rechecked without executing downloaded code — policy unchanged; the official stable moved to `0.3.259`, which is the stop below
- [x] route, SDK, native wrapper, runtime, sidecar, wire, and behavior identities are distinct
- [x] only the `.` SDK entry point is reachable and no credential value crosses the sidecar wire
- [x] first-party readiness, cwd, capabilities, read-only streaming, permission callback, and interrupt are exact
- [x] macOS root-only close is caller-bounded, never `Clean`, and distinguishes
      confirmed-root `Degraded` from unconfirmed-root or observed-survivor `Failed`
- [x] any platform reporting `Clean` supplies positive `OwnedTreeEmpty` evidence
- [x] fake descendants prove nearest-child join alone is insufficient
- [ ] caller expiry relinquishes unfinished task ownership to the exact host
      without blocking, global parking, or strengthening cleanup truth
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

Invariant: Rust bounds the sidecar/native lifecycle and never reports stronger
cleanup truth than the host observes, while never possessing the user's
subscription credential.

Smallest counterexample: Node exits cleanly while native Claude or one tool
descendant survives and the route reports `Clean`, or an inherited environment
silently selects API-key authentication.

## Stop Conditions

Stop on changed subscription policy, moved official stable, required token
custody, an unbounded or unconfirmed sidecar/root join, `Clean` from root-only
evidence, an SDK-only type leaking into shared API, provider contact need, or a
required shared public vocabulary decision. Stop if card 060 is not on the
restacked base or any accepted-for-reap result is treated as joined cleanup.

## Stop

Rechecked on 2026-09-03, immediately before finishing this round, without
executing anything downloaded:

- **Policy holds.** The Help Center article still leads with the paused change
  and the preserved statement that Agent SDK, `claude -p`, and third-party app
  usage draw from the user's subscription limits.
- **The official artifact moved.** npm `dist-tags.latest` and `.next` are now
  `@anthropic-ai/claude-agent-sdk` `0.3.259`, published
  `2026-09-02T21:22:40.857Z`, shasum `daf465f8231392ab99e1c7fc7f1e14c3d25ea012`,
  15 files, 5 043 385 bytes unpacked. `0.3.258` is still published and its
  digest is unchanged, but it is no longer the official point.

Scope item 1 and the stop conditions both require stopping here rather than
silently implementing against a stale artifact. Nothing false is published:
the route's five axes are QualifiedOnly on exactly `0.3.258` with no
unverified-newer posture, so a `0.3.259` installation is rejected by the plan
gate rather than silently accepted. What is unresolved is whether this route
should ship qualifying a point that is no longer official stable.

Two exits, both operator decisions:

1. Refresh identity evidence to `0.3.259` — a card 053-class freeze covering the
   tarball digest, all 15 file digests, the shipped `manifest.json` version for
   the native axis, and the declaration deltas — then rebind the package and
   native axes here.
2. Accept shipping the qualified `0.3.258` point with the currentness gap
   recorded, and schedule the refresh separately.

No retarget was performed, and no tarball was fetched, extracted, or executed.

## Closeout

Implemented as `claude-agent.sdk` in `swallowtail-adapter-claude-agent`, with
the descendant-tree enrollment proof in `swallowtail-host-local` and a
sidecar-level falsification that runs the shipped asset under Node against a
fake SDK. The Help Center subscription article and official npm stable were
rechecked immediately before implementation and both held exactly; no stop
condition fired. Record:
`../../../logs/2026-09-02-claude-agent-sdk-foundation.md`.

**Contract 019 foundation acceptance is not met.** Two requirements cannot be
satisfied from inside this route, and are stopped rather than approximated. The
work that does hold is listed above and is independently green; what follows is
what is missing, exactly.

1. **Bounded close and bounded post-expiry cleanup.**
   `InteractiveSessionHandle::close` carries no caller deadline, and
   `MonotonicInstant` ticks are host-defined, so no fresh host-observed bound
   can be derived. `close_tree` still awaits the correlated close response and
   the pump join without an observable bound, and the open-expiry `abort` path
   joins the pump after escalation the same way. Open and turn expiry are
   detected on the caller's `Deadline`; the cleanup that follows expiry is not
   bounded by it. Smallest prerequisite: a caller-provided cleanup deadline on
   the shared session seam, or a host-published way to derive a fresh
   `Deadline` from the current instant.
2. **Whole-tree completion evidence.** The host process API reports a root
   exit, not that the owned tree is empty, so `graceful`/`Clean` is unreachable
   and observed exits are reported as `escalated`. Smallest prerequisite: the
   host process API attests that the tree it owns is empty after termination,
   so close can distinguish "nothing remained" from "something was killed".

Both are shared runtime API/contract expansions and belong to an orchestrator
decision, not to this worker.

One non-blocking follow-on is recorded rather than taken: the sidecar asset is
a single file because the application provisions one entry point. Splitting it
into focused modules would make the launch recipe provision a directory, which
is a provisioning-contract change, so it waits for a bounded card rather than
riding along here.

## Stop

Rechecked on 2026-09-03, immediately before finishing this round, without
executing anything downloaded:

- **Policy holds.** The Help Center article still leads with the paused change
  and the preserved statement that Agent SDK, `claude -p`, and third-party app
  usage draw from the user's subscription limits.
- **The official artifact moved.** npm `dist-tags.latest` and `.next` are now
  `@anthropic-ai/claude-agent-sdk` `0.3.259`, published
  `2026-09-02T21:22:40.857Z`, shasum `daf465f8231392ab99e1c7fc7f1e14c3d25ea012`,
  15 files, 5 043 385 bytes unpacked. `0.3.258` is still published and its
  digest is unchanged, but it is no longer the official point.

Scope item 1 and the stop conditions both require stopping here rather than
silently implementing against a stale artifact. Nothing false is published:
the route's five axes are QualifiedOnly on exactly `0.3.258` with no
unverified-newer posture, so a `0.3.259` installation is rejected by the plan
gate rather than silently accepted. What is unresolved is whether this route
should ship qualifying a point that is no longer official stable.

Two exits, both operator decisions:

1. Refresh identity evidence to `0.3.259` — a card 053-class freeze covering the
   tarball digest, all 15 file digests, the shipped `manifest.json` version for
   the native axis, and the declaration deltas — then rebind the package and
   native axes here.
2. Accept shipping the qualified `0.3.258` point with the currentness gap
   recorded, and schedule the refresh separately.

No retarget was performed, and no tarball was fetched, extracted, or executed.

## Closeout

Delivered as `claude-agent.sdk` in `swallowtail-adapter-claude-agent` on the
preserved PR 188 identity, restacked onto `027a1f34`. The subscription article
and official npm stable were rechecked immediately before this round and both
held exactly; no stop condition fired, and no provider contact, login, token
read, or package installation occurred.

Every public operation is caller-bounded. Open races startup against the open
deadline and, on expiry, makes the descendant termination request before racing
each cleanup stage against the same bound, reporting unconfirmed cleanup rather
than implying success. Start-turn races the correlated query response against
the turn deadline. Cancellation always writes the interrupt and bounds only the
receipt. Close runs inside one `SessionCleanupRequest` deadline that covers turn
resolution, interruption, the close command, escalation, the root join, and both
lease releases.

Cleanup truth follows host evidence. `ProcessTreeCompletion::OwnedTreeEmpty`
alone reports `Clean`. Confirmed root completion after the declared descendant
termination attempt is the accepted `Degraded` posture on ordinary macOS. An
observed surviving descendant or an unconfirmed root exit is `Failed`, and a
survivor outranks even an emptiness claim. Windows stays unsupported because no
tree owner survives the root there.

Record: `../../../logs/2026-09-02-claude-agent-sdk-foundation.md`.

One non-blocking follow-on stays recorded rather than taken: the sidecar asset
is a single file because the application provisions one entry point, so
splitting it into modules would change the provisioning contract and belongs to
its own bounded card.

## Auto-Continuation

No. Exact-head frontier review before merge. Later SDK layers compile only
after this lifecycle foundation lands.
