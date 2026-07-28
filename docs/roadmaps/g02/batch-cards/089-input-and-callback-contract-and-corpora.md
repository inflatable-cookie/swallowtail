# 089 Input And Callback Contract And Corpora

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../027-input-and-callback-feature-closure.md`
Depends on: card 088

## Objective

Promote the smallest missing input/callback rules and freeze the selected
route corpora.

## Scope

1. Promote only distinctions required by Research 050's selected tranche:
   - Pi RPC attachment input
   - OpenCode HTTP attachment and approval-or-question exchange
   - Anthropic Messages attachment, consumer-tool continuation, and
     provider-owned external search
2. Keep requested, planned, dispatched, provider-requested, consumer-resolved,
   and effective states separate.
3. Preserve exact media, size, count, schema, callback, and timeout bounds.
4. Keep provider-owned tools and search separate from consumer callbacks.
5. Keep one-attempt structured inference separate from Contract 030 direct
   tool continuation; qualify the adjacent Anthropic interactive role.
6. Freeze:
   - Pi `0.80.10` image prompt and rejection records
   - OpenCode `1.14.48..=1.18.4` file-part, permission, and question records
   - Anthropic `2023-06-01` image, client-tool, and provider-search records
7. Freeze success, rejection, cancellation, deadline, drift, cleanup, and
   unverified-newer records without live access.
8. Make card 090's prepared evidence and conformance expectations exact.

## Acceptance Criteria

- [x] contracts make the selected tranche deterministic
- [x] fixtures require no live access
- [x] authority and correlation remain exact
- [x] unsupported and observed-only behavior remain explicit
- [x] card 090 names exact routes and cells

## Result

- Contract 041 separates finite attachments, native consumer tools,
  provider-owned tools, approval or question extensions, and external search.
- Pi `0.80.10` freezes one bounded base64 `image/png` prompt plus rejection,
  abort, and joined cleanup.
- OpenCode freezes file, permission, and question shapes across all 45
  qualified releases. Four exact surface revisions preserve error-schema and
  message-id changes without losing the selected capability.
- Anthropic `2023-06-01` freezes image input, an explicit Contract 030 client
  tool continuation, provider-owned `web_search_20250305`, rejection,
  cancellation, and credential-last cleanup.
- Nine focused corpus tests pass without live access.

## Auto-Continuation

Satisfied. Continue to card 090.
