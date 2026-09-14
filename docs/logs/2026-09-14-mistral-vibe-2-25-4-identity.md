# 2026-09-14 Mistral Vibe 2.25.4 Identity

Research 321 froze official GitHub/PyPI `2.24.2` plus all eight published
stable successors through `2.25.4` as exact source tarballs and
registry-matched sdist/wheel pairs. The `2.24.2` baseline reproduces
Research 150's digests, GitHub-only `2.24.4` stays a named PyPI packaging
gap, and every selected input is classified provider-free across the chain.

The selected streaming wire is byte-stable at every hop:
`vibe/cli/programmatic.py`, `vibe/core/middleware.py`, and even upstream's
`tests/cli/test_programmatic.py` never move; the public-history union,
generation status, `LIMIT`-only stop reason, `SessionOptions` shape, builtin
`plan` profile, trust application, workdir authority, missing-key failure,
and console script are unchanged. The one material finding: `2.25.1`
introduces `resolve_harness_selection`, which reads the ambient GrowthBook
rollout cache and can swap the session backend to the internal Unified
Harness when that native module is bundled — GitHub zips from `2.25.1`
bundle it, PyPI never ships it. The selected argv therefore pins the legacy
harness with adapter-private `--legacy-harness` (first precedence upstream),
keeping the corpus-covered backend deterministic on both channels. The
Unified Harness and `--smart-approve` stay unmapped.

The identity corpus is frozen in
`crates/swallowtail-adapter-mistral-vibe/tests/fixtures/mistral-vibe-headless-2.25.4/`.
No downloaded artifact was executed, no prompt or provider call was made,
and no install, login, credential use, or host mutation occurred.
