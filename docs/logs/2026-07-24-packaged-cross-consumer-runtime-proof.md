# Packaged Cross-Consumer Runtime Proof

Date: 2026-07-24
Roadmap: g02.006
Card: 015

## Outcome

The candidate-consumer gate now proves runtime preparation instead of stopping
at compilation. It builds a transient clean 23-package candidate, extracts the
package family, copies every consumer source dependency into isolated
snapshots, patches exact `=0.1.0` package artifacts, and runs consumer Codex
selectors plus the packaged Codex adapter suite.

The original Nucleus, Soundcheck, Soundcheck Library, and Signal repositories
remain untouched by the proof.

## Identities

- candidate source:
  `1df7bd7c313469c84f8067f06f9b03817638273c`
- package checksum-manifest digest:
  `07dbbfc85772e1821394f0789c12ae3bd2d72beb364a4ed61632c839c6e77c34`
- Nucleus snapshot:
  `b585625f63ceebb4b280478af31b661df68d0a25`
- Soundcheck snapshot:
  `2ced34b19c548d2f87b18d96acd8b84ef153e392`
- Soundcheck Library snapshot:
  `262e8e1a81f8b859682c73b4b6cec94b7d969874`
- Signal snapshot:
  `18be4dc2a45435283826254a7235ce6f42f9bc6a`
- consumer evidence:
  `c0ce8d84d7e36081462969e5cb0660cd051ee9e69e210a4f7bc4272f078e476a`

## Validation

- full package assembly, content and forbidden-path audit, checksums, extracted
  workspace check, and test compilation: pass
- packaged Codex tests reuse the candidate source's frozen Cargo lock: pass
- Nucleus packaged selectors: 14 passed, 2 live probes ignored
- Soundcheck packaged selectors: 4 passed, 1 live probe ignored
- packaged Codex adapter: 89 passed
- exact-version catalogue, read-only, bounded-workspace, structured exec,
  reasoning, search, schema, attachment, tool/callback, access provenance,
  failure-before-effects, cancellation, deadline, cleanup, and redaction:
  pass
- credentials: absent
- provider calls: none
- `git diff --check`: pass

`effigy doctor` still reports the known 19 oversized-file findings: seven
errors and 12 warnings. The category and count are unchanged.

## Recorded Next Step

Card 016 is ready to replace the held unpublished candidate and its handoffs
from one validated prepared-facade source snapshot. Publication remains a
separate operator decision.
