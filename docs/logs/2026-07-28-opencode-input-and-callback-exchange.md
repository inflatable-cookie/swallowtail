# 2026-07-28 OpenCode Input And Callback Exchange

## Changed

- Added explicit PNG and provider-callback opt-ins to prepared OpenCode
  structured runs and interactive sessions.
- Bound one `image/png` attachment up to one MiB through the host attachment
  service, exact data-URL file part, actual-byte check, and one shared cleanup
  lease.
- Added exact `opencode/permission` and `opencode/question` callback
  namespaces.
- Exposed only `once` or `reject` permission replies. Persistent `always`
  remains unsupported.
- Preserved ordered question answers, provider request identity, callback
  identity, and exact run or turn ownership.
- Made callback-enabled operations declare ambient read/write authority.
  Default OpenCode sessions remain deny-first and read-only.
- Added provider-neutral run-extension callback construction instead of
  relabelling structured-run callbacks as turn callbacks.
- Changed OpenCode attachments and approval-or-question matrix cells from
  `No` to `Yes`.

## Evidence

- Focused OpenCode suite: 74 passed; one installed-server probe ignored.
- Prepared fixtures cover structured and interactive dispatch, exact file
  part, one-shot and reject replies, ordered answers, run/turn mismatch,
  persistent permission rejection, duplicate and late responses,
  cancellation abandonment, pre-materialization cancellation rejection,
  redaction, and one attachment release.
- Provider-route matrix check passes with 441 remaining `No` cells and 71
  remaining input/callback `No` cells.
- No installed server, live credential, provider request, or consumer
  repository was used.

## Risks

- OpenCode permission callbacks run against an ambient attached harness.
  Consumer mediation is authorization transport, not process containment.
- Unverified-newer stable servers retain the latest qualified mapping as
  best-effort behavior. The guaranteed range remains `1.14.48..=1.18.4`.
- Consumer-defined OpenCode tools still require a separate bounded MCP bridge.
  Provider-owned search remains unqualified.

## Next

Continue card 090 with Anthropic Messages image input, client-tool
continuation, and provider-owned web search. Card 091 remains the closeout.
