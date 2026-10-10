# Research 438: Copilot ACP Permission Ordering Correction

Task 172 corrected the normal-host Copilot proof after reviewing its first
notification stop in Research 437. That record preserves the original
`tool-call-without-host-permission` harness label and bytes. The notification's
status was not retained, and the runner stopped before waiting for a later
permission callback. The stop cannot establish that no later callback would
arrive or that the vendor executed the announced operation without permission.

The shared original/fake exchange now follows the repository's frozen ACP
schema v1.24.1 identity in
[`protocol.json`](../../crates/swallowtail-protocol-acp/tests/fixtures/acp-schema-v1.24.1/protocol.json):
`ToolCall` SHA-256
`f8dc46888da9e4fa1edfc9cd88f93bc074a839ecc6d08ddb6d2d2ac12fe568fa`,
`ToolCallUpdate` SHA-256
`89464d21aa887ee0a13ffe3df39ae23f1598eb6a9a15a0a2af14b46f7e4b4968`,
`RequestPermissionRequest` SHA-256
`8d28e54c28364666fbb9a6db2877f0c51f1b54745192ddcd8737ab640afb6336`, and
`RequestPermissionResponse` SHA-256
`bd893d42a9ab8d11c255e51e3cd8f2b3d2c60658cecedd09e365d3936dfc3761`.
The frozen decoder defaults omitted status to `pending` and omitted kind to
`other`; update status and locations are optional replacements in the
[tool decoder](../../crates/swallowtail-protocol-acp/src/activity/decode/tool.rs)
and [typed record](../../crates/swallowtail-protocol-acp/src/activity/tool_record.rs).
ACP v1 describes `edit` as the file
modification kind, allows optional `rawInput`, and defines `session/cancel` as
a notification. The current protocol pages corroborate these semantics:
[tool calls](https://agentclientprotocol.com/protocol/v1/tool-calls) and
[prompt turn cancellation](https://agentclientprotocol.com/protocol/v1/prompt-turn).

The proof correlates the active session and opaque tool-call ID across the
announcement, sparse updates and permission snapshot. It waits only while the
call remains pending and the action directory remains unchanged, bounded by
60 seconds and 64 messages. A non-pending status before permission, malformed
or uncorrelated identity, duplicate request, wrong action, missing permission,
unexpected prompt end or observed file change fails the proof. It records
bounded status and kind enums, counts, and correlation/action/effect booleans;
raw IDs, payloads, paths and authentication data are not persisted.

The action matcher requires `kind: edit`. It attributes the requested sentinel
write only from a diff with the exact path and replacement bytes; if an old
value is supplied, it must match the existing sentinel. A supplied location
must agree with the sentinel path. Non-null `rawInput` remains opaque and makes
the action unattributable; ACP v1 does not define keys for interpreting its
contents. Missing or unrecognized action fields and titles do not count as
evidence. For prompt
cancellation, the runner sends `session/cancel` as a notification, responds to
the pending permission request with outcome `cancelled`, and accepts the
result only when the prompt reports its actual `cancelled` stop reason. The
fake covers pending status both explicit and omitted, sparse snapshots, wrong
IDs/session/action/kind, `in_progress` and `completed` before permission,
spontaneous effect, duplicate requests, missing permission, timeout,
post-cancel notifications, and a `tool_call` announcement with no `kind`.
That missing-kind case remains safely unattributable under the frozen `other`
default; it no longer escapes the runner or omits its execution record.

The task-specific proposal is
[`task-172-attempt-proposal.json`](../../crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-host-permission-proof/task-172-attempt-proposal.json).
It remains disabled and requires separate original authority. It preserves
native executable `df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1`
and archive
`f254651a3195e125b91d723c800e71e6541f8db3832d269854ae982254263eeb`, the
direct `--model auto --acp --stdio` launch, `betterthanclay` and Auto, child-only
update/cache controls, one sentinel prompt, and the 60-second inclusive
cleanup limit. It allocates new exclusive attempt and prompt ledgers without
changing Research 437; the consumed original allowance remains consumed and
two of three shared prompt slots remain. No original ran, and qualification,
the exact `1.0.80` claim, or released contracts changed.

## Validation

The fake-only proof and record checks were run through the task's covering
Effigy selectors. The required source, proposal, existing Research 437 fake
pass, and original execution record checks passed. Documentation and retired
concept checks passed. No original command, prompt, login or network operation
was run.
