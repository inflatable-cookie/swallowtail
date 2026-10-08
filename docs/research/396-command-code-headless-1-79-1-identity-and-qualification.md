# Command Code Headless 1.79.1 Identity and Qualification

## Decision

Qualify only command-code.headless on command-code.npm through official npm
latest 1.79.1. Preserve baseline 1.65.0, claim
command-code.headless-window-1, and QualifiedOnly. The claim has an exact
segment for each of the 33 published stable points:

1.65.0, 1.65.1, 1.65.2, 1.65.3, 1.65.4, 1.65.5, 1.66.0, 1.67.0, 1.68.0,
1.69.0, 1.70.0, 1.71.0, 1.72.0, 1.72.1, 1.72.2, 1.72.3, 1.72.4, 1.73.0,
1.73.1, 1.73.2, 1.73.3, 1.73.4, 1.74.0, 1.74.1, 1.74.2, 1.74.3, 1.75.0,
1.75.1, 1.76.0, 1.77.0, 1.78.0, 1.79.0, and 1.79.1.

Keep behavior revision command-code.agent-event-ndjson-v1 through exact
1.72.4. The approved private milestone
command-code.agent-event-ndjson-v1-model-selection-v2 covers exact published
points from 1.73.0 through 1.79.1. The v2 revision records the version-specific
plan-lane model-selection truth; it adds no public operation or permission
path. The unpublished patch points 1.65.6, 1.66.1, 1.67.1, 1.68.1, 1.69.1,
1.70.1, 1.71.1, 1.72.5, 1.73.5, 1.74.4, 1.75.2, 1.76.1, 1.77.1, and
1.78.1 remain incompatible. 1.79.2 was absent from the observed registry
stable list. No later point is qualified or executable under this claim.

The 1.73.0+ plan-mode model precedence changes the earlier explicit-model
guarantee. Contract 036 places this pre-1.0 behavior guarantee change in the
separately assessed minor, not the urgent v0.5.2 patch. No release version,
tag, or publication is authorized here.

## Official package identity

At 2026-10-08T12:42:45Z, npm dist-tags.latest resolved to
command-code@1.79.1, published 2026-10-07T22:09:28.916Z:

- registry: https://registry.npmjs.org/command-code
- tarball: https://registry.npmjs.org/command-code/-/command-code-1.79.1.tgz
- SHA-1: bcf1f23e54f94dee0d285d3aa986b0ca4225896c
- SHA-256: 55ed3d986070d8b1b749dcb8b47e2daacc877616f284ae4dde47c0d9fa20490b
- npm integrity: sha512-EO/15K8AEvu4K/2i6in+Oe83cFqQzvZ2qvcABtpH6A1F/eDCwtXErMolcxRtllAkiEEd0XM5LFxMvIIhIlYMzg==
- 72 files; unpacked size 4,357,918 bytes; package Node engine >=22
- full file-tree inventory SHA-256:
  b330e09dc23cb1eb6a74c45f20b8f13d1b722b9a8bfcf3c9abf79ba0361f5355

The npm manifest has no repository URL or gitHead; the exact official tarball
metadata and complete extracted tree identify the shipped artifact. The
recorded Node.js 22.23.2 binary hash identifies only the static inspection
worker, not a consumer host or supported runtime point.

## All-hop evidence

The retained evidence in
[command-code-1.79.1](../../crates/swallowtail-adapter-command-code/tests/fixtures/command-code-1.79.1/)
freezes all 33 exact artifacts, each complete 72-path tree and its sorted
inventory hash, and all 32 adjacent-hop added, removed, changed, identical,
and per-file classification sets. The identity test recomputes every set from
the frozen file digests and asserts exact keys.

dist/index.mjs is byte-identical through all points. dist/cli.mjs changes on
every hop. The selected JSON event serializer, model_request_start.model,
fixed command options, selected event/result/usage/failure keys, and mapped
lifecycle paths remain present. The first resolveLaneModel implementation and
documented feature-model:planning setting appear at 1.73.0.

The ledger classifies each changed file at each hop. It separates the selected
model-lane transition from package-version metadata, changelog discovery,
bundled documentation, and the unselected VS Code extension. The bundled
settings documentation corroborates the planning-model setting; it is not
used as completeness evidence. No source repository identity was inferred
from the changelog.

## Selected model evidence

In plan mode from 1.73.0, Command Code may select configured
featureModels.planning ahead of the base model forwarded by -m, subject to
its own active-lane, model-access, BYOK, and ZDR checks. Swallowtail keeps
plan-mode permissions and does not change configuration or settings.

For the v2 milestone, an opt-in InterfaceVersion debug observation reports:

- requested model ID from the prepared PreflightPlan
- CLI-selected model ID from model_request_start.model
- route command-code.headless, stage model-selection, and the existing
  request correlation ID

Each observed ID is accepted only when nonempty, trimmed, control-free, and at
most 128 bytes. A missing or invalid event value remains unknown; it is never
inferred from argv. The event value is the model Command Code passes to its
SDK request path. It does not prove which remote provider/backend served the
request. The observer is opt-in and fail-soft. No public event, catalogue,
operation, persistent config write, authentication relocation, provider
artifact execution, or provider request was added.

The deterministic plan-model-selection.json fixture supplies conflicting
requested and configured planning IDs and a fake complete NDJSON stream. It
proves decoder and debug projection behavior without a live session or
settings mutation.

## Live evidence and limits

Research 347's paid-model completion, usage, and private two-turn continuity
remain exact 1.65.0 evidence on deepseek/deepseek-v4-flash. Research 330
remains exact 1.54.0; Research 116/118 remain exact 1.15.1. Static package
identity and deterministic fixtures do not transfer live completion, billing,
provider honor, or backend identity to newer versions.

No provider prompt/session, credentials, installation, host update, settings
write, package execution, consumer repin, release, or tag occurred.
