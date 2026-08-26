# 225 llama.cpp Owned Reasoning-Controls Evidence

Status: promoted
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Roadmap: [g04.078 llama.cpp Owned Reasoning Controls](../roadmaps/g04/078-llama-cpp-owned-reasoning-controls.md)
Card: [216 llama.cpp Owned Reasoning Controls Evidence](../roadmaps/g04/batch-cards/216-llama-cpp-owned-reasoning-controls-evidence.md)

## Question

Can exact `llama-cpp.owned` point `b10069-178a6c449` bind any closed
adapter-local `--reasoning on|off|auto` or `--reasoning-budget -1|0|N` row
whose model/template applicability is known before process work, without live
inference, ambient drift, silent fallback, or a portable reasoning claim?

## Decision

Partly. One reasoning row qualifies; the reasoning budget does not.

Research 225 admits one exact deliver-now selection: `--reasoning off` on exact
`llama-server` `b10069`, as dispatch-only owned-serving configuration. Caller
omission keeps the current eleven-argument launch with no reasoning argument.

`--reasoning off` is the only value whose effect on stored server state is
template-independent. Tagged source short-circuits `enable_thinking` to `false`
before the chat template is consulted, and injects the render variable
`enable_thinking = false`. Nothing about the operator-supplied GGUF has to be
known before process work for that dispatch to be exact.

Every other candidate is rejected:

- `--reasoning auto` and `--reasoning-budget -1` are byte-equivalent to
  omission in stored parameters. They are parser acceptance, not behavior.
- `--reasoning on` differs from the default only inside a per-request template
  render that this route cannot observe, and it forces `enable_thinking = true`
  even for a template the server itself probed as unsupported.
- `--reasoning-budget 0` and `N > 0` are silently discarded whenever the
  applied template yields an empty thinking end tag. That tag is a per-request
  value, invisible before launch, at readiness, and on `/props`.

Accepted, applied-to-inference, effective, and observed reasoning state stay
withheld. No prompt-free channel on this route reports reasoning selection,
budget, or template thinking support at default verbosity.

No Contract 029, driver-id, behavior, or configured-instance revision moves.
The selection stays adapter-local on the owned serving profile. It is not a
portable reasoning capability, Contract 040 control, composer field, or
`llama-cpp.attached` request option.

## Frozen Official Evidence

Fetched without credentials on 2026-08-26. Complete-body digests identify the
retrieved specimens and are not a compatibility guarantee.

| Surface | URL | Date | Identity | Complete-body SHA-256 |
| --- | --- | --- | --- | --- |
| exact `b10069` README | <https://raw.githubusercontent.com/ggml-org/llama.cpp/b10069/tools/server/README.md> | `Wed, 26 Aug 2026 21:46:02 GMT` | tag `b10069` / commit `178a6c44937154dc4c4eff0d166f4a044c4fceba` | `bdf35ef84d5d3f61effa071d56b35ded3413de1319f719f96691ea87c5aa5054` |

That digest is byte-identical to the exact `b10069` README frozen by Research
203 on 2026-08-24. The specimen did not move between the two lanes.

The README documents five reasoning options. Only the first two are in lane:

| Documented option | README text |
| --- | --- |
| `-rea, --reasoning [on\|off\|auto]` | "Use reasoning/thinking in the chat ('on', 'off', or 'auto', default: 'auto' (detect from template))"; env `LLAMA_ARG_REASONING` |
| `--reasoning-budget N` | "token budget for thinking: -1 for unrestricted, 0 for immediate end, N>0 for token budget (default: -1)"; env `LLAMA_ARG_THINK_BUDGET` |
| `--reasoning-format FORMAT` | `none` / `deepseek` / `deepseek-legacy`; env `LLAMA_ARG_THINK` |
| `--reasoning-budget-message MESSAGE` | injected before the end-of-thinking tag when budget is exhausted; env `LLAMA_ARG_THINK_BUDGET_MESSAGE` |
| `--reasoning-preserve`, `--no-reasoning-preserve` | preserve reasoning trace across history; needs `supports_preserve_reasoning` |

The README is generated from the option help strings and understates the
parser in two decisive ways recorded below: the accepted value set is wider
than `on|off|auto`, and "0 for immediate end" holds only when the applied
template exposes a thinking end tag.

## Frozen Exact Tag Source

Exact official tag `b10069` at commit `178a6c44937154dc4c4eff0d166f4a044c4fceba`.
Fetched 2026-08-26 without clone, install, build, server launch, or model load.

| Path | SHA-256 |
| --- | --- |
| `common/arg.cpp` | `006dc24c757326cdcae64b6a6b5ce7bad8712f4a79ce71a648057584b0bf40ad` |
| `common/common.h` | `b9961a58055d158329e3d25c087c9fd34e4dad357769628aad45212e6ffe3624` |
| `common/chat.h` | `f90ed844711bf62be95178db26e96c4b6ad9e8d49fc54c154b7b7a974388adc2` |
| `common/chat.cpp` | `6b04e0551d3e28784e00a29a5945ea7bd2e74aa7e753cb5c27874e22ba81a92d` |
| `common/jinja/caps.h` | `30f25aee0be5e9deed745c522eca969998564a27926f59238bdfe2a5a160cfd0` |
| `common/jinja/caps.cpp` | `2b2db183108ff4fe5e58a31f6dc3e7da4ed0162d4f51e47761b0da84cb663d35` |
| `common/log.h` | `cd065a0799ffc9a8adcb96f5188f77aec139e2520cb76316bf091ca289bb5bcb` |
| `tools/server/main.cpp` | `7ec684588e04538c5f9d44cda54e67780cf68b19c3c2660699df632f5c55fe4c` |
| `tools/server/server.cpp` | `a171ab7c273ce6510aedc2eada1a3b373f47d6d57d004d58a0e7745ac63f9a9d` |
| `tools/server/server-context.cpp` | `f28c3aab9b7367c488ff2b23394ae447c57f1731c6936cc9963426f54055500f` |
| `tools/server/server-common.cpp` | `2a67b239d3484fce399dfcd380088217ce33b1057d3337de7d460fdab95ef211` |
| `tools/server/server-task.cpp` | `8d1ad595547cb5627dd04c3151b0863ebcb14204fbcb86ea2ddbc758e4e4d206` |
| `tools/server/server-task.h` | `c0f89a78114d98efdc59414e8dd5fda28389d867328d08242f4f4c1351da96ef` |
| `tools/server/server-chat.cpp` | `40effcac1c531f7a1ca12950db1165d83622630956cdfabfc0bb0fc1ba1c24cc` |

`common/arg.cpp`, `common/common.h`, `tools/server/server.cpp`, and
`tools/server/server-context.cpp` digests match Research 203 exactly.

### Parser

`common/arg.cpp` registers both flags for `LLAMA_EXAMPLE_SERVER`:

```cpp
{"-rea", "--reasoning"}, "[on|off|auto]",
[](common_params & params, const std::string & value) {
    if (is_truthy(value)) {
        params.enable_reasoning = 1;
        params.default_template_kwargs["enable_thinking"] = "true";
    } else if (is_falsey(value)) {
        params.enable_reasoning = 0;
        params.default_template_kwargs["enable_thinking"] = "false";
    } else if (is_autoy(value)) {
        params.enable_reasoning = -1;
    } else {
        throw std::invalid_argument(...);
    }
}

{"--reasoning-budget"}, "N",
[](common_params & params, int value) {
    if (value < -1) { throw std::invalid_argument("invalid value"); }
    params.sampling.reasoning_budget_tokens = value;
}
```

The accepted value set is wider than the README's `on|off|auto`:

| Predicate | Accepted literals | Stored `enable_reasoning` |
| --- | --- | --- |
| `is_truthy` | `on`, `enabled`, `true`, `1` | `1` |
| `is_falsey` | `off`, `disabled`, `false`, `0` | `0` |
| `is_autoy` | `auto`, `-1` | `-1` |

Comparison is exact string equality, so the set is case-sensitive; `Off` is
not accepted. `--reasoning-budget` uses the `handler_int` path and therefore
`std::stoi`, which takes a leading integer (`8.5` → `8`) and throws on
non-numeric or overflowing input.

Defaults in `common/common.h`:

- `int enable_reasoning = -1; // -1 = auto, 0 = disable, 1 = enable`
- `int32_t reasoning_budget_tokens = -1;` in `common_params_sampling`

`--reasoning auto`, `--reasoning -1`, and `--reasoning-budget -1` therefore
store exactly the default. `auto` is the only accepted `--reasoning` value that
writes no `default_template_kwargs` entry, so its resulting `common_params` is
byte-equivalent to omission.

### Placement, repetition, and precedence

`common_params_parse_ex` builds an exact name table. A `--`-prefixed argument
has `_` normalised to `-`; anything absent from the table throws
`error: invalid argument`. There is no `--flag=value` form, so `--reasoning=off`
is a hard parse error. The value is always the next `argv` token, and option
order is irrelevant.

Environment variables are applied before command-line arguments. A CLI
occurrence overwrites an env-derived value and prints
`warn: <ENV> environment variable is set, but will be overwritten by command
line argument <arg>` to stderr. A repeated CLI flag logs a `DEPRECATED` warning
and the last value wins.

| Competitor | Effect on the prepared selection |
| --- | --- |
| `LLAMA_ARG_REASONING` / `LLAMA_ARG_THINK_BUDGET` | applied first; a dispatched CLI flag overrides them with a stderr warning. With omission they silently set the value instead. |
| repeated CLI flag | last occurrence wins; Swallowtail dispatches at most one occurrence. |
| request body `chat_template_kwargs.enable_thinking` | overrides the launch selection for that request. |
| request body `reasoning_budget_tokens` / `thinking_budget_tokens` | any value other than `-1` replaces the launch budget for that request. |
| model metadata / chat template | supplies thinking support and thinking tags; cannot be read before process work. |

Invalid input on either flag throws `std::invalid_argument`;
`common_params_parse` prints it to stderr and returns false, and
`tools/server/server.cpp` returns `1`. The process exits before any socket
listens, which Swallowtail already surfaces as
`swallowtail.llama_cpp.serving_process_exited`.

### Selection application

`tools/server/server-context.cpp` resolves thinking exactly once at model load:

```cpp
const bool template_supports_thinking =
    params_base.use_jinja && common_chat_templates_support_enable_thinking(chat_templates.get());
enable_thinking = params_base.enable_reasoning != 0 && template_supports_thinking;
```

`use_jinja` defaults to `true` in `b10069`, so the term reduces to the template
probe. `common_chat_templates_support_enable_thinking` renders a throwaway
one-message prompt and returns `common_chat_params::supports_thinking`.

The comparison is `!= 0`. `on` (`1`) and `auto` (`-1`) therefore produce an
identical `enable_thinking` here; only `off` (`0`) short-circuits, and it does
so without consulting the template at all.

At request time `tools/server/server-common.cpp` seeds
`inputs.enable_thinking = opt.enable_thinking`, merges command-line
`default_template_kwargs` with the request's `chat_template_kwargs`, then
applies the merged `enable_thinking` kwarg unconditionally:

```cpp
auto enable_thinking_kwarg = json_value(inputs.chat_template_kwargs, "enable_thinking", std::string(""));
if (enable_thinking_kwarg == "true")  { inputs.enable_thinking = true; }
else if (enable_thinking_kwarg == "false") { inputs.enable_thinking = false; }
```

`common/chat.cpp` then injects `enable_thinking` as a top-level jinja render
variable and copies `default_template_kwargs` into `extra_context`. Unused
extra context is inert, so a template that never references `enable_thinking`
cannot fail because the variable was supplied.

This is where `on` and `off` diverge sharply:

- `off` sets both the derived bool and the kwarg to `false`. The result is
  `false` for every template.
- `on` sets the derived bool to `template_supports_thinking` and then forces
  the kwarg to `true`. For a template the server probed as unsupported, `on`
  re-enables the render variable the server had just resolved to `false`.

### Budget application

`chat_params.reasoning_budget` is seeded from
`params_base.sampling.reasoning_budget_tokens`, but it reaches sampling only
through this request-time gate in `tools/server/server-common.cpp`:

```cpp
int reasoning_budget = json_value(body, "reasoning_budget_tokens",
                       json_value(body, "thinking_budget_tokens", -1));
if (reasoning_budget == -1) {
    reasoning_budget = opt.reasoning_budget;
}
if (!chat_params.thinking_end_tag.empty()) {
    llama_params["reasoning_budget_tokens"]    = reasoning_budget;
    llama_params["reasoning_budget_start_tag"] = chat_params.thinking_start_tag;
    llama_params["reasoning_budget_end_tag"]   = chat_params.thinking_end_tag;
    llama_params["reasoning_budget_message"]   = json_value(body, "reasoning_budget_message", opt.reasoning_budget_message);
    llama_params["reasoning_control"]          = json_value(body, "reasoning_control", false);
}
```

Two facts kill the budget as a deliver-now row:

1. **Silent discard.** When `chat_params.thinking_end_tag` is empty the whole
   block is skipped. No warning, no error, no log line, no response field. A
   launch-time `--reasoning-budget 0` is then indistinguishable from omission.
2. **Invisible applicability.** `thinking_end_tag` is produced per request by
   `common_chat_templates_apply`, either from a specialized format handler
   (`common/chat.cpp` sets `<think>`/`</think>`, `[THINK]`/`[/THINK]`,
   `<|channel>thought`, and similar) or from the differential autoparser
   (`auto_params.supports_thinking = autoparser.reasoning.mode != reasoning_mode::NONE`).
   It is not a startup property and is not reported anywhere.

A request that explicitly sends `reasoning_budget_tokens: -1` is also
indistinguishable from omission, so a consumer cannot ask for "unrestricted"
against a launch value of `0`.

### Observation

Owned readiness has three prompt-free channels. None reports reasoning state.

`GET /props` (`tools/server/server-context.cpp`) emits `chat_template_caps`
from `jinja::caps::to_map()`, whose key set in `common/jinja/caps.cpp` is
exactly the eight keys Swallowtail already decodes:

`supports_string_content`, `supports_typed_content`, `supports_tools`,
`supports_tool_calls`, `supports_parallel_tool_calls`, `supports_system_role`,
`supports_preserve_reasoning`, `supports_object_arguments`.

`supports_preserve_reasoning` describes `--reasoning-preserve` history
retention. It is not `supports_thinking`, not the resolved `enable_thinking`,
and not a reasoning-capability signal for this lane.

`/props` also emits `default_generation_settings.params` from
`task_params::to_json(true)`. Neither `to_json` branch contains
`reasoning_budget_tokens` or `enable_thinking`. The `reasoning_format` and
`reasoning_in_content` keys it does emit come from a default-constructed
`chat_parser_params` on a local `task_params tparams;` whose only assigned
member is `sampling`, so they do not report the server's configured
`--reasoning-format` either.

`GET /health` and `GET /v1/models` carry no reasoning field.

Startup stderr does not help. The only line that reports the resolved value is

```cpp
SRV_TRC("%s: chat template, thinking = %d\n", __func__, enable_thinking);
```

`LOG_TRC` requires verbosity `>= LOG_LEVEL_TRACE` (`4`) and
`common_params::verbosity` defaults to `3`. Owned launch passes no verbosity
flag, so the line is never emitted. The two `preserve_reasoning` messages that
are emitted at INFO/WARN concern a different flag.

Observed reasoning state is therefore unavailable on this route by every
prompt-free means.

## Current Swallowtail Mapping

`LlamaCppOwnedServingSelection` carries artifact, model route, and an optional
`LlamaCppContextSize`. `launch_arguments` emits eleven fixed arguments plus an
optional `--ctx-size N`:

```
--model <lease> --alias <route> --host 127.0.0.1 --port 0
--offline --no-ui --no-agent
```

Driver id `swallowtail.llama-cpp.owned-b10069-openai-chat`, runtime
`b10069-178a6c449`, behavior `llama-cpp.owned-openai-chat-b10069`. Owned
serving has no inference or catalogue role. Readiness requires health,
properties build/alias, and single-model catalogue identity. Artifact
acquisition precedes process start; stop and join precede endpoint
invalidation and artifact release.

`crate::protocol::ChatTemplateCapabilities` already decodes all eight
`chat_template_caps` keys including `supports_preserve_reasoning`, and
`validate_evidence` uses alias and build only. The attached inference profile
continues to reject unqualified reasoning content. Nothing in this lane changes
that boundary.

## Preflight Applicability

Card 216 asked for the exact model/template evidence available before process
work. The audit is empty:

| Candidate preflight source | Reasoning fact available |
| --- | --- |
| `ModelArtifactBinding` | opaque artifact identity; no GGUF metadata is read |
| `LlamaCppOwnedPreparationInput` / prepared integration | caller-supplied identity only |
| immutable `PreflightPlan` / prepared evidence | Swallowtail-side authority, not model truth |
| launch argv | what was requested, never what applies |
| `/props`, `/health`, `/v1/models` | post-launch, and carry no thinking-support key |
| startup stderr at default verbosity | endpoint line only |
| local fixtures | observed protocol shapes, not operator GGUF truth |

Consequently a row may only be admitted when its correctness does not depend
on a model/template fact. `--reasoning off` is the sole candidate that meets
that bar, because `enable_reasoning != 0` short-circuits before the template
probe. `--reasoning on` and every positive budget fail it.

## Parser Domain Versus Useful Public Domain

| Input class | Upstream `b10069` | Swallowtail disposition |
| --- | --- | --- |
| caller omission | `enable_reasoning = -1`, no template kwarg, budget `-1` | **deliver-now preserve**; no reasoning argument; current argv |
| `--reasoning off` | `enable_reasoning = 0`; kwarg `enable_thinking=false`; `enable_thinking` false for every template | **deliver-now dispatch** `--reasoning off` |
| `--reasoning on` | `enable_reasoning = 1`; kwarg forces `enable_thinking=true` at request time even when the startup probe said unsupported | **withhold**; distinction is unobservable and not preflight-bindable |
| `--reasoning auto` | stores the default and writes no kwarg | **withhold**; byte-equivalent to omission |
| `disabled`, `false`, `0` | accepted `is_falsey` aliases | **not exposed**; canonical argv emits `off` only |
| `enabled`, `true`, `1`, `-1` | accepted `is_truthy`/`is_autoy` aliases | **not exposed** |
| unknown or wrong-case value | `std::invalid_argument`; exit `1` before listen | **unconstructible**; the type admits one value |
| `--reasoning-budget -1` | stores the default | **withhold**; byte-equivalent to omission |
| `--reasoning-budget 0` | stored, then discarded when `thinking_end_tag` is empty | **withhold**; silently inert, applicability invisible |
| `--reasoning-budget N > 0` | same gate; also replaced by any non-`-1` request field | **withhold**; same reason |
| `--reasoning-budget < -1` | `std::invalid_argument`; exit `1` | **withhold**; never dispatched |
| `--reasoning-budget` non-integer / overflow | `std::stoi` truncation or throw | **withhold**; never dispatched |
| `--reasoning-format`, `--reasoning-budget-message`, `--reasoning-preserve` | separate flags | **withhold**; out of lane |
| `LLAMA_ARG_REASONING`, `LLAMA_ARG_THINK_BUDGET` | env equivalents applied before CLI | unmapped; owned argv is the only dispatch path |

## Application States

| State | Truth on this route |
| --- | --- |
| requested | `--reasoning off` or omitted |
| prepared | immutable on the serving selection, prepared evidence, and configured driver |
| dispatched | `--reasoning off` or no reasoning argument; exact agreement with input and evidence |
| parser-accepted | proven by source for the emitted literal; a bad value would exit `1` before listening, and readiness already covers that |
| applied | `enable_reasoning = 0` and `enable_thinking = false` in server state, template-independent |
| effective | withheld; whether a template or model honours `enable_thinking` is model-specific, and any request may override the kwarg |
| observed | withheld; no prompt-free channel reports selection, budget, or thinking support at default verbosity |

Successful startup, health `ok`, template capabilities, or catalogue presence
does not confirm reasoning behavior.

## Immutable Start Representation

Keep the value adapter-local, following Research 203:

- closed `LlamaCppReasoningSelection` on `LlamaCppOwnedServingSelection`
- the same optional value on prepared integration and immutable start evidence
- configured `LlamaCppOwnedDriver` launch arguments
- omit from provider-neutral `StartServingRequest`

`new(artifact, model)` remains omission. A builder adds the one admitted value.
No raw string or integer enters the public surface, and no rejected value can
be constructed at all.

## Revision Posture

No movement:

- Contract 029 membership stays exact opaque `b10069-178a6c449`
- driver id and behavior id stay; argv gains an optional pair, and the exact
  behavior of every existing row is unchanged
- configured-instance revision stays; this is prepare-input, not a stored
  config field
- attached `b9910` route stays untouched, including reasoning-content rejection

## Failure And Lifecycle

The admitted value is total, so there is no new pre-dispatch reject class
beyond "not constructible". Launch, early exit, readiness timeout, build or
route mismatch, cancellation, stop, endpoint invalidation, and artifact release
keep current joined ordering whether the reasoning flag is absent or present.
Context-size composition is independent: both flags may be selected, and each
appears exactly once. Default QA still launches no server and loads no model.

## Deliver-Now Table

| Candidate | Route profile | Disposition |
| --- | --- | --- |
| omission | `llama-cpp.owned` `b10069-178a6c449` | deliver-now preserve; no reasoning argument |
| `--reasoning off` | same | deliver-now dispatch-only |
| `--reasoning on` | same | withhold; unobservable distinction, not preflight-bindable |
| `--reasoning auto` / `-1` | same | withhold; equals omission |
| non-canonical accepted aliases | same | withhold; canonical argv only |
| `--reasoning-budget -1` | same | withhold; equals omission |
| `--reasoning-budget 0` / `N > 0` | same | withhold; silently discarded without a thinking end tag |
| `--reasoning-budget < -1`, non-integer, overflow | same | withhold; upstream exits `1` |
| accepted / effective / observed reasoning state | same | withhold |
| `--reasoning-format`, `--reasoning-budget-message`, `--reasoning-preserve` | same | withhold; out of lane |
| portable reasoning effort or budget, Contract 040 control | n/a | withhold |
| `llama-cpp.attached` reasoning content or output | n/a | withhold |
| another llama.cpp build or flag | n/a | withhold |

## Claim Boundary

Swallowtail may claim qualified dispatch of `--reasoning off` on exact owned
`b10069`, and that the exact tagged source resolves that flag to
`enable_reasoning = 0` with `enable_thinking = false` for every chat template.
It may not claim that a model stops reasoning, that output contains no
reasoning text, that a template honours the render variable, that a consumer
request preserves the selection, that any budget applies, or anything about
quality, latency, cost, or billing. It may not claim reasoning capability,
attached-route reasoning support, or a portable control.

## Primary Sources

- [llama.cpp release `b10069`](https://github.com/ggml-org/llama.cpp/releases/tag/b10069)
- [tagged `llama-server` README](https://github.com/ggml-org/llama.cpp/blob/b10069/tools/server/README.md)
- tagged `common/arg.cpp`, `common/common.h`, `common/chat.cpp`,
  `common/chat.h`, `common/jinja/caps.cpp`, `common/log.h`,
  `tools/server/server.cpp`, `tools/server/server-context.cpp`,
  `tools/server/server-common.cpp`, `tools/server/server-task.cpp`
- [Research 203 llama.cpp Owned Context Size](./203-llama-cpp-owned-context-size-evidence.md)
- [Research 008 Owned llama.cpp Serving Lifecycle](./008-owned-llama-cpp-serving-lifecycle-evidence.md)
- `crates/swallowtail-adapter-llama-cpp/src/driver/owned.rs`
- `crates/swallowtail-adapter-llama-cpp/src/prepared/owned/`
- `crates/swallowtail-adapter-llama-cpp/src/protocol.rs`

## Promotion

- adapter-local owned-serving binding: g04.078 cards 217-218
- no contract, currentness, shared-capability, or sibling-route change
