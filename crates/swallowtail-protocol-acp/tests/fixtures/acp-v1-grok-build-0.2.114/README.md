# Grok Build 0.2.114 Authenticated ACP Evidence

Captured 2026-07-30 from the exact installed, signed darwin-arm64 executable
and matched to the public npm artifact.

The operator had already authenticated the normal Grok installation. The probe
sent ACP `initialize`, one `cached_token` activation with headless metadata,
and `session/new` in an empty temporary working directory. It sent no prompt,
model request, tool request, login, API key, update, or installation request.

The activation response contained provider-private account metadata. The
normalized transcript removes that metadata completely. Tests require the
production mapping to treat any successful result as opaque and discard it.
No credential content, account identity, host path, or live session id is
retained.

The probe created one empty durable local session and normal Grok-owned
bookkeeping. The existing authentication file hash was unchanged.
