# 2026-08-19 Route Readiness Facade

Status: promoted
Owner: Tom
Lane: g04

## Context

Operator asked to close g03, open g04, and shape a general route-availability
facade that can drive Poodle-style connection pickers and T3 Code-style
admitted connection lists. Screenshots are canned Poodle specimens plus a live
T3 Code provider list.

Swallowtail should supply library components, not a server. More shared
mechanism is better if it does not import product policy.

## Surfaces In The Evidence

1. Addable-route picker grouped Hosted / Installed / Local runtime, with
   Available, Unavailable, and Unsupported.
2. Add-connection auth: API-key fields or "Sign in with browser" plus wait
   state.
3. Admitted instance list: multiple instances of one provider, instance
   labels, enable/disable as host preference, readiness chips.
4. Expanded instance: model list with default, hide, reorder, and mixed
   gateway entries.
5. T3 Code extras: version, update affordance, authenticated-as identity,
   display name, accent color, env vars, binary path, API endpoint, model
   favourite/reorder/hide.

## Existing Swallowtail Coverage

- Contract 008 already has discovery, configured instances, access dimensions,
  and sign-in actions.
- Contract 006 already separates credential, entitlement, endpoint, and
  runtime readiness, and says Swallowtail may expose safe auth status and
  credential requirements.
- Contract 032 covers installed executable observation.
- Contract 020 covers model catalogues bound to one instance.
- Contract 037 covers prepared facades after a connection exists.
- Contract 047 covers consumer-assembled snapshots of already configured
  instances. It is not an add-connection facade, has no watcher or refresh,
  and forbids account identifiers.

The gap is the pre-session lifecycle: what can be added, how it is admitted,
how auth runs, and how readiness is refreshed.

## Recommendations, Not Decisions

- Keep 047 as the ready-to-select snapshot. Add a lifecycle facade in front of
  it.
- Swallowtail describes credential fields and sign-in actions; the host stores
  secrets and places the browser.
- Display name, color, favourites, hide/reorder, and enable/disable stay
  consumer overlays.
- Do not flatten gateway models from one provider into another connection.
- Reuse Contract 029 for updates instead of a second currentness system.

## Operator Answers (2026-08-19)

1. T3 shows account email, blurred until clicked. Worth supporting on the
   connection facade, not as a 047 selection field.
2. Sign-in: as much as a library can do.
3. Persistence: Swallowtail interface, maybe a simple adapter, consumers may
   supply complex stores.
4. Overlay: operator unsure; planning uses the bound-overlay recommendation.

Also: cut a current-source tag before building the facade.

## Disposition

Promoted into Contract 057. Authenticated subject, library-max sign-in,
persistence port, overlay, the pre-facade tag, and the named seam amendments
are no longer open product questions. `v0.3.3` is tagged at `51d18620`. Spec
011 is archived. g04.005 is merged (PR 4, `c64bd64d`). g04.006 is merged (PR 5,
`5cdffebb`). g04.007 is merged (PR 6, `91e14e3d`). g04.012 is merged (PR 10,
`33a35749`). g04.013 is merged (PR 11, `e23e795f`). g04.014 is merged (PR 12,
`7810453f`). The first-proof-plus-consumer-path goal is complete. Hosted OAuth
stays a remaining gate. 2026-08-20 reassessment: no production adapter does
hosted URL-open plus loopback. Claude/Codex/Grok/Kimi/Cursor/Antigravity are
installed or delegated logins; hosted routes are API keys. Do not reclassify
those as the 057 hosted-OAuth first-proof. Second-proof inventory (Research
170) names DeepSeek continuation, Claude Agent ACP subscription-only, and
llama.cpp attached. g04.016 DeepSeek cards 045-047 are ready. Optional 047
overlay presentation metadata stays later and must not change `Ready` /
`NotReady`.
