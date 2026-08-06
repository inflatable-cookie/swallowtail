# v0.2.0 Muse And Rust-Floor Source Release Selection

Date: 2026-08-06
Roadmap: g03.046

## Decision

The operator initially selected an additive `v0.1.2` source release for Muse
Code. Current source then intentionally raised the workspace Rust floor from
the `v0.1.x` split Rust `1.90.0` / `1.94.1` posture to one Rust `1.95.0` floor.
Contract 036 classifies an MSRV raise as breaking before 1.0. The operator
therefore rebased the lane to `v0.2.0` rather than misclassifying it as a patch.

The existing 27 package APIs remain compatible; Muse adds package 28 and
production route 34. A tag is preferable to requiring consumers to pin commit
`7604cfc4`, while the minor version tells them the compiler requirement has
changed.

Release preparation first removes the two new Muse error-severity structural
findings in `events.rs` and `tests/corpus.rs`. The 22 inherited structural
errors remain outside this bounded release lane.

## Boundary

Cards 139-140 may produce a complete local candidate. Commit, push, CI, and tag
mutations remain separately gated. No tag, GitHub Release, registry
publication, consumer mutation, or provider work is authorized by this
selection alone.

## Next

Card 139 is complete. Prepare the local `v0.2.0` candidate under card 140.
