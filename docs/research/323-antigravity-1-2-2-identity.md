# 323 Antigravity 1.2.2 Identity

Status: promoted
Owner: Tom
Date: 2026-09-15
Card: g05.075 (Research 308 useful-newer campaign)
Authority: Contracts 017, 023, and 029; Research 177, 283, 308, and 322; the
Antigravity prepared guide; and the official GitHub channel. Tom ruled on
2026-09-15 that the official release notes are the behavioural authority for
this requalification; exhaustive binary scanning stopped at that ruling and
already-collected hashes are preserved.

## Question

Do official GitHub `google-antigravity/antigravity-cli` `1.1.27`, `1.1.28`,
`1.2.0`, `1.2.1`, and `1.2.2` extend the catalogue and headless claims
qualified through `1.1.17`, or does the Research 283 `1.1.22`
provider-managed-retry stop still hold, per claim?

## Remaining AllowUnverified rank

Named family only.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Antigravity catalogue and headless | installed official `1.1.19` | `1.1.9..=1.1.17` | operator-dispatched family; official GitHub stable is `1.2.2` |

Gemini follows as the sixteenth Research 308 family. `antigravity-acp` is
another family. `1.1.8` stays independently incompatible.

## Method

Re-probed the official GitHub releases and tags on 2026-09-15: latest stable
is `1.2.2`, published `2026-09-12T03:51:08Z`, tag commit
`ba985e6b5de2ac8aa09860a154a102831eb7722b`. Retrieved the official
linux-x64 and mac-arm64 tarballs for all five new points into `/tmp`; every
tarball SHA-256 matches GitHub's declared asset digest exactly, every
archive holds only the single `antigravity` binary, and each binary carries
its own version literal. Re-downloading `1.1.26`, `1.1.21`, and `1.1.22`
linux-x64 reproduced the Research 283 digests byte-for-byte, so the frozen
boundary stands. Host `agy` was not executed; its binary is byte-identical
to the official signed `1.1.19` mac-arm64 build already frozen in
Research 283.

Before the operator ruling, the pre-ruling probe also verified the selected
flag, value, and catalogue literals and the retry-control absence across
`1.1.21`, `1.1.22`, `1.1.26`, and all five new points, plus the extracted
binary digests, sizes, and ELF build ids recorded in the fixture. That
collected evidence is retained. After the ruling, no further binary
forensics ran; the published release notes are the behavioural
classification authority.

No provider operation, prompt, login, credential, installation, host
update, or downloaded-binary execution occurred at any point.

## Identity

Host `agy` remains official `1.1.19`: binary SHA-256
`96fae3fccfb444c7fb2c6d8d70426e5c978e4f21cfc4507a541f612a8b8ffeef`, size
178046224, Developer ID `Google LLC (EQHXZ8M8AV)`.

| Version | Published | Tag commit | linux-x64 tarball SHA-256 / size | linux-x64 binary SHA-256 / size / Build ID | mac-arm64 tarball SHA-256 |
| --- | --- | --- | --- | --- | --- |
| `1.1.27` | 2026-09-05T04:23:25Z | `1ae9cb7b51667192c051b73a91099c71e816ca5f` | `f874d4f6b8a73c2df660f580f25fb656fcb6e64adbfd746e6692e837fd9a20be` / 56789301 | `93eb2118b778a4005700b54cdd7e08b896fbe665d5ff338e38e9e53da9a091ea` / 210551040 / `f9e9161520c4891e4727593e196cba3d` | `e901e5c8fd20ab4c21c01df306030079286d08e6d372cdb535d5ccc7a3f565f4` |
| `1.1.28` | 2026-09-09T00:43:52Z | `baef32d9f7d91aa5f8f1747b4a8a938723132a19` | `074ff4f732a750ad727aeed5fc82ed34b1fb72fda2a6ceba6c8e652ffd0a94b0` / 56914961 | `a8793092fbe6eea0b8228fc20582ec306f7f7698d1151be526902b1556a76f3a` / 210923776 / `b3f253d9868abc95dc081fe2fcf9f702` | `8f642cffce8bc14aa3e49d1a75780bb2bd99fe7a3016389627476d9e3ec911eb` |
| `1.2.0` | 2026-09-10T01:43:29Z | `34406bef8e87fc103783c0c9715e5e2cce3c3e1b` | `d9bfee1ae6e4329562cb87da1f5fc3c886d18594837e73e25c3aae00a49499b9` / 57224342 | `195bf11b249deebe67028305a9b7b1d19ac38e9ab281b786a163a7d2fc8ff428` / 212304128 / `734ed857e46e56054b0ce7d40927d3b7` | `8fee3c120142490f2eddd286ff7d532b3e660c92d17e60a3612cc0ad1122478f` |
| `1.2.1` | 2026-09-11T06:46:48Z | `e4afe6b6f3aa115b1ba31e26db6508a23b5e42e5` | `6a2c53db6c681fc114f9a1e499e7b4771357ab2852242e56acbd43197d4807f9` / 57458208 | `38f130cdd0757e1d22e151baa48ace4074a5bd3d960eb2dcc7f44bdf2ad4c0fd` / 213422336 / `c61f1d72352c28d668f23f19c79fd7b9` | `b80425e10a7b92f20679eee5df3bb18e3f9154b3653a373fad47e2f72614248e` |
| `1.2.2` | 2026-09-12T03:51:08Z | `ba985e6b5de2ac8aa09860a154a102831eb7722b` | `2cfa5c9a4a1edd96db6d4058f34970be60d3bcacda866e2bdce6aefb2451b48e` / 57596853 | `e8f90ef67943b56c1148d73bc0e102d0b44d18935ffb11d49a2d870a095f416b` / 213582080 / `fa00846652bf39ea18e012d43e3e161f` | `f90ff6094a196f1be3854ac45d999a542d47ec66a1513aa882a8505b947b9a0f` |

Every mac-arm64 extracted binary digest and size is recorded in the frozen
corpus. GitHub has no stable release or tag `1.2.3`; that is the first
unpublished later point. `1.1.27` and `1.1.28` each own a distinct tag
commit; every public tag hop changes only `CHANGELOG.md`, so the published
release notes carry the behavioural record.

## Published selected-path classification per claim

The official release notes name no selected-path change to `agy models` in
any of the five hops. The published retry, timeout, and stop-reason changes
all live in the agent model-request loop and print-mode turn lifecycle that
the headless claim maps:

| Hop | Catalogue | Headless |
| --- | --- | --- |
| `1.1.26→1.1.27` | unchanged | print denied actions become a notice with `denied_actions` in the JSON output; print exits after session shutdown so trailing history reaches disk |
| `1.1.27→1.1.28` | unchanged | transient model API errors retry "for much longer" with exponential backoff; `--print-timeout` expiry now returns partial output and exits zero; print waits for background tasks bounded by `--print-timeout` and leaves daemon tasks running; fatal errors gain a stable `error:` stderr marker and a truncation note; plan review proceeds automatically; URL fetch defaults to asking first |
| `1.1.28→1.2.0` | unchanged | content-safety-filter blocks surface a clear content-filter stop reason instead of silent or retried ends |
| `1.2.0→1.2.1` | unchanged | transient `genai.APIError` failures (502, 503, 504, per-minute 429, mid-stream interruptions) automatically retry in-process with exponential backoff |
| `1.2.1→1.2.2` | unchanged | none published |

## HTTP retry and print-timeout ruling

The Research 283 `1.1.22` stop stands and the later notes widen, not bound,
the same behaviour. `1.1.28` advertises "much longer" exponential retry and
`1.2.1` advertises broader automatic in-process retry; neither publishes a
finite attempt bound, a disable control, or a deterministic provider-neutral
mapping, and no separate operator acceptance exists for this lane. Contract
023 keeps the host deadline distinct from provider-native retry policy, so
the headless run deadline cannot substitute. Under Tom's ruling the absence
of a published bound is recorded directly as the incompatible stop without
deriving a hidden bound. `--print-timeout` expiry changes terminal shape
(`1.1.28`: partial output plus successful exit) but is a host-side turn
bound, not a retry policy, and the notes publish no default or caller bound
change that the adapter selects.

`1.2.0`'s content-filter stop reason is a published selected headless
failure-shape addition on the model-request path the `1.1.22` stop already
blocks, so it changes nothing about the catalogue/headless split.

## Decision

- **Catalogue: compatible extension.** Advance the catalogue claim to
  maintained `1.1.9..=1.2.2` with the
  `antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1` behavior revision,
  baseline `1.1.9`, and `AllowUnverified` unchanged. No published
  selected-path change touches the catalogue command across any of the
  five hops, so each intermediate is qualified. Synthetic `1.2.3` stays the
  visible `UnverifiedNewer` point.
- **Headless: stop stands, gap explicit.** Keep the headless claim at
  maintained `1.1.9..=1.1.17` with the
  `antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1` behavior revision.
  `1.1.18..=1.2.2` stay unqualified at the exact `1.1.22` hop; the gap is
  named, not erased, and no later point is admitted because the retry
  policy is unknown and broader.
- No private milestone, no new public operation, no provider call, and no
  host change. Decoder specimens stay on `antigravity-cli-1.1.9`.
- Reopen headless when official evidence names a finite retry policy plus a
  deterministic disable or bound, or the operator separately accepts the
  exact provider retry behavior under Contract 023.

## Sources

- [GitHub `1.2.2`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.2.2)
  and the release notes for `1.1.27` through `1.2.2`
- [Changelog at `1.2.2`](https://github.com/google-antigravity/antigravity-cli/blob/1.2.2/CHANGELOG.md)
- official `agy_cli_linux_x64.tar.gz` and `agy_cli_mac_arm64.tar.gz` for
  `1.1.27`..=`1.2.2`
- official `agy_cli_linux_x64.tar.gz` for `1.1.21`, `1.1.22`, and `1.1.26`
  (Research 283 boundary reproduction)
- frozen `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.2.2/`
- [Research 283](./283-antigravity-1-1-26-identity.md) and
  [Research 308](./308-all-route-version-currentness-checkpoint.md)
- [Contract 023](../contracts/023-harness-operation-isolation-and-native-boundary.md)
  and [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
