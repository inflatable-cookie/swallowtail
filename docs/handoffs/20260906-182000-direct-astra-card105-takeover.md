# Direct operator Astra takeover — Card 105

Tom directs repair/release without the Coordinator and explicitly requests one
Astra medium implementation worker. This overrides coordinator-only dispatch
and planning-only profile defaults for this bounded delivery. No duplicate writer.

Placement: existing Card105 workspace wks_1faad0588e26c7cf, branch
`g05-card105-termination-cause-propagation`, starting HEAD
`294872d214e80f016d3c50cccfaa383729d27320`.
Previous writer `deb29885-c418-4785-ac97-8adb194aa3d9` was cancelled and verified
idle before takeover. After a later restart caused concurrent reversions, root
confirmed the old agent archived with the workspace preserved. The successor
is the sole writer. Preserve existing implementation, tests and docs.

Objective: finish the Claude SDK termination repair, distinguish a proven
consumer fix from improved diagnostics, and produce one clean reviewable PR.
Read Card105, inspect existing changes and recorded checks; rerun affected
checks and required final gates. Do not rebuild unrelated features.

Desktop PR102 has a real session/open and one Send but no reply. Native visible
window and restore/refresh are repaired. v0.4.2 drops Terminal(failure)'s inner
code. Unknown SDK message is a hypothesis, not observed live evidence. Preserve
safe diagnostics and use fake SDK or bounded isolated reproduction to establish
the cause. Do not silently ignore arbitrary SDK messages, weaken permissions,
expose credentials/provider content, alter global auth/tools or edit Desktop.

Root `129c4bc3` owns direct independent review, merge and release sequencing
under Tom's instruction. Worker stops at a clean PR and returns exact head,
validation, real-cause evidence versus uncertainty, and release inputs. No
self-review, tag, publication or unrelated feature changes. No coordinator
messages. Retain this workspace for repairs and release preparation.

## Direct scope extension — pinned rate-limit projection

After Opus PASS and green CI on `fd00ae03`, root/operator explicitly authorized
repair of the known pinned `rate_limit_event` gap in the same workspace/card/PR245.
No separate card or coordinator dispatch is required. Well-formed `allowed`,
`allowed_warning`, and `rejected` updates now project payload-free progress;
SDK result/error still owns turn failure. Malformed/unknown messages remain
terminal. Fake-SDK sequence, rejection, interrupt and close proofs qualify this
bounded repair. This supersedes the prior diagnostic-only release limitation.
Real Desktop cause remains unproved until tagged consumer Send evidence. Root
reuses the independent reviewer and owns merge/release; worker stops at updated PR.

## Pre-tag persistence repair

Root authorized the demonstrated idle-notification edge after `b5934ce2` PASS.
Known rate-limit messages are validated regardless of turn state, but emit
progress only during an active turn. The generic Rust `event_without_turn`
guard and unrelated message projections stay unchanged. Two-turn and idle
malformed/unknown fixtures cover this delta; the focused asset batch passes.
Root requests the same reviewer delta check on the new PR245 head.
