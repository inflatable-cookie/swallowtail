# Bedrock Catalogue SDK 1.161.0 Identity

Task 112 qualifies only the `bedrock.catalogue` Rust SDK axis from the
workspace's `1.150.0` pin through the current official stable,
`aws-sdk-bedrock 1.161.0`, observed on 2026-10-08. The adapter's public
binding constant and exact claim were still at `1.148.0` at observation, so
the identity record preserves that released point while documenting the
pin-to-claim mismatch. This record freezes the published artifacts and
selected `ListFoundationModels` behavior before the claim changes.

## Official channel and artifacts

The official channel is the stable release set for
[`aws-sdk-bedrock`](https://crates.io/crates/aws-sdk-bedrock). The
[crates.io versions API](https://crates.io/api/v1/crates/aws-sdk-bedrock/versions)
reported `1.161.0` as the maximum non-yanked stable at
`2026-10-08T07:00:34.255421Z`. The next stable `1.162.0` and patch
`1.161.1` were absent. The exact current artifact checksum is
`254169f7bd61c2189a0142067000ed89ab15a032b8b8cebf426fabeb425ad611`.

Each downloaded `.crate` archive matched the crates.io API checksum. The
source is [`awslabs/aws-sdk-rust`](https://github.com/awslabs/aws-sdk-rust),
package path `sdk/bedrock`. Exact publication time, crate size, source commit,
Rust version, Smithy codegen/protocol metadata, declared runtime dependency
requirements, and archive checksum for all twelve artifacts are frozen in
[`identity.json`](../../crates/swallowtail-adapter-bedrock/tests/fixtures/bedrock-control-plane-1.161.0/identity.json).

| Version | Published | Crate SHA-256 | Packaged source commit | Yanked |
| --- | --- | --- | --- | --- |
| 1.150.0 | 2026-07-24 | `81186bb96a4e98ff93f7b4336deec0afc1f90ca282099fc5395fcf5de6c0389d` | `4412bdcfc85b94c30e3b158970ca2fc693632ca7` | no |
| 1.151.0 | 2026-08-17 | `2cb8e2aff3ed7af0df6c54961d38a4526b581bb00f93bee6e6a12581d05b1de2` | `e3a8e3db1bd094e96c0e707f15e6f38baf92fe07` | no |
| 1.152.0 | 2026-08-20 | `1779d9b47d6a549caa8cde4dd7811f3dfc2111308c0d696d202807a7f5892bda` | `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db` | no |
| 1.153.0 | 2026-08-24 | `ad5175fd0be54231cc4f9d0a5cbb0a6384548fd6aa5c846c1d6945960d35d840` | `feff37ab0d80eef00a798cbbb740cf3b15453900` | no |
| 1.154.0 | 2026-08-25 | `4422d8bd0834d25f5c35b60ad77efea0b552e309691b30c975c0eef7dfd222b0` | `e2d4cb15aab1e2a68d3d0ee6d5828f0333839ef0` | no |
| 1.155.0 | 2026-09-04 | `42c7a359ec7c2904bd8f541d7931015902a73e380ee68760ce1e409f69809e8c` | `653085fbbc4a50138cf955445b65431bea3599d4` | no |
| 1.156.0 | 2026-09-15 | `d01490add1001bfe6529a3f36a84ca51f1d3d5ecc59f2ebd959ac76c04e2bc95` | `82b4cf5c430a4177f7675a8c78986a9f3c74c8a7` | **yes** |
| 1.157.0 | 2026-09-16 | `347e5d5ea47f43415003507fbd8aacecc8f7ccd09a2455f2b5e376921905e48f` | `7d6ed1a3296cf582c7efeefcd35998975b188e09` | no |
| 1.158.0 | 2026-09-22 | `0024daf189f4c0026c7aa2580d6450af163525c9afb7f84b019666661f7e8a6b` | `dd4d62780ee72db9adae16134e5dac47a9b6393e` | no |
| 1.159.0 | 2026-09-25 | `42e438d72b1c900df6926ef6cfd4c378432ccb1ab2639dc943921a08f048f3c4` | `2ea89feffd8b76d5b4f1d9030f38f956ec4b6957` | no |
| 1.160.0 | 2026-09-30 | `fe7904fdf1267ae7c37d42b6ab231b58ccd42d50adbdcad446be243586a6a67d` | `d3d147c208ab27790dfded9cdc724aa9b7bed6b6` | no |
| 1.161.0 | 2026-10-01 | `254169f7bd61c2189a0142067000ed89ab15a032b8b8cebf426fabeb425ad611` | `7101aefb7632e44cce586886a1df595151409b5f` | no |

The previously exact `1.148.0` point remains qualified. Published `1.149.0`
is non-yanked but lies between that point and the existing `1.150.0` pin;
it remains an independently unqualified hole. Yanked `1.156.0` also remains
excluded. The crate version is not a Bedrock service-model date.

## Complete tree and hop classification

[`dist-inventory.json`](../../crates/swallowtail-adapter-bedrock/tests/fixtures/bedrock-control-plane-1.161.0/dist-inventory.json)
freezes all 1,470 packaged files at each of the twelve versions and every
adjacent hop from `1.150.0` through `1.161.0`. All hops retain the same file
count, with no added or removed paths. The ledger has 502 changed-file hop
records; 1,240 paths are byte-identical through every release. Exact changed
path sets, old/new hashes, selected-file hashes, and a classification with a
rationale for every changed path are asserted by the identity test.

The selected generated request, response, output types, enums, service error
types, client, endpoint and retry configuration files are separately hashed
for every version. The selected route stays one explicit `GET
/foundation-models` request with no filters or pagination, explicit region,
endpoint and delegated credentials, and one SDK attempt. Its typed response
continues to project the same bounded model summary, enum, and lifecycle
fields. The selected failure variants and redaction boundary remain unchanged.

Selected source changes are bounded:

- `1.151.0` adds a by-provider input-capture interceptor and opt-in telemetry
  configuration. Telemetry remains off in the adapter's fresh config, and
  its unfiltered generated input has no `byProvider` value.
- `1.155.0` adds clock-skew retry classification and an SDK configuration
  override. The adapter fixes `max_attempts` at one, so the classifier cannot
  cause an additional provider request; it does not set the new override.
- `1.158.0` changes only a configuration example. `1.159.0` passes a
  `ConfigBag` to the selected response/error decoder, whose added argument is
  unused and whose body is unchanged.

Other operation modules, serializers and new SDK exports belong to sibling
Bedrock operations that this adapter does not call or expose. The 1.152 to
1.153 `ModelConfiguration` change is documentation-only and belongs to
Advanced Prompt Optimization. The 1.156 serialization changes affect other
request shapes; the point is yanked and excluded. The full inventory bounds
these unmapped changes by exact path and digest.

No crate artifact was executed. No AWS request, live catalogue, credential,
host SDK installation, or host update was used.

## Claim decision

The observation had exact opaque SDK claim
`amazon-bedrock.catalogue-sdk-window-1` at `1.148.0`, behavior revision
`amazon-bedrock.catalogue-sdk-1`, while `Cargo.toml` was already pinned to
`1.150.0`. This compatible extension preserves the claim id, behavior
revision, exact `1.148.0` point, and all route, endpoint, authentication,
region, capability, service, failure, and lifecycle boundaries. It adds
maintained semantic segments `1.150.0..=1.155.0` and
`1.157.0..=1.161.0`, retaining `1.149.0` as an unqualified gap and
`1.156.0` as a yanked exclusion. Later stable points remain
`UnverifiedNewer`. No public operation, driver, capability, service API
revision, or consumer-visible behavior is added or narrowed.

The current route claim and catalogue guide reference this record. Runtime
SDK and Runtime service claims are unchanged.
