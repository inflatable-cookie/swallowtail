# Grok Build 0.2.0 ACP Evidence

Captured 2026-07-24 from the exact public npm launcher and darwin-arm64
platform artifacts. The executable was decompressed into a temporary
directory, never installed, and run with an empty `HOME` and `GROK_HOME`,
`--no-auto-update`, and host-denied network access.

Only `--version`, ACP `initialize`, and an unauthenticated `session/new` were
sent. No credential, login, authentication request, model request, prompt, or
paid provider operation was used.

The transcript removes volatile host path, hostname, agent id, and process
instance id fields. It preserves the observed protocol, capability,
authentication-method, model, and error semantics.

This is an inspected comparison point, not a qualified release.
