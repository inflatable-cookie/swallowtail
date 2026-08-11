# Claude Code Response-Only Thinking Progress

Date: 2026-08-11
Roadmap: g03.066

## Outcome

Exact Claude Code `2.1.227` response-only medium-effort runs now accept the
consumer-proven thinking-progress sequence without exposing thought content.
The implementation commit is
`71db67be4f8929199afe3b2d9b3db8d432f79169`.

## Contract And Projection

Contract 039 now admits `system/thinking_tokens` only after the exact init and
before assistant text. Positive integer totals are capped at 1,000,000 and
must rise exactly by each positive delta from zero. Each valid frame becomes a
content-free coalescible `ProgressSnapshot`, not usage or reasoning.

Exact live evidence also exposed one assistant `thinking` envelope before the
text envelope. Its thinking text is empty and its opaque signature is
non-empty. Swallowtail validates session, message, model, role, stop reason,
order, empty thought text, and signature, then discards the envelope. The
following text envelope must use the same message id. Unknown system or
assistant shapes still fail closed.

## Boundary

- init still requires `tools=[]` and `mcp_servers=[]`
- command, prepared API, access, host services, retention, cancellation,
  deadline, cleanup, and redacted failures are unchanged
- local Max/OAuth succeeds with `HOME`, `USER`, and `LOGNAME`; the live probe
  rejects any ambient `ANTHROPIC_API_KEY`
- no workspace, schema, tool, MCP, callback, attachment, retry, continuation,
  fallback, session binding, or structured-output capability was added
- `claude-code.headless` is unchanged

## Evidence

- exact synthetic fixture: init, two cumulative progress frames, empty private
  thinking envelope, assistant text, and matching result
- deterministic session, sequence, integer type, zero, upper-bound, delta,
  missing-field, signature, thought-text, message-id, duplicate, and unknown
  subtype failures
- focused validation: 80 tests passed
- affected-package extraction and dependency closure passed
- guide and route matrix gates passed
- separately gated prepared-facade live probe passed normal text, complex
  medium-effort progress, local cancellation, clean process wait/join, and
  unchanged repository status
- docs links and vision, logs, and research indexes passed; the existing
  Effigy roadmap-index relative-path defect remained the only docs failure

Effigy doctor's inherited god-file, stale-graph, and generated-in-source
findings were not introduced by this lane. `PAPERCUTS.md` was not modified.
No release or tag was created.

## Next Task

In Figmatic, link Swallowtail commit
`71db67be4f8929199afe3b2d9b3db8d432f79169` and replay unit
`fc335758-3c1a-4bda-bb71-a8c6119fe876`. Keep the returned text under the
existing downstream parser, schema-v4 validation, compiler, gates, and
operator acceptance.
