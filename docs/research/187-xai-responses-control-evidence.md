# 187 xAI Responses Control Evidence

Status: promoted
Owner: Tom
Date: 2026-08-22

## Question

Which exact current xAI Responses WebSocket model, reasoning-effort, and
maximum-output-token combinations can Swallowtail bind without changing the
qualified facade or overstating connection-local continuation?

## Method And Boundary

Official xAI documentation was retrieved on 2026-08-22. The work used public,
secret-free pages and local SHA-256 snapshots. No credential, account, live
provider request, prompt, provider output, catalogue response, or provider
state was used.

The selected operation remains only `xai.responses-websocket`:

- public API-key access;
- exact facade `xai-responses-websocket-2026-04-23`;
- one-response structured run;
- serial, connection-local interactive session;
- no tools, search, code execution, files, citations, multi-agent controls,
  encrypted reasoning export, storage, reconnect, or reattachment.

The separate xAI language-model catalogue is not route qualification. Model
aliases are not exact model identities. Omitted controls retain the existing
route behavior and do not turn provider defaults into an explicit selection.

## Frozen Official Sources

| Source | Exact finding used | Retrieved | SHA-256 snapshot |
| --- | --- | --- | --- |
| [WebSocket mode](https://docs.x.ai/developers/advanced-api-usage/websocket-mode) | `/v1/responses`; `response.create` uses the Responses create body minus `stream` and `background`; first and `previous_response_id` continuation specimens; serial connection-local cache | 2026-08-22 | `7c9bac3c6d069a4605ff7e7da36750aaafda1a071d633001b7a99bd860e893dc` |
| [Reasoning](https://docs.x.ai/developers/model-capabilities/text/reasoning) | Grok 4.5 low/medium/high; Grok 4.6 low/medium/high/xhigh; default high; xhigh is treated as high on unsupported models; multi-agent effort controls agent count | 2026-08-22 | `f21efa89c7c2b7e2dd70a66d00e700a1d7dec5baefeb9083258a253bf9f2c907` |
| [Grok 4.5 model page](https://docs.x.ai/developers/models/grok-4.5) | Exact model identity `grok-4.5`; aliases are separate names; model metadata advertises reasoning values but does not override the reasoning guide's xhigh treatment | 2026-08-22 | `36112deb6801236f98b0da06464174e157b547b560a27aac68e0b717f894aab8` |
| [Grok 4.6 model page](https://docs.x.ai/developers/grok-4-6) | Exact model identity `grok-4.6`; Responses support; no intrinsic text output limit; low/medium/high/xhigh reasoning | 2026-08-22 | `468f25247fab16b8df56658fcbb939d77e405af9704b09e7765ccea68081786c` |
| [Models](https://docs.x.ai/developers/models) | Exact IDs and aliases remain distinct; catalogue/model-page visibility does not qualify a route alias | 2026-08-22 | `dfedec471fd959365e0a54e6ce8d80fd78ed2126665d6e790e01933465e0e9c9` |
| [Responses reference](https://docs.x.ai/developers/rest-api-reference/inference/chat) | `max_output_tokens` is an optional int32 request field and covers output plus reasoning tokens; unset has a provider default; selected caller values remain explicit bounds | 2026-08-22 | `c342fdfa754bc791fcc3f7ad0739ed3c2e72e4fe18cf1b35d2ed1086cb971281` |
| [Responses comparison](https://docs.x.ai/developers/model-capabilities/text/comparison) | Responses maps the generation maximum to `max_output_tokens`; continuation uses `previous_response_id` | 2026-08-22 | `2232433dd9aca65d3fb6777a4a9e625aadc2ebfbcd2347399679b3c9d5876292` |
| [Release notes](https://docs.x.ai/developers/release-notes) | Current release notes corroborate Grok 4.5 low/medium/high and Grok 4.6 low/medium/high/xhigh plus no intrinsic Grok 4.6 text output limit | 2026-08-22 | `61dd0b9f6cf54e481151443854ddee5e68addea7560c58f5417897590ea73fe2` |

The HTML snapshots preserve the source pages at retrieval time. The exact
JSON specimens below are the stable provider examples normalized only for the
secret-free fixture and control comparison; they are not provider output.

## Exact WebSocket Specimens

The official WebSocket first-turn specimen is:

```json
{
  "type": "response.create",
  "model": "grok-4.6",
  "store": false,
  "input": [{
    "type": "message",
    "role": "user",
    "content": [{"type": "input_text", "text": "..."}]
  }],
  "tools": []
}
```

The later-turn specimen adds only the connection-local continuation id:

```json
{
  "type": "response.create",
  "model": "grok-4.6",
  "store": false,
  "input": [{
    "type": "message",
    "role": "user",
    "content": [{"type": "input_text", "text": "..."}]
  }],
  "tools": [],
  "previous_response_id": "resp_example"
}
```

The Responses reasoning and bound fields fit that body without transport
fields. A qualified control specimen is therefore:

```json
{
  "type": "response.create",
  "model": "grok-4.6",
  "store": false,
  "input": [{
    "type": "message",
    "role": "user",
    "content": [{"type": "input_text", "text": "..."}]
  }],
  "tools": [],
  "reasoning": {"effort": "xhigh"},
  "max_output_tokens": 512
}
```

`max_output_tokens` has a positive portable domain of `1..=2_147_483_647`:
the lower bound is Contract 040's positive caller maximum and the upper bound
is the official request schema's signed int32 range. The field includes
reasoning tokens, so this record claims dispatch of a caller bound only. It
does not claim an exact text length or effective reasoning depth.

## Exact Dispositions

### Model and value rows

| Exact model input | Portable reasoning values | `max_output_tokens` | Disposition |
| --- | --- | --- | --- |
| `grok-4.5` | `low`, `medium`, `high` | `1..=2_147_483_647` | Deliver for structured run and serial session |
| `grok-4.5` | `xhigh` | any | Withhold: official reasoning guidance says unsupported-model xhigh is treated as high; no exact portable mapping |
| `grok-4.6` | `low`, `medium`, `high`, `xhigh` | `1..=2_147_483_647` | Deliver for structured run and serial session |
| `grok-4.5-latest`, `grok-build-latest`, or another alias | any explicit control | any | Withhold: alias is not an exact model route |
| `grok-4.20-multi-agent` | any effort value | any | Not applicable: effort controls agent count, not reasoning depth; multi-agent is outside this route |
| any other model id | any explicit control | any | Withhold: no exact Research 187 qualification |

The model pages' embedded metadata lists `xhigh` for Grok 4.5 while the
reasoning guide explicitly documents the unsupported-model treatment. The
conservative exact mapping follows the guide and withholds Grok 4.5 xhigh;
the discrepancy is recorded rather than flattened.

### Profile and control rows

| Profile | Exact model/value requirement | Reasoning selection | Output bound | Disposition |
| --- | --- | --- | --- | --- |
| Structured run | `grok-4.5` or `grok-4.6` | omitted | omitted | Existing absent-control path; preserve current body |
| Structured run | exact admitted model/value above | one admitted reasoning value | omitted | Deliver; add `reasoning.effort` only |
| Structured run | exact admitted model | omitted | positive int32 value | Deliver; add `max_output_tokens` only |
| Structured run | exact admitted model/value above | one admitted reasoning value | positive int32 value | Deliver; controls are independent |
| Serial session | `grok-4.5` or `grok-4.6` | omitted | omitted | Existing absent-control path; preserve current body |
| Serial session | exact admitted model/value above | one preparation-time reasoning value | omitted | Deliver on first, later, and fresh replacement turns |
| Serial session | exact admitted model | omitted | positive int32 value | Deliver on first, later, and fresh replacement turns |
| Serial session | exact admitted model/value above | one preparation-time reasoning value | positive int32 value | Deliver; one fixed selection spans the connection-local chain |

The selection is bound before endpoint or credential use. A failed turn does
not change it. Fresh-session restoration reuses the prepared plan and request;
it does not restore provider state or introduce a per-turn override.

## Contract And Facade Verdict

The existing `xai-responses-websocket-2026-04-23` facade remains sufficient.
The official WebSocket guide states that `response.create` follows the
Responses create body with only transport-only fields removed, and both
`reasoning` and `max_output_tokens` are body controls. Contracts 037 and 040
already require immutable prepared input, exact model-qualified constraints,
request/plan/evidence/dispatch agreement, fail-closed numeric validation, and
separation of requested dispatch from provider acceptance or effectiveness.

No facade segment, contract amendment, model alias, multi-agent mapping,
client-side truncation, or effectiveness claim is required. Card 108 may bind
only the rows marked Deliver above.

## Promotion

- promoted exact evidence and digests for card 107;
- qualified `grok-4.5` low/medium/high and `grok-4.6`
  low/medium/high/xhigh reasoning;
- qualified positive int32 `max_output_tokens` independently for both route
  profiles;
- withheld Grok 4.5 xhigh, aliases, other models, and multi-agent semantics;
- left production claims, matrices, shared indexes, and architecture unchanged.
