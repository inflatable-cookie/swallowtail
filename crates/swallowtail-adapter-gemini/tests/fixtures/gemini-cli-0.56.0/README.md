# Gemini CLI 0.56.0 currentness corpus

This secret-free identity corpus freezes official npm
`@google/gemini-cli@0.56.0`, GitHub tag `v0.56.0`, and the darwin-arm64
unsigned release asset before Swallowtail widens the separate ACP and
headless claims.

The installed host is `0.53.0`; its npm bin link is unsigned and was not
changed. Published stable points after the existing ceilings are `0.53.0`,
`0.53.1`, `0.54.0`, `0.54.4`, `0.55.1`, and `0.56.0`. The first unpublished
later stable is `0.56.1`; `0.57.0-preview.0` is ignored.

Selected help output is byte-identical across the six published points.
ACP launch, initialize, session, callback, and retention sources are stable
through `0.56.0` except for provider-private invalid-stream categories added
in `0.53.1`. Headless invocation, stream-json event names and fields,
terminal result shape, and retention sources remain stable; the same
invalid-stream additions only refine provider error text and metadata.
Those additions stay unmapped. The historical ACP and headless decoder
corpora remain authoritative.

Both axes are compatible extensions:

- ACP keeps `gemini-cli.acp.v0.51.0`, baseline `0.51.0`, and raises its
  qualified ceiling to `0.56.0`.
- Headless keeps `gemini-cli.headless.stream-json.v1`, baseline `0.51.0`,
  and raises its qualified ceiling to `0.56.0`.

The selected access boundary is an enterprise-owned Gemini Developer API key.
Browser login, individual-account service, prompts, authenticated sessions,
live catalogues, host replacement, and account state are outside this corpus.

No fixture contains a credential, host path, account identity, provider
payload, real prompt, or real session id.
