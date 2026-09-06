# v0.4.2 Source Release

Date: 2026-09-06
Status: tagged; source-only

The operator authorized creation and push of annotated `v0.4.2` at the Card
101 candidate merge commit after the exact prepare and hosted CI gates.

- Candidate head: `ab3787717b7dd4d7c23bd08da92dae55d8ae3673`
- Merge commit / peeled tag commit: `f94dd16f2e4db79c5b7c4440cc1eb2d20f8b6af8`
- Tag: `v0.4.2`
- Tag object: `927d14eccc16b5f24fc427913e087cd47fcfa499`
- Prepare report SHA-256: `bb41cdc943c39359a9ba5e7eb3a28de9574f620bc0f9f778c9123ea9124dfe64`
- Exact-SHA workflow: `34019752262`, all checks green
- Card 100 accepted review: `https://github.com/inflatable-cookie/swallowtail/pull/237#issuecomment-5556008605`
- Card 101 accepted review: `https://github.com/inflatable-cookie/swallowtail/pull/240#issuecomment-5557836641`

The release is source-only: no crate publication, GitHub Release, binary,
sidecar publication, or consumer mutation. Card 100's live editing result
remains an unresolved typed `ProviderFailed` outcome; Card 103 is fixture-only
and adds no production content. Card 102 must use only this exact tag for its
consumer proof. Card 082 may now proceed under its separate accepted review,
but its merge remains governed by its own gate.
