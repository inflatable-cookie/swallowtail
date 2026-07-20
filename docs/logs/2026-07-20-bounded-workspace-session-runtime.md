# 2026-07-20 Bounded Workspace Session Runtime

## Decision

Swallowtail now supports one explicit bounded writable interactive-session
profile alongside the unchanged read-only default.

The portable policy keeps resource access, filesystem representation,
approval, provider network, external search, and provider-request handling as
independent typed dimensions. Preflight binds their exact capability, service,
extension, resource, and execution-host requirements before provider effects.

## Realized Mapping

- the host resolves one opaque working resource as a read/write filesystem
  lease
- Codex receives that path as the sole thread and turn writable root
- provider network and external search remain disabled
- ambient temporary roots remain excluded
- approval posture remains `never`
- approval and user-input requests are rejected, correlated, interrupted, and
  surfaced as observe-and-stop outcomes
- the session releases the resource lease after joined provider cleanup

Read-only requests preserve their prior Codex JSON shape. No raw path,
secondary root, Nucleus identity, or consumer workflow type enters the shared
request API.

## Evidence

- provider-neutral preflight and lease conformance covers local and
  remote-authoritative host identities
- Codex fixtures assert exact thread and turn payloads
- approval and user-input fixtures assert correlation and deterministic stop
- existing Agent Chat and lifecycle suites remain green
- full repository QA passes with 119 tests
- `docs/roadmaps/g01/nucleus-task-execution-handoff.md` records the downstream
  consumer mapping

## Result

Roadmap 010 and cards 031-034 are complete. Swallowtail should now change only
in response to concrete Nucleus adoption evidence or a separately selected
provider-expansion roadmap.
