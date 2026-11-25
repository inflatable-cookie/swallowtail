# Retired ACP analysis chain

Brief v4 (2026-11-17) removes the abandoned static-provenance/test-host chain
from active source. It is not required to run trusted disposable qualification.

Exact archived source is at commit `98067e2e2e49f946f9eeac6d386e946c749c48ad`.
The historical archive was 332,243 bytes with SHA-256
`58335cc9a18e532ab17bad24640b802f0a10f52236a5849837d96e684da96ecf`.
The [retained reconstruction](https://github.com/inflatable-cookie/swallowtail/pull/558)
contains the same 51 Git source files and an `archive-index.json` with each
path, byte size, SHA-256 and Git head. It is 332,378 bytes with SHA-256
`a6bc700855d4a3a064d0265b5e9e2afa7eedfd85a8047a28f98dbea391e48369`.
No runtime, build or CI depends on either archive.
Use `git show <head>:<path>` to reproduce an original file without a new loader.

Removed: exhaustive import/callsite catalogues, static admission/invocation/
startup receipt models and verifiers, old component proposal/runner, prototype
idle sandbox and their task selectors. These served the superseded admission
project. Accepted source/fake reviews and consumed original component evidence
retain their exact historical head scopes; old failures stay failures.

Kept: frozen npm integrity and dist/dependency/native identities, production
checked graphs and idle guard, permission/native fixtures, actual Rust lifecycle
regressions, startup refusal tests and the current original-runner workflow.
Frozen graph reconstruction used the retired builder at its original head;
current tests pin the shipped graphs directly. No source coverage is inferred
from missing catalogues, and no original qualification is inferred from history.

The old shared Python supervisor/kernel proposal and installed-runtime wrapper
are also archived. Startup-shaped fallback and refusal regressions now live in
the direct owned fixture and captured-preload tests. Per-file original commits
are in the archive index; the later snapshot is `c7ef76625e41b9f9b79396c03660075873897d2c`.
