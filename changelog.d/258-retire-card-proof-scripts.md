# 258: Retire spent card proof scripts and trim architecture history

- Remove the spent one-off proof scripts and their exclusive fixtures:
  `copilot-acp-offline-proof.py`, `copilot-acp-host-permission-proof.py`,
  `copilot-acp-no-child-diagnostic.py`,
  `copilot-acp-no-child-original-diagnostic.py`,
  `kiro-acp-owner-state-offline-proof.py`,
  `stage-copilot-acp-offline-artifacts.py`, the compiled fake and launcher
  sources, the Kiro Dockerfile, and `scripts/release/backports/sdk-v0.5.2/`.
  About 17k lines leave `scripts/`.
- Remove their 33 dead card, ticket, hop, and backport selectors from
  `effigy.toml`, plus the removed `sdk-v0.5.2` script class and frozen route
  digests in the Oxide routing guard. Active QA, CI, release, docs, route,
  routing, `validate:focused`, and `package:*` selectors stay unchanged.
- Trim `docs/knowledge/architecture/system-architecture.md` from 2373 to 233
  lines: keep the crate families, dependency direction, cross-cutting role
  ownership, and durable session-role separations, and delegate routes,
  version windows, capability cells, and the package roster to the provider
  route matrix and release topology.
- Record the retirement in `docs/guides/validation-tiers.md`,
  `docs/releases/sdk-v0.5.2-backport.md`, `docs/knowledge/questions.md`
  (Q-008), `scripts/README.md`, and the two retained Copilot fixture READMEs.
  No `docs/research/` record changed; every retained record keeps its bytes,
  including the frozen plans, authority, fake-pass, original, and consumed
  guard records.
