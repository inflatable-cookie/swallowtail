# 2026-08-01 Provider Session Discovery And Import Roadmap Compilation

## Result

Swallowtail now has an authority-backed runway for discovering harness-origin
sessions, explicitly importing one, loading its history, and continuing it.
No runtime or provider code changed.

## Evidence And Decision

Research 092 found complete or near-complete first-tranche surfaces in Codex
app-server, stable ACP through Kimi Code, and OpenCode attached HTTP. Existing
Swallowtail load, ordered replay, resume binding, host, access, resource, and
version contracts already cover the continuation half.

Contract 046 adds two separate roles:

- a provider-session catalogue returns bounded, non-authoritative candidates
- explicit import revalidates the exact attachment dimensions before issuing
  the existing `SessionResumeBinding`

The consumer owns durable thread records, provider-binding mappings, imported
message persistence, duplicate detection, merge policy, presentation, and
routing. The first tranche is browse, select, import, load, and resume. It is
not automatic polling or bidirectional synchronization.

## Compiled Runway

- g03.019 and cards 049-051: shared records, runtime roles, and conformance
- g03.020 and cards 052-054: Codex range evidence and production proof
- g03.021 and cards 055-057: stable ACP codec and Kimi import proof
- g03.022 and cards 058-060: OpenCode attached-HTTP proof
- g03.023 and cards 061-063: provider-wide truth, package proof, and Nucleus
  handoff

Kimi local server, Claude, Cursor, Pi, and other harnesses remain separately
classified. No route inherits support from another transport or protocol.

## Boundaries

- no Swallowtail thread database, UI, repository scan, or sync daemon
- no raw provider id as attachment authority
- no implicit provider, route, host, access, version, resource, or policy
  fallback
- no management-binding persistence promotion
- no Nucleus or Soundcheck edit
- no registry publication

## Validation

- `effigy qa:docs` passed
- `effigy qa:northstar` passed
- `git diff --check` passed
- no Rust tests ran because this batch changes planning and authority docs only

## Next

Execute card 049. Add and validate the provider-neutral catalogue and import
records before implementing a provider mapping.
