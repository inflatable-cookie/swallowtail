# 055 Claude Agent SDK Provider-Free Foundation

Status: complete; caller-bounded lifecycle on card 060 relinquishment, accepted root-only degraded cleanup, and the `0.3.259` identity on PR 188
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
2. Add the distinct `claude-agent.sdk` route. Bind exact SDK `0.3.259`, native
   wrapper `2.1.259`, Node runtime, source-tagged sidecar, private wire, and
   behavior revision independently. The points were `0.3.258`/`2.1.258` when
   this card was written; scope item 1 fired and the operator selected the
   refresh, recorded under Artifact Refresh. Start with one exact QualifiedOnly point;
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

- [x] policy and official artifact are rechecked without executing downloaded code — policy unchanged; the official stable moved, and the operator-selected refresh rebound the exact points to `0.3.259` and `2.1.259`
- [x] route, SDK, native wrapper, runtime, sidecar, wire, and behavior identities are distinct
- [x] only the `.` SDK entry point is reachable and no credential value crosses the sidecar wire
- [x] first-party readiness, cwd, capabilities, read-only streaming, permission callback, and interrupt are exact
- [x] macOS root-only close is caller-bounded, never `Clean`, and distinguishes
      confirmed-root `Degraded` from unconfirmed-root or observed-survivor `Failed`
- [x] any platform reporting `Clean` supplies positive `OwnedTreeEmpty` evidence
- [x] fake descendants prove nearest-child join alone is insufficient
- [x] caller expiry relinquishes unfinished task ownership to the exact host
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

## Artifact Refresh

The 2026-09-03 recheck fired the artifact-currentness stop: official npm stable
had moved off `0.3.258`. The operator selected the refresh exit, so the exact
points were rebound rather than the route shipping a stale artifact.

Research 280 and
`../../../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.259/`
hold the frozen evidence. Both tarballs were downloaded to `/tmp`, hashed, and
extracted; nothing was executed, no platform package was fetched, and no
provider session, login, or token read occurred.

- **Clean hop.** `0.3.258` re-hashed to the digest Research 278 froze, and no
  published stable exists between the two points.
- **Deterministic inventory.** 15 files both versions: 7 identical, 8 changed,
  0 added, 0 removed, each with per-file digests.
- **Every changed file classified, none mapped.** The new `permissionPrompts`
  selector is forwarded only when set, so `canUseTool` still governs admission
  and the asset is asserted never to name it; `user_message_uuids` are
  correlation siblings the route does not read; the task `summary` change is
  documentation; `managedMcpServers` is managed-settings tier that
  `settingSources: []` never loads; the `sdk-tools` additions belong to skill
  publishing, which this route prohibits.
- **Boundaries held.** `exports` is byte-identical so the `.` entry rule is
  unchanged, `bridge.d.ts` and `browser-sdk.d.ts` are byte-identical, and the
  ten-pattern credential search returns the same three prose hits with no login
  export.
- **Native rotation without protocol movement.** All eight platform binaries
  rotated with `2.1.258` → `2.1.259`, but `sdkCompat.harnessSchema` stays `1`.

Rebound: `claude-agent.sdk.package` to exact `0.3.259` and
`claude-agent.sdk.native` to exact `2.1.259`. The Node, wire, and sidecar
source-tag axes, every claim id, the `claude-agent.sdk-v1` behavior revision,
and the QualifiedOnly posture are unchanged, and `0.3.258` becomes unqualified
rather than a second supported point.

## Closeout

Delivered as `claude-agent.sdk` in `swallowtail-adapter-claude-agent` on the
preserved PR 188 identity, restacked onto the g05.024 card 060 merge commit
`6543c905`, with the exact identity
refreshed to official `0.3.259` and native `2.1.259` under the operator's
selected exit. No provider contact, login, token read, package installation,
live turn, release mutation, or tag occurred.

Every public operation is caller-bounded through its return, and the host
termination request cannot be skipped by a stalled stage.

Open runs acquisition, launch, and readiness as one future inside the caller's
open deadline. A host-owned guard is armed before the first acquisition and
records every lease, process, and task as it is taken, so expiry can drop the
public future without stranding a partial open: the guard still terminates and
releases, and the caller sees the deadline rather than the collapsing
connection's next error. Claim and cleanup are a single atomic transition, and
cleanup takes the ledger only once the open future can no longer record, so
neither a late recording nor a boundary-time readiness can leave an acquisition
orphaned or let open report success over a terminated process. Start-turn races the correlated query response against
the turn deadline. Turn cancellation bounds both the wire write and the receipt;
session cancellation performs no host call at all, because that seam carries no
caller deadline, and bounded close owns the termination instead. An accepted
turn reaches its terminal outcome at its deadline without waiting on an
interrupt receipt. Close runs inside one `SessionCleanupRequest` deadline that
covers turn resolution, interruption, the close command, the termination
request, the root join, and both lease releases, with a close guard that makes
the termination request on the deadline even if the sidecar accepts input and
never answers.

Cleanup truth follows host evidence. `ProcessTreeCompletion::OwnedTreeEmpty`
alone reports `Clean`. Confirmed root completion after the declared descendant
termination attempt is the accepted `Degraded` posture on ordinary macOS. An
observed surviving descendant or an unconfirmed root exit is `Failed`, and a
survivor outranks even an emptiness claim. Windows stays unsupported because no
tree owner survives the root there.

Tool admission stays fail-closed on both sides without letting a race poison
the transport. An out-of-set tool is still a transport failure. An in-set
request the sidecar had already written when the turn ended is denied on the
wire instead, so the interrupt and close that follow still have a usable
connection.

The open guard owns the whole ordered cleanup: terminate, wait, join the pump,
then release the resource and credential leases, in that Contract 019 order and
inside one host-owned task. Process exit is not evidence that the pump task
ended, so neither lease is released and no cleanup completion is reported until
the pump is actually joined. If the caller's deadline expires first, open
reports `open_cleanup_unconfirmed` and the same guard keeps owning the rest of
the sequence, finishing it without any further call from the route.

Host task joins go through the task seam, not the join. A `JoinedTask::join`
may block the thread it is polled on, and dropping an unfinished handle joins
as well, so the route waits on `is_finished`/`register_waker` and joins only a
task that reports finished. A task still running at the caller's deadline is
transferred through card 060's `ScopedTaskService::relinquish`, using the exact
selected execution host and the exact spawn scope. `AcceptedForReap` is
ownership transfer only: it is never a join and never strengthens cleanup, so a
transferred pump still closes `Failed` on an unconfirmed root. A refused
transfer leaves ordinary join-and-drop ownership unchanged. The host's outer
reaper shutdown stays with the execution-host lifecycle; this route neither
calls nor claims it.

Adversarial proofs cover a stalled credential acquisition, resource resolution,
process start, open cleanup, close response, accepted turn with no interrupt
response, stalled wire write, stalled escalation, and an admission request the
turn's own end raced, plus barrier tests for both open-guard interleavings and
real local-host regressions where a stalled host task crosses the caller
deadline: exact-host and exact-scope transfer, both mismatch refusals, a
finished task that still joins, and a public close whose transferred pump keeps
the exact `Failed` root-unconfirmed truth. Records:
`../../../logs/2026-09-02-claude-agent-sdk-foundation.md` and
`../../../logs/2026-09-03-claude-agent-sdk-0-3-259-identity.md`.

One non-blocking follow-on stays recorded rather than taken: the sidecar asset
is a single file because the application provisions one entry point, so
splitting it into modules would change the provisioning contract and belongs to
its own bounded card.

## Auto-Continuation

No. Exact-head frontier review before merge. Later SDK layers compile only
after this lifecycle foundation lands.
