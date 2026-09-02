# 055 Claude Agent SDK Provider-Free Foundation

Status: blocked; PR 188 preserved behind the unresolved macOS owned-tree posture decision
Owner: Tom
Created: 2026-09-02
Milestone: `../022-claude-agent-dual-route-parity.md`
Depends on: completed cards 053-054; Research 278-279; Contracts 019 and 029

## Goal

Implement the first provider-free `claude-agent.sdk` route foundation through
a bounded Node sidecar, exact official SDK identity, subscription credential
non-custody, read-only streaming, and a host-owned joined descendant lifecycle.

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
6. Launch the sidecar through the host's descendant-tree authority. Prove
   enrollment on each supported platform. Sidecar close holds an independently
   joinable native-process handle, joins to a declared bound, escalates through
   host authority, rejoins, and reports `graceful`, `escalated`, or
   `unconfirmed`; the last is cleanup failure.
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
- [ ] full descendant-tree close yields one of three explicit outcomes; unconfirmed fails
- [ ] fake descendants prove nearest-child join alone is insufficient
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

Invariant: Rust owns and confirms the whole sidecar/native descendant lifecycle
while never possessing the user's subscription credential.

Smallest counterexample: Node exits cleanly while native Claude or one tool
descendant survives, or an inherited environment silently selects API-key
authentication.

## Stop Conditions

Stop on changed subscription policy, moved official stable, required token
custody, inability to prove platform descendant enrollment, an SDK-only type
leaking into shared API, provider contact need, or a required shared public
vocabulary decision.

## Auto-Continuation

No. Exact-head frontier review before merge. Later SDK layers compile only
after this lifecycle foundation lands.
