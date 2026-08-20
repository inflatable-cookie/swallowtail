# Hosted Interactive OAuth

Status: deferred
Owner: Tom
Source: Contract 057; Research 169; `docs/logs/2026-08-20-g04-hosted-oauth-reassessment.md`

## Deferred Work

The library-max sign-in loop is realized. Hosted URL-open plus loopback is
not a realized consumer path. No production adapter registers `UrlOpen` or
`LoopbackCallback`, or calls `start_sign_in` with
`SignInMethod::InteractiveOauth`.

Do not reclassify Claude subscription, Codex ChatGPT cached login, or
other installed/delegated logins as that proof. Do not extract tokens or
invent an OAuth client Swallowtail does not own.

## Promotion Gate

Promote only when:

- a named production route actually performs hosted URL-open plus loopback
- secret extraction stays out of portable records
- current contracts can represent the loop without changing 047 `Ready` /
  `NotReady`
- the operator approves that route as the OAuth proof

There is no implied revisit date. Parking this is sequencing, not a
contract change.
