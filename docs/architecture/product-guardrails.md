# Product Guardrails

Status: active
Owner: Tom
Updated: 2026-07-19

## Non-Negotiables

- Mechanism, not product: Swallowtail does not own an application's agent loop.
- Capability-led: provider differences remain visible and inspectable.
- Adapter-specific: each useful provider surface may have its own driver and
  native extensions.
- Host-neutral: no Tauri, UI, Nucleus, or Soundcheck dependency in core crates.
- Topology-neutral: local client execution is never assumed.
- Authority-light: hosts own prompts, tools, permissions, scheduling, and state.
- Access-explicit: harness/inference and subscription/API routes never collapse
  into one hidden fallback path.
- Safe by default: public diagnostics exclude secrets and raw provider payloads.
- Small surface: prefer a few composable, powerful operations over atomic tool
  proliferation.
- Testable boundaries: failure, interruption, and lifecycle behavior must have
  fixture coverage.

## Anti-Patterns

- a generic `send_prompt` facade hiding incompatible provider lifecycles
- copying consumer task, goal, memory, or review models into shared crates
- storing credentials or application state inside Swallowtail
- treating provider wire payloads as stable public API
- assuming every provider supports tools, resume, streaming, or schemas
- treating one provider name as one adapter when it exposes several transports
- treating a direct model API as an agent harness without lifecycle evidence
- treating a harness subscription token as a general provider API credential
- silently crossing execution layer, credential, entitlement, endpoint,
  support-authority, billing, or privacy policy
- routing by model name without preserving adapter instance and transport
- one catch-all crate containing vocabulary, processes, adapters, and fixtures
- wholesale extraction of Nucleus modules before their product logic is split
