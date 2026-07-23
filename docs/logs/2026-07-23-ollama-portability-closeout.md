# Ollama Portability Closeout

Date: 2026-07-23
Cards: 108-109
Roadmap: 038

## Changed

- Completed the first production attached-runtime driver with a maintained
  multi-release compatibility window.
- Proved installed, running, and selected-detail catalogue observations under
  local and remote-authoritative host identities.
- Proved one-attempt native NDJSON output and usage, version drift before
  inference, independent runs, provider failure, malformed stream,
  disconnect, cancellation, deadline, redaction, and joined cleanup.
- Added a host catalogue observation clock distinct from monotonic deadline
  time and provider timestamps.
- Split production and test modules after doctor exposed two new size
  warnings; the inherited structural baseline is restored.
- Compiled roadmap 039 and card 110 for installed-harness compatibility ranges,
  beginning with Codex exec and app-server.

## Boundary

- Ollama remains external and runtime-managed.
- Swallowtail adds no install, model acquisition, cloud, credential, process,
  artifact, server lifecycle, unload, restoration, retry, fallback, container,
  or Monkey authority.
- Independent runs are not serialized without a provider or contract
  requirement.
- The live installed-runtime probe remains separately gated.

## Validation

- 522-test inventory: 518 runnable tests pass; four installed/live probes are
  ignored by default.
- Workspace all-target check, warnings-denied clippy, docs QA, formatting, and
  doc tests pass through `effigy qa`.
- Doctor remains at 19 inherited findings: seven errors and twelve warnings.
- `git diff --check` passes.

## Next

Card 110 revalidates installed harness version ranges. Codex exec and
app-server are the first candidates.
