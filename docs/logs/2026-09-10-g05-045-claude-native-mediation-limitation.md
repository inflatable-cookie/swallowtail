# 2026-09-10 g05.045 Claude Native Mediation Limitation

Date: 2026-09-10
Task: `../roadmaps/g05/045-claude-sdk-consumer-multi-turn-editing-acceptance.md`
No provider call, runtime edit, matrix Yes/No change, version/range change,
release, or tag.

## Result

Provider-limitation reconciliation. Desktop g02.049's three merged capsules
prove the qualified native tool set cannot satisfy universal consumer
mediation. Research 303 binds that evidence. Guide and Claude matrix-row
notes no longer claim every admitted native tool crosses `canUseTool`.
`permission_exchange=Yes` stays scoped to SDK-emitted callbacks. Registered
MCP remains the Contract 063 before-dispatch route. g05.029's historical
clause-1 fail stands.

## Frozen identities

Desktop task `bc4acd98-91a3-4e14-86e0-38376b037da7`. PR #205 accepted head
`8002833f45a66e63df1071f5bb99053d6bf85922`. Independent exact-head review
`5610373055`. Merge `e0217483ed8584ddca1bc4f4fdfedce53bcbaa76`. Closeout
`941dfcdb5361d673dfd5c7adb54950cacbbbe478`.

Released Swallowtail `v0.4.4` peel
`49c9e3b291609c9ebf5b35a284c08302f3b8d5e3`, tree
`1a9db12742b68e839dfebe42f0633f5d1aa0265b`. Tuple: SDK `0.3.259`, native
`2.1.259` darwin-arm64
`884baa38fe1a624be25c4a91568bf5a08b5cf4e7d7acf29b7760e3525d964898`, Node
`22.23.2`, sidecar `swallowtail-claude-agent-sdk-sidecar@0.4.4`,
`permissionMode` `default`, `claude-sonnet-5`.

| Capsule | SHA-256 | Failure |
| --- | --- | --- |
| `claude-mediated-multi-turn-editing.capsule.json` | `10b77ede7a85b68831967731e6d5636c0d44b72a56b1774b29938ed4fd519cd6` | `open.model_or_mode_not_admitted` |
| `claude-mediated-multi-turn-editing-renewed.capsule.json` | `8921fa5edc0ba869a8d565e2834fd9e138178dd87bd0bbec8e1561205ffc1386` | `ordering.tool_start_before_decision` |
| `claude-mediated-multi-turn-editing-successor.capsule.json` | `4f23e55c548469ae61666052266497ae70b864566e0e3cc7de1c3a208a777dba` | `ordering.tool_result_before_decision` |

Successor scored facts: one open, one stopped turn, zero retries. Chain is
one native `Read` proposal with `preceding_decision=false`. Two
`sidecar:activity` frames, then `ordering.tool_result_before_decision`.
Allow 0, deny 0, writes 0. No `canUseTool` field. Fixture digest stayed
`sha256:b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060`.
Cleanup degraded accepted, zero survivors.

## Surfaces

Research 303. Fixture
`crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.259/native-read-mediation-limitation.json`
and focused test. Guide native-mediation note. Claude matrix-row notes
only. g05.029 follow-up subsection. This log.

## Next

Independent exact-head review, then queue merge and reserved-index
closeout. Desktop g02.051 owns the registered-tool editing route.
