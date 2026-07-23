# DeepSeek Continuation Records And Corpus

Date: 2026-07-22
Status: completed
Card: 103

## Changed

- added provider-neutral locally continued direct-session policy, bounds,
  exact-model preflight, and provider-cache posture
- added redacted attempt, tool-call, tool-result, continuation-binding, and
  pure explicit-authorization records
- added the twelfth Contract 011 synthetic profile without changing the prior
  eleven profile claims
- extended compatible-chat messages only for bounded structural null content
  and per-message extensions
- added a fixture-only DeepSeek adapter boundary with the exact dated facade
  claim and V4 Pro offline corpus

## Boundary

The consumer still selects and executes tools. Swallowtail produces another
provider-attempt authorization only after a user turn or all exact correlated
tool results. Harness callback exchange is not reused.

Provider-private reasoning bytes live only in the adapter-private protocol
module, use bounded zeroizing memory, and are replayed only while constructing
the next exact request. Shared records expose redacted binding and byte-count
evidence, not readable reasoning, serialization, or resume state.

The corpus fixes one buffered tool-bearing attempt, streaming final attempts,
later-turn continuation, cache hit and miss usage, status classes, provider
error inside SSE, local cancellation limits, host-monotonic deadline posture,
disconnect, model mismatch, unknown semantic fields, and exact opaque facade
revision rejection. It makes no network, credential, account, cache-entry,
retry, or paid inference request.

## Validation

- 148 focused tests pass across core, runtime, testkit, compatible chat, and
  DeepSeek
- focused warnings-denied clippy passes
- full repository QA passes; the final inventory is 482 tests, with 479
  runnable and three separately gated ignored probes
- a post-QA module split passes 96 focused runtime and testkit tests plus
  warnings-denied clippy
- doctor retains the inherited 19 oversized-file findings

## Next

Card 104 is ready: implement the separately registered production driver,
prove both host identities and the full failure matrix, then close roadmap 036.
