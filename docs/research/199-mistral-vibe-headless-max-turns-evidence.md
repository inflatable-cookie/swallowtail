# 199 Mistral Vibe Headless Maximum-Turn Evidence

Status: complete; deliver-now subset admitted
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Card: g04.052 / 145-147

## Question

Can exact Mistral Vibe `2.24.2` route `mistral-vibe.headless` expose a typed
caller-decreasing positive `--max-turns` selection while preserving current
omission, exact native limit terminal truth, and every fixed route boundary?

## Decision

Yes, for exact release `2.24.2` only. Admit caller-decreasing `--max-turns`
values `1..=8` as adapter-local typed input. Caller omission keeps the current
argv byte `8`. Upstream omission of the flag stays forbidden: argparse has no
default and that state is unbounded.

`--max-turns N` is a per-child cap on completed assistant LLM turns inside one
`vibe --prompt` process. It is not a Contract 040 `OutputTokenLimit`, a generic
budget, or proof that the provider completed less work. Tool executions,
retries, compaction, and Swallowtail host turns are distinct.

No shared `Capability`, OperationPolicy field, or Contract 029 currentness
change is required. The route is already exact `2.24.2` only, so selected
values ride the existing qualified-only membership. Name a feature-local
behavior revision only as a post-merge orchestrator delta; the worker must not
edit Contract 029.

Zero, negatives, fractions, overflow, raised values above eight, and unbounded
flag omission stay withheld or invalid. Zero stops before the first assistant
LLM call and is not useful on this one-prompt print route.

## Frozen Official Evidence

Current official README and exact tag `v2.24.2` README were byte-identical when
fetched without credentials on 2026-08-23 at 21:57:46 GMT.

| Surface | URL | Date | ETag | Complete-body SHA-256 |
| --- | --- | --- | --- | --- |
| README `main` | <https://raw.githubusercontent.com/mistralai/mistral-vibe/main/README.md> | `Sun, 23 Aug 2026 21:57:46 GMT` | `"0c70f58bfeef18837c67d22b99ca1fb220c74a65dcf7027307cafeb6020b3e67"` | `6016ef2167e602c955577706452b37b04241244ea37fad3ffae8a7c44a1ad421` |
| README `v2.24.2` | <https://raw.githubusercontent.com/mistralai/mistral-vibe/v2.24.2/README.md> | `Sun, 23 Aug 2026 21:57:46 GMT` | `"0c70f58bfeef18837c67d22b99ca1fb220c74a65dcf7027307cafeb6020b3e67"` | `6016ef2167e602c955577706452b37b04241244ea37fad3ffae8a7c44a1ad421` |

Official text:

- `--max-turns N`: limit the maximum number of assistant turns; the session
  stops after N turns
- example: `vibe --prompt "..." --max-turns 5 --max-price 1.0 --max-tokens
  50000 --output json`
- `--max-price` and `--max-tokens` are sibling programmatic limits, not this
  control

Official docs do not define the counter, off-by-one boundary, argparse domain,
or exit mapping. Exact tag source owns those facts.

## Frozen Exact Release Evidence

Inspected from official GitHub tag `v2.24.2` commit
`5e6aa0f6beb3454454f4c1de74a7652ba577ab05` (Research 150 identity). No package
was installed. No live Vibe process, login, credential, catalogue, or prompt
was used.

| Path | SHA-256 |
| --- | --- |
| `vibe/cli/entrypoint.py` | `b1d17309da8f2b24c2232a18549ca4e19522be66e689f08f2271a760422136df` |
| `vibe/cli/cli.py` | `295ce3994f8bb73ae53af8a129fdcc66b889ebfd0b7a1c68d3a9192093fff1dc` |
| `vibe/cli/programmatic.py` | `36a5008914136714a851880b3c6921f6ba196371e28422f58a45c8a728b25b34` |
| `vibe/core/middleware.py` | `ede809d72ced3328986a3a7c93d4793f66506467182e45f89168fd8ae8343a7a` |
| `vibe/core/agent_loop/_loop.py` | `8ff6a0daf3d5626470340bab9b88735812f1e018fc063496e4b72bcd04bdc0d8` |
| `tests/cli/test_programmatic.py` | `42cb58a1210e398fd8e33991307d43822d8da32f4e74797017d276dd859337b4` |

Existing route fixture:
`crates/swallowtail-adapter-mistral-vibe/tests/fixtures/mistral-vibe-headless-2.24.2/`.
Entrypoint, CLI, and programmatic hashes match Research 150
`tagged_headless_sources`.

## Parser Domains Versus Swallowtail Domains

`entrypoint.py` declares `--max-turns` as `type=int` with no default and no
range. Python argparse `int()` accepts `0`, negatives, and unbounded integers.
It rejects fractions and non-integers. Flag omission stores `None`.

CLI copies `args.max_turns` into session options. `TurnLimitMiddleware` is
installed only when that value is not `None`. Protocol `SessionOptions.max_turns`
is `int | None` on the CLI path. A later settings-update type uses
`NonNegativeStrictInt`; that is not the headless `--prompt` parser.

| Input class | Upstream | Swallowtail |
| --- | --- | --- |
| caller omission | flag omitted → unbounded | retain current `--max-turns 8` |
| `1..=8` | integer accepted; N completed assistant LLM turns | deliver-now |
| `0` | immediate STOP before first LLM; test raises `ProgrammaticLimitError` | withheld; not useful for a one-prompt child |
| `9..` / raised | parser-accepted | withheld; caller-increasing |
| negative | parser-accepted; `steps - 1 >= negative` stops immediately | withheld/invalid |
| fractional / non-integer | argparse rejects | invalid; never a public constructor |
| overflow / huge int | Python `int` unbounded; never hits the cap in practice | withheld |
| upstream flag omission | unbounded | forbidden; Swallowtail always emits the flag |
| aliases | none | invalid |

`--max-price` and `--max-tokens` stay unmapped.

## Counter Definitions And Lifetime

`AgentStats.steps` starts at `0`.

Non-injected `_open_user_turn` increments `steps` once when appending the user
message. Injected middleware messages do not increment at open.

`TurnLimitMiddleware.before_turn` then checks
`context.stats.steps - 1 >= max_turns` **before** the LLM call. After the user
increment, the first check sees `0` completed LLM turns.

Each completed `_perform_llm_turn` increments `steps` again. Compaction
overflow-and-retry does **not** increment (source comment). Tool executions are
not a separate counter; a tool-result continuation is another LLM turn and
counts.

So `--max-turns N` allows N completed assistant LLM calls in that child, then
STOP before LLM N+1.

| Value | First check after user open | Observable |
| --- | --- | --- |
| `0` | `0 >= 0` | STOP before first assistant LLM |
| `1` | `0 >= 1` is false | one assistant LLM; tool follow-up LLM is blocked |
| `8` | allows eight completed LLM turns | current Swallowtail default |

Lifetime is one `vibe --prompt` child. This route has no continue/resume.
Teleport summarization also increments `steps`; that path is unselected.

Do not describe this flag as Swallowtail operation-turn accounting. Host
deadline, cancellation, and the adapter's one-child lifecycle stay separate.

## Terminal Truth

Middleware STOP yields an `AssistantEvent` with tagged reason
`Turn limit of {N} reached` and `stopped_by_middleware=True`, then returns from
the conversation loop.

After `session.act(...)`, programmatic mode inspects the last public turn. When
`stop_reason is PublicTurnStopReason.LIMIT`, it raises `ProgrammaticLimitError`
with last assistant text or
`The configured conversation limit was reached`. CLI prints that exception to
stderr and exits `1`. `session.close()` still runs in `finally`.

Research 150 and the route fixture already freeze:

- stderr containing `The configured conversation limit was reached`
- exit `1`
- Swallowtail `swallowtail.mistral-vibe.headless.max_turns` /
  `ProviderFailed`
- not `Completed`

If a prior assistant message exists, stderr may be that last text instead of
the default phrase. Swallowtail then classifies exit `1` as generic
`provider_failed`, still not success. Do not promote last-assistant-text stderr
into the stronger max-turns diagnostic. Do not claim a LIMIT stream record:
this route's streaming decoder never maps that reason into a public event.

Completed public history lines may already have been written before LIMIT is
raised. Earlier output must not flip the native limit into `Completed`. Current
finalization attaches output only on success.

Cancellation (`Cancelled`, kill, join) and host deadline (`TimedOut`) stay
distinct. The middleware does not claim a limit if the child was already
aborted.

Zero is truthful upstream and not useful here: the child stops before the first
assistant LLM. Withhold it.

## Plan And Evidence Representation

Carry an adapter-local optional `MistralVibeMaxTurns` (`1..=8`) on
`MistralVibeHeadlessRunProfileInput` and `MistralVibeHeadlessPreparedRun`. Copy
it onto `MistralVibeHeadlessDriver` for argv construction. Do not add a shared
`Capability` or portable policy field. Reject unconstructable numbers at the
typed constructor. Preserve current constructors and omission argv `--max-turns
8`. The whole route is already exact `2.24.2`; no extra version-range gate is
required beyond existing preparation.

## Deliver-Now Table

| Release | Profile | Turns | Disposition | Reason |
| --- | --- | --- | --- | --- |
| `2.24.2` | ordinary structured run | omit | retain current | argv `8` |
| `2.24.2` | ordinary structured run | `1..=8` | deliver-now | per-child decreasing assistant LLM turns |
| any other release | any | selected | reject before start | route is exact `2.24.2` only |
| any | any | `0`, raised, negative, fraction, overflow | withheld/invalid | see parser table |
| any | any | upstream flag omission | forbidden | unbounded |
| any | ACP / TUI / continue / resume / teleport | — | not applicable | not this route |
| any | `--max-price` / `--max-tokens` | — | not applicable | unmapped siblings |

## Behavior Revision And Compatibility

Keep `mistral-vibe.headless.stdio-streaming-v1` and Contract 029 exact `2.24.2`
qualified-only membership. Selected maximum turns are a feature-local adapter
control inside `swallowtail-adapter-mistral-vibe`. Prior fixture evidence for
fixed `8` remains historical proof of omission.

## Validation

Evidence-only inspection on 2026-08-23. No install, login, credential,
catalogue, or provider prompt.
