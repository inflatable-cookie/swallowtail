# Provider Request Reference Representation

Date: 2026-07-31

## Outcome

`ProviderRequestRef` now preserves `Text` or `SignedInteger` representation as
typed portable metadata beside its opaque canonical value. Equal visible forms
compare, order, and hash as distinct references. Default formatting continues
to redact the value.

Codex app-server uses this common reference directly at callback admission,
activity start, and `serverRequest/resolved`. Its earlier private typed lookup
key is gone. The callback hub still retains the original JSON value, so integer
request ids receive integer JSON-RPC responses.

## Contract

Contracts 014, 041, and 044 now state that qualified scalar representation is
part of portable provider-request correlation. This is provider-neutral
identity metadata, not JSON-RPC payload exposure or consumer wire authority.

Existing `ProviderRequestRef::new` callers remain text. The other adapters did
not change and infer no new representation.

## Regression Evidence

- core coverage proves text `"900"` and signed integer `900` retain equal
  canonical display values but distinct equality and hash identity
- both forms remain redacted by default
- frozen Codex corpus starts and resolves both forms to their matching distinct
  activities
- the full numeric typed-question fixture exposes `SignedInteger`, answers once,
  repeats numeric JSON-RPC id `900`, and reaches normal completion
- invalid and unmatched request-id behavior remains unchanged
- `effigy validate:focused swallowtail-core swallowtail-runtime
  swallowtail-adapter-codex`: 293 passed
- `effigy package:verify-affected swallowtail-core swallowtail-runtime
  swallowtail-adapter-codex`: passed
- `effigy package:api`: passed after recording the intentional core declaration
  delta

## Boundaries

- No consumer repository changed.
- No provider-native payload, callback vocabulary, or persistence codec was
  added.
- No executable installation, authentication, model call, live provider test,
  publication, or other provider effect ran.

## Next

Resume g03.002 at card 004. The Claude/Gemini corpus scope and sequence are
unchanged.
