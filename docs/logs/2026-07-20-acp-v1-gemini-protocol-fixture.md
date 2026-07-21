# ACP v1 And Gemini Protocol Fixture

Date: 2026-07-20
Roadmap: 014
Card: 045

## Changed

- revalidated ACP stable wire version 1 and schema release `v1.19.0`
- revalidated Gemini CLI stable `0.51.0` and ACP SDK `0.16.1`
- added Contract 015 for negotiation, callbacks, access, cancellation, and
  process-close boundaries
- added the fixture-first `swallowtail-protocol-acp` crate
- froze bounded bidirectional transcripts for the first Gemini subset

## Decisions

- wire, schema, SDK, and agent versions remain separate identities
- omitted capabilities are unsupported; optional methods never become baseline
- the first driver uses new text-only Plan Mode sessions
- advertised auth methods, modes, models, richer content, and MCP transports are
  evidence only
- the driver does not call Gemini authentication because it can mutate cached
  credentials and user settings
- Google consumer membership is not an admissible current access profile;
  exact paid API-key, Cloud, and enterprise profiles stay separate
- Gemini load is excluded because replay completion is not awaited before its
  response in `0.51.0`
- Gemini does not advertise stable session close; owned process cleanup is the
  close path
- filesystem reads require a new bounded host callback port; writes and
  terminals are not advertised

## Evidence

- official schema and Gemini source artifacts are pinned by version, commit,
  and SHA-256 in the fixture manifest
- eight focused fixture tests pass
- fixtures require no Gemini binary, provider credential, or ambient config

## Risks

- Plan Mode alone permits some search and can inherit policy; the production
  instance must use isolated host-approved Gemini state and deny-first policy
- Gemini's filesystem bridge can fall back to native filesystem access; process
  sandbox and policy remain mandatory even when read callbacks are enabled
- ACP v1 continues to add stable optional methods without changing wire major
  version; every optional capability stays explicitly gated

## Continuation

Card 046 is ready for the bounded ACP transport, read-only host callback port,
and Gemini process mapping. Cards 047-050 remain in bounds.
