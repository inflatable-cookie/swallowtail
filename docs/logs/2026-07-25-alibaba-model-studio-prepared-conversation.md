# 2026-07-25 Alibaba Model Studio Prepared Conversation

Status: complete

## Changed

`swallowtail-adapter-alibaba-model-studio` now exposes an adapter-local
prepared integration and one typed conversation-open value.

Preparation binds the existing Singapore workspace-dedicated configured
instance, its revision, exact regional audience, general Model Studio API-key
profile, pay-as-you-go metering, host services, and access provenance.
Conversation preparation requires the exact route and
`qwen3.7-plus-2026-05-26` model plus explicit
`DurableConversationDeleteOnClose` provider state.

The bound open delegates to the unchanged low-level interactive-session
driver. Returned turns and close retain the native provider-conversation
lifecycle.

## Current Evidence

Official Model Studio documentation still recommends workspace-dedicated
production domains and keeps region, workspace, API key, endpoint, and model
list bound. Singapore Conversations supports separate item and conversation
deletion; Responses remains synchronous and provider storage stays explicit:

- [regions and access domains](https://help.aliyun.com/en/model-studio/regions/)
- [general API keys](https://help.aliyun.com/en/model-studio/get-api-key)
- [Conversations API](https://help.aliyun.com/en/model-studio/openai-compatible-conversations)
- [Responses API](https://help.aliyun.com/en/model-studio/qwen-api-via-openai-responses)

Current API-key formatting and workspace-domain guidance do not change the
selected credential mechanism or opaque host-approved endpoint boundary. No
research or contract delta was required.

## Native Boundaries

- another region, workspace, endpoint, key, plan, route, or model cannot
  substitute
- Coding Plan and subscription metering do not satisfy general API access
- retention is visible and agreed before provider conversation creation
- provider conversation context is not consumer transcript persistence
- each turn remains one synchronous streaming provider attempt
- close inventories items, deletes each item, then deletes the conversation
- item and conversation deletion truth remain separate
- cancellation and deadline do not fabricate remote stop or deletion truth
- no resume, retry, response storage, background execution, or fallback was
  added

## Validation

- 18 unit, selection, protocol, driver, conformance, prepared-facade, and
  example targets pass
- prepared two-turn sessions pass under local and remote-authoritative host
  identities
- plan-key metering, retention omission, model alias, and target drift fail
  before effects
- credential release occurs after nine joined blocking operations

## Next

Card 029 adds separate OpenAI background Responses create and bounded
reattachment preparation. Cards 029-036 remain in the provider-wide facade,
package-proof, and replacement-candidate runway.
