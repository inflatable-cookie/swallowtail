# 2026-09-21 OpenCode HTTP 1.18.31 Claim

## Result

Raised `opencode.server` from qualified `1.18.30` through official `1.18.31`
as a compatible extension of `surface-19`. The single published patch hop is
qualified from complete source-tree inventories because the published GitHub
tags diverge. Baseline `1.14.48`, every historical gap and segment boundary,
the claim id, behavior revisions, and `AllowUnverified` stay unchanged.
Synthetic later stable `1.18.32` remains visible `UnverifiedNewer`.

Selected HTTP/SSE route files and OpenAPI are byte-identical. Remote-config
auth defect-to-400 mapping, OpenCode ACP restore/config-option work, the TUI
exit-status change, GitHub Copilot summarized-thinking request shaping, and
provider SDK bumps stay unmapped or provider-facing. Host `opencode` is not
on `PATH`.

No public API, generation card, provider contact, install, or host change
entered the claim. Research 331.

## Validation

`cargo fmt -p swallowtail-adapter-opencode` then focused nextest and clippy on
`swallowtail-adapter-opencode`: 137 tests passed. `package:verify-affected`
failed in this checkout because the content audit treats repo-root
`/workspace` as a leaked host path and matches existing `/workspace/fixture`
fixtures plus official `.../routes/workspace/...` inventory paths. The same
selector passed from `/tmp/swallowtail-verify` (190-file package, isolated
`cargo check --all-targets`). Official npm/GitHub latest was still `1.18.31`
at the pre-push recheck.
