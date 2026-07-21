# xAI WebSocket Protocol Fixture

Date: 2026-07-20
Roadmap: 017
Card: 052

## Changed

- rechecked the official xAI WebSocket, Responses, authentication, and cost
  surfaces
- added fixture-first `swallowtail-adapter-xai` protocol evidence without a
  production driver surface
- froze the `/v1/responses` WebSocket upgrade, bearer shape, one exact model,
  `store=false`, serial text turns, and latest-response continuation
- added bounded ordered-event parsing, exact per-turn USD ticks, safe provider
  failures, correlation and output checks, and fail-closed unknown events
- added a deterministic loopback endpoint for handshake, turn, disconnect,
  cancellation-by-close, and concurrent-turn assertions

## Decisions

- the directory date is an evidence snapshot tied to the current guide, not a
  claimed xAI API version
- response ids remain inside the fixture conversation state; callers cannot
  inject or branch continuation ids
- terminal response usage is authoritative; any earlier cumulative snapshot
  is replaced, not summed
- cancellation closes the WebSocket and invalidates the session; no text
  cancellation frame, reconnect, retry, storage, or resume is invented
- event names and order use xAI's documented Responses-stream equivalence;
  synthetic fixtures are not presented as captured provider transcripts

## Evidence

- eight focused tests pass
- strict xAI crate clippy, formatting, and diff checks pass
- docs and Northstar checks pass; doctor retains the pre-existing 19 findings
  with no xAI file added
- all credentials, ids, prompts, outputs, and costs are synthetic
- no xAI credential, provider client, or external inference request was used

## Continuation

Card 053 is ready. Add only the Contract 016 provider-neutral records and the
production session driver needed by this manifest. Card 054 remains planned
and in bounds.
