# Codex Request-ID Canonicalization

Date: 2026-07-31

## Outcome

Swallowtail now accepts both Codex app-server `RequestId` representations at
callback admission and `serverRequest/resolved`: string and signed `int64`.
The same strict canonicalizer rejects null, boolean, array, object,
floating-point, and out-of-range integer shapes.

One private typed lookup key preserves string versus integer wire identity.
The existing opaque portable reference remains the raw string or decimal
integer text. The callback hub continues retaining the original JSON value, so
a numeric provider request receives a numeric JSON-RPC response.

## Consumer Evidence

Nucleus reproduced the defect with Codex CLI `0.146.0`, ChatGPT login,
`gpt-5.4-mini`, low reasoning, and Plan mode. Its typed single-choice question
was answered once, then the legal numeric resolution failed as malformed.
Nucleus added no provider parser or workaround and stopped before retry.

This repair stays in Swallowtail. Nucleus can update its local path dependency
and rerun native card 026 from a fresh isolated state root.

## Regression Evidence

- frozen app-server corpus covers string `"900"` and integer `900`
- private request keys and activity ids remain distinct for that same lexical
  value
- each resolution completes its matching started activity and opaque
  correlation
- duplicate or unmatched legal resolution emits no activity
- invalid request-ID shapes retain the safe malformed-notification class
- full numeric typed-question fixture returns one numeric JSON-RPC response,
  emits matching request start and completion, and completes the turn
- `effigy validate:focused swallowtail-adapter-codex`: 137 passed
- `effigy package:verify-affected swallowtail-adapter-codex`: passed

## Boundaries

- Contracts 014, 041, and 044 already governed the fix; no contract changed.
- No callback, activity, provider, or consumer vocabulary changed.
- No consumer repository changed.
- No executable installation, authentication, model call, live provider test,
  publication, or other provider effect ran.

## Follow-On

Roadmap g03.004 completed the portable contract extension immediately after
this repair. `ProviderRequestRef` now retains text versus signed-integer
representation, and Codex uses it in place of the private typed key. The
original JSON value remains adapter-private response authority.

## Next

Resume g03.002 at card 004. The Claude/Gemini corpus scope and sequence are
unchanged.
