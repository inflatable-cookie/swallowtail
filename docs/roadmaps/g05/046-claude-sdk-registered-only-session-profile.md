# g05.046 Claude SDK Registered-Only Session Profile

Status: ready; provider-free; serial after g05.045
Owner: Tom
Created: 2026-09-10
Depends on: Contracts 013, 017, 019, 029, 041, and 063; g05.045; Desktop
g02.051 stop; released `v0.4.4`
Vision tags: Claude route, registered tools, native-tool exclusion, resource access

## Outcome

Add one explicit Claude SDK registered-only session shape. It binds a non-empty
registered-tool selection, admits zero native SDK tools, and carries explicit
`Read` or `ReadWrite` working-resource access without using a native write or
`Bash` tool as the lease signal.

Keep ordinary profiles and additive registered tools byte-compatible. Empty
native admission remains invalid unless the registered-only binding is present.
Do not infer filesystem access from MCP presence, tool names, or the generic
registered-tool effect posture.

## Ready-State Rubric

- [x] Desktop g02.051 stopped before edits, PR, or provider contact on the
      published `v0.4.4` expressiveness condition.
- [x] The exact gap is source-confirmed: `new([])` and `from_names([])` reject
      `profile.tool_set_empty`; registered MCP names append to native tools;
      `resource_access()` derives only from native write or `Bash` admission.
- [x] Native `Read` cannot remain as a harmless fallback: g02.049 proved it may
      complete without `canUseTool`.
- [x] Tom already authorized the separate registered-tool editing route on
      2026-09-10.
- [x] No provider or release authority belongs to this task.

## Decisions

- Select the zero-native route. A native remainder would violate the strict
  before-dispatch oracle and repeat the limitation g05.045 records.
- Make the valid state structural: registered-only requires a qualified,
  non-empty registered-tool binding. Empty native plus no registered binding
  fails before plan, lease, process, or provider contact.
- Carry working-resource access explicitly on that registered-only session
  shape. `ReadWrite` is granted because the consumer selected a registered
  workspace-writing route, not because some MCP server exists or a generic
  tool declaration is mutating.
- Keep `ClaudeAgentSdkSessionProfile::new` and `from_names` empty-set rejection
  for ordinary native profiles. Keep today's additive `with_registered_tools`
  behavior unchanged.
- Render only selected registered MCP tool spellings in `tools[]`. Put
  `Read`, `Glob`, `Grep`, `Edit`, `Write`, `MultiEdit`, and `Bash` in
  `disallowedTools` when the registered-only shape is selected.
- This is an additive source/API repair after `v0.4.4`. It does not move that
  tag and does not authorize `v0.4.5`, a candidate, publication, or a Desktop
  dependency change.

## Dispatch Manifest

- **State:** ready; queue behind task
  `0c79d680-4a5e-42cc-a377-7c4e63311b56` so g05.045 lands first.
- **Completion:** one structural registered-only profile/binding; exact access
  propagation through plan, driver validation, lease resolution, open request,
  and sidecar options; provider-free positive and counterexample coverage;
  guide and API documentation; one log and closeout.
- **Owned mutable paths:** this task;
  `crates/swallowtail-adapter-claude-agent/src/sdk/profile.rs` and its tests;
  the narrow registered binding, preparation, driver, validation, startup, MCP,
  and sidecar files needed for the invariant; focused Claude adapter tests;
  `docs/guides/claude-agent-sdk-prepared-integration.md`; one new log and
  indexes; `PAPERCUTS.md` append only.
- **Reserved closeout surfaces:** `docs/roadmaps/README.md`,
  `docs/roadmaps/g05/README.md`, and `docs/roadmaps/generation-index.md`.
- **Worker:** Rust implementation worker; no provider access.
- **Serial edges:** g05.045 first because both tasks own the Claude guide.
  Desktop g02.051 remains blocked until this source change is released and
  Desktop pins that exact later tag.
- **Excluded:** generic registered-tool contract redesign; inference from
  `RegisteredToolEffectPosture`; native-tool mediation claims; Desktop mutation;
  dependency or version changes; provider calls; credentials; release, tag, or
  publication.
- **Escalation:** Chatterbox owns profile/access semantics. Tom owns any later
  candidate, tag, Desktop repin, or live acceptance.

## Work

1. Preserve the existing native profile constructors and additive registered
   path. Introduce the smallest route-specific binding that makes zero-native
   admission inseparable from a non-empty qualified registered selection and
   explicit working-resource access.
2. Propagate that access through prepared capability requirements, instance and
   session policy, driver agreement, and host lease resolution. Reject every
   mismatch before acquisition or provider contact.
3. Render registered-only open options with only the selected carrier tool
   names admitted and every native SDK tool structurally disallowed. Preserve
   omission and ordering behavior on all existing profiles.
4. Add provider-free fixtures and tests for `Read` and `ReadWrite`, missing or
   empty registration, additive-path stability, access mismatch, native-tool
   exclusion, sidecar echo, and open validation.
5. Update the Claude integration guide and public docs. Validate the affected
   adapter and documentation surfaces. Open one PR for independent exact-head
   review; the queue owns merge and closeout.

## Review Oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Registered-only is structural | construct empty native profile without registered tools | public construction or prepare fails before any acquisition |
| Access is explicit | infer `ReadWrite` from MCP presence, name, or generic mutating effect | binding carries exact `Read`/`ReadWrite`; unrelated registered selections do not elevate it |
| No native remainder | retain `Read`, `Edit`, or `Bash` to satisfy old validation | open `tools[]` contains only selected carrier spellings; all seven native tools are disallowed |
| Existing callers are stable | change `new`, `from_names`, defaults, or additive registration | byte/fixture equality and focused regressions preserve current paths |
| Lease agrees end to end | plan says `ReadWrite`, driver resolves `Read`, or sidecar echo widens | exact access matches plan, policy, lease, driver validation, and open evidence |
| Provider-free means provider-free | use Claude to prove option rendering | deterministic Rust/sidecar fixtures only; zero credential or provider contact |

## Stop Conditions

- Stop if zero-native admission cannot be tied structurally to a non-empty
  registered binding.
- Stop if the implementation needs generic effect posture to mean filesystem
  access, changes existing additive behavior, or weakens empty native-profile
  rejection outside the new path.
- Stop if the sidecar cannot prove all native tools disallowed while selected
  registered tools remain admitted.
- Stop before any provider call, credential access, version change, release,
  tag, publication, Desktop edit, or live-acceptance inference.

## Operator Ruling

Tom authorized landing the native limitation and building a separate strict
registered-tool editing route on 2026-09-10. This prerequisite follows that
settled direction: zero native tools, explicit registered-route resource
access, no provider call. A native remainder is rejected.

## Evidence

On completion record the exact source head, API and wire shape, access
propagation, native disallow list, provider-free test results, independent
review, merge, and closeout. Record zero provider contact and every retained
non-claim.

## Next Task

Prepare a separately authorized patch candidate carrying this source change.
Desktop g02.051 stays blocked until a later exact source tag is authorized and
pinned. Live acceptance remains a separate operator gate.
