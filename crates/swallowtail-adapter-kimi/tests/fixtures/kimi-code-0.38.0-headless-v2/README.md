# Kimi Code 0.38.0 headless v2 corpus

Secret-free identity and decoder corpus for exact official npm
`@moonshot-ai/kimi-code@0.38.0` default agent-core-v2 `runV2Print` stream-json.

Naked `kimi -p --output-format stream-json` at `0.38.0` dispatches to
`runV2Print` unless `KIMI_CODE_LEGACY_FLAG` is truthy. Swallowtail does not set
that flag. The v2 runner reuses the shared `PromptJsonWriter` from
`prompt-render.ts` (byte-identical to `0.37.2`) but prepends a
`system.version` meta line before turn output.

Legacy v1 print blobs through `0.38.0` remain frozen in the historical
`kimi-code-0.38.0` identity corpus only. Qualified headless `0.29.0..=0.37.2`
stays on `kimi.headless.stream-json.v1`. Exact `0.38.0` qualifies under
`kimi.headless.stream-json.v2`.

Synthetic JSONL fixtures exercise the Swallowtail parser. They are not official
provider emissions. Source-proved shapes are listed in `protocol.json`.

No credential, bearer token, host path, account identity, provider payload, or
live session id appears in this directory.
