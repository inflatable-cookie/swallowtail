# Claude Agent SDK native proof fixture

Frozen identities and selected-flow evidence for the317 control and318/319/321/322
pairs on Linux x64 and Darwin arm64. The separate
[trusted-original qualification](../../../../../docs/knowledge/operations/claude-agent-sdk-trusted-qualification.md)
records actual original SDK/native and shipped-sidecar results through322.
These data and cooperative fakes alone grant no qualification.

- `probe-plan.json` and its schema retain exact artifact digests, selected options
  and the historical disabled-original plan. Its bytes are frozen; current test
  authority is Tom’s decision1ec44a26 v2, exercised by the trusted runner.
- `selected-flow.json` binds per-hop wrapper classification to exact SDK bytes.
- `probe-admission.json`, `probe-options.json` and `probe-execution.sha256` bind
  the cooperative fake runner. The admission stays fake-only, pins its source
  and peer identities and refuses original mode. Independently qualified
  production pairs are selected by the maintained product map.
- `claude_agent_sdk_native_proof_harness.rs` exercises the actual prepared driver
  and shipped sidecar with fake SDK/native peers, checking permissions,
  projection, identity refusal and joined cleanup. The trusted runner supplies
  original credit. Obsolete custom-host machinery is preserved by exact Git
  [provenance](../../../../../docs/knowledge/operations/claude-agent-sdk-prototype-retirement.md).

Focused checks:

```sh
effigy validate:claude-agent-sdk-native-proof-harness
effigy validate:claude-agent-sdk-probe-admission
effigy validate:claude-agent-sdk-trusted-native
```

The last selector defaults to owned fakes locally. Frozen original execution
requires the authorized disposable CI path; it does not run on the persistent Mac.

Reproduce wrapper classification with `effigy validate:claude-agent-sdk-selected-flow`
and `effigy check:claude-agent-sdk-selected-flow`. The classifier accepts only
exact frozen extracted trees as inert input; it never downloads or executes them.
