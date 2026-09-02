# 055 Claude Agent SDK Provider-Free Foundation

Status: ready after card 060 lands; PR 188 preserved
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

- [ ] policy and official artifact are rechecked without executing downloaded code
- [ ] route, SDK, native wrapper, runtime, sidecar, wire, and behavior identities are distinct
- [ ] only the `.` SDK entry point is reachable and no credential value crosses the sidecar wire
- [ ] first-party readiness, cwd, capabilities, read-only streaming, permission callback, and interrupt are exact
- [ ] macOS root-only close is caller-bounded, never `Clean`, and distinguishes
      confirmed-root `Degraded` from unconfirmed-root or observed-survivor `Failed`
- [ ] any platform reporting `Clean` supplies positive `OwnedTreeEmpty` evidence
- [ ] fake descendants prove nearest-child join alone is insufficient
- [ ] caller expiry relinquishes unfinished task ownership to the exact host
      without blocking, global parking, or strengthening cleanup truth
- [ ] provider-free fixtures cover bounds, ordering, failure, redaction, cancellation, and cleanup
- [ ] existing ACP and Claude Code behavior and claims are unchanged
- [ ] route/docs/API/god-file gates hold

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

## Auto-Continuation

No. Exact-head frontier review before merge. Later SDK layers compile only
after this lifecycle foundation lands.
