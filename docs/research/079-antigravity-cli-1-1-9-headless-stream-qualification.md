# 079 Antigravity CLI 1.1.9 Headless Stream Qualification

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

Does exact Antigravity CLI `1.1.9` expose enough documented machine behavior
for bounded structured execution without permission bypass, implicit
sandboxing, or a live provider prompt during qualification?

## Method

This pass combined the exact installed `agy --help` surface, the official CLI
`1.1.8` headless specification, the shared `1.1.8`/`1.1.9` repository commit
and changelog, and deterministic documentation-derived fixtures. The installed
artifact is the Google-signed `1.1.9` binary fixed in Research 078.

No provider prompt ran. The fixtures are synthetic records built from the
official closed event vocabulary and examples. They are not captured account
transcripts.

## Selected Invocation

One run launches the host-approved executable with explicit prompt, model, and
machine format:

```text
--print <prompt> --output-format stream-json --model <model>
```

Read-only resource authority adds `--mode plan`. Optional provider sandboxing
adds `--sandbox` only when the immutable plan and request select
`ProviderEnforced`. Explicit reasoning adds `--effort low|medium|high`.
Inline JSON Schema output adds `--json-schema <schema>`.

The driver never selects `--dangerously-skip-permissions`. It requires the
stream init record to report `permission_mode: request-review`; an
`always-proceed` observation fails closed. Ambient isolation and provider
sandboxing remain separate profiles.

Antigravity documents the prompt as a print-mode argument. Swallowtail follows
that exact interface, so the prompt can be visible to execution-host process
inspection. It is excluded from stable diagnostics and runtime event debug
output, but the provider interface does not make it secret from the host.

## Qualified Stream

The parser accepts a bounded sequence of:

- one `init` with exact model, cwd, tools, and request-review permission mode
- `step_update` records correlated by conversation id and step index
- assistant `text_delta` records
- provider-owned tool lifecycle with safe tool label only
- completion-only subagent snapshots with opaque conversation identity
- per-step and terminal usage
- exactly one terminal `result`

Tool parameters, output, errors, cwd, subagent log URI, workspace URI, prompt,
raw provider payloads, and stderr do not enter stable events or diagnostics.
Subagent records preserve bounded conversation identity and safe label only.

Usage preserves input, output, thinking, and cache-read tokens. The provider's
`total_tokens` must equal input plus output. Thinking remains a distinct
reasoning-token field; Swallowtail does not reinterpret whether the provider's
output total already includes it.

## Structured Output And Failure

Inline UTF-8 JSON Schema objects are bounded to 16 KiB and must use
`json-schema-2020-12` plus provider-native enforcement. A successful terminal
record must carry matching `structured_output`, `json_schema`, and serialized
response evidence.

An exact invalid-model error becomes
`swallowtail.antigravity.headless.invalid_model` without exposing the returned
catalogue or raw error. Other provider errors, malformed streams, missing or
duplicate terminal results, process failures, cancellation, deadline, event
delivery, and cleanup remain distinct outcomes.

## Contract Result

No shared contract change is required. Existing structured-run, exact model,
reasoning, structured output, resource access, optional isolation, activity,
usage, cancellation, deadline, diagnostics, and joined-cleanup contracts fit
the route.

This route does not claim interactive callbacks. Permission-required tools are
soft-denied by provider policy unless users configure scoped provider rules.
Consumer-mediated tool approval is not inferred from the TUI.

## Risks

- The public repository does not expose implementation source. Exact behavior
  depends on official protocol documentation plus deterministic fixtures.
- Prompt arguments are visible to the execution host.
- Provider-owned ambient permission settings can affect tool availability.
- Subagent state is completion-only in the documented stream shape used here.
- Continuation is separately qualified on card 017.

## Primary Sources

- [Antigravity headless mode](https://antigravity.google/docs/cli/headless)
- [Antigravity CLI reference](https://antigravity.google/docs/cli/reference)
- [Antigravity CLI repository changelog](https://github.com/google-antigravity/antigravity-cli/blob/1.1.9/CHANGELOG.md)
- exact installed `agy` 1.1.9 help and artifact evidence from Research 078
