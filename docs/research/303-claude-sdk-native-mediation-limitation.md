# 303 Claude SDK Native Mediation Limitation

Status: complete; provider limitation; no live/provider claim
Owner: Swallowtail g05.045
Created: 2026-09-10

## Question

On the qualified `claude-agent.sdk` tuple, does every admitted native tool
cross `canUseTool` before it can run?

## Evidence

Desktop g02.049 merged three immutable capsules through PR #205. Identities:

- Northstar task `bc4acd98-91a3-4e14-86e0-38376b037da7`
- accepted head `8002833f45a66e63df1071f5bb99053d6bf85922`
- independent exact-head review `5610373055`
- merge `e0217483ed8584ddca1bc4f4fdfedce53bcbaa76`
- canonical closeout `941dfcdb5361d673dfd5c7adb54950cacbbbe478`

Released Swallowtail source is `v0.4.4` peel
`49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`, tree
`1a9db12742b68e839dfebe42f0633f5d1aa0265b`. Tuple: SDK `0.3.259`, native
`2.1.259` darwin-arm64 digest
`884baa38fe1a624be25c4a91568bf5a08b5cf4e7d7acf29b7760e3525d964898`, Node
`22.23.2`, sidecar `swallowtail-claude-agent-sdk-sidecar@0.4.4`, wire
`swallowtail-claude-agent-sdk-jsonl-v1`, `permissionMode` `default`,
`claude-sonnet-5`, `allowedTools` omitted, persistence disabled. Digests
below were recomputed from the merged Desktop tree.

| Capsule | SHA-256 | Failure | Opens / retries / turns |
| --- | --- | --- | --- |
| `docs/proofs/claude-mediated-multi-turn-editing.capsule.json` | `10b77ede7a85b68831967731e6d5636c0d44b72a56b1774b29938ed4fd519cd6` | `open.model_or_mode_not_admitted` | 1 / 0 / capsule `counts.turns=2` with empty `session` |
| `docs/proofs/claude-mediated-multi-turn-editing-renewed.capsule.json` | `8921fa5edc0ba869a8d565e2834fd9e138178dd87bd0bbec8e1561205ffc1386` | `ordering.tool_start_before_decision` | 1 / 0 / 0 |
| `docs/proofs/claude-mediated-multi-turn-editing-successor.capsule.json` | `4f23e55c548469ae61666052266497ae70b864566e0e3cc7de1c3a208a777dba` | `ordering.tool_result_before_decision` | 1 / 0 / one stopped turn |

The first two capsules are earlier bounded failures. They are preserved
byte-for-byte. They do not prove native-read completion.

The successor capsule is the limitation proof. Open reached ready under
`default` with `claude-sonnet-5` requested and admitted. Turn 1 recorded
exactly one chain item: `tool_proposal` for native `Read`, `phase=started`,
`preceding_decision=false`. Frames:
`sidecar:started`, two `sidecar:progress`, two `sidecar:activity`. Then the
harness aborted `ordering.tool_result_before_decision`: a completed activity
arrived while no callback had set an awaiting start or end. Recorded
`allow_count=0`, `deny_count=0`, `write_successes=0`. The capsule has no
`canUseTool` field; that absence is not an unobserved optional. Fixture
`note.txt` stayed at initial digest
`sha256:b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060`
(`before_digest` equals `last_digest`). Counts: `opens=1`, `retries=0`,
`fallbacks=0`, `reconnects=0`, `turn_ceiling=2`, `turns=1`, `completed=false`.
Cleanup: route-qualified `degraded`, accepted, zero listeners, zero leases,
reapers joined. Redaction truncated; no credential, token, or host-path
leak in the scored fields.

This is not the renewed capsule's proposal-versus-execution stop. The
successor harness treated streamed activity-start as a proposal. The stop
is a completed native `Read` activity with zero recorded decisions and an
unchanged fixture.

No Bash, Git, web, MCP, outside-path, retry, fallback, or second completed
turn appears in the successor chain or counts.

## Decision

Universal native-tool `canUseTool` mediation is unavailable on this tuple.
`canUseTool` is a permission request the SDK may omit for default-allowed
in-workspace reads. Swallowtail still withholds `allowedTools` and still
answers every callback the SDK emits. `permission_exchange=Yes` stays
scoped to those emitted callbacks. Registered MCP calls remain separately
qualified through the Contract 063 before-dispatch bridge. Bounded
workspace writing is still `No`.

g05.029's historical clause-1 fail stands. The native route cannot close
that every-call clause. No fourth Claude call, observer patch, model
change, matrix Yes/No flip, version/range change, runtime edit, release, or
tag follows.

## Consequence

g05.045 binds this evidence provider-free, corrects the overclaim that
every admitted native tool crosses `canUseTool`, and leaves registered-tool
and emitted-callback truths in place. Desktop g02.051 owns the
registered-tool editing route. Any later live native acceptance needs a
separate operator gate.
