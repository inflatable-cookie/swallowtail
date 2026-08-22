# 185 Anthropic Messages Effort Evidence

Status: promoted
Owner: Tom
Created: 2026-08-22
Updated: 2026-08-22

## Question

Which exact Anthropic Messages models, `output_config.effort` values, and
operation profiles can Swallowtail bind under Contract 040?

## Method

Official Anthropic documentation only, retrieved 2026-08-22. No account,
credential, catalogue call, prompt, or provider request was used. The existing
route identity is the `anthropic.messages` direct Messages facade at
`anthropic-2023-06-01`; the prepared profiles are the resource-free structured
one-attempt route and the fixed direct-continuation session from Research 004,
067, and 169.

## Finding

The smallest exact subset that survives Contract 040 is:

| Model | Effort values | Profiles | Disposition |
| --- | --- | --- | --- |
| `claude-opus-4-7` | `low`, `medium`, `high`, `xhigh`, `max` | one-attempt structured; direct continuation with one value fixed at preparation and repeated on every attempt and fresh restoration | **deliver-now** |

Anthropic documents `output_config.effort` as a request-level control and
separates it from thinking. The effort control does not require thinking to be
enabled. The Opus 4.7 guidance names `xhigh` as the starting level for coding
and agentic work and documents the five values above as the usable range for
this model. The route therefore emits only `output_config.effort`; it does not
add `thinking`, infer a value from output text, clamp values, or claim effective
effort without provider confirmation.

The existing fixture ids (`claude-fixture-primary` and
`claude-fixture-search-capable`) are not official model support evidence and
remain **withheld** for effort. Other Anthropic model ids remain **evidence-
gated** for this route-local tranche: family membership, catalogue presence,
another Claude product, or a current model overview does not qualify an exact
Messages model/value/profile row. Managed Agents, Claude Code, Ultracode, Fast
mode, newer web search, and Messages thinking are **not applicable** to this
finding.

`high` is still an explicit deliver-now value even where Anthropic describes it
as the default recommendation for some workloads. Omission remains the
existing no-selection behavior and is not rewritten as an explicit effort
value. `xhigh` and `max` are distinct values; neither is a portable default.

## Contract 040 Mapping

Each deliver-now row maps to one exact portable `ReasoningMode` inside a
`Capability::ReasoningSelection` requirement. The immutable plan, prepared
evidence, operation policy or session options, driver validation, and wire
request must agree on that same value. An unsupported model, value, profile, or
request/plan mismatch fails before endpoint authorization or credential use.

For sessions, the value is selected once during preparation. Every provider
attempt in a turn and every later turn uses the prepared value; fresh session
restoration clones the prepared request. There is no per-turn raw override.

Claim bounds stop at planned, dispatched, and provider-request acceptance
states. This evidence does not prove provider-effective reasoning depth.

## Official Sources

| Source | Use | Retrieved | SHA-256 of retrieved source body |
| --- | --- | --- | --- |
| [Effort](https://platform.claude.com/docs/en/build-with-claude/effort) | `output_config.effort`, five values, model availability, thinking independence, and Opus 4.7 recommendations | 2026-08-22 | `c77cac7528a8432a30bc3d8122fae17f5f34162a9bddeec11e3a16321de557d3` |
| [Messages create API](https://platform.claude.com/docs/en/api/messages/create) | request shape and `output_config.effort` enum | 2026-08-22 | `74d36f6799a49bcfb03e476bf9625e77f9077a2c4bb91402f672bd94303124b8` |
| [Model IDs and versions](https://platform.claude.com/docs/en/about-claude/models/model-ids-and-versions) | exact `claude-opus-4-7` model identity form | 2026-08-22 | `e7814e491478d19bff0e372c71382c439d1d5d8261b49ef974abebf6131ce2af` |
| [Extended thinking models](https://platform.claude.com/docs/en/about-claude/models/extended-thinking-models) | thinking is a separate control; no thinking synthesis in this tranche | 2026-08-22 | `e609bbf72cf59eda1f02ae4c268eae2cac01296150bfc1963eea7cadb9f5b02c` |

## Deterministic Specimen

The selected request shape is the existing Messages request with one additive
field. For `xhigh`, the relevant object is:

```json
{"max_tokens":64,"messages":[{"content":"fixture prompt","role":"user"}],"model":"claude-opus-4-7","output_config":{"effort":"xhigh"},"stream":true}
```

No-selection requests remain byte-identical to the existing
`tests/fixtures/anthropic-2023-06-01/message-request.json` and
`client-tool-result-request.json` fixtures. The specimen is a wire-shape
fixture, not provider acceptance or effective-effort evidence.
