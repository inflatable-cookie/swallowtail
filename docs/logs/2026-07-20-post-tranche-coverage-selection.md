# 2026-07-20 Post-Tranche Coverage Selection

## Outcome

Selected xAI Responses WebSocket as the next provider proof. It adds the first
connection-scoped direct-model interactive session and exact provider-billed
cost evidence. This is more architectural information than another ACP agent,
an SDK wrapper around a CLI, or owned lifecycle for the already attached
llama.cpp facade.

## Evidence

- xAI documents a provider-supported long-lived Responses WebSocket with serial
  turns, connection-local continuation, a 25-minute limit, explicit reconnect
  limits, and WebSocket-specific errors.
- xAI API keys are team, endpoint, and model scoped. Public API access remains
  separate from Grok product access.
- `cost_in_usd_ticks` is provider-declared exact charged cost per request, not a
  price estimate.
- current Kimi Code `0.28.0` is the strongest second ACP proof, but its safe
  baseline repeats Gemini; its novel load/resume and write surfaces need a
  separate persistent-session and callback-authority contract.
- Qwen and Kimi SDK candidates currently require non-Rust runtimes or an
  external CLI. They do not prove a Rust in-process SDK lifecycle.
- Monkey already owns `monkey serve` and `monkey up` lifecycle. A future
  Swallowtail Monkey route attaches; it does not take over model serving.

No provider credential was read and no authenticated or external inference
request was made.

## Promotion

- Research 005 records the current comparison and sequence.
- Contract 016 fixes resource-free direct sessions, session-bound endpoint and
  credential leases, connection-local continuation, cancellation-by-close, no
  implicit recovery, and provider-billed-cost evidence.
- Roadmap 017 and cards 052-054 compile the xAI fixture, driver, and conformance
  lane inside g01.

## Validation

- `effigy qa:docs` passed.
- `effigy qa:northstar` passed.
- `git diff --check` passed.

## Remaining Risks

- xAI does not expose a separate version header for this WebSocket surface; the
  fixture must name its evidence snapshot and fail closed on semantic drift.
- the documented text Responses WebSocket has no native turn-cancel message;
  cancellation ends the socket and session.
- live entitlement, ACL, rate, model availability, billing, and ZDR state are
  account-specific and remain outside deterministic QA.
- Kimi login delegation, persistent ACP replay, filesystem write callbacks,
  and owned serving remain later contract work.

## Continuation Record

The roadmap front door now points to card 052: freeze the xAI Responses
WebSocket protocol and deterministic fake endpoint without a live credential.
