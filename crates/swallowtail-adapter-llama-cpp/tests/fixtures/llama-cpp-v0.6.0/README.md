# llama.cpp v0.6.0 Attached Identity

This is a static identity and source comparison for the attached route. No
server binary, model, credentials, provider prompt, live catalogue, or session
was used.

The official stable release `v0.6.0` contains `nightly-tag.txt` with `b11429`.
The `b11429` tag and peeled `v0.6.0` tag resolve to the same full source commit
`d81235049384534c167caea52b85a694f6103d14`. Source build metadata formats the
runtime identity as `b11429-<commit prefix>`, so the adapter continues to bind
its opaque runtime axis to the value exposed by `/props`; the semantic release
is the official stable point that maps to that exact source/runtime identity.

`dist-inventory.json` records the complete source tree comparison from the
qualified `b9910` source commit through `v0.6.0`. `protocol.json` records the
selected request subset, changed mapped files, and explicitly excluded APIs.
The `GET /v1/models` API-key change applies only to configured authenticated
servers, outside this route's local-unauthenticated profile; `/props` already
requires the key in both tags. No authentication capability is claimed.
