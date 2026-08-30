# 2026-08-30 g05.003 Claude Watcher Post-Bridge Reassessment

Status: complete
Owner: Tom
Date: 2026-08-30

## Result

Contract 060 and card 016 close the provider-neutral HTTP bridge prerequisite.
The installed Claude Code and npm `latest` are exact `2.1.251`; both existing
Claude Code route claims still end at `2.1.241`. npm published `2.1.242`,
`2.1.243`, `2.1.245`, `2.1.246`, `2.1.247`, `2.1.248`, `2.1.250`, and
`2.1.251`; `2.1.244` and `2.1.249` are unpublished gaps.

The former card ordering was circular: card 010 depended on a closed live
same-turn gate, while card 011 after card 010 owned that live proof. The
reassessed sequence is:

1. g05.005 cards 017-018 qualify the existing headless and response-only route
   axes through current `2.1.251`, without watcher mapping or provider work;
2. card 010 later owns credential-free Claude watcher binding and deterministic
   provider fixtures, without a support claim;
3. card 011 remains separately gated on operator-authorized live same-turn
   acceptance, then publishes only the exact proved watcher claim.

## Boundaries

This checkpoint does not start Claude wiring, run a provider prompt, authorize
credentials or paid work, advertise watcher support, reopen skill cards 005-006,
or promote the open consumer route-feature projection triage. Card 011 may
update existing watcher activity and route documentation after live acceptance;
the cohesive route-feature facade remains separate planning.

## Next

Execute g05.005 cards 017-018 as one serial currentness worker lane. After that
PR lands, reassess card 010. Live provider work remains separately authorized.
