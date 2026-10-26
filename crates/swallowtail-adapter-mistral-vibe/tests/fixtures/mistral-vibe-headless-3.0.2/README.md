# Mistral Vibe 3.0.2 headless identity stop

Observed 2026-10-26 from the official PyPI project and GitHub stable
releases. The qualified ceiling on `mistral-vibe.headless` stays `2.26.0`.

The fixture freezes PyPI sdist digests for `2.26.0`, `2.26.1`, `3.0.0`,
`3.0.1`, and `3.0.2`, the linux x86_64 wheel that was downloaded and
hashed, registry digests for the other platform wheels, and selected
source flags. Selected sdist files match the GitHub tag tarball
byte-for-byte. The linux wheel's Python copies of those files match the
sdist.

`2.26.1` removes the mapped turn-limit stderr and exits 3. `3.0.0` removes
`--legacy-harness`, the builtin `plan` agent, and the Python programmatic
serializer. `vibe` always execs the Rust CLI. `~/.vibe` and `VIBE_HOME`
stay the config home through `3.0.2`.

No downloaded artifact was executed. No Vibe installation, provider prompt,
credential use, live session, or host mutation occurred.
