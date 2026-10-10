# Research 439: Copilot ACP Pending Announcement Cancellation Observation

Task 173 ran exactly one renewed Copilot CLI `1.0.93` original under decision
`d7b9664a-98bc-441a-9f01-f91b9c236699`. The frozen wrapper, native package,
manifests, archive SRI and complete extracted inventories passed before launch.
The staged native executable matched
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`; its archive
was `f254651a3195e125b91d723c800e71e6541f8db3832d269854ae982254263eeb`. Wrapper
archive SHA-256 was
`a8e704fb6874364af1b268aed2170bb597e0ca8086f3182b8fe5cb86ca3e43e1`. The
wrapper/native manifest hashes were `5f29c2061d18be20a8cb1677161359253a85bb49a7b4ea1e6c11db8ae1d9b64b`
and `44de35dc12ce1b582678cf565d9f8ddddb786b00dd083373948d6e453e1eb729`. The
reviewed inventory SHA-256 was
`2d122117ccbb52dd547a783117ea3b1699df8e15357bca86a4d27a65416c8b0f`; archive
SRI and every extracted file were checked against that inventory.

The direct argv was `--model auto --acp --stdio`. The public version matched
`1.0.93` before session creation and the prompt. The binding-only fake proof
preserved Research 437 and the disabled Task 172 proposal, kept the corrected
pending/cancellation state machine byte-identical, and confirmed the consumed
Task 171 authority was refused before effects. It used the new exclusive
attempt, prompt and execution ledgers allocated in the reviewed Task 172
directory.

The one prompt produced one correlated pending `tool_call` announcement with
kind `execute`, followed by one permission request. The permission action was
attributable but did not match the exact sentinel edit. The runner therefore
recorded `failed` with `permission-action-mismatch`; it did not guess at the
action or retry. It sent `session/cancel` and the cancelled permission reply.
ACP returned the actual prompt result `cancelled`. No tool execution status or
file effect was reported; the sentinel and action directory were unchanged.

The observation took 12,242 ms. The root exited, its process group was empty,
both stream readers joined, and the temporary package cache was removed. The
root exit code was `-9`; cleanup of arbitrary vendor descendants remains
unknown. No raw protocol, stderr or ACP session/tool-call identifiers were
retained. The selected model was unobserved.

This consumed the third invocation and second of three shared prompt slots,
leaving one prompt slot. The attempt and prompt records have SHA-256
`584bca2436a634489a619622dfdf5691738a771d8036039555595c76cb7b1047` and
`5324b8abfca8c4725378f00ab3b9942b03b26a0d8df358fbb610e14bd8d0d6ee`; the
execution record has SHA-256
`3bf8a6914e11772dca0f12f3a33564c17fed1206fb60d2db643a5939a5e454a5`.
Research 437 remains unchanged. The exact `1.0.80` route claim and released
contracts are unchanged; this finite failure does not qualify `1.0.93`.
Reviewers validate the records and do not start another original.

## Retained records

- [Final plan](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-host-permission-proof/task-173-execution-plan.json), SHA-256 `e6015b2c28a5189af4d00616dea9f86338e7fe923427ea276143838a7646f422`
- [Execution authority](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-host-permission-proof/task-173-execution-authority.json), SHA-256 `64b59b81d20152993f13cac47e25c2d3203f9b6032007f971dc8e8b9a21e36be`
- [Binding fake-pass record](./439-copilot-acp-pending-announcement-cancellation-fake-pass.json), SHA-256 `a01f8fecee4fc85450b1cbda3ff717ff9416142a774b47140b8e6c903c9608d5`
- [Consumed attempt record](./439-copilot-acp-pending-announcement-cancellation-attempt.json)
- [Consumed prompt-slot record](./439-copilot-acp-pending-announcement-cancellation-prompt-slot.json)
- [Sanitized execution record](./439-copilot-acp-pending-announcement-cancellation-execution.json)
