# 2026-09-17 Goose ACP 1.50.1 Claim

g05.081 reopens `goose.acp` at the single exact `QualifiedOnly` point
`1.50.1`. The new adapter-private behavior revision is
`goose.acp.stdio-v2.auth-required`: typed provider authentication failures on
`session/new` and `session/prompt` map to
`swallowtail.goose.acp.auth_required`. Other provider/model resolution errors
retain their existing diagnostic. Builtin, mode, lifecycle, permission,
process, and sibling capability claims remain independent.

The production claim is provider-free and carries no login, configure,
installation, live ACP, release, or tag action.
