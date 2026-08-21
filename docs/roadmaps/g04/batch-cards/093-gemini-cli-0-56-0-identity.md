# 093 Gemini CLI 0.56.0 Identity

Status: completed
Owner: Tom
Milestone: [g04.034 Gemini CLI 0.56.0 Useful Newer](../034-gemini-cli-0-56-0-useful-newer.md)
Created: 2026-08-22

## Task

Freeze official `@google/gemini-cli@0.56.0` identity evidence and classify the
separate ACP and headless segment shapes. Do not edit production claims in this
card.

## Method

1. Re-probe npm `latest` and GitHub release `v0.56.0`; stop if the official
   stable point moved
2. Record host `gemini 0.53.0`, executable digest, size, and publisher evidence
   without changing the host
3. Download official npm and GitHub assets to `/tmp`; record package, asset,
   and extracted executable/source digests
4. Enumerate the published stable points after the existing ceilings:
   `0.53.0`, `0.53.1`, `0.54.0`, `0.54.4`, `0.55.1`, and `0.56.0`; keep any
   independently unqualified or withdrawn point incompatible
5. Compare the selected ACP launch/initialize/session/callback subset and the
   selected headless invocation/event/terminal/retention subset against the
   frozen `0.51.0` and `0.51.0..=0.52.0` evidence
6. Classify every addition as mapped, unmapped, auth-only, or argv0/help noise;
   do not map it in this card
7. Freeze secret-free `identity.json`, `protocol.json`, and `README.md` under
   `crates/swallowtail-adapter-gemini/tests/fixtures/gemini-cli-0.56.0/`
8. Write promoted Research 182 and name the segment shape separately for ACP
   and headless

Do not invoke a model, send a prompt, authenticate, install, update, or inspect
account state. The selected access boundary is the existing API-key profile
with an enterprise-owned key; browser login and individual-account service are
not qualification evidence.

## Expected Shape

Compatible-extension on both axes unless selected mapped behavior differs. A
private milestone is allowed only when adapter-private mapping changed without
changing the public lifecycle. A public lifecycle change, auth-only evidence,
or a required new public operation is a stop.

## Acceptance

- Exact host and official identity evidence is frozen without secrets
- Published intermediates and the first synthetic later stable are named
- ACP and headless selected protocol comparisons are explicit and separate
- Research 182 is promoted and indexed
- Each axis is classified compatible-extension, private-milestone, or stop
- Production claims, current matrices, architecture, and changelog are
  unchanged
- `effigy validate:focused swallowtail-adapter-gemini` passes
- `effigy qa:northstar` passes
- `git diff --check` passes

## Result

Research 182 and the secret-free `0.56.0` identity/protocol corpus are
promoted. Official npm and GitHub identity stayed at `0.56.0`; host `0.53.0`
was not changed. The selected ACP and headless surfaces are compatible
extensions with separate behavior decisions. The only selected source deltas
are provider-private invalid-stream classifications and error guidance; they
remain unmapped. Production claims remain unchanged pending card 094.

Auto-continue to card 094 only when neither axis is a stop.

## Stop Conditions

- Official stable moves during the run
- Exact artifact identity cannot be corroborated
- Selected ACP or headless behavior cannot be compared without a provider
  prompt or live authentication
- Browser login or individual-account access is needed to justify the route
- Either public lifecycle changed materially or a new public operation is
  required
- Evidence would flatten Gemini CLI onto Gemini Live or Gemini Models

On stop, leave both claims unchanged and return for an explicit keep-or-remove
decision on Gemini CLI.

## Out Of Scope

- Production claim or matrix edits
- Mapping unused flags, auth modes, fields, or features
- Provider work, live probes, install, update, or account inspection
- Gemini Live, Gemini Models, or another family
- Per-route feature completion
