# Mistral Vibe 2.26.0 headless identity

Observed 2026-10-08 from the official PyPI project and GitHub stable releases.
The fixture freezes every PyPI wheel file from the retained `2.25.4` point
through `2.26.0`, the exact sdist identities, selected source files for every
published hop, and the platform-specific native payloads. Wheel inventories
are sorted by archive path and record each regular file's SHA-256 and
uncompressed size.

The `v2.26.0` tag points to `376f6a33413a3eec9b3795b0c0e004066c47b5c`.
PyPI provenance names `7cb91894c40bb25173abcfa36e5ea2b4b81eb28c`; the complete
`vibe/` source tree matches the tag byte-for-byte. The PyPI sdist and each
wheel's Python package also match the tag apart from the build-time Sentry DSN
in `vibe/observability/sentry.py`; Sentry remains gated by Vibe's telemetry
configuration. Windows wheel Python files use CRLF line endings; their
line-ending-normalized contents match other platforms.

No downloaded artifact was executed. No Vibe installation, provider prompt,
credential use, live session, or host mutation occurred.
