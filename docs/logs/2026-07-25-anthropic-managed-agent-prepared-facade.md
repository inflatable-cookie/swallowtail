# 2026-07-25 Anthropic Managed Agent Prepared Facade

Status: complete

## Changed

`swallowtail-adapter-anthropic` now exposes a prepared integration and typed
run value for Claude Managed Agents.

Preparation binds the first-party endpoint audience, exact
`managed-agents-2026-04-01` facade, public API-key pay-as-you-go access,
provider support authority, one operator-owned agent identity and numeric
version, model route, execution host, endpoint target, services, and access
provenance.

Run preparation requires durable provider retention, managed recovery, one
authoritative-history reattachment, an explicit deadline, and zero to eight
consumer-declared custom tools. The bound `start_run` delegates to the
unchanged provider-managed structured-run driver.

## Current Evidence

Current first-party documentation still separates versioned agents,
environments, sessions, sandbox instances, and persisted events. It retains
the exact beta header, `idle`, `running`, `rescheduling`, and `terminated`
states, `user.interrupt`, custom-tool results, limited environment networking,
and session deletion rules:

- [Start a session](https://platform.claude.com/docs/en/managed-agents/sessions)
- [Session event stream](https://platform.claude.com/docs/en/managed-agents/events-and-streaming)
- [Session operations](https://platform.claude.com/docs/en/managed-agents/session-operations)
- [Cloud environment setup](https://platform.claude.com/docs/en/managed-agents/environments)
- [Managed Agents reference](https://platform.claude.com/docs/en/managed-agents/reference)

Memory-store endpoints now use a separate beta authority. That surface and
every other newly documented broad feature remain excluded. No contract or
fixture delta was required.

## Native Boundaries

- Anthropic Messages and Managed Agents use different preparation types,
  driver identities, execution layers, plans, and low-level drivers
- the operator-owned agent is retrieved and validated but never mutated or
  deleted
- the driver owns only one limited environment and one session for the run
- provider rescheduling stays visible and never becomes Swallowtail retry
- one disconnect reconciles authoritative persisted history before reattach
- custom tools are correlated callbacks executed downstream
- interruption, session deletion, environment deletion, task join, and
  credential release remain separate facts
- session deletion precedes environment deletion
- repository, provider filesystem, external sandbox network, built-in tools,
  MCP, skills, vaults, memory, schedules, webhooks, files, and containers
  remain excluded

## Validation

- Anthropic adapter lint passes with warnings denied
- 44 direct, managed, protocol, conformance, prepared-facade, and example tests
  pass
- full repository QA passes
- the public API declaration baseline passes for all 23 crates
- prepared managed runs pass under local and remote-authoritative hosts
- prepared recovery creates no second session and performs one authoritative
  history reconciliation
- prepared callbacks retain run and callback correlation
- prepared cancellation sends one interrupt, deletes both owned resources in
  order, then releases the credential
- weaker retention, recovery, reattachment, and access bindings fail before
  endpoint or credential effects
- `effigy doctor` returns the known structural-debt failure: 19 oversized-file
  findings, including the same seven errors; this batch adds no finding

## Next

Card 031 starts g02.011 with separate xAI, OpenAI, and Gemini realtime
connection facades. Cards 031-036 remain in the provider-wide facade,
package-proof, and replacement-candidate runway.
