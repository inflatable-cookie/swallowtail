# Nucleus Consumer Adoption

Date: 2026-07-20
Consumer: Nucleus Agent Chat

## Outcome

- Nucleus replaced its direct Agent Chat Codex app-server transport with
  `SwallowtailCodexSessionRuntime` behind the existing consumer facade.
- model discovery, sessions, turns, event streaming, callbacks, deadlines, and
  cleanup now use Swallowtail
- Nucleus retains host/resource approval, prompts, tool meaning and execution,
  receipts, persistence, conversation identity, and UI
- exact model selection is enforced by sending
  `allowProviderModelFallback: false` at Codex thread start
- sibling path dependencies are accepted for local multi-repo development;
  version pinning waits for consumer distribution

## Evidence

- Swallowtail Codex app-server integration: 14 passed
- Nucleus Swallowtail adapter: 8 passed
- authenticated Nucleus model catalogue: passed
- full Nucleus `effigy qa`: passed

## Remaining Gate

Nucleus native Agent Chat acceptance remains: multi-turn continuity, one task
or Goal callback and receipt, safe route change, restart transcript context,
and responsive panel switching. Nucleus task execution remains a separate
direct transport and needs its own migration mapping after this gate.
