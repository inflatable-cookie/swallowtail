# ACP v1 activity schema corpus

This secret-free corpus freezes the stable ACP v1 activity-bearing session
updates at schema artifact `schema-v1.20.0`.

Exact authority:

- repository: `https://github.com/agentclientprotocol/agent-client-protocol`
- SDK release: `v1.6.0`
- source commit: `5e89c71497fe07dd4ae633c181a17224f4a8956d`
- schema artifact: `schema-v1.20.0`
- stable schema SHA-256:
  `92c1dfcda10dd47e99127500a3763da2b471f9ac61e12b9bf0430c32cf953796`
- schema metadata SHA-256:
  `e0bf36f8123b2544b499174197fdc371ec49a1b4572a35114513d56492741599`
- Rust core SDK: `agent-client-protocol = 2.0.0`
- Rust remote transport SDK: `agent-client-protocol-http = 2.0.0`

`updates.jsonl` covers every stable `SessionUpdate` variant relevant to the
selected lane plus permission and prompt completion. `malformed.jsonl`
freezes fail-closed outcomes for unsafe shapes. `stdio.ndjson` and
`remote.jsonl` carry the same semantic update through separate physical
transport identities.

The fixtures contain no provider identity, endpoint, credential, real session
id, raw provider response, or account observation.
