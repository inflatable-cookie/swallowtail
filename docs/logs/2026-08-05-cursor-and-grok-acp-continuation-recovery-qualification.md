# 2026-08-05 Cursor And Grok ACP Continuation Recovery Qualification

Roadmap: `../roadmaps/g03/035-acp-continuation-recovery-expansion.md`
Card: `../roadmaps/g03/batch-cards/090-cursor-and-grok-acp-load-replay-qualification.md`

## Changed

- inspected both exact qualified Cursor ACP source bundles
- inspected all four exact qualified Grok platform executables
- added route-local continuation-recovery qualification corpora
- blocked Cursor because history and per-turn replay failures are suppressed
  before a successful load response
- blocked Grok because stripped artifacts and embedded skip paths cannot prove
  complete client-visible replay
- kept both production drivers and Contract 050 mappings unchanged
- superseded cards 091-092 and moved the sole ready pointer to card 093

## Current State

Cursor Agent ACP and Grok Build ACP still advertise `loadSession`. Neither
advertisement grants Swallowtail continuation-recovery authority. Research 105
records independent promotion gates.

Claude Agent ACP and Kimi ACP remain the only continuation-recovery mappings.
Roadmap g03.035 is complete negatively. Roadmap g03.036 now owns explicit
reconciliation-then-attachment composition.

No authenticated provider work, prompt, session load, provider mutation, or
workspace mutation ran. Network access was limited to official registry and
package artifacts already named by the qualified corpora.

## Validation

- `effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-cursor swallowtail-adapter-grok`
  — 154 tests passed; focused package checks passed
- `effigy qa:docs`
- `effigy qa:routes`
- `git diff --check`

## Next Move

Execute card 093. Promote the observe-then-attach contract before composing
Codex, OpenCode, or Kimi local-server operations.
