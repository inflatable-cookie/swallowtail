# 2026-07-29 Non-ACP Harness Activity Inventory

## Changed

- promoted Research 066
- classified all eight remaining production non-ACP harness routes
- clarified option-dependent partial and preview profiles in Contract 044
- added one provider-free machine-readable route inventory
- froze OpenCode, Pi, Kimi local-server, Managed Agents, Claude, Gemini,
  Kimi headless, and Qwen activity corpora
- recorded route-native lifecycle, disclosure, correlation, tool ownership,
  unknown posture, and exact absences

## Decisions

- selected executable capability does not imply selected partial or preview
  behavior
- Qwen retains partial-message lifecycle because its qualified command selects
  it
- Claude headless remains completion-only because its qualified command does
  not select partial messages
- Managed Agents remains on authoritative persisted completion records;
  best-effort previews stay excluded
- provider-owned tools, consumer custom tools, permissions, and UI relays
  remain distinct
- current stable releases above qualified bounds remain permitted unverified
  newer and cannot widen activity profiles

## Evidence

- OpenCode: 45 qualified releases, 18 selected surfaces, exact `1.14.51`
  activity gap, current `1.18.9`
- Pi: exact `0.80.10`; current `0.82.1` adds unqualified bash and
  summarization-retry records
- Kimi local server: exact `0.28.1` and `0.29.0..=0.29.2` schema segments;
  current `0.30.0` is source-stable but unverified newer
- Managed Agents: persisted event authority and provider/MCP/custom tool split
- Claude `2.1.220`, Gemini `0.51.0..=0.52.0`, Kimi
  `0.29.0..=0.29.2`, and Qwen `0.19.11` exact headless shapes

## Validation

- `cargo test -p swallowtail-testkit --test non_acp_harness_activity_corpus`
  — 4 passed
- `cargo fmt --all -- --check` — passed after formatting
- `effigy qa:docs` — passed
- `effigy check:rust` — passed

No executable, credential, account, model request, paid inference, attached
server, or consumer repository was used.

## Next

Card 129 is ready. Implement Pi RPC first, then Kimi local server, OpenCode,
and Managed Agents. Cards 130-137 remain in bounds.
