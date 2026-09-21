# 2026-09-21 Ollama 0.34.2 Identity

## Result

Official Ollama `v0.34.2` was frozen against the `0.33.2` claim. GitHub
reports `v0.34.2` as latest stable; `v0.34.3-rc1` is a prerelease. Tag
tarballs were retrieved and never executed. All four published hops after
the ceiling (`0.33.3`, `0.34.0`, `0.34.1`, plus `0.34.2`) are frozen with
tag, commit, tree, tarball digest, and selected-file hashes. Seven
selected structs are byte-identical through `v0.34.2`; `Options` changes
only by a `typical_p` comment; `ShowHandler` and `PsHandler` are
byte-identical. Research 313's `0.33.3` `prompt_eval_cached_count` stop
is the named decoder-tolerance follow-up: accept and ignore that key,
then qualify through official `0.34.2` as a compatible extension of
`ollama.native-text-v1`. Remaining deltas stay bounded unmapped. Host
`ollama` is absent; the host was not mutated.

Production claims stay at `0.33.2` in this record.

## Next

Apply the compatible-extension decision: decoder-tolerance, then raise
`ollama.runtime` through `0.34.2`.
