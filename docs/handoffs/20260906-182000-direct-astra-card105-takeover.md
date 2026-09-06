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
