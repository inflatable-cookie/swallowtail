# 2026-08-25 Kimi Code 0.38.0 Headless V2 Claim

## Result

Card 180 split the headless claim. `0.29.0..=0.37.2` stays qualified under
`kimi.headless.stream-json.v1` as `Deprecated`. Exact `0.38.0` is
`Maintained` under `kimi.headless.stream-json.v2`. Public facade remains
`kimi-headless-stream-json-v1`; runtime enforces matching `system.version`
preamble on v2 without a new public facade identity. Synthetic `0.38.1` stays
permitted `UnverifiedNewer` on the v2 revision. v2 decoder corpus covers
complete, tools, retry, malformed, unknown, mismatch, incomplete, and
interrupted streams. ACP and local-server `0.38.0` qualifications are
unchanged.

g04.064 is complete, cards 179-180. Contract 029 currentness remains standing.
Next Task reassesses g04.063 blocked cards 177-178.

Focused Kimi proof, package verify, `qa:routes`, `qa:northstar`, and named
docs indexes passed.

## Next

Reassess g04.063 blocked headless reasoning-effort cards 177-178.
