# Mistral Vibe headless 2.25.4 useful-newer identity corpus

Provider-free identity, per-hop ledger, and classification for the exact
`mistral-vibe.release` rebind from `2.24.2` to official GitHub/PyPI `2.25.4`
(g05.073, Research 308 campaign family thirteen).

All nine GitHub tags (`v2.24.2`..`v2.25.4`) were retrieved as exact source
tarballs into `/tmp`, and the eight published PyPI points as sdist+wheel
pairs with registry-matched SHA-256 digests (the `2.24.2` pair reproduces
Research 150). PyPI `2.24.4` is absent: a named packaging gap that is never
qualified by adjacency. The darwin-aarch64 release zips were listed (never
extracted or executed) to date the bundled internal harness module.

Selected wire and lifecycle facts frozen here:

- `vibe/cli/programmatic.py` and `vibe/core/middleware.py` are byte-stable
  across all nine points: completed-only public-history NDJSON, id dedup,
  LIMIT-only stop reason, callback denial, close-in-finally.
- `tests/cli/test_programmatic.py` is byte-stable at every tag.
- `SessionOptions = AgentConfig` field shape is unchanged.
- The builtin `plan` profile definition is byte-identical.
- Missing API key fails with identical stderr text and exit code at both
  boundaries.

The one material adaptation: from `2.25.1`, harness selection consults the
ambient GrowthBook rollout cache and can pick the internal Unified Harness
backend when it is bundled (GitHub zips from `2.25.1`; never PyPI). The
selected argv therefore adds adapter-private `--legacy-harness`, which has
first precedence and pins the legacy Python harness the corpus freezes.
The Unified Harness, `--smart-approve`, and the `smart-approve` profile
stay unmapped.

No fixture contains a credential, host path, account identity, provider
payload, or real session id. No artifact was executed. No prompt, login,
installation, or host mutation occurred.
