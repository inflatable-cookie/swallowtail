# 2026-07-31 Grok Build Range Claim And Conformance

## Changed

- extended the maintained Grok Build ACP claim from exact `0.2.114` through
  exact `0.2.117`
- retained the existing behavior through `0.2.116` and introduced one private
  task-control behavior revision at `0.2.117`
- required the exact source revision for every guaranteed executable point
- covered both behavior segments, later unverified execution, revision
  mismatch, ACP initialization, prepared latest evidence, and route lifecycle

## Current State

All 24 focused Grok tests pass. The extracted 44-file package compiles.
`0.2.118+` remains permitted but visibly unverified and inherits the latest
private behavior mapping. No public task-control authority was added.

## Next

Execute card 032. Prove installed exact `0.2.117` through host-approved
discovery, reconcile public route truth, and close g03.012.
