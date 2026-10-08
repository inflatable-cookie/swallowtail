# 406 llama.cpp Owned Scheme Transition Identity Stop

Status: identity evidence only; qualification held for separate adaptation ruling  
Owner: Tom  
Created: 2026-10-08  
Updated: 2026-10-08  
Task: swallowtail#136

## Decision

Official GitHub stable release `v0.6.0` is current as of 2026-10-08 15:40 UTC. The
published stable path after the existing `llama-cpp.owned` point
`b10069-178a6c449` is `v0.2.0`, `v0.3.0`, `v0.4.0`, `v0.4.1`, `v0.5.0`, and
`v0.6.0`. Each stable release's `nightly-tag.txt` identifies a corresponding
`b` source and runtime build; each semantic release tag resolves to the same
commit as that build tag.

This record freezes exact identities only. It does not qualify any hop or claim
that mapped behavior remained compatible. The production claim stays at the
existing exact `b10069-178a6c449` point, with its existing behavior revision,
claim ID, and exclusions. No route guide, matrix, or Unreleased changelog claim
moves.

The requested path cannot be represented in the current claim shape without an
adaptation. The claim uses the `Opaque` scheme and one exact segment. Core
validation rejects multiple opaque segments. Changing that claim to a semantic
range would discard the opaque `b10069` point. The owned driver and readiness
check are also fixed to build `10069` / commit prefix `178a6c449`. Do not change
the claim or runtime binding in this task.

### Adaptation needed

A separate scoped design and ruling must define how a provider-neutral claim can
retain the existing opaque `b10069` point while expressing the exact semantic
release identities, and how owned runtime preparation selects and validates
the corresponding private build/commit mapping. The adaptation must retain the
current owned lifecycle and public behavior contract, and must qualify every
selected hop against its own frozen source and runtime identity. Any consumer-
visible, public claim-model, or lifecycle change needs its own explicit ruling.

## Channel and release identity

The selected channel is the official semver stable release channel at
[ggml-org/llama.cpp Releases](https://github.com/ggml-org/llama.cpp/releases).
The official release API was re-probed at 2026-10-08 15:40 UTC: `v0.6.0`
remained the latest non-prerelease semantic release, published 2026-10-05 at
16:56:22 UTC. The separate `b11505` build release was marked prerelease and
published 2026-10-08 at 14:37:16 UTC; it is not a later stable semantic
release.

The stable releases published after the current exact point are:

| Stable release and publication (UTC) | Pointer asset SHA-256 | Pointer build and exact commit | Stable source-tag archive SHA-256 |
| --- | --- | --- | --- |
| `v0.2.0` — 2026-08-21 18:32:48 | `86383822e6176cca60227f593abdc61a605efae23efac02c3bfb1569077811c0` | `b10566` — `bb4caa7540188872173c44d161602d9271386413` | `72e6c3e70c584f84e61697e449ee388f43458d662ef8f3bd3f6b4a054c947958` |
| `v0.3.0` — 2026-08-25 10:22:58 | `46637e1a90db3d9912a7722806c7657cc73a3965e21c0016cc6b087b3be84205` | `b10621` — `c1d0e7a004015f23bc0233470b747b596f29b264` | `d94c02d86db22d68692f6bb5b3854763d5091e52142868dc7251995517c666d1` |
| `v0.4.0` — 2026-09-04 19:56:47 | `bd0afc13ffe4aaa2f02eb7df8882a2617907d7f232012ae2ca0f21f6b865fbe0` | `b10809` — `5266f24da75dc449bd56cbed7addb9c8e4a6a73e` | `9c2948aa9c79c92dd0e4c98e11ff5cf76dfdcaebdeb18e3e93409e9a98aefdab` |
| `v0.4.1` — 2026-09-14 18:27:29 | `3d64814d6d05845a4b749392f4cb916b84709ab5b1393edcec5e680e2ddd4663` | `b10964` — `b29c606e28a01b1bc8c1351026a0fa6e616bf6c4` | `ef3d5b1907a391500ae11b5e61a8e2022e0deaac9790899cad9c4e02f03bfb9a` |
| `v0.5.0` — 2026-09-23 20:50:06 | `defb2cdfd758ca455a291fe3e37c0f9a442f5ed4fcafe4cbc179237cfb43374f` | `b11146` — `7fe450e19305b828c199d602c23a8337aaa1f03b` | `fef9ed754f4e031fb5c663c29260feda4ebc241abb68d64a81c0f1df5f1748e2` |
| `v0.6.0` — 2026-10-05 16:56:22 | `5677c5a4561fad44f2a58fc14187dd9cde253874584e958b37094c723ea8e8a5` | `b11429` — `d81235049384534c167caea52b85a694f6103d14` | `09b36dba235fcac180efff18514da7038e65d8e5b9e4aefa59238468abfdec12` |

For each row, the pointer digest is for that stable release's `nightly-tag.txt`
asset; the source archive digest is for the exact stable semver tag. Git tag
resolution confirmed that each `v` tag and its pointed `b` tag resolve to the
same full commit shown above. The official source archives are available under
`https://github.com/ggml-org/llama.cpp/archive/refs/tags/<tag>.tar.gz` and the
release metadata under
`https://api.github.com/repos/ggml-org/llama.cpp/releases`.

The existing qualified point's source identity is tag `b10069`, commit
`178a6c44937154dc4c4eff0d166f4a044c4fceba`; its source archive SHA-256 is
`293a7c65a11e2203c5468a06d0d0e8d21dfff16ad08712b16c61efbe0d93e097`.

## Runtime artifact identity

For every point, the official `b` release's macOS arm64 archive was downloaded
and its SHA-256 checked against the GitHub release asset digest. Only the
`llama-server` member was extracted to a temporary directory and hashed; no
artifact was executed.

| Build | Official macOS arm64 asset bytes | Archive SHA-256 | Extracted `llama-server` SHA-256 |
| --- | ---: | --- | --- |
| `b10069` | 10,600,037 | `022469e0b22f4b84dcd0a323867d7f5a31dae21894931ee6a24a35abd2a60359` | `a4998768a70ba2be02617ec9d8773accc2952516f4f5a8f38f621ece54cbf04b` |
| `b10566` | 11,095,544 | `533f546dab2ce2f8e29ce3070f26acc55acc59528e177f2cd0d52b7f69b44f50` | `9f34137ff2559c40a0fe9b130937fd745726130074173665127d863846152198` |
| `b10621` | 10,954,823 | `429c8270608600188035e5e92f7d78dffb7900904fe7dd7e6a84f48068cd13cf` | `c32a2010dec561243448447599b7c13789022052ed59a336005758fbacf03639` |
| `b10809` | 11,123,196 | `7d692df9e1e386e62f1c12b843903218041e6cd74c9415aa39a7ed3176f9eaa2` | `d707b6db4c1397a7383176fba12d339e5b33c7513669d74c8fbc2a76f6979a72` |
| `b10964` | 11,149,739 | `033c845c1df9bf945ff37bb193238b40910b2244be3e1e637b2ceb5878f1a6f5` | `4216ddf73348bd30d4ced17e510ee57edf597181ca009635d33a0bf26b33b5d2` |
| `b11146` | 11,189,714 | `1ad3f9eff80edb9dbef4259ad564d1720612ef7eea48fa4afed0e54f5f3d5711` | `41df13c126456f8e5fab2057c86a790067a85ea1dfd8fbc0071cc45fbba56262` |
| `b11429` | 11,971,406 | `740288ec6887be94280a5dfa25b5e23a78285cab104519e6c7e218904ee82459` | `1bf5602a2be50097c44282a6ecc63234d94ee8f96d39fd45fed6eff4e74b6b41` |

The already-installed Homebrew `llama-server --version` observation reported
`0.6.0`, build `11429`, commit prefix `d81235049`, on Darwin arm64. Its binary
hash differed from the official release binary, so it is an environment
observation and was not used as official artifact proof.

## Complete source trees

Each recursive official Git tree response had `truncated=false`. The retained
TSV files contain every entry sorted by path with `path`, `mode`, `type`, Git
object SHA, and size. Their SHA-256 values make the deterministic inventories
checkable.

| Build | Commit | Entries (blobs, trees) | Inventory |
| --- | --- | ---: | --- |
| `b10069` | `178a6c44937154dc4c4eff0d166f4a044c4fceba` | 3,592 (3,232, 360) | [tree TSV](./llama-cpp-owned-scheme-transition-tree-b10069.tsv), SHA-256 `9128280ac3f3b37674a32d6091da490495297fad37c1617be76cd867e35e706b` |
| `b10566` | `bb4caa7540188872173c44d161602d9271386413` | 3,819 (3,446, 373) | [tree TSV](./llama-cpp-owned-scheme-transition-tree-b10566.tsv), SHA-256 `e7b9fc8501fda86b0cee23218bba9c85b22d1de1863e95a1d53e82060c60bee1` |
| `b10621` | `c1d0e7a004015f23bc0233470b747b596f29b264` | 3,869 (3,494, 375) | [tree TSV](./llama-cpp-owned-scheme-transition-tree-b10621.tsv), SHA-256 `6b0cf4e178f0029dbe97fa6d8dc5285b54a6a4932c9f97b2c52c7b227ef74bd0` |
| `b10809` | `5266f24da75dc449bd56cbed7addb9c8e4a6a73e` | 3,899 (3,527, 372) | [tree TSV](./llama-cpp-owned-scheme-transition-tree-b10809.tsv), SHA-256 `5ca88f750b0fd2af05ce191a98c03f9ed947d392e1d281a92af94efe59f410fa` |
| `b10964` | `b29c606e28a01b1bc8c1351026a0fa6e616bf6c4` | 3,962 (3,588, 374) | [tree TSV](./llama-cpp-owned-scheme-transition-tree-b10964.tsv), SHA-256 `2fab9bbd3d2a19f303106c6f97b1ab24f54c93ddd09c6bcf4ce185ab5e8bc33c` |
| `b11146` | `7fe450e19305b828c199d602c23a8337aaa1f03b` | 3,993 (3,619, 374) | [tree TSV](./llama-cpp-owned-scheme-transition-tree-b11146.tsv), SHA-256 `7ce9a533957bd9e69e390a92eff650c5be02564256dc6674f121d76c684da00a` |
| `b11429` | `d81235049384534c167caea52b85a694f6103d14` | 4,063 (3,687, 376) | [tree TSV](./llama-cpp-owned-scheme-transition-tree-b11429.tsv), SHA-256 `f8fb08d4d4da7d74c5be646e0474cfaac9153bc0f92f0823d1bfe4cac9d5cda9` |

Whole source-file path inventory differences (added, removed, changed blob
identity) by selected stable hop were: `b10069` → `b10566` (334, 120, 1,056),
`b10566` → `b10621` (54, 6, 193), `b10621` → `b10809` (52, 19, 686),
`b10809` → `b10964` (66, 5, 340), `b10964` → `b11146` (36, 5, 378), and
`b11146` → `b11429` (73, 5, 524). These counts describe identity differences
only; no selected behavior classification was performed for this stop.

## Existing claim and limits of this record

The current implementation binds owned serving to one identity:

- `crates/swallowtail-adapter-llama-cpp/src/selection/interface.rs` declares
  the opaque `b10069-178a6c449` binding and one exact maintained segment under
  `llama-cpp.owned-runtime-window-1`.
- `crates/swallowtail-core/src/interface_version/claim.rs` rejects an opaque
  claim with more than one segment.
- `crates/swallowtail-adapter-llama-cpp/src/protocol.rs` expects build `10069`
  and commit prefix `178a6c449` in readiness properties.
- `crates/swallowtail-adapter-llama-cpp/src/driver/owned.rs` uses the fixed
  `swallowtail.llama-cpp.owned-b10069-openai-chat` driver identity and binding.

The existing fake owned-driver lifecycle and readiness proof stays exactly
where it is. This record adds no version-specific launch, no inference proof,
and no evidence for the attached route. No upstream artifact was executed; no
runtime was started, no model was downloaded, and no provider prompt, catalogue,
credential, or live session was used.
