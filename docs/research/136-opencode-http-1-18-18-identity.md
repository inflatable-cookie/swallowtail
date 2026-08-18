# 136 OpenCode HTTP 1.18.18 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 246

## Question

After g03.078 qualified Cursor Agent August exact milestones, which
AllowUnverified family should move first, and is OpenCode HTTP `1.18.18` a
compatible extension of `opencode.server` through `1.18.10`, a new public
operation, or a stop?

## Remaining AllowUnverified rank

Cursor Agent is done. Remaining host-drifted families, Research 127 numbers
unless noted:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | OpenCode HTTP | `1.18.18` | published segments through `1.18.10` | named next after Cursor |
| 2 | Kimi ACP / headless / local-server | `0.34.0` | through `0.31.1` | later family |
| 3 | Ollama attached | `0.32.9` | `0.14.0..=0.32.1` excluding `0.32.2` | later family |

Host still on a qualified bound (registry newer only): Claude Agent ACP,
Pi, Qwen, Antigravity. Rank those after host-drifted families.

Gemini stays deferred. Do not flatten this HTTP/SSE claim onto OpenCode ACP.

Research 127 already classified OpenCode as visible unverified-newer: host
and npm `1.18.18`, qualified through `1.18.10`. This card asks whether
selected HTTP/SSE evidence now justifies moving the qualified boundary.
Leaving that point UnverifiedNewer would skip useful-newer support.

## Method

Compared host `opencode --version` / `--help`, npm `opencode-ai@1.18.18`,
GitHub tags `v1.18.11` through `v1.18.18`, each tag's
`packages/sdk/openapi.json`, the frozen 51-release compatibility corpus
through `1.18.10`, and the production `opencode.server` claim.

For every newer tag the check hashed the full OpenAPI document and
recursively closed the six selected execution operations plus delete and
import/continuity operations through every local JSON `$ref`. Object keys
were recursively sorted before SHA-256 for comparison. Current server
documentation was corroboration only.

No provider prompt, attached-server start, host install, update, or claim
edit in this research card.

## Identity

| Fact | Value |
| --- | --- |
| host CLI | `1.18.18` at `/Users/tom/.opencode/bin/opencode` |
| host executable SHA-256 | `4f5979c2dadb06fbff1335335afaaea274e58f92e79aa43cf2ed98618d555422` |
| host size | 143182562 |
| npm package | `opencode-ai` |
| npm latest | `1.18.18` (published 2026-08-13T01:13:43.814Z) |
| npm integrity | `sha512-J+5HFq8tf+wPBBpBpMPSNjSytF2/EkNWYfFZh4si1d9auFbQriqDyqZv+vFUsLWERfdMU32Eajwuiq3rKBvZLQ==` |
| npm shasum | `a78971b6affe7ed27a207218465d1a80e36a018c` |
| GitHub tag `v1.18.18` | `31406ccc51b4bd2a4e1e086b2bcaa5f7f804f26d` |
| Research 127 host/npm | `1.18.18` |

Published stables `1.18.11` through `1.18.18` are contiguous. No unpublished
patch gap in that span. Earlier unpublished gaps (`1.15.8`, `1.16.1`,
`1.17.21`) stay closed.

| Version | Tag commit | Published | OpenAPI SHA-256 |
| --- | --- | --- | --- |
| `1.18.10` | `7902e04c3a67f7c69726bc955efb46e29214c797` | 2026-07-30 | `063e1cc745665f3846be7911e1eb793dcfe45bca5ae3cc425ab246b80eeec4ce` |
| `1.18.11` | `012c2f57f976489d88bd4598a056b4bdcdd428ee` | 2026-08-01 | `5bbd6493a1a488ef4294889341c896e420f814ecea95822100aaa9f3f95ab2d1` |
| `1.18.12` | `0dd6950d1b06958fbcdcadf0ad56258257ab7fdb` | 2026-08-04 | same as `1.18.11` |
| `1.18.13` | `a105350812f05f914c768e468559dbd6bd508d8e` | 2026-08-04 | same as `1.18.11` |
| `1.18.14` | `65cf14df16c191f3e9684f0d9a8bae69103ced6d` | 2026-08-05 | same as `1.18.11` |
| `1.18.15` | `d7b115f623760e68a4749d16508a9eca350f246f` | 2026-08-07 | same as `1.18.11` |
| `1.18.16` | `a3647eb025c7615159d417dcc49fc39fdaeba65b` | 2026-08-10 | same as `1.18.11` |
| `1.18.17` | `02546dfc2e4515a4f90aaf9ceb3890df2ac2b479` | 2026-08-12 | same as `1.18.11` |
| `1.18.18` | `31406ccc51b4bd2a4e1e086b2bcaa5f7f804f26d` | 2026-08-13 | same as `1.18.11` |

`1.18.10` OpenAPI SHA-256 matches the frozen corpus row. Path count stays
162. Operation count stays 188. No selected path was added or removed.

## Protocol comparison

Selected execution operations remain present at every newer tag:

- `GET /global/health`
- `GET /provider`
- `POST /session`
- `POST /session/{sessionID}/prompt_async`
- `GET /event`
- `POST /session/{sessionID}/abort`

The six operation objects themselves are unchanged from `1.18.10`. Delete
(`DELETE /session/{sessionID}`), import (`session.list` / `status` / `get` /
`messages` / `prompt_async`), and continuity closures are identical.

Host help still documents operator-owned `serve`, `attach`, `--hostname`,
and `--port`. Swallowtail still does not start, stop, upgrade, or attach
through the CLI. Unused help deltas include `acp`, `web`, `upgrade`,
`--auto`, and `--mdns`.

The only selected-closure delta is `GET /provider`'s transitive `Model`
schema. `Model.capabilities.interleaved.field` changed from enum
`reasoning` / `reasoning_content` / `reasoning_details` to an anyOf that
adds `reasoning_text` plus a free string. `ProviderConfig` has a similar
unselected interleaved delta. Swallowtail does not map interleaved field
names. Auth, lifecycle, deletion, cancellation, and cleanup truth are
unchanged.

That Model change sits inside the six-route closed surface, so it is not
an unselected `1.18.8`-style artifact delta. It is a new private execution
surface. It is not a new public operation.

## Segment decision for card 247

Compatible extension of the qualified window, with private `surface-19`.

Same axis `opencode.server`. Keep baseline `1.14.48`, AllowUnverified, and
surfaces `01` through `18`. Keep unpublished gaps closed. Do not flatten
onto one closed `1.14.48..=1.18.18` interval.

- keep `1.18.0..=1.18.10` on `surface-18`
- add `1.18.11..=1.18.18` on `surface-19`
- extend delete-02, import-07, continuity-07, callback, runtime-02, and
  reconciliation through `1.18.18`
- record `1.18.11` artifact delta
  `selected-model-interleaved-field-enum`
- after qualification, synthetic later-stable UnverifiedNewer is `1.18.19`

Decoder specimen remains `opencode-1.14.48`. Card 247 owns the claim
change.
