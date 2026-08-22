# 186 DeepSeek Reasoning Control Evidence

Status: promoted
Owner: Tom
Created: 2026-08-22
Updated: 2026-08-22

## Question

Which exact DeepSeek V4 reasoning-effort and thinking-mode combinations can
Swallowtail bind without changing the qualified facade or weakening private
direct continuation?

## Method And Evidence Boundary

The official DeepSeek documentation was fetched and rechecked on 2026-08-22.
The review used only public, secret-free documentation. It did not authenticate,
inspect an account or balance, send a prompt, invoke a model, or mutate
provider state.

The fetched HTML specimens and SHA-256 digests are:

| Surface | URL | Specimen SHA-256 |
| --- | --- | --- |
| Chat Completions API | <https://api-docs.deepseek.com/api/create-chat-completion/> | `452902008200767f318c8353cc225fca241777d8cd3f0b764fb94ffa7a612dea` |
| Thinking Mode guide | <https://api-docs.deepseek.com/guides/thinking_mode/> | `d9c7bf018583b542431aa91c995ee64a8c7aa3df32286c10634a04d2e1661982` |
| Tool Calls guide | <https://api-docs.deepseek.com/guides/tool_calls/> | `d50d330bc0e1f30b84ee804d77f1ad3f7073e5d2557a6a80545d2d2696ea2471` |
| Models and Pricing | <https://api-docs.deepseek.com/quick_start/pricing/> | `c6db3039404a4d108cf3c4e9b6c1891e4f446a6e8e8a3cfdf3c1eff739e194e6` |

The digests identify the complete fetched HTML response, not a provider
payload. They are provenance for the dated review; they do not make a mutable
provider page a permanent compatibility guarantee.

## Frozen Official Findings

The current OpenAI-format Chat Completions schema names:

```json
{
  "model": "deepseek-v4-pro",
  "thinking": {"type": "enabled"},
  "reasoning_effort": "low|high|max"
}
```

`thinking.type` is exactly `enabled` or `disabled`, and its documented default
is `enabled`. The schema documents `low`, `high`, and `max` as the current
reasoning-effort values. It also says that `medium` and `xhigh` are mapped to
`high` for compatibility. The API response schema identifies
`reasoning_content` as thinking-mode-only output.

The Models and Pricing page lists `deepseek-v4-flash` and `deepseek-v4-pro`
on the OpenAI base URL and records tool calls and both thinking/non-thinking
modes. This run remains restricted to the already-qualified
`deepseek-v4-pro`; V4 Flash is a separate model route and is not promoted by
this evidence.

The Thinking Mode and Tool Calls guides require the complete private
`reasoning_content` assistant material to be sent back on subsequent requests
when a request carries tools. The material remains provider-private in
Swallowtail. This confirms the existing bounded replay lifecycle and does not
authorize consumer-visible reasoning output, automatic tool execution, or a
larger continuation loop.

The API schema also shows that the exact thinking toggle is an operation
field, not a portable `ReasoningMode` value. No generic boolean or provider
options map is admitted here.

## Exact Disposition

### Model and facade

| Item | Structured run | Direct continuation | Disposition |
| --- | --- | --- | --- |
| `deepseek-v4-pro` | exact | exact | Deliver now |
| `deepseek-v4-flash` | not qualified | not qualified | Withhold; separate route evidence required |
| `deepseek-chat` / `deepseek-reasoner` | retired aliases | retired aliases | Reject; never normalize |
| `deepseek-openai-chat-2026-07-22` | sufficient | sufficient | Keep current facade |

The current facade already has the exact OpenAI-format request and response
shape required by the new field values. No new facade revision, behavior
segment, or Contract 030 amendment is required.

### Reasoning effort

| Portable `ReasoningMode` | Wire `reasoning_effort` | Structured run | Continuation | Disposition |
| --- | --- | --- | --- | --- |
| `low` | `low` | enabled | enabled | Deliver now |
| `high` | `high` | enabled | enabled | Deliver now; preserve existing path |
| `max` | `max` | enabled | enabled | Deliver now |
| `medium` | would map to `high` | — | — | Reject before provider work |
| `xhigh` | would map to `high` | — | — | Reject before provider work |
| unknown or provider alias | not exact | — | — | Reject before provider work |

The adapter sends the exact selected value. It never clamps, aliases, or
reports an effective reasoning depth from output or usage.

### Thinking mode

| Wire `thinking.type` | Structured run | Continuation | Disposition |
| --- | --- | --- | --- |
| `enabled` | exact and fixed | exact and fixed | Deliver now |
| `disabled` | documented upstream | not compatible with the admitted private replay proof | Withhold in this run |

`disabled` is not mapped onto a portable effort value. It is withheld for
structured runs because this run has no qualified typed adapter-local control
for the independent field, and withheld for continuation because the route's
tool-bearing session proof requires thinking-mode private replay. A later
card may qualify a structured-only typed control with its own evidence; this
run does not imply that it is impossible upstream.

## Attempt And Profile Proof Boundary

Every admitted profile uses one preparation-time effort selection and sends
that exact value on every request attempt:

| Profile | Initial | Tool-result continuation | Final stream | Later user turn | Restoration |
| --- | --- | --- | --- | --- | --- |
| Structured run | SSE, `thinking=enabled` | not applicable | one attempt | not applicable | not applicable |
| Direct continuation | buffered, `thinking=enabled` | SSE, same effort | SSE, same effort | SSE, same effort | fresh session reuses same prepared request |

The direct session keeps the selected effort in the prepared request and
session binding. Existing limits remain unchanged: two user turns, three
provider attempts, one tool result, bounded private continuation and history,
no automatic retry, and no provider-session resume.

Failure, cancellation, deadline, route mismatch, model mismatch, or cleanup
failure invalidates the existing private lifecycle. No failure path changes
effort, thinking mode, model, facade, cache posture, endpoint, credentials,
tool-loop bounds, or output limits.

## Contract And Compatibility Verdict

The result is a typed extension of the existing exact `ReasoningMode` path.
It satisfies Contract 040 because the portable request, capability constraint,
immutable plan, prepared evidence, driver binding, and wire field all retain
the same exact value. It satisfies Contract 030 because one fixed selection is
reused across the entire direct session and fresh local restoration while
`reasoning_content` remains adapter-held, bounded, replayed only to the same
route, and never disclosed.

No facade revision, compatibility segment, contract change, shared architecture
change, provider route-matrix change, or currentness claim is made by this
record.

## Promotion

Research 186 promotes the following deliver-now subset for card 105:

- exact `deepseek-v4-pro`
- exact `deepseek-openai-chat-2026-07-22`
- `ReasoningMode` values `low`, `high`, and `max`
- fixed `thinking.type=enabled`
- structured and direct-continuation profiles, with the existing attempt,
  cache, tool, output, endpoint, credential, and privacy bounds unchanged

Card 105 may bind this subset. Card 106 must prove exact request agreement and
the unchanged high/enabled fixture path without claiming effective reasoning
depth or provider acceptance beyond deterministic dispatch evidence.
