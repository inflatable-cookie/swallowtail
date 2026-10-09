# Research 421: llama.cpp v0.6.0 Attached Identity

Status: promoted

Owner: `llama-cpp.attached`
Date: 2026-10-09
Official channel: GitHub latest non-prerelease release

## Question

Which exact opaque runtime does the current semantic stable identify, and what
published stable points lie between it and the retained attached runtime?

## Method and limits

Re-probed the official latest release page on 2026-10-09; it still resolves to
`v0.6.0`. Compared all six published non-prerelease release hops after the
retained `b9910` point, from `v0.2.0` through `v0.6.0`. For each hop, the
release's `nightly-tag.txt`, semantic tag commit, nightly tag commit, source
archive digest, and complete extracted source tree were checked. Changelog text
was used only for discovery. The currentness channel excludes prereleases and
tags without a GitHub release object.

No runtime binary was available in the releases. No server or model was
installed, executed, or updated, and no provider prompt, catalogue, session, or
credential was used. Research 369's local `llama-server --version` probe timed
out after eight seconds and was not retried. The source-derived build identity
does not claim an observed binary or a live deployment.

## Stable-hop ledger

Each `bNNNN-<commit prefix>` value below is an opaque runtime identity derived
from the exact source. It is not the semantic release tag. The release asset
and source archive digests are frozen in the identity fixture.

| Published stable | `nightly-tag.txt` | Derived runtime identity | Peeled source commit | Source archive bytes / SHA-256 | `nightly-tag.txt` SHA-256 | Qualification |
| --- | --- | --- | --- | --- | --- | --- |
| [v0.2.0](https://github.com/ggml-org/llama.cpp/releases/tag/v0.2.0), 2026-08-21 18:32:48 UTC | `b10566` | `b10566-bb4caa754` | `bb4caa7540188872173c44d161602d9271386413` | 36,885,549 / `13a71b442e5376bb000b26f4e8b4b06565758cb5e47a573d882e9c299d8b2e04` | `86383822e6176cca60227f593abdc61a605efae23efac02c3bfb1569077811c0` | unqualified exact gap |
| [v0.3.0](https://github.com/ggml-org/llama.cpp/releases/tag/v0.3.0), 2026-08-25 10:22:58 UTC | `b10621` | `b10621-c1d0e7a00` | `c1d0e7a004015f23bc0233470b747b596f29b264` | 36,960,114 / `2d1bb0124f338c154d19f098ce56869a9e405d2d94d723bef7b8e575f901aeeb` | `46637e1a90db3d9912a7722806c7657cc73a3965e21c0016cc6b087b3be84205` | unqualified exact gap |
| [v0.4.0](https://github.com/ggml-org/llama.cpp/releases/tag/v0.4.0), 2026-09-04 19:56:47 UTC | `b10809` | `b10809-5266f24da` | `5266f24da75dc449bd56cbed7addb9c8e4a6a73e` | 37,292,617 / `1ebab599be928beaba548cc003035128e830d168bb5caed15137b2c8d58e6725` | `bd0afc13ffe4aaa2f02eb7df8882a2617907d7f232012ae2ca0f21f6b865fbe0` | unqualified exact gap |
| [v0.4.1](https://github.com/ggml-org/llama.cpp/releases/tag/v0.4.1), 2026-09-14 18:27:29 UTC | `b10964` | `b10964-b29c606e2` | `b29c606e28a01b1bc8c1351026a0fa6e616bf6c4` | 37,424,281 / `2d069b4acf5bbc59fefb3f38e393320e7ed77001da2c1a4075466e6cb100bd5b` | `3d64814d6d05845a4b749392f4cb916b84709ab5b1393edcec5e680e2ddd4663` | unqualified exact gap |
| [v0.5.0](https://github.com/ggml-org/llama.cpp/releases/tag/v0.5.0), 2026-09-23 20:50:06 UTC | `b11146` | `b11146-7fe450e19` | `7fe450e19305b828c199d602c23a8337aaa1f03b` | 37,568,594 / `3233591576fa0d6527ab92b0c40f55a5e02e1bc167a50d61bfe23c290b4be84b` | `defb2cdfd758ca455a291fe3e37c0f9a442f5ed4fcafe4cbc179237cfb43374f` | unqualified exact gap |
| [v0.6.0](https://github.com/ggml-org/llama.cpp/releases/tag/v0.6.0), 2026-10-05 16:56:22 UTC | `b11429` | `b11429-d81235049` | `d81235049384534c167caea52b85a694f6103d14` | 37,876,239 / `a56a6273847c92c2d516b2a971343eaec05b588abe74dc361094cc28493a1fac` | `5677c5a4561fad44f2a58fc14187dd9cde253874584e958b37094c723ea8e8a5` | qualified exact point |

For every release, the peeled semantic tag and the `bNNNN` tag resolve to the
same full source commit. Source build metadata formats `build_info` as
`b{build number}-{short commit}`. The `bNNNN` value therefore maps to an exact
opaque runtime point; the semantic tag is only its release evidence.

`v0.1.0` and `v0.1.1` have tags but no GitHub release object. `v0.1.2` is a
prerelease published 2026-08-18. They are excluded from the selected
latest-non-prerelease-release channel. Nightly tags are not independent stable
release points.

The finite approved claim retains `b9910-f5525f7e7` and adds only
`b11429-d81235049`. The five published intermediate runtime identities remain
unqualified exact gaps. Their source evidence does not admit an interval or
transfer support across the missing points.

## Source inventory and selected behavior

`tests/fixtures/llama-cpp-v0.6.0/dist-inventory.json` retains the exact complete
`b9910` to `v0.6.0` file sets and their path-set digests: 3,031 baseline files
and 3,687 target files, with 819 added, 163 removed, 1,403 changed, and 1,465
identical. It also records each of the six adjacent stable hops with file
counts, digests for every added/removed/changed/identical path set, the exact
selected-behavior changed paths, and hashes for those paths at every release.
The selected protocol fixture classifies every changed selected file on each
hop.

The selected endpoints remain `GET /health`, `GET /props`, `GET /v1/models`,
and `POST /v1/chat/completions`. The selected request remains one plain-text
chat attempt with streaming, bounded output, no tools, and no typed content.
The response mapping remains assistant text, stop/length finish, usage, and
`[DONE]`; failures retain an error type and the adapter redacts raw payloads.
Attached cleanup closes only local request work and preserves the external
server.

The per-hop classification includes the previously omitted paths:

- `common/jinja/caps.cpp` and `caps.h` produce `chat_template_caps`, which the
  adapter consumes in its existing text-only guard. The analyzer adds
  reasoning-effort reporting and changes template-dependent string/typed
  content detection. The adapter adds no reasoning or typed-content dispatch;
  exact computed flags for arbitrary installed model templates are not claimed.
- `common/build-info.h` changes the `llama_print_build_info` declaration;
  `llama_build_info()` remains the selected build identity accessor. The
  source-derived `b{build}-{commit}` mapping remains in the generated source
  and build metadata path.
- `tools/server/server-models.cpp` and `.h` contain router-mode `/props` and
  `/v1/models` handlers. New model routing and management remain outside the
  attached facade. Router properties without the selected template fields and
  catalogues with more than one model fail closed under existing checks.
  `v0.6.0` adds architecture metadata to `/v1/models`; the decoder ignores
  additive fields and retains its exact `id`/`object` and one-model checks.

The source change in `common/jinja/caps.cpp` is model-template-dependent.
Because this task does not execute a server or install a model, it establishes
the exact source and preserves the adapter's fail-closed check; it does not
claim identical capability booleans for every template. No new public
operation or adapter mapping is introduced.

One permission delta is explicit: `v0.6.0` removes `/v1/models` from the
API-key-exempt endpoint set. API-key-configured deployments are outside the
adapter's local-unauthenticated profile; `/props` is already protected by an
API key at both endpoints. No credential or authority capability is claimed.

New `/v1/systemone`, typed multimodal embeddings, MCP, tools, model routing,
UI/model-download flows, model architecture, and backend-kernel behavior
remain unmapped. The adapter does not call those endpoints, send tools or
multimodal content, manage server processes, or claim model quality or
performance behavior.

## Decision

Classify the current stable as a **compatible extension** for the existing
attached route contract. Keep the opaque runtime scheme, exact
`QualifiedOnly` posture, claim ID, behavior revision, baseline, attached
lifecycle, and exclusions. Add only exact `b11429-d81235049`, correlated to
official stable `v0.6.0`. Retain `v0.2.0` through `v0.5.0` as unqualified
exact gaps. Keep `llama-cpp.owned` independent at `b10069`; this record grants
it no qualification.

## Sources

- [Official latest release](https://github.com/ggml-org/llama.cpp/releases/latest) — rechecked 2026-10-09; resolves to `v0.6.0`
- [Official releases](https://github.com/ggml-org/llama.cpp/releases)
- Release and source tags for [v0.2.0](https://github.com/ggml-org/llama.cpp/tree/v0.2.0), [v0.3.0](https://github.com/ggml-org/llama.cpp/tree/v0.3.0), [v0.4.0](https://github.com/ggml-org/llama.cpp/tree/v0.4.0), [v0.4.1](https://github.com/ggml-org/llama.cpp/tree/v0.4.1), [v0.5.0](https://github.com/ggml-org/llama.cpp/tree/v0.5.0), [v0.6.0](https://github.com/ggml-org/llama.cpp/tree/v0.6.0)
- Frozen artifact and source tree identities: [`identity.json`](../../crates/swallowtail-adapter-llama-cpp/tests/fixtures/llama-cpp-v0.6.0/identity.json), [`protocol.json`](../../crates/swallowtail-adapter-llama-cpp/tests/fixtures/llama-cpp-v0.6.0/protocol.json), [`dist-inventory.json`](../../crates/swallowtail-adapter-llama-cpp/tests/fixtures/llama-cpp-v0.6.0/dist-inventory.json)
