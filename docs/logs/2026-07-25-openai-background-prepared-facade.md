# 2026-07-25 OpenAI Background Prepared Facade

Status: complete

## Changed

`swallowtail-adapter-openai` now exposes a prepared integration and one typed
background-run value for the public Responses API.

Preparation binds the exact public endpoint and audience, API-key pay-as-you-
go access, provider support authority, configured-instance revision, host
services, access provenance, dated background facade, and GPT-5.6 route.
Operation preparation requires a positive output bound, deadline, background
execution, temporary retention, and exactly one permitted stream
reattachment.

The bound `start_run` delegates to the unchanged low-level structured-run
driver. Create, cursor reattachment, bounded retrieve, and native cancel remain
management of one provider inference attempt.

## Current Evidence

The current OpenAI background-mode guide still documents:

- `background=true` for provider-owned asynchronous execution
- polling queued and in-progress responses through retrieve
- native background-response cancellation
- cursor reattachment only when the original response used `stream=true`
- temporary provider retention with `store=false`
- roughly ten minutes of temporary disk retention for ZDR background requests

Evidence: [OpenAI Background mode](https://developers.openai.com/api/docs/guides/background).
The public endpoint inventory also still includes Responses create, retrieve,
and cancel. No research or contract delta was required.

## Native Boundaries

- background mode is selected through a background-specific prepared method
- the full input constructor keeps execution, retention, and reattachment
  policy visible
- the named shortcut states temporary retention and one reattachment
- `store=false` is not durable storage and not a no-retention claim
- one response create remains one inference attempt
- reattachment and retrieve never recreate input or authorize retry
- provider cancellation remains confirmed, raced, or unconfirmed
- ChatGPT, Codex, subscription, community OAuth, retry, and credential fallback
  remain excluded
- no durable response binding or cross-process reattachment was added

## Validation

- OpenAI adapter lint passes with warnings denied
- 31 unit, protocol, driver, conformance, prepared-facade, realtime, and
  example tests pass
- the 23-crate public-API declaration baseline and docs QA pass
- prepared success passes under local and remote-authoritative hosts
- cancellation-race and deadline cases retain provider truth and release the
  credential after joined work
- attached execution, omitted retention, wrong reattachment bound, route
  drift, and model drift fail before endpoint or credential effects
- Effigy doctor retains the known 19 oversized-file findings: 12 warnings and
  seven errors; this batch added none

## Next

Card 030 adds the Anthropic Managed Agent prepared lifecycle. Cards 030-036
remain in the provider-wide facade, package-proof, and replacement-candidate
runway.
