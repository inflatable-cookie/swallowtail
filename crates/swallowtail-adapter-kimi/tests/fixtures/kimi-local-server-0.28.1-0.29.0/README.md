# Kimi Code local-server selected corpus

This bounded corpus qualifies the selected REST and WebSocket v2 boundary in
Kimi Code `0.28.1` and `0.29.0`.

Exact source:

- `0.28.1`: annotated tag `0032545b65f95c139ecba5a48ba1b911844e1ffe`,
  commit `efacf0452d46f5dbd67499eabc053869495d5213`
- `0.29.0`: annotated tag `03c34eefa49513e6216390a9773326077a37f414`,
  commit `8bf5bacba9e524c38fb808c0122070037ead25a8`
- repository: `https://github.com/MoonshotAI/kimi-cli`

Selected source files are byte-identical between those commits:

- `packages/kap-server/src/protocol/envelope.ts`
- `packages/kap-server/src/protocol/error-codes.ts`
- `packages/kap-server/src/protocol/rest-meta.ts`
- `packages/kap-server/src/protocol/rest-session.ts`
- `packages/kap-server/src/protocol/ws-control.ts`
- `packages/kap-server/src/middleware/auth.ts`

Fixtures are synthetic, secret-free instances of those schemas. OpenAPI and
AsyncAPI are bounded projections of the generated documents. They retain only
the route and channel identities selected by Swallowtail.

The selected HTTP surface has archive and restore actions. It has no session
DELETE method. The source's deprecated `deleteSessionResponseSchema` is an
alias of `archiveSessionResponseSchema`; it does not define a delete route or
delete effect.

All `/api/*`, `/openapi.json`, `/asyncapi.json`, and WebSocket access is bearer
protected. Only `GET /api/v1/healthz` is unauthenticated. No credential appears
in this corpus.
