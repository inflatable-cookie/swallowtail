# v0.5.1 Annotated Source Tag

Date: 2026-09-13
Task: `../roadmaps/g05/055-v0-5-1-annotated-source-tag.md`
Handoff: `../handoffs/20260913-074444-v0-5-1-annotated-source-tag.md`

## Result

Operator authorization created and pushed annotated tag `v0.5.1` at the exact
candidate merge `e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27` (tree
`d375b3227985e8e552ba9346d8f9b8631936db5f`). The tag object is
`97a6933abe13b2e8f05441e1ab950962e0683e65`; the object type is `tag`; local
and remote refs agree byte-for-byte and both peel to the exact candidate. The
annotation body is the approved three paragraphs, including the canonical
hosted CI identity `34731113171`, with SHA-256
`67e4f71c812ab5428e44b24d859e36efed9b661cbbc5de4e84f4156c85ee1626`. The
push carried only `refs/tags/v0.5.1` with no branch, force, follow-tags, or
atomic multi-ref push. The source-only release has no crates.io publication,
GitHub Release, binary, sidecar, installer, model artifact, provider mutation,
or consumer-repository mutation.

## Verification

One read-only preflight ran immediately before mutation and confirmed: worker
tree clean at planning commit `b1bc5dff5b6d65f2f57502d2ba9bc8360f62cb56`;
remote URL `git@github.com:inflatable-cookie/swallowtail.git`; candidate merge
and tree present and an ancestor of canonical `main`; tagged `crates/` tree
`186f3ba42a4aa896c03a3bddcacb007b1afbd8cc` and Grok subtree
`1a6f777f84f9aef80c146badf6944811568e23cd` equal to Desktop-qualified source
`0209dd7f`; workspace version `0.5.1`, `publish = false`, Rust floor `1.95`;
release baselines and candidate evidence retained; independent exact-head
review `5649975685` recorded against the identical candidate tree; local and
remote `v0.5.1` absent before mutation; immutable `v0.5.0` at `582d01d6` (tag
object `c772c583`).

Qualifying pre-tag gate: exact-SHA push run
https://github.com/inflatable-cookie/swallowtail/actions/runs/34731113171
passed all 11 jobs at the candidate. Tag-triggered CI run
https://github.com/inflatable-cookie/swallowtail/actions/runs/34743711387
(event `push`, head branch `v0.5.1`, head SHA the exact candidate) completed
`success` with all 11 jobs green: Roadmap number uniqueness, Stable format and
lint, Stable examples metadata and route contracts, Stable nextest (shard
1/2), Stable nextest (shard 2/2), Stable process-spawning nextest, Pinned MSRV
floor, Pinned MSRV floor tests, External Git-source consumer, Documentation and
semantic API, and Dependency security licenses and sources. The tag was never
moved, deleted, recreated, or force-pushed.

## Publication Evidence

The tagged `crates/` and Grok subtree identities were reproved against source
`0209dd7f`; every package remains version `0.5.1` with `publish = false`. No
GitHub Release object exists (`gh release list` empty; `v0.5.1` release not
found), the tag run has zero workflow artifacts, and an authoritative crates.io
query returns that `swallowtail-core` does not exist. Registry absence is
recorded structurally from that successful query, not inferred.

## Reconciled Surfaces

The release note and its index entry, Contract 036 current tagged identity,
g05.055, the g05 and roadmap front doors, the generation index, and this log
now describe the tagged state. Documentation closeout, independent exact-head
review, and merge are queued through one documentation PR; the queue owns merge
and canonical `main` synchronization. No consumer, provider, GitHub Release,
registry, binary, or further publication work follows automatically.

## Next

Desktop g02.089 Phase B may resume against the ordinary immutable `v0.5.1`
pin. No Swallowtail consumer mutation, provider call, registry publication,
GitHub Release, or further release follows automatically.

## Closeout

Documentation closeout opens as one PR against `main` from the task branch
after `effigy qa:docs`, `effigy qa:northstar`, and `git diff --check` pass at
the reviewed head. Independent exact-head review and merge are owned by the
northstar queue; the immutable tag is unaffected by that review. No deferred
failures; the tag remains immutable and source-only.
