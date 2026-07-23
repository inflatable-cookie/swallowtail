# Post-Continuation Coverage And Ollama Selection

Date: 2026-07-23

## Changed

- audited all twenty production routes and twelve common profiles
- confirmed only Pi and DeepSeek expose descriptor compatibility claims; both
  remain one-point windows
- revalidated remote ACP, Grok Build, Claude Agent SDK, Cursor SDK, Z.AI, and
  attached runtimes
- corrected remote ACP from Active to Draft
- promoted Research 024 and Contract 031
- selected attach-only Ollama native API as the first maintained-range proof
- compiled roadmap 038 and cards 106-109 inside g01

## Decision

Qualify stable Ollama `0.14.0` through `0.32.1` as one text-only native segment.
Freeze `0.14.0`, `0.18.0`, `0.30.0`, and `0.32.1` qualification points.

The route observes exact runtime version, installed models, running models, and
one selected model before one native NDJSON chat attempt. Inference-caused
runtime residency is explicit. The driver gets no install, model acquisition,
mutation, unload, lifecycle, cloud, credential, container, or Monkey authority.

## Deferred

- Codex and other installed harness ranges need later qualification
- Grok Build follows only when harness breadth outweighs ACP/JSONL overlap
- Claude subscription-backed third-party access remains authority-split
- Cursor remains a public-beta foreign-language bridge
- remote ACP waits while the transport RFD is Draft
- Z.AI Coding Plan cannot authorize arbitrary Swallowtail inference
- vLLM and SGLang remain later attach-only deployment breadth

## Next

Card 106 realizes attached-runtime version, catalogue-scope, digest, residency,
and reusable closed-window assertions before Ollama protocol code.
