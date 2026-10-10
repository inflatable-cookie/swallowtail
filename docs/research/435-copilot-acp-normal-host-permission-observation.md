# Research 435: Copilot ACP Normal-Host Permission Observation

Task 169 ran the one original Copilot `1.0.93` attempt authorized by the
normal-host direction. The result is a finite failure at initialization. The
runner observed an `agentInfo.version` mismatch and stopped before accepting
initialization, creating a session, or sending a prompt. No permission request
or cancellation was observed. The exact mismatching value was not retained.

The original selector staged and verified the reviewed wrapper and Darwin
ARM64 native archives, their SRI values, package manifests and payload
inventories before launch. It launched the native executable directly with
`--model auto --acp --stdio` under the existing host environment. The attempt
record binds native archive
`f254651a3195e125b91d723c800e71e6541f8db3832d269854ae982254263eeb`, executable
`df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`, wrapper
archive `a8e704fb6874364af1b268aed2170bb597e0ca8086f3182b8fe5cb86ca3e43e1`,
the preparation plan and final runner identities, and the task authority.

The process ran for 3,685 ms. The harness terminated the owned process group
during bounded cleanup after the version mismatch. It observed root exit,
empty process group, and joined stdout and stderr readers. Cleanup of arbitrary
escaped descendants remains unknown. The sentinel bytes and action directory
were unchanged. No structured model identity was exposed in the retained
result. Raw protocol, stderr, auth responses, secrets and host paths were not
persisted.

The invocation was consumed before launch. No prompt slot was consumed: the
prompt ledger was not created, and all three shared permission prompts remain
available. There was no retry, resend, fallback or reviewer original attempt.
This observation does not prove cancellation or no-effect behavior and does
not qualify `1.0.93`. The `1.0.80` route claim and released contracts remain
unchanged.

## Retained records

- [Original execution authority](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-host-permission-proof/original-execution-authority.json), SHA-256 `8df1d77fe399084345c0dd94c6b954428540d37e50380578542626fc0ad17eed`.
- [Fake-pass record](./435-copilot-acp-normal-host-permission-fake-pass.json), SHA-256 `135d5058ce49241a466fb4ddf97f2812bc34f8d7f24de275d2cd7c148df43760`.
- [Consumed attempt record](./435-copilot-acp-normal-host-permission-attempt.json), SHA-256 `30f16a9f2d6b0e168b0486cacb71c0f96a719bc3f7f13687807a81c11731929c`.
- [Sanitized original result](./435-copilot-acp-normal-host-permission-observation.json), SHA-256 `982d716e615daaa7d92612c74ae84ac13ab62c26fb19005841eb1ec09ea6ac94`.

The record-check selector validates the result and consumed invocation ledger.
No prompt-slot record exists, matching the result's
`prompt_slot_fsynced_before_send` value of `false`.
