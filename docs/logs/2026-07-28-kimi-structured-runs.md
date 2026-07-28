# Kimi Structured Runs

Date: 2026-07-28

## Changed

- added a separate Kimi headless descriptor, compatibility claim, discovery,
  stream-JSON process driver, prepared operation, and bounded corpus path
- added `StructuredRun` independently to the Kimi local-server driver
- projected one private local-server session and one prompt without exposing a
  reusable session or management binding
- required durable retention and preserved the Kimi thread on close
- retained callback, provider-cancellation, deadline, disconnect, and cleanup
  truth through the projection
- added one installed Kimi Code facade with explicit ACP or headless selection
- kept local-server preparation separate because endpoint, bearer, topology,
  and lifecycle authority differ
- documented prompt-in-process-arguments host visibility and the audited
  default-engine requirement

## Evidence

Headless fixtures cover exact argv and environment, assistant and tool
activity, retry, resume hint, malformed and incomplete output, native failure,
cancellation, deadline, force-stop, and joined cleanup under local and
remote-authoritative hosts.

Local-server fixtures cover attached and owned topology, one create and one
prompt, manual approval and question callbacks, reasoning, cancellation,
timeout, disconnect, durable retention mismatch before effects, no archive or
delete, and run-before-server cleanup ordering.

## Validation

- `cargo test -p swallowtail-adapter-kimi`: 75 passed; one gated live probe
  ignored
- strict Kimi Clippy and rustdoc pass
- all workspace examples compile
- route, lifecycle, 21-solution CSV, and docs checks pass
- the CSV has 45 columns, 21 provider-sorted rows, 25 route identities,
  18 structured `Yes`, and three `No`

## Next

Card 079 classifies llama.cpp owned as `Not applicable`, retains Gemini Live
and OpenAI Realtime as `No`, and runs provider-wide packaged closeout.
