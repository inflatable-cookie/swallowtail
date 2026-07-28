# 2026-07-28 Input/Callback Contract And Corpora

## Changed

- promoted Contract 041
- froze Pi `0.80.10` base64 image input
- froze OpenCode file, permission, and question shapes across all 45 qualified
  releases
- recorded four exact OpenCode surface revisions
- froze Anthropic `2023-06-01` image, client-tool continuation, and
  provider-owned search requests
- kept persistent OpenCode permission, ambient MCP, and live access excluded

## Current State

Card 089 is complete. Card 090 is ready for six cells:

- Pi attachments
- OpenCode attachments and approval-or-question exchange
- Anthropic Messages attachments, consumer-tool exchange, and external search

Anthropic client tools require a separate Contract 030 interactive role. The
existing structured role remains one attempt. Provider search stays separate
from client tools and requires explicit provider network, model, organization,
access, and billing evidence.

Cards 090-091 remain in bounds.

## Validation

- Pi input/callback corpus: 2 passed
- OpenCode input/callback corpus: 3 passed
- Anthropic input/callback corpus: 4 passed
- `effigy qa:routes`
- `effigy qa:docs`

No account, credential, provider request, container, or model server was used.

## Risks

- OpenCode's four range revisions change error and identifier schemas; the
  implementation must dispatch by the qualified behavior segment.
- Persistent OpenCode `always` permission remains visible upstream but
  unsupported.
- Anthropic provider search can fail for model or organization policy after
  dispatch without authorizing fallback.
- MCP-backed consumer tools remain later work.

## Next

Execute card 090. Implement the six selected cells through public prepared
paths and deterministic conformance.
