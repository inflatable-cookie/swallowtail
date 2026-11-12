# Claude Agent SDK native proof fixture

Evidence and the executable plan for the native behavior proof that Q-009 item
1 names. Nothing here qualifies a claim. The package and native axes stay
`0.3.284..=0.3.317` and `2.1.284..=2.1.317`.

- `probe-plan.json` — the reviewable plan: exact SDK/native artifacts and
  digests for the Darwin arm64 and Linux x64 builds of `2.1.318`, `2.1.319`,
  `2.1.321` and `2.1.322`, the selected native argv, the isolation, environment
  and egress rules, the bounds, the observations, the failure classes and the
  cleanup steps. Original-artifact execution is `enabled: false`.
- `probe-plan.schema.json` — the structural shape of that plan.
- `selected-flow.json` — the bounded wrapper classification produced by
  `scripts/claude-agent-sdk-selected-flow.py` from the exact tarballs named in
  `../claude-agent-sdk-0.3.322/identity.json`. It records the resolved entry
  point, the import closure and the masked digests of every selected surface
  across the accepted base and each published hop.

## Why the plan exists before the authority

Running a vendor binary needs the operator's exact authority and a brief
amendment. Building the plan and proving the harness first means the authorised
run has nothing left to invent: the artifact digests, the argv, the observed
facts, the bounds and the cleanup are already fixed and tested against fake
peers.

The harness that consumes this plan is
`crates/swallowtail-adapter-claude-agent/tests/claude_agent_sdk_native_proof_harness.rs`
with its peers in `../native_proof_support/`. It launches the shipped sidecar
under real Node against a fake SDK module that speaks the native stdio wire, a
fake native process and a loopback endpoint. `effigy
validate:claude-agent-sdk-native-proof-harness` runs it.

## Admission caveat

Both the prepared driver and the shipped sidecar admit only the maintained pair
map, so every planned hop above `0.3.317` is refused with `sdk_version_mismatch`
before the SDK loads. The harness therefore proves the pipeline on the accepted
`0.3.317`/`2.1.317` pair and proves the refusal for the newer hops. An
authorised original run for a newer hop needs an explicit probe admission, named
in `probe_admission` in the plan; the harness never widens the map itself.

## Reproducing the wrapper classification

```sh
effigy validate:claude-agent-sdk-selected-flow
effigy check:claude-agent-sdk-selected-flow
```

With extracted package trees in a scratch directory, the same script re-derives
`selected-flow.json` and refuses a tree whose bytes do not match the committed
per-file ledger:

```sh
python3 scripts/claude-agent-sdk-selected-flow.py --corpus <dir> --json <out>
```

It never downloads, installs or executes anything.
