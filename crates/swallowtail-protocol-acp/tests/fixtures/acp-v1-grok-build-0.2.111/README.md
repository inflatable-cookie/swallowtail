# Grok Build 0.2.111 ACP Evidence

Captured 2026-07-24 from the exact public npm launcher and darwin-arm64
platform artifacts. The executable was decompressed into a temporary
directory, never installed, and run with an empty `HOME` and `GROK_HOME`,
`--no-auto-update`, and host-denied network access.

Only `--version`, help, ACP `initialize`, and an unauthenticated
`session/new` were sent. No credential, login, authentication request, model
request, prompt, or paid provider operation was used.

The transcript removes volatile host path, hostname, agent id, and process
instance id fields. It preserves the observed protocol, capability,
authentication-method, model, reasoning, and error semantics.

The bundled docs and exact behavior disagree with the public ACP example about
the advertised authentication method ids. They also confirm that permission
policy, sandboxing, ambient configuration, hooks, and session retention are
separate. This release remains a candidate, not a qualified support point.
