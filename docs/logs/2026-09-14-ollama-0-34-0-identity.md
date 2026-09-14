# 2026-09-14 Ollama 0.34.0 Identity

## Result

Official Ollama `v0.34.0` was frozen against the `0.32.15` claim. GitHub
reports `v0.34.0` as latest stable with no newer point; tag tarballs were
retrieved and never executed. All five published hops after the ceiling
(`0.33.0`, `0.33.1`, `0.33.2`, `0.33.3`, plus `0.34.0`) are frozen with
tag, commit, tree, and selected-file hashes: the eight selected structs
are byte-identical from `v0.32.15` through `v0.34.0`, and `routes.go` is
byte-identical through `v0.33.2`. Hops through `0.33.2` classify as a
compatible extension with the only types delta confined to the unselected
experimental model-recommendation mappings surface. `0.33.3` is the stop:
`Metrics` gains optional `prompt_eval_cached_count`, plumbed into selected
`POST /api/chat` NDJSON records, and the strict `ollama.native-text-v1`
chat decoder fail-closes on that unmapped key; the frozen counterexample
record proves rejection while the keyless twin decodes. The host keeps
observation-only client `0.33.3` from a 50-byte launcher shim with no
reachable runtime; the host install was not changed.

Production claims stayed at `0.32.15` in this record; two
mutation-sensitive identity tests enforce that boundary.

## Next

Apply the compatible-extension-with-stop decision on the Ollama claim
surfaces through g05.063: qualify `0.33.0` through `0.33.2`, leave
`0.33.3` and `0.34.0` `UnverifiedNewer` with the named decoder reason.
