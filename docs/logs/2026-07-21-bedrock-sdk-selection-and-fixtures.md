# 2026-07-21 Bedrock SDK Selection And Fixtures

## Outcome

Selected Amazon Bedrock Runtime `ConverseStream` as Swallowtail's first true
SDK-native route. Promoted Contract 019 and froze the exact
`aws-sdk-bedrockruntime = 1.136.0` generated event and error boundary before
production network work.

## Evidence

- OpenAI Rust remains community-supported; Anthropic, Gemini, and xAI publish
  no provider-supported Rust client.
- Claude, Qwen, and Kimi SDKs use Python, TypeScript, Go, Node.js, provider
  binaries, or external processes.
- Kimi's current Rust agent is an experimental standalone Wire-mode binary,
  not an embeddable Rust library.
- ACP's replacement Rust SDK proposal remains Preview and overlaps the ACP v1
  transport already owned and proven by Swallowtail.
- AWS publishes and supports the official Rust SDK. Bedrock Runtime exposes
  typed `ConverseStream` EventStream and service errors in-process.
- AWS SDK defaults include ambient credential discovery and three attempts.
  Contract 019 requires explicit credential-provider, region, endpoint,
  timeout, and one-attempt configuration instead.

No provider credential, AWS configuration, account, endpoint, or paid model was
used.

## Changed

- added Research 013 with the current cross-provider comparison and ranked
  continuation sequence
- added Contract 019 for exact SDK identity, adapter-private executors, no
  ambient configuration, delegated SDK credentials, one attempt, joined work,
  and separate runtime/control-plane clients
- added fixture-first `swallowtail-adapter-bedrock` to the workspace
- pinned `aws-sdk-bedrockruntime = 1.136.0` with default runtime and network
  features disabled for this fixture batch
- added a manifest for endpoint audience, region source, cloud identity,
  cloud-account billing, provider support, exact route, one attempt, bounded
  projection, and exclusions
- added a generated-type stream decoder for ordered text, stop, and usage plus
  fail-closed missing, reordered, unknown, and unsupported semantics
- added safe generated service-error classification and an exact generated
  one-attempt retry configuration assertion

## Decisions

- SDK-native means the provider SDK is linked into the Rust adapter process.
  Process wrappers, FFI, containers, and language sidecars keep those identities.
- the first production route will use one exact delegated AWS credential
  provider configured on the selected execution host. The driver will not load
  the AWS default credential chain.
- Bedrock Runtime inference and Bedrock control-plane catalogue remain separate
  drivers and endpoint audiences.
- gateway identity, underlying model provider, model route, and model identity
  remain separate. No default model or region is selected.
- live AWS access remains separately gated.

## Validation

- six focused Bedrock SDK tests pass
- full `effigy qa` passes with 277 tests
- `effigy qa:docs` and `git diff --check` pass
- `effigy doctor` retains the inherited 19 oversized-file findings: 12 warnings
  and 7 errors; no new finding was added

## Remaining Risks

- the production SDK features add a private Tokio, HTTP, TLS, SigV4, and
  EventStream lifecycle that card 069 must join under host authority
- the SDK documents special automatic retries for model-not-ready failures;
  the production configuration must prove that one attempt still wins
- complete-stream deadline handling cannot rely only on the SDK operation
  timeout because post-response stream consumption has separate lifetime
- IAM permission, credential refresh, model access, region availability, rate,
  quota, and billing remain account-specific live evidence
- the separate Bedrock catalogue driver remains continuation work after the
  runtime proof

## Continuation Record

Card 069 is ready. Implement one exact Bedrock Runtime structured run through
the official SDK, then prove local and remote-authoritative hosted-direct
conformance without live AWS access. Bedrock catalogue, Kimi Rust Wire, and ACP
SDK rechecks remain in bounds after roadmap 020.
