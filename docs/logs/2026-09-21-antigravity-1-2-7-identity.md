# 2026-09-21 Antigravity 1.2.7 Catalogue Identity

Research 331 froze official GitHub `google-antigravity/antigravity-cli`
identity for `1.2.3` through `1.2.7` and classified the hops per claim
under Tom's 2026-09-15 release-notes authority ruling. Catalogue-only.
Headless stays stopped.

Every linux-x64 and mac-arm64 tarball SHA-256 matches GitHub's declared
digest. Each archive holds only `antigravity` with its own version
literal. Public git hops change only `CHANGELOG.md`. Host `agy` is not
on `PATH` and was not installed. No downloaded binary was executed.

Catalogue is a compatible extension through official `1.2.7` because the
notes name no selected-path change to `agy models`. Headless keeps
`1.1.17`: `1.2.6` changes the published default `--print-timeout` to
unlimited and replaces agent/model API failure with `AGY_ERROR` plus
exit 3; `1.2.7` caps per-attempt backoff at 30 seconds but still
publishes no finite attempt bound or disable control. The gap becomes
`1.1.18..=1.2.7`. Headless does not have to move with the catalogue
advance, and the `1.1.22` stop is not reopened.
