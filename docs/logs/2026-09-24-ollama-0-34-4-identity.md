# 2026-09-24 Ollama 0.34.4 Identity

## Result

Official Ollama `v0.34.4` was frozen against the `0.34.2` claim. GitHub
reports `v0.34.4` as latest stable. Tag tarballs for `v0.34.2`, `v0.34.3`,
and `v0.34.4` were retrieved and never executed. Research 342's `v0.34.2`
hashes reproduce. Both published hops after the ceiling (`0.34.3`,
`0.34.4`) are frozen with tag, commit, tree, tarball digest, and
selected-file hashes. `ShowResponse.thinking` is additive and ignored by
the catalog decoder. `ChatRequest.Think` is comment-only. Remaining
ChatHandler and leftover routes.go deltas stay bounded unmapped. Decision
is compatible extension of `ollama.native-text-v1`. Host client is
`0.33.3` with no running instance; the host was not mutated. Research 350.

Production claims stay at `0.34.2` in this record.

## Next

Apply the compatible-extension decision: raise `ollama.runtime` through
`0.34.4`.
