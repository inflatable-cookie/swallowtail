# 2026-07-27 Kimi Local Server Route Planning

## Changed

- revalidated current Kimi Code `kimi web` documentation and exact `0.28.1`
  and `0.29.0` tagged source
- corrected the earlier bundled-UI-only authority assessment
- recorded the separate `kimi-code.local-server` REST/WebSocket route in
  Research 040, Contract 038, and system architecture
- qualified the planned route for reversible archive and restore only
- kept hard delete unsupported and left `kimi-code.acp` unchanged
- required explicit ACP-to-server management-binding import under exact
  release, host, state-root, configured-instance, access, and target evidence
- compiled roadmap g02.020 and cards 061-065
- paused card 059 at its existing canonical-source gate and made card 061 the
  sole next task

## Evidence

Both exact tagged trees expose:

- foreground `kimi web --no-open`
- local REST plus WebSocket protocol version `2`
- authenticated OpenAPI and AsyncAPI
- exact server metadata and liveness
- persistent bearer-token and live-instance mechanisms
- native session archive and restore
- no session hard-delete route

No Kimi installation, login, credential, provider request, server process,
workspace mutation, or session effect was used.

## Lane State

- roadmap g02.019 is paused; card 059's transient package proof remains valid
  but retained replacement waits for canonical source history
- roadmap g02.020 is active
- card 061 is ready
- cards 062-065 remain in bounds
- card 060 remains planned behind card 059

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy qa:routes`
- `git diff --check`

## Next

Card 061 completed the exact local-server compatibility and protocol corpus.
Card 062 now owns the production lifecycle driver.
