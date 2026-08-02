# 2026-08-02 Codex Thread Import Acceptance

## Result

Card 054 and roadmap g03.020 are complete. Codex is the first production route
to pass the complete provider-session catalogue, explicit import, load, and
resume chain.

## Evidence

The Codex suite invokes the provider-neutral import contract, then exercises
the production prepared facade under local and remote-authoritative execution
host identities. Exact host, resource, cursor, candidate, and imported binding
identity remains stable across both topologies.

After-dispatch cancellation and deadlines stop and join held catalogue work.
List disconnect, read disconnect, and cleanup failure remain distinct typed
failures and issue no binding. Existing new, load, resume, replay, and
management regressions remain in the focused package run.

The Codex guide and compile-tested example now keep browse, select, import,
load, and resume separate. They state the exact qualified segments and exclude
consumer persistence, synchronization, prompts, and lifecycle authority.

## Validation

- `effigy validate:focused swallowtail-adapter-codex swallowtail-testkit`
  passed 239 tests
- `effigy package:verify-affected swallowtail-adapter-codex` passed
- `effigy qa:docs` passed
- no live provider, consumer, or broad workspace suite ran

## Next

Execute card 055. Freeze stable ACP `session/list` and add its bounded common
codec before qualifying any production ACP agent.
