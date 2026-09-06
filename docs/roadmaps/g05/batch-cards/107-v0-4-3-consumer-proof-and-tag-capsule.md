# 107 v0.4.3 Consumer Proof And Tag Capsule

Status: complete; `v0.4.3` tagged at `cbd4ddc8`; source consumer passed; tag capsule relayed to Acowtancy
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Milestone: `../033-v0-4-3-release-readiness.md`
Depends on: ready when the operator-authorized `v0.4.3` tag exists

## Goal

Run the source consumer on the released `v0.4.3` tag, relay the tag capsule to Acowtancy for the Desktop repin and one real Send, flip the release note and Contract 036 identity, and attach the Desktop result as consumer evidence.

## Readiness

Ready on the condition in the status line; the manifest in the milestone
roadmap carries the full scope, owned paths, validation, and stop rules.

## Result

The annotated `v0.4.3` tag was created under operator direction from the
Bovine project. Its verified capsule is:

- Tag: `v0.4.3`
- Tag object: `d83004302801222258c3496791de1e3305571860`
- Peeled commit: `cbd4ddc8f9d6aa55bd947b92a55ea3a779582b79`
- Date: 2026-09-06
- Message: `v0.4.3: Claude SDK rate-limit notification compatibility and bounded termination diagnostics; source-only, no publication`

Chatterbox relayed this capsule to the Acowtancy coordinator now for the
Desktop exact-tag repin and one real Send. This closeout did not perform the
Desktop, provider, authentication, or consumer-repository action.

The required source-consumer proof ran first from the clean detached worktree
`/private/tmp/swallowtail-v043-root-consumer` at the peeled commit:

```text
effigy package:source-consumer
external source consumer passed at exact commit cbd4ddc8f9d6aa55bd947b92a55ea3a779582b79
```

The run resolved the four direct probes (`swallowtail-core`,
`swallowtail-runtime`, `swallowtail-host-local`, and
`swallowtail-adapter-codex`) to the exact tag commit. Retained command output
is `/private/tmp/card107-source-consumer-20260906.txt`, SHA-256
`d9ecc02873e0fd00dd7bfb03f30c8c150992d603c86705b742b73a751e6b7fa9`.

The Acowtancy coordinator owns the later Desktop result. No live provider
success, Desktop reply, or additional route diagnosis is claimed here.
