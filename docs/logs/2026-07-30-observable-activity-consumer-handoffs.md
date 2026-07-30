# Observable-Activity Consumer Handoffs

Date: 2026-07-30
Status: completed

## Result

Card 137 and roadmap g02.040 are complete.

- Nucleus has a bounded adoption handoff over public prepared evidence and the
  existing runtime event stream
- assistant messages and work activity remain separate consumer-owned durable
  projections
- runtime turn and activity identity, exact lifecycle, disclosure, content
  stream, provider label, and callback or tool correlation remain available
- grouping and collapsed presentation remain Nucleus view policy
- provider-unspecified assistant messages and namespaced unknown activity are
  not overclassified
- reasoning is named only as a provider-intended readable summary
- Soundcheck may ignore every activity observation without losing final output
- optional Soundcheck progress uses portable kinds and exact lifecycle, not
  provider-native event names
- consumer thread lifecycle, authorization, review, persistence, retention,
  analytics, and UI remain downstream

No Nucleus or Soundcheck source changed. No provider, package, candidate,
publication, tag, push, or release state changed.

## Public Proof

Two examples compile against only `swallowtail-runtime`:

- `observable_activity_nucleus`
- `observable_activity_soundcheck`

Focused command:

```text
cargo check -p swallowtail-runtime \
  --example observable_activity_nucleus \
  --example observable_activity_soundcheck \
  --locked
```

The check passed in 18.51 seconds after stale full-workspace `nextest`
processes from the previously interrupted broad validation were terminated.
No broad workspace test run was repeated.

Card 136's immediately preceding package API evidence remains authoritative:
card 137 adds examples and documentation and changes no public library item.

## Programme State

Roadmaps g02.035-g02.040 and cards 119-137 are complete. The observable-agent-
activity programme has no unexplained prepared-profile gap and no implicit
provider or consumer queue.

## Next Task

The operator decides whether to authorize Nucleus or Soundcheck observable-
activity adoption or select another g02 stabilization target. No numbered card
is ready and no consumer edit is implicit.
