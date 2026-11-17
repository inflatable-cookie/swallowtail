# Retired ACP analysis chain

Brief v4 (2026-11-17) removes the abandoned static-provenance/test-host chain
from active source. It is not required to run trusted disposable qualification.

Exact archived source: commit `98067e2e2e49f946f9eeac6d386e946c749c48ad`. The Nucleus proof archive contains
38 original files and an `archive-index.json` with each path, byte size,
SHA-256 and Git head. Archive SHA-256: `d51ffee5f1acd48571c08bf5bfbe8c2ddb242660e5964605d9cc832c025dcfc7`; compressed size:
300,143 bytes. No runtime, build or CI depends on that archive.
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
