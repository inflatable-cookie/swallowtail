# Bedrock Runtime SDK 1.148.0 Identity

Task 113 qualifies the `bedrock.runtime` Rust SDK axis from the workspace's
`1.139.0` pin through the current official stable, `1.148.0`, observed on
2026-10-08. The selected distribution is the exact crates.io package
`aws-sdk-bedrockruntime`; the selected operation is `ConverseStream`. This
record freezes identity and selected-surface evidence before the production
claim change.

## Official channel and artifacts

The official channel is the crates.io stable release set for
[`aws-sdk-bedrockruntime`](https://crates.io/crates/aws-sdk-bedrockruntime).
At `2026-10-08T00:28:15Z`, the versions API reported `1.148.0` as its maximum
non-yanked stable. `1.149.0` and `1.148.1` were absent. Selection follows the
maximum stable non-yanked version, not a `latest` label. The channel was
re-probed immediately before the identity push at `2026-10-08T00:36:58Z`; it
still reported `1.148.0`, with the same checksum, and neither `1.149.0` nor
`1.148.1` had appeared. The final qualification re-probe at
`2026-10-08T00:49:45Z` returned the same stable version and checksum, with
`1.149.0` and `1.148.1` still absent and `1.148.0` not yanked.

Each `.crate` download matched the checksum returned by the official
[crates.io versions API](https://crates.io/api/v1/crates/aws-sdk-bedrockruntime/versions).
The source repository is
[`awslabs/aws-sdk-rust`](https://github.com/awslabs/aws-sdk-rust), package
path `sdk/bedrockruntime`. The table gives artifact digest and packaged source
commit. `1.144.0` is yanked and remains excluded.

| Version | Crate SHA-256 | Packaged source commit | Yanked |
| --- | --- | --- | --- |
| 1.139.0 | `8dffe4bd3da45048fa0d30b50febaedf093e93a25d832cc21f2845a6e0c72616` | `8fc277d8adbdd59e0918e8c1189c24d887321e60` | no |
| 1.140.0 | `5bca4d762965c0b7f5fb88ec0a4b48cfa1a22502e1a0283e5fd5d9498dae52b6` | `e3a8e3db1bd094e96c0e707f15e6f38baf92fe07` | no |
| 1.141.0 | `6418b1f5feb3a8cc0fa10fb9af5693c88135ee03bae3ff48c8cdbf6a918daa07` | `3c6d526c9d4775f41a8ef1ed2ef574d1b14481db` | no |
| 1.142.0 | `4844547a354fb4e41895f4c9c423e7a7de13e3b2adc3f3493a2f2d8aa397c294` | `e2d4cb15aab1e2a68d3d0ee6d5828f0333839ef0` | no |
| 1.143.0 | `492a63a8e903a7b8f7c77537ef05fc75d71ba268cf23a0eee41fb6f0da605029` | `653085fbbc4a50138cf955445b65431bea3599d4` | no |
| 1.144.0 | `a2a05220aa89f7db7883abc73cbcaf5f96929f4cdec577c50e3fadedeb574871` | `82b4cf5c430a4177f7675a8c78986a9f3c74c8a7` | **yes** |
| 1.145.0 | `aed4de4a98241f9dd95ddad4223755b7afc3327d0a148a2a29286c95ef64db5e` | `7d6ed1a3296cf582c7efeefcd35998975b188e09` | no |
| 1.146.0 | `8aab5c776d9d4b5d7bc476368394a8f683ae280c7afcf19fb83e9ed6840408c7` | `dd4d62780ee72db9adae16134e5dac47a9b6393e` | no |
| 1.147.0 | `36e7de5d63ec9b699713b498b458720c5219ad7eb66b861e65fe51a21ab043df` | `2ea89feffd8b76d5b4f1d9030f38f956ec4b6957` | no |
| 1.148.0 | `3f2542c0f038223ba2f9aefe3bd38c7395ccc6899d8fc58a99b8ddfeab3354f5` | `7101aefb7632e44cce586886a1df595151409b5f` | no |

## Complete tree and hop classification

The companion
[`dist-inventory.json`](../../crates/swallowtail-adapter-bedrock/tests/fixtures/bedrock-runtime-1.148.0/dist-inventory.json)
contains the SHA-256 manifest of every packaged file at all ten versions, the
per-hop added, removed, changed, and identical path sets, both hashes for each
changed file, and a classification and rationale for every changed path. Each
published package has 544 files; no files were added or removed across these
hops. There are 119 changed-file hop records and 503 paths byte-identical
through every version. The adapter identity test reconstructs each manifest
from the frozen base and deltas and checks the classifications cover each
changed path exactly.

The selected request, EventStream, usage, and error files are frozen by
per-version hashes in that inventory. The generated ConverseStream input and
message/content/role/inference types are byte-identical from `1.139.0` through
`1.148.0`. Event variants, usage fields, and the generated error enum are also
unchanged. The selected operation changes are bounded:

- `1.140.0` adds opt-in model ID telemetry capture. The adapter does not
  configure telemetry. Its other EventStream serializer change belongs to
  `InvokeModelWithBidirectionalStream`.
- `1.143.0` adds retry classifiers and clock-skew handling. The adapter
  configures one attempt and creates a client per inference, so this does not
  admit retries or later requests.
- Yanked `1.144.0` changes serialization settings for the optional
  `additionalModelRequestFields` member; the adapter does not set it. A
  separate tool-result serializer change is outside the user-text-only
  selected request. `1.145.0` restores both serializers.
- `1.147.0` threads a ConfigBag through ConverseStream response deserialization;
  the argument is unused and the output decoder is unchanged.
- Other changes affect metadata, documentation, or different SDK operations.

No downloaded artifact was executed. No AWS calls, credentials, provider
prompts, host SDK installation, or host mutation were used.

## Frozen selected protocol

[`protocol.json`](../../crates/swallowtail-adapter-bedrock/tests/fixtures/bedrock-runtime-1.148.0/protocol.json)
freezes the exact private route boundary for the fake-transport regression:
one POST to the explicit ConverseStream operation using model ID, a single
user text message, and `inferenceConfig.maxTokens = 7`. The expected body is:

```json
{"inferenceConfig":{"maxTokens":7},"messages":[{"content":[{"text":"hello"}],"role":"user"}]}
```

Optional request members remain absent. The route uses one SDK attempt, typed
EventStream decoding, bounded assistant text and input/output/total token
usage, fail-closed handling of unknown or unsupported event semantics, and
does not retain provider messages in failures. The fixture also records the
approved endpoint, region, and delegated-identity configuration boundaries.

## Claim decision

The code at the identity observation had an exact opaque SDK claim at `1.136.0`
and a Cargo pin at `1.139.0`; its `SDK_VERSION` constant was also `1.136.0`.
This qualification preserves exact `1.136.0`, leaves `1.137.0` and `1.138.0`
unqualified, and adds the selected semver segment `1.139.0..=1.148.0` under
the existing `amazon-bedrock.runtime-sdk-window-1` claim and
`amazon-bedrock.runtime-sdk-1` behavior revision. The yanked `1.144.0` point
is explicitly excluded. Newer stable versions are `AllowUnverified` and do
not inherit this evidence. No operation, capability, service API claim,
catalogue SDK claim, or consumer-visible behavior is added or narrowed.

The fixture's `identity.json` and `protocol.json` freeze the official metadata
and selected wire/lifecycle bounds. The identity test checks their exact
digests, the crate checksums, complete reconstructed tree inventory, and
selected-source file sets.
