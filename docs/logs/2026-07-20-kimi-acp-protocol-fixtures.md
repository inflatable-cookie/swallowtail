# 2026-07-20 Kimi ACP Protocol Fixtures

## Changed

- added a versioned Kimi Code `0.28.1` ACP fixture corpus
- pinned adapter `0.3.4`, exact SDK `0.23.0`, wire `1`, schema `v1.19.1`,
  tagged source hashes, access, and exclusions independently
- froze new, load, and resume as different transcripts
- required load history before readiness and prohibited resume replay
- covered text prompt/update, native cancellation, bounded write replacement,
  version and capability drift, auth failure, and incomplete disconnect
- added wrong-session, early-response, path-escape, and read-only write
  rejection cases

## Current State

Card 056 is complete. The provider-neutral ACP decoder needed no Kimi branch.
Card 057 is ready for an evidence-first execution-host containment proof.
Production Kimi mapping remains gated because callback mediation does not
contain filesystem or child-process paths that Kimi executes locally.

## Validation

- 10 Kimi fixture tests pass
- all 20 `swallowtail-protocol-acp` tests pass
- focused clippy passes with warnings denied
- workspace formatting, fixture whitespace, and diff checks pass
- no Kimi binary, account, credential, login, or live endpoint ran

## Risks

- load replay is provider history evidence, not consumer transcript authority
- only the prior exact Swallowtail binding may load or resume a provider
  session
- write callbacks prove bounded host mutation only; they do not prove provider
  tool approval or process containment
- native ACP session close remains absent; owned-process shutdown remains the
  cleanup boundary

## Next

Execute card 057. Revalidate host containment mechanisms first. Stop if the
only viable mechanism needs ambient root, container, VM, or deployment
authority outside Swallowtail's execution-host grant.
