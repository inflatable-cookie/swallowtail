# 080 Claude SDK Read-Write Session And Permission Policy

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: `v0.4.0` `claude-agent.sdk`; Contracts 017, 019, 023, 041; Research 278 §8 layer 2

## Goal

Let a consumer run a multi-turn editing session on `claude-agent.sdk`: `Edit`,
`Write`, and `MultiEdit` admitted through consumer-mediated per-call
admission on a read-write leased working directory, with `permissionMode`
selectable at open and changeable mid-session. The read-only default is
unchanged.

## Scope

1. Extend `ClaudeAgentSdkSessionPreparation` (or an additive profile input on
   it) with an explicit admitted tool set and a permission mode. Admitted
   values: the existing read set plus `Edit`, `Write`, and `MultiEdit`;
   permission modes `default`, `plan`, `acceptEdits`. Reject
   `bypassPermissions` and any unknown tool name before launch with a typed
   failure.
2. Require `ResourceAccess::ReadWrite` on the working-resource lease when any
   write tool is admitted; reject at preparation otherwise. The lease is a
   location scope under `AmbientHost`; make no bounded-filesystem claim.
3. Sidecar: pass the admitted set as `tools`; never set `allowedTools`; pass
   `permissionMode` at open; add a `set_permission_mode` command on the
   private wire that calls the SDK's mid-session mode change and returns the
   confirmed mode; keep `canUseTool` forwarding the intact tool name and
   input for every call, including writes.
4. Expose `set_permission_mode` on the session handle as an additive method
   that returns the confirmed effective mode; a rejected or unconfirmed
   change is a typed failure, never a silent success.
5. Record the admitted tool set and effective mode as prepared evidence and,
   where the existing Contract 061 projection for this route publishes them,
   as observed values; do not widen projection vocabulary.
6. Extend the fake-SDK fixture so provider-free tests prove: a two-turn
   session edits a file under the leased cwd with each write admitted first;
   a denied write does not touch disk; `set_permission_mode` round-trips
   `plan` and `default`; `acceptEdits` skips per-call admission for edits
   only; bypass and write-on-read-only are rejected before launch.
7. Update the SDK prepared guide (tool admission, permission modes, the
   `acceptEdits` caveat, the ambient posture statement), the
   `claude-agent.sdk` matrix cells, and `CHANGELOG.md` `[Unreleased]`.
8. Regenerate the Claude Agent adapter API baseline file additively. Stop
   after one reviewable PR.

## Out Of Scope

Bash or terminal (card 081); model or effort change (082); resume, fork, or
listing (083); MCP servers (084); ACP routes; discovery or version pins;
hosted OAuth; live provider calls.

## Acceptance Criteria

- [ ] default profile behaviour is byte-identical to `v0.4.0`
- [ ] no write tool is admitted without a read-write lease
- [ ] `allowedTools` is never set; every call passes through `canUseTool`
- [ ] `bypassPermissions` cannot reach the SDK
- [ ] mid-session mode change returns the confirmed mode or fails typed
- [ ] guide, matrices, changelog, and fixtures agree with the code
- [ ] semantic API diff is additive only

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:guides`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

## Review Oracle

Invariant: the sidecar never auto-allows a tool and the consumer sees every
write before it runs. Smallest counterexample: `allowedTools` set, a write
admitted on a read-only lease, `bypassPermissions` reaching the SDK, or a
changed default profile.

## Auto-Continuation

No. Stop after one reviewable PR for exact-head review; Chatterbox then
compiles the `v0.4.1` release roadmap.

## Stop Conditions

The SDK cannot change permission mode without reopening the session; a write
path bypasses `canUseTool`; a Contract 017 or 023 reinterpretation becomes
necessary; scope widens into a later card.

## Result

Delivered in two PRs: permission policy first, then read-write tool admission
once card 089 narrowed the shared preflight guard to the bounded-profile
boundary claim.

First PR (221):

- `ClaudeAgentSdkSessionProfile`, `ClaudeAgentSdkTool`, and
  `ClaudeAgentSdkPermissionMode` as additive prepared inputs, bound through
  `ClaudeAgentSdkSessionPreparation::with_session_profile`. `from_names`
  rejects an unknown tool, a repeat, and an empty set with exact codes.
- `bypassPermissions`, `auto`, and `dontAsk` are unrepresentable in Rust and
  are refused by name inside the sidecar before the SDK module is imported.
- Sidecar: the admitted set crosses as `tools`, `allowedTools` is never set,
  every withheld admissible tool joins `disallowedTools`, `permissionMode` is
  passed at open and echoed back, and `set_permission_mode` calls the SDK's own
  mid-session change and returns the confirmed mode.
- `ClaudeAgentSdkPreparedSession::open_route_session` returns the route-local
  `ClaudeAgentSdkSessionHandle`, whose additive `set_permission_mode` returns
  the confirmed mode or fails typed (`permission_mode_rejected`,
  `permission_mode_unconfirmed`). `permission_mode()` reports only a confirmed
  value.
- Open verifies the exact admitted set and the selected mode from the
  sidecar's own echo; a widened set or a drifted mode fails `open_mismatch`.
- Fake-SDK proofs: a two-turn editing session writes only what the host
  admitted and a denied write never touches disk; `acceptEdits` skips
  admission for edits and nothing else; `plan`/`default` round-trip; bypass and
  an unadmitted tool name are refused before the SDK is constructed.
- The default profile is byte-identical in behaviour: the same read-only set,
  the same `default` mode, the same access policy, the same instance policy id.

Second PR, after card 089 merged:

- `prepare_claude_agent_sdk_session` no longer refuses a write profile.
  `swallowtail.claude-agent.sdk.preparation.write_admission_unavailable` is
  gone; admitting `Edit`, `Write`, or `MultiEdit` binds
  `ResourceAccess::ReadWrite` into the plan, the session access policy, and
  the `claude-agent-sdk-ambient-read-write` instance policy, and keeps
  `Capability::ToolCalls`.
- A host that resolves a read-only lease fails the agreement with
  `swallowtail.session_access.resource_access_mismatch` before the sidecar
  starts, so no write tool reaches a read-only working resource.
- The read-only default is unchanged: same set, same `default` mode, same
  `claude-agent-sdk-ambient-read` policy, same ambient read access policy.
- The guide's write section, the route matrix and feature-matrix
  `claude-agent.sdk` cells, and `CHANGELOG.md` `[Unreleased]` state the
  admitted write path and the ambient posture. The bounded-workspace cell
  stays `No`: this route claims no filesystem boundary.

The `Status:` line and the batch-card index are reserved shared closeout
surfaces, so this card still reads `ready`; the coordinator owns both.
