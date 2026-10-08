# Research 406: llama.cpp v0.6.0 Attached Identity

Status: promoted

Owner: `llama-cpp.attached`
Date: 2026-10-08
Official channel: GitHub latest non-prerelease release

## Question

How does the new semantic release tag map to the attached runtime's frozen
`b9910` build identity, and does the selected attached OpenAI chat subset
remain compatible?

## Method and limits

Requeried the official release API on 2026-10-08. Downloaded the official
`nightly-tag.txt` asset and GitHub source archives for tags `b9910` and
`v0.6.0`. Compared the resolved tag commits, archive digests, and complete
source inventories. Reviewed each changed source file feeding runtime
identity, selected chat requests and responses, HTTP/SSE delivery, usage,
errors, and selected lifecycle. Changelog text was used only to find new
surfaces. No runtime binary was available in the release; no server or model
was installed, executed, or updated, and no provider prompt, catalogue, session,
or credential was used. Research 369's local `llama-server --version` probe
timed out after eight seconds and was not retried.

## Identity

| Point | Exact identity |
| --- | --- |
| Qualified baseline | `b9910`; source commit `f5525f7e7a7e7cbecd386144299493ea40499bd3`; source archive SHA-256 `3b055e2a705fea27392f0661e1cb88b67abda6dadef9315debc64ffbed7f84f2` |
| Official stable | `v0.6.0`, published 2026-10-05 16:56:22 UTC; peeled source commit `d81235049384534c167caea52b85a694f6103d14`; source archive SHA-256 `a56a6273847c92c2d516b2a971343eaec05b588abe74dc361094cc28493a1fac` |
| Release mapping | `nightly-tag.txt` contains `b11429`, SHA-256 `5677c5a4561fad44f2a58fc14187dd9cde253874584e958b37094c723ea8e8a5`; the `b11429` tag and peeled `v0.6.0` tag resolve to the same full commit |
| Runtime identity | The tagged source formats `build_info` as `b{build number}-{short commit}`. The stable source therefore maps to runtime revision `b11429-d81235049`; the source commit prefix can vary in length. No binary digest is published or claimed. |

The release API reported only stable `v0.6.0` for the selected channel. There
are no intermediate stable hops between the old exact pin and this release.
Nightly `bNNNN` tags remain excluded from the stable channel. Synthetic next
stable `v0.6.1` remains unqualified by the route's `QualifiedOnly` posture.

## Source inventory and selected behavior

`tests/fixtures/llama-cpp-v0.6.0/dist-inventory.json` compares all source-tree
entries: 3,031 at `b9910` and 3,687 at `v0.6.0`, with 819 added, 163 removed,
1,403 changed, and 1,465 identical entries. The fixture stores the exact path
sets, source-file hashes for all selected changed files, and SHA-256 digests of
each sorted path set.

The four selected routes remain `GET /health`, `GET /props`, `GET /v1/models`,
and `POST /v1/chat/completions`. Source retains the health status, required
properties, one-model catalogue, plain-text chat request, streamed OpenAI chat
chunks, final usage fields, and error type envelope. The adapter remains
attached: it starts and stops no server and claims no remote interruption.
Changes in common chat parsing, server handlers, stream transport, queueing,
task serialization, and bundled HTTP code are classified in the frozen
`protocol.json` file ledger. The selected mapped wire and adapter lifecycle do
not change.

One permission delta is explicit: `v0.6.0` removes `/v1/models` from the
API-key-exempt endpoint set. API-key configured servers are outside the
adapter's local-unauthenticated profile, and `/props` is already protected by
an API key at both tags, so the selected preparation path rejects that profile
before catalogue on either point. No credential or authority capability is
added.

New `/v1/systemone`, typed multimodal embeddings, MCP, tools, model routing,
UI/model-download flows, model architecture, and backend-kernel changes remain
unmapped. The adapter does not call those endpoints, send tools or multimodal
content, manage server processes, or claim model-quality or performance
behavior.

## Decision

Classify as **compatible extension**. Keep the opaque runtime scheme, exact
`QualifiedOnly` posture, claim ID, behavior revision, `b9910` point, attached
route, and exclusions. Add only the exact `b11429-d81235049` runtime point
correlated to official stable `v0.6.0`. Keep `llama-cpp.owned` independent at
`b10069`; this record grants it no qualification.

## Sources

- [Official v0.6.0 release](https://github.com/ggml-org/llama.cpp/releases/tag/v0.6.0)
- [Latest release API](https://api.github.com/repos/ggml-org/llama.cpp/releases/latest)
- [v0.6.0 source tag](https://github.com/ggml-org/llama.cpp/tree/v0.6.0)
- [b11429 source tag](https://github.com/ggml-org/llama.cpp/tree/b11429)
- [b9910 source tag](https://github.com/ggml-org/llama.cpp/tree/b9910)
- [Frozen identity and source tree inventory](../crates/swallowtail-adapter-llama-cpp/tests/fixtures/llama-cpp-v0.6.0/README.md)
