# 203 llama.cpp Owned Context-Size Evidence

Status: promoted
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Card: g04.056 / 155

## Question

Can exact `llama-cpp.owned` runtime `b10069-178a6c449` expose a typed positive
`--ctx-size` serving selection while preserving current omission, honest
application-state claims, and the owned artifact/process/endpoint lifecycle?

## Decision

Yes, as dispatch-only owned-serving configuration.

Research 203 admits one exact deliver-now set: a typed positive integer
`1..=2147483647` encoded as `--ctx-size N` on exact `llama-server` `b10069`.
Caller omission keeps the current eleven-argument launch with no context flag.
Explicit zero is not an omission alias.

Accepted, effective, and observed context size stay withheld. Nested
`GET /props` `default_generation_settings.n_ctx` and catalogue `meta.n_ctx`
expose post-pad, post-train-cap slot context. Current Swallowtail decoding
does not read those fields, and reading them would not confirm the requested
value. Model training context, host memory, and allocation success remain
host- and artifact-specific.

No Contract 029, driver-id, behavior, or configured-instance revision moves.
The selection stays adapter-local on the owned serving profile. It is not a
portable context-window capability, Contract 040 control, composer field, or
`llama-cpp.attached` request option.

## Frozen Official Evidence

Fetched without credentials on 2026-08-24. Complete-body digests identify the
retrieved specimens and are not a compatibility guarantee.

| Surface | URL | Date | Identity | Complete-body SHA-256 |
| --- | --- | --- | --- | --- |
| current `llama-server` README | <https://raw.githubusercontent.com/ggml-org/llama.cpp/master/tools/server/README.md> | `Mon, 24 Aug 2026 17:30:07 GMT` | master `f280b26983ad0fdb705a0d9ebf0503e76f2899b0` | `8902ec2869b37fc0fc9cae682ca443c7fa84abbc625e926eb47775da1422f65f` |
| exact `b10069` README | <https://raw.githubusercontent.com/ggml-org/llama.cpp/b10069/tools/server/README.md> | `Mon, 24 Aug 2026 17:30:08 GMT` | tag `b10069` / commit `178a6c44937154dc4c4eff0d166f4a044c4fceba` | `bdf35ef84d5d3f61effa071d56b35ded3413de1319f719f96691ea87c5aa5054` |
| tag commit metadata | <https://api.github.com/repos/ggml-org/llama.cpp/commits/b10069> | `Mon, 24 Aug 2026 17:30:08 GMT` | `178a6c44937154dc4c4eff0d166f4a044c4fceba` | `78989734bed0f3f7e587b1ffc33e5035ee83f898a03f9599a9b5440d319fd1f2` |

Both READMEs document the same flag:

`-c, --ctx-size N` — size of the prompt context (default: `0`, `0` = loaded
from model); env `LLAMA_ARG_CTX_SIZE`.

`GET /props` remains read-only unless `--props` is set. The documented
response includes nested `default_generation_settings.n_ctx`. Swallowtail
owned launch does not pass `--props`, so POST mutation stays off.

Current official documentation does not change the `b10069` flag text. This
lane does not move currentness.

## Frozen Exact Tag Source

Exact official tag `b10069` at commit `178a6c44937154dc4c4eff0d166f4a044c4fceba`.
Fetched 2026-08-24 without clone, install, server launch, or model load.

| Path | SHA-256 |
| --- | --- |
| `common/arg.cpp` | `006dc24c757326cdcae64b6a6b5ce7bad8712f4a79ce71a648057584b0bf40ad` |
| `common/arg.h` | `93eeadfc68934b31c413b135b23ba2a5b124e5e1b6d99c12f446a0f9e73bc8fb` |
| `common/common.h` | `b9961a58055d158329e3d25c087c9fd34e4dad357769628aad45212e6ffe3624` |
| `common/common.cpp` | `4b041bb61251f7cf6583afeeeb449150d87cdbabb4580cb7ad45bb3420f9d141` |
| `common/fit.h` | `59a5ceeda8bc2a4126ca23a478cdcf608beccdf80d94d2c41b8061966bcf4b87` |
| `common/fit.cpp` | `6e3f31098b1ecdbca626c92e9128590914408827437b2e378d8e582e6bd06c8e` |
| `include/llama.h` | `2331631b6a3567311abc0402c55aa9a867ee99759f2550bdfa261ec3693a21f6` |
| `src/llama-context.cpp` | `7e5a6656cf1b4b24c6bc825d90fcab4c0aab677c53a868f51a61c742b8361c76` |
| `tools/server/server-context.cpp` | `f28c3aab9b7367c488ff2b23394ae447c57f1731c6936cc9963426f54055500f` |
| `tools/server/server.cpp` | `a171ab7c273ce6510aedc2eada1a3b373f47d6d57d004d58a0e7745ac63f9a9d` |

### Parser and storage

`common_params.n_ctx` is `int32_t` default `0` ("context the model was trained
with"). `--ctx-size` uses `handler_int` and `std::stoi`:

```cpp
{"-c", "--ctx-size"}, "N",
...
[](common_params & params, int value) {
    params.n_ctx = value;
    if (value == 0) {
        params.fit_params_min_ctx = UINT32_MAX;
    }
}
```

`std::stoi` accepts signed integers, including negatives and leading-integer
forms such as `4.5` → `4`. Overflow throws. That parser breadth is not the
Swallowtail public domain.

Explicit `0` stores `n_ctx = 0` and sets `fit_params_min_ctx = UINT32_MAX`,
which asks fit-params not to shrink context. Omission leaves `n_ctx = 0` and
`fit_params_min_ctx = 4096`. Explicit zero is therefore not omission.

### Fit, pad, and train-cap

`fit_params` defaults true. `common_fit_params` modifies context size if and
only if `cparams.n_ctx == 0`. A positive user value is logged as "context size
set by user" and left unchanged by fit.

`llama_context` construction then:

1. replaces `n_ctx == 0` with `hparams.n_ctx_train`
2. pads `n_ctx` with `GGML_PAD(..., 256)`
3. derives `n_ctx_seq` from unified-KV vs `n_seq_max` (`n_parallel` default 1;
   `kv_unified` default false)
4. warns, but does not reject, when `n_ctx_seq` exceeds `n_ctx_train`

Owned launch does not pass `--parallel` or `--kv-unified`. Slot setup then
caps usable slot context:

```cpp
int n_ctx_slot = llama_n_ctx_seq(ctx_tgt);
if (n_ctx_slot > n_ctx_train) {
    n_ctx_slot = n_ctx_train;
}
slot.n_ctx = n_ctx_slot;
```

`include/llama.h` states requested `llama_context_params` may differ from the
values later reported by `llama_n_ctx` / `llama_n_ctx_seq`.

Negatives stored in `int32_t n_ctx` are assigned into `uint32_t` context
params and become large unsigned values. Swallowtail must reject them before
dispatch.

Resource-infeasible values fail during model or context allocation and surface
as existing owned startup failure (process exit, readiness timeout, cleanup).
That is not a numeric public bound.

### Observation surfaces

Single-model `GET /props` (`tools/server/server-context.cpp`) emits:

- `default_generation_settings.n_ctx` = `meta->slot_n_ctx`
- `model_alias`, `build_info`, `chat_template`, `modalities`, …

`GET /v1/models` emits `meta.n_ctx` (slot) and `meta.n_ctx_train`.

Current Swallowtail `parse_properties` retains alias, build, template, and
modalities only. `parse_models` retains the catalogue id only. Nested `n_ctx`
is ignored. Decoding it would observe post-transform slot context, not the
requested or dispatched integer, and would not prove allocation of `N`.

## Current Swallowtail Mapping

`LlamaCppOwnedServingSelection` carries artifact and model route only.
`LlamaCppOwnedPreparedEvidence` carries operation evidence and artifact.
`launch_arguments` emits eleven fixed arguments and no `--ctx-size`:

```
--model <lease> --alias <route> --host 127.0.0.1 --port 0
--offline --no-ui --no-agent
```

Driver id `swallowtail.llama-cpp.owned-b10069-openai-chat`, runtime
`b10069-178a6c449`, behavior `llama-cpp.owned-openai-chat-b10069`. Owned
serving has no inference or catalogue role. Readiness still requires health,
properties build/alias, and single-model catalogue identity. Artifact
acquisition precedes process start; stop/join precedes endpoint invalidation
and artifact release.

## Parser Domain Versus Useful Public Domain

| Input class | Upstream `b10069` | Swallowtail disposition |
| --- | --- | --- |
| caller omission | `n_ctx = 0`, fit may shrink toward 4096, then model train context | **deliver-now preserve**; no `--ctx-size` member; current eleven-argument argv |
| explicit `0` | `n_ctx = 0` and `fit_params_min_ctx = UINT32_MAX` | **reject** before effects; not an omission alias |
| positive `1..=2147483647` | `std::stoi` into `int32_t n_ctx`; fit leaves it; later pad/cap | **deliver-now dispatch** `--ctx-size N` |
| negative | `std::stoi` succeeds; later unsigned conversion | **reject** before effects |
| fraction / non-integer | `std::stoi` may take a leading integer | **reject**; public API is a typed positive integer |
| `> 2147483647` | `std::stoi` overflow | **reject** |
| above model `n_ctx_train` | context may allocate; slot capped to train | dispatch allowed; accepted/effective withheld |
| resource-infeasible | allocation/startup failure | existing owned failure and joined cleanup; no numeric public bound |
| `LLAMA_ARG_CTX_SIZE` | env equivalent of the flag | unmapped; owned argv is the only dispatch path |
| `--fit` / `--fit-ctx` / `--parallel` / `--kv-unified` / rope / batch | separate flags | withheld |

The public ceiling is `i32::MAX` because tagged storage is `int32_t`. It is
not a model, host, or allocation guarantee. Values that are not multiples of
256 remain dispatchable; `GGML_PAD` is a post-start transform, not an argv
rewrite.

## Application States

| State | Truth on this route |
| --- | --- |
| requested | typed positive `N` or omitted |
| dispatched | `--ctx-size N` or no context flag; exact agreement with input, immutable evidence, and configured driver |
| accepted | withheld; fit does not rewrite positive `N`, but pad, `n_ctx_seq`, and train-cap can |
| effective | withheld; readiness does not prove allocated or usable context |
| observed | withheld; `/props` nested `n_ctx` is slot context after those transforms and is not decoded |

Successful startup, health `ok`, or catalogue presence does not confirm the
requested value.

## Immutable Start Representation

Keep the value adapter-local:

- typed `LlamaCppContextSize` on `LlamaCppOwnedServingSelection`
- the same optional value on prepared integration and immutable start evidence
- configured `LlamaCppOwnedDriver` launch arguments
- omit from provider-neutral `StartServingRequest`

`new(artifact, model)` remains omission. A builder adds one admitted positive
value. Invalid values never become a typed selection.

## Revision Posture

No movement:

- Contract 029 membership stays exact opaque `b10069-178a6c449`
- driver id and behavior id stay
- configured-instance revision stays; this is prepare-input, not a stored
  config field
- attached `b9910` route stays untouched

## Failure And Lifecycle

Knowable rejects (zero, negative, non-integer, overflow) fail before artifact
acquisition or process start. Launch, early exit, readiness timeout,
build/route mismatch, cancellation, stop, endpoint invalidation, and artifact
release keep current joined ordering whether the context flag is absent or
present. Default QA still launches no server and loads no model.

## Deliver-Now Table

| Candidate | Route profile | Disposition |
| --- | --- | --- |
| omission | `llama-cpp.owned` `b10069-178a6c449` | deliver-now preserve; no `--ctx-size` |
| positive `--ctx-size N` for `N` in `1..=2147483647` | same | deliver-now dispatch-only |
| explicit zero | same | reject |
| negative, fraction, overflow | same | reject |
| above-training / resource-infeasible | same | not a public bound; startup failure remains existing lifecycle |
| accepted / effective / observed `n_ctx` | same | withhold |
| portable context / Contract 040 / attached inference | n/a | withhold |
| another llama.cpp build or flag | n/a | withhold |

## Claim Boundary

Swallowtail may claim qualified dispatch of one exact positive integer as
`--ctx-size N` on exact owned `b10069`, or qualified preservation of the
current no-flag command. It may not claim provider acceptance, effective
allocation, model fit, usable token budget, inference capacity, output,
quality, latency, cost, or billing.

## Primary Sources

- [llama.cpp release `b10069`](https://github.com/ggml-org/llama.cpp/releases/tag/b10069)
- [current `llama-server` README](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md)
- [tagged `llama-server` README](https://github.com/ggml-org/llama.cpp/blob/b10069/tools/server/README.md)
- tagged `common/arg.cpp`, `common/common.h`, `common/fit.h`,
  `src/llama-context.cpp`, `tools/server/server-context.cpp`
- [Research 008 Owned llama.cpp Serving Lifecycle](./008-owned-llama-cpp-serving-lifecycle-evidence.md)
- `crates/swallowtail-adapter-llama-cpp/src/driver/owned.rs`
- `crates/swallowtail-adapter-llama-cpp/src/prepared/owned/`
- `crates/swallowtail-adapter-llama-cpp/src/protocol.rs`

## Promotion

- adapter-local owned-serving binding: g04.056 cards 156-157
- no contract, currentness, shared-capability, or sibling-route change
