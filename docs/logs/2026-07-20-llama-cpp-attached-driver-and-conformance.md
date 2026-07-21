# llama.cpp Attached Driver And Conformance

Date: 2026-07-20
Roadmap: 015
Cards: 049-050

## Changed

- implemented attached b9910 readiness, properties observation, single-model
  catalogue, and one-attempt streaming Chat Completions
- registered model-catalogue and structured-run roles with endpoint, task,
  blocking-work, and time services only
- added deterministic capability drift, provider failure, cancellation,
  deadline, topology, redaction, and server-preservation tests
- connected the adapter to the provider-neutral attached-self-hosted profile
- split modules before closeout so doctor gained no oversized-file finding

## Decisions

- `/props` is mandatory deployment evidence
- the driver accepts exact loopback HTTP, local unauthenticated access, build
  9910, ChatML, string content, system role, and text-only modalities
- catalogue training metadata does not become an effective input-token limit
- model alias is route/model identity; no provider id is inferred
- output bound stays consumer-owned; retry, tools, reasoning, structured
  output, attachments, and provider-side network remain absent
- close owns connections and tasks only; no process or serving-lifecycle role

## Validation

- 17 focused llama.cpp tests pass
- full `effigy qa` passes with 207 tests; Gemini and OpenCode installed probes
  remain ignored by default
- doctor remains at 19 known findings: 12 warnings and 7 errors
- `git diff --check` passes

## Risks

- no operator-supplied model was present for a live b9910 probe
- b9910 trails upstream b10069 and other versions fail closed
- server API-key auth, remote endpoints, router mode, tools, reasoning, schema
  output, and multimodality are not covered

## Continuation

Roadmap 015 is complete. Roadmap 016 card 051 is ready: revalidate and select
the next highest-information proof, led by WebSocket direct inference and a
second ACP agent.
