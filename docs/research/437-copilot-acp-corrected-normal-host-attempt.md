# Research 437: Copilot ACP Corrected Normal-Host Attempt

Task 171 ran exactly one corrected Copilot CLI `1.0.93` normal-host attempt
under decision `acb7a075-390a-48b6-99b5-a6eea4d9920e`. The reviewed wrapper
and Darwin ARM64 native archives, SRI values, package manifests, extracted
inventories and native executable identity passed before launch. The direct
native command was `--model auto --acp --stdio`; the one prompt was sent only
after the public reported version matched `1.0.93`.

Initialization and `session/new` succeeded. After the prompt, the process
emitted one tool-call update without a permission request. The runner stopped
with `tool-call-without-host-permission`; it observed no permission action,
cancel reply or prompt result. The sentinel bytes and action directory stayed
unchanged. The owned process exited, its process group was empty, both stream
readers joined, and the temporary package cache was removed. Cleanup of
arbitrary vendor descendants remains unknown. The process exit code was
`-9`; no raw protocol or stderr was retained. A structured model identity was
not exposed.

The one authorized invocation was consumed before launch. The prompt slot was
consumed and fsynced before the send, so one of the three shared permission
slots is used and two remain. Research 435's earlier consumed invocation and
all historical records remain unchanged. This is a finite failed permission
observation; it does not qualify `1.0.93`, change the exact `1.0.80` route
claim, or alter released contracts.

The finalized task plan SHA-256 is
`7b04154a4e86beeaf6049d4b9ed673d53aa6ce690e76a7a5553c0901688e9a9d`; the
runner SHA-256 is
`5d534c17d9e998636552f56c8aa3d0280bb6544ce55f63f71c2f057e17d58040`. The
execution authority SHA-256 is
`bd06e2544216e8ce300dbae59d1f13e055e23bb137862025e92c4288ab22450c`; it
binds the exact task, decision, plan, runner and fake-pass record. Fake proof
covered successful cancel/no-effect behavior, version guards, timeout
cleanup, injected package-cache cleanup `OSError`, record validation, fsynced
consumed ledgers and replay refusal. Fake proof ran no original.

## Retained records

- [Final corrected plan](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-host-permission-proof/corrected-execution-plan.json)
- [Corrected execution authority](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-host-permission-proof/corrected-execution-authority.json)
- [Fake-pass record](./437-copilot-acp-corrected-normal-host-fake-pass.json), SHA-256 `905866db7aaeac5876e7fcfd5845127b522647b918b7b98d14eda993f420cc1d`
- [Consumed invocation record](./437-copilot-acp-corrected-normal-host-attempt.json), SHA-256 `10639b7fdd2cd10899d52eb13cd44b66c51167672407ff2a069e42ffbf65d928`
- [Consumed prompt-slot record](./437-copilot-acp-corrected-normal-host-prompt-slot.json), SHA-256 `e2dec4e3714a282b640ab1b9e9a5820e473b0cea69f7199fe28c5c293a554c58`
- [Sanitized execution record](./437-copilot-acp-corrected-normal-host-execution.json), SHA-256 `1de648daa5e5491bb75a37c4fda44565e49876caa9f8e6df7ec4a31e706f0bca`

The original selector ran once. Reviewers validate these records and do not
start another original.
