# 214 OpenCode HTTP Web Search Evidence

Status: promoted; evidence stop
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Card: g04.067 / 187

## Question

Which exact OpenCode HTTP versions, providers/backends, permission rules,
operation profiles, and shared search/network policy rows can admit native
`websearch` without ambient configuration authority, permission/network
conflation, or live-provider inference?

## Method And Boundary

Evidence was collected on 2026-08-26 with no OpenCode install, attached-server
mutation, credential capture, account or backend-key inspection, provider
prompt, hosted search, or paid operation. Official GitHub tag `v1.18.20` was
downloaded to a disposable `/tmp` path and digested as the binding corpus.
Current `opencode.ai` permissions/tools HTML was retrieved as corroboration
only; those bodies are SPA shells and do not replace the tagged source.

The route is `opencode.http`, driver `swallowtail.opencode.http`, axis
`opencode.server`, qualified ceiling `1.18.20`, and existing behavior
`opencode.http-sse.surface-19`. Research 176 remains the identity authority.
This record does not amend ACP, web fetch, or another OpenCode transport.

The adapter, fixtures, and guide were inspected and not changed. No production
claim, public API, shared contract, or Contract 029 window movement follows.

## Frozen Sources

| Source | Use | Retrieved | Digest / identity |
| --- | --- | --- | --- |
| GitHub tag `v1.18.20` | binding release identity | 2026-08-26 | tag object commit `7248bc1964b13fa67e601733f89ee9dc6dfa0563` (matches Research 176); GitHub release published 2026-08-21T08:09:31Z |
| GitHub archive `v1.18.20.tar.gz` | exact tree | 2026-08-26 | SHA-256 `10129b7a233d8ea227fe8a65c158d3df4adc3d1296e3af5a136d94080b25a630` |
| npm `opencode-ai@1.18.20` | package identity reconfirm | 2026-08-26 | integrity `sha512-8c2yJ/Oe1qFi9KYE0KS9WCyy6O1QtI9odzBmBWGOeyOgXTn/hGOwCp/fgcHY2qVQ2TVgkQXze7jXjJ6AFyeU0Q==`; shasum `79ace165fba034da1599fb3411691611228c409a` (matches Research 176) |
| `packages/opencode/package.json` | package version `1.18.20` | 2026-08-26 | SHA-256 `fbb20da20dd2ab17147ae690eae6b6f1fbfabb1867a83d3d9ecc9649ec07448b` |
| `packages/opencode/src/tool/websearch.ts` | tool id, permission ask, backend selection, MCP dispatch | 2026-08-26 | SHA-256 `edb175726b7830d242f59417ce6961f44d39e500bccc068bf2dddbf61d5ca92a` |
| `packages/opencode/src/tool/mcp-websearch.ts` | Exa/Parallel URLs, optional keys, request/parse | 2026-08-26 | SHA-256 `44cec72f6a8995ff172bc774e4a2e689fd1aa77fb9cfb9575d66a0e4bab24a88` |
| `packages/opencode/src/tool/websearch.txt` | tool description template | 2026-08-26 | SHA-256 `f31c862691e3bb9a90e81766f376b24d69ae7f408eaa9f3bb5507dc75a00e692` |
| `packages/opencode/src/tool/registry.ts` | `webSearchEnabled` visibility gate | 2026-08-26 | SHA-256 `a8b24a6d58a80c42307e251905dbaa4f25ca0724569b1e531e412da934ab00fe` |
| `packages/opencode/src/effect/runtime-flags.ts` | Exa/Parallel env flags | 2026-08-26 | SHA-256 `5b580cb96f9d5300f8ff995a29ec5e602e0e3e0200661dfff971998562f2a1a9` |
| `packages/opencode/src/permission/index.ts` | last-match evaluate, ask/reply, tool hiding | 2026-08-26 | SHA-256 `5b9e4aa65290a39363722b9fae4c68080188d8ed76896afa0d96cd9dbfd2821d` |
| `packages/schema/src/v1/permission.ts` | rule/request/reply vocabulary | 2026-08-26 | SHA-256 `eaa6c288030c691429e9fd63db59b0609e55337c63d6fa932b30b3c1c6956d89` |
| `packages/core/src/util/wildcard.ts` | `*` / `?` match used by evaluate | 2026-08-26 | SHA-256 `58803dad815e7086ad3e71746bb39fd63d48ab3041644cbd5f9a546c5617157c` |
| `packages/opencode/src/session/session.ts` | session-create stores caller ruleset | 2026-08-26 | SHA-256 `0c56ae3535e29cae0de51156eaba2842c0896309f1d6b12525566b4a8ba4c7f2` |
| `packages/opencode/src/session/tools.ts` | merge agent+session rules at ask time | 2026-08-26 | SHA-256 `ec76a577f6f68d44c6202dcd3fae1d3b859c8a3bda603939364048ce033be095` |
| `packages/opencode/src/session/llm/request.ts` | `Permission.disabled` hides denied `*` tools | 2026-08-26 | SHA-256 `7e30f06a8a6536fac3f554e62f846a0bd24f4ce6a8930a110df6d7010350043c` |
| `packages/opencode/src/agent/agent.ts` | default agent `*` allow, explore `websearch` allow | 2026-08-26 | SHA-256 `e781c571d584fa996200e320f14c2735e32968256b8db194391e7c06aed7433e` |
| `packages/opencode/test/tool/websearch.test.ts` | visibility and backend-selection unit truth | 2026-08-26 | SHA-256 `9cbe50bfdf6023f496304f5db46eaad8f5053d970020a8bf1bd210112ba41390` |
| `packages/core/src/tool/websearch.ts` | v2 local-tool comment; same env/backends | 2026-08-26 | SHA-256 `ba8cd7ec891f8f6bd51dce609dbdce5616a1981d8b9975250e0b8b582a134768` |
| tagged `permissions.mdx` | last-match docs; `websearch` matches query | 2026-08-26 | SHA-256 `e929098f6f772186eeb4f0eb55e332c477d88e30cb1efc45d0839dfa333b5163` |
| tagged `tools.mdx` | OpenCode/OpenCode Go or `OPENCODE_ENABLE_EXA`; Exa no-key claim | 2026-08-26 | SHA-256 `26a553033979aab97271c33b0efb8c187faba6a57b8af0b69080a46e0e696d79` |
| live [permissions](https://opencode.ai/docs/permissions/) HTML | corroboration only | 2026-08-26 | SHA-256 `f3c787c39dfcb47beebe33a863d9c04fdbd0346daf39099fae04d8c583a59aef` (88666-byte SPA) |
| live [tools](https://opencode.ai/docs/tools/) HTML | corroboration only | 2026-08-26 | SHA-256 `711fe3b8ef090048548ad745e085ced62802530d380d4c2b7c8d8291769752fa` (101742-byte SPA) |

## Frozen Official Semantics

### Tool registration and visibility

Exact `v1.18.20` registers builtin tool id `websearch`. Registry
`webSearchEnabled(providerID, flags)` is true only when:

- `providerID` is `opencode` or `opencode-go`, or
- runtime flag `enableExa` is true, or
- runtime flag `enableParallel` is true.

Exact unit tests freeze that `openai` is false with both flags off and true
when either flag is on. `enableExa` is `OPENCODE_EXPERIMENTAL` or
`OPENCODE_ENABLE_EXA` or `OPENCODE_EXPERIMENTAL_EXA`. `enableParallel` is
`OPENCODE_ENABLE_PARALLEL` or `OPENCODE_EXPERIMENTAL_PARALLEL`. Those flags
are process environment on the operator-managed attached server.

Tagged tools docs mention only OpenCode / OpenCode Go / `OPENCODE_ENABLE_EXA`.
They omit Parallel enablement. Source and tests are binding; the docs gap is
not a second visibility path.

Registry presence therefore does not prove the selected attached session can
see the tool. Visibility is a function of the selected model `providerID` and
ambient server flags.

### Backend selection and dispatch

Execute always selects `exa` or `parallel`, then POSTs JSON-RPC `tools/call`
to a public MCP URL:

- Exa: `https://mcp.exa.ai/mcp`, or the same URL with `?exaApiKey=` when
  `EXA_API_KEY` is set; tool name `web_search_exa`
- Parallel: `https://search.parallel.ai/mcp`; optional
  `Authorization: Bearer $PARALLEL_API_KEY`; tool name `web_search`

Selection order:

1. `OPENCODE_WEBSEARCH_PROVIDER` if `exa` or `parallel`
2. else Parallel flag
3. else Exa flag
4. else `parseInt(checksum(sessionID), 36) % 2 === 0 ? "exa" : "parallel"`

Session id is assigned at `POST /session`. Swallowtail does not know it at
prepare time. Without mutating attached-server env, backend choice is a
post-create checksum, not a host-approved prepared fact.

There is no separate OpenCode-hosted search HTTP path in the HTTP-route tool.
`packages/core/src/tool/websearch.ts` states this local tool "invokes the
legacy Exa/Parallel product backends itself" and is distinct from
provider-hosted search.

Tagged tools docs say Exa MCP needs no API key. Source still routes half of
unflagged sessions to Parallel. Parallel success without `PARALLEL_API_KEY`,
Exa success from an arbitrary attached server, billing, and entitlement were
not proved. Proving them would require contacting a search backend.

### Permission name, patterns, ordering, and hiding

Permission key is `websearch`. The tool asks with `patterns: [query]` and
`always: ["*"]`. Tagged permissions docs: `websearch` matches the query;
rules are last-match; `*` is a wildcard.

`evaluate` uses `findLast` over flattened rulesets. Default when nothing
matches is `ask` with `pattern: "*"`. `Permission.merge` concatenates. Ask
sites merge agent rules first, session rules second, so a matching session
rule wins.

`Permission.disabled` hides a tool when the last permission-name match has
`pattern === "*"` and `action === "deny"`. For `websearch` the permission
name is the tool id. A session `*` deny therefore hides the tool before
model visibility. A session `*` ask does not hide it. Allow last-match skips
`ask` and proceeds to MCP.

Reply vocabulary is `once | always | reject`. `once` continues without
persisting. `always` appends allow rules for `always` patterns (`["*"]` here)
into in-memory approved state. `reject` fails the ask and rejects other
pending asks in the same session. Deny from evaluate raises `DeniedError`
before the MCP POST.

### Prompt, SSE, and failure observation

Ask runs after local metadata (`provider` label) and before `callProvider`.
Deny/reject therefore stop before external search HTTP. Allow dispatches
without a permission event.

SSE has `permission.asked` / `permission.replied`. Tool results arrive as
generic `message.part.updated` tool parts (`pending|running|completed|error`).
There is no search-specific SSE type. Parser tests cover JSON-RPC and SSE MCP
bodies; they do not prove backend acceptance. Abort uses the tool abort
signal; timeout dies at 25 seconds. Those shapes are source-level only.

## Production Route Audit

Current Swallowtail session-create JSON is:

```json
[
  {"permission": "*", "pattern": "*", "action": "deny"|"ask"},
  {"permission": "read", "pattern": "*", "action": "allow"},
  {"permission": "glob", "pattern": "*", "action": "allow"},
  {"permission": "grep", "pattern": "*", "action": "allow"}
]
```

`deny` is the callback-free fallback; `ask` is callback-enabled. read/glob/
grep do not match permission `websearch`, so last match for search is the
wildcard row. Callback-free sessions hide `websearch`. Callback-enabled
sessions leave it visible only when `webSearchEnabled` is true, then ask.

Structured runs reject any `external_network` other than `Denied` and any
`external_search` other than `Disabled`. Shared `OperationPolicy` also
rejects `Enabled` search with `Denied` network. Interactive session access
uses `ambient_harness` / `ambient_harness_with_consumer_mediated_requests`:
`AmbientHost` network and `Disabled` search. Open-session validation requires
that exact access policy, so an `Enabled` search session would fail as
non-ambient today.

Permission callbacks are one-shot `once|reject` under `opencode/permission`.
Source `always` is not in the production reply corpus. Wildcard `ask` is not
search authority.

Prepared evidence records server health/version, selected catalogue
provider/model, access, and plan. It does not observe attached-server env,
`OPENCODE_ENABLE_EXA` / `OPENCODE_ENABLE_PARALLEL` /
`OPENCODE_WEBSEARCH_PROVIDER`, `EXA_API_KEY` / `PARALLEL_API_KEY`, or
outbound MCP reachability. Catalogue `provider_id` is host-selected and is
enough to evaluate the source visibility predicate, not backend success.

SSE maps a `websearch` tool part to generic `ToolState`. That is not
`ExternalSearchProgress`, backend acceptance, or effective model use.

Disabled omission is the current four-rule JSON plus Denied/Disabled
structured policy and ambient Disabled search on sessions. No search row is
present.

## Compatible Policy Pair, If A Row Existed

The only shared pair that can request search is
`ExternalNetworkPolicy::HostApproved` plus `ExternalSearchPolicy::Enabled`.
Denied+Enabled is unrepresentable. AmbientHost+Enabled would still be ambient
network, not host-approved search authority, and would change the current
interactive access constructor.

An exact later binding would also need a last-match `websearch` session rule
after the deny-first wildcard, a provider/flag availability fact bound from
prepared evidence, and fail-closed rejection when that fact is missing. This
lane cannot supply the availability fact.

## Claim Strength

| Claim | Strength at exact evidence boundary |
| --- | --- |
| tool id `websearch` is registered at `v1.18.20` | observed in tagged source and tests |
| visibility for `opencode` / `opencode-go` without flags | observed in source and unit tests |
| visibility for other providers without flags | false in source and unit tests |
| visibility via Exa/Parallel runtime flags | observed; flags are attached-server env |
| last-match session permission, deny-hides, ask-then-MCP, allow-skips-ask | observed in tagged permission/session source |
| current Swallowtail JSON is deny/ask-first with no `websearch` row | observed in production `session_create` |
| structured run rejects any enabled search or non-denied network | observed in `handle.rs` |
| backend is Exa or Parallel MCP, not a separate OpenCode search API | observed in tagged execute path |
| backend choice without env override | session-id checksum after create; not prepared |
| unauthenticated Exa MCP works from this attached server | unproved; live search forbidden |
| Parallel MCP works without `PARALLEL_API_KEY` | unproved; live search forbidden |
| selected catalogue provider is `opencode` on a consumer server | not a route invariant; fixtures use `anthropic` |
| permission approval grants network or search authority | false; separate policies |
| tool part completed proves backend acceptance or model use | false |
| disabled omission byte-equivalence for a future binding | current JSON/policy frozen; no binding admitted |

## Deliver-Now Table

| Row | Exact evidence | Disposition |
| --- | --- | --- |
| `websearch` allow on any catalogue provider | visibility needs `opencode`/`opencode-go` or ambient flags | withheld |
| `websearch` allow on `opencode` / `opencode-go` | tool may be visible; backend still checksum/env; MCP success unproved | withheld |
| `websearch` ask plus one-shot callback | wildcard ask is not search authority; dispatch still needs backend facts | withheld |
| `ExternalSearchPolicy::Enabled` + `HostApproved` | representable shared pair; cannot bind availability without ambient inference | withheld |
| structured Denied+Disabled / session ambient Disabled | current production | unchanged |
| web fetch, generic tools/permissions, task subagents | out of scope | rejected |

Deliver-now rows: **none**.

No new private behavior, guide capability claim, matrix row, or production
binding follows. Cards 188 and 189 remain blocked.

## Decision

Card 187 is complete as an evidence stop. Native `websearch` is a registered
local tool whose model visibility and Exa/Parallel MCP backend depend on
attached-server provider selection, process environment, and a session id
assigned after create. Swallowtail can send an exact permission rule and can
name the HostApproved+Enabled policy pair. It cannot bind those availability
facts through existing preparation without inspecting or mutating the attached
server, injecting credentials, or running hosted search.

Cards 188 and 189 are blocked and were not executed. Keep the current
deny-first session JSON, Disabled search claims, and Contract 029 ceiling.
A future lane needs a host-approved availability predicate that does not rest
on ambient env or live MCP success.
