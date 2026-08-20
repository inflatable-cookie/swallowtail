# g04.016 Hosted API-Key DeepSeek Continuation

Date: 2026-08-20
Roadmap: `../roadmaps/g04/016-hosted-api-key-deepseek-continuation.md`
Cards: `../roadmaps/g04/batch-cards/045-deepseek-continuation-addable-descriptor.md`,
`../roadmaps/g04/batch-cards/046-deepseek-continuation-admission-and-api-key.md`,
`../roadmaps/g04/batch-cards/047-deepseek-continuation-refresh-subject-and-047.md`

## Result

`swallowtail-adapter-deepseek` exports a hosted `AddableRouteDescriptor` for
`deepseek.continuation`. Topology is hosted, not `ExecutionLayer`. The
secret API-key field has no environment name. The endpoint config is an
opaque `ApiEndpoint` field id.

Admission writes `AdmittedInstanceRecord` through the 057 store. API-key
collection completes without URL-open, loopback, or device-code ports and
stores `CredentialRef` only. `prepare_deepseek_direct` still prepares after
admission with a host `InstanceTargetRef`.

Refresh writes host-supplied `AccessStatus` without changing enablement.
Authenticated subject stays Absent. Overlay keys `deepseek` catalogue rows
without changing 047 `Ready` / `NotReady`. `public-api-0.3.3` is unchanged.
Additive API is in `public-api-unreleased`.

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-44e77f3c`
Worker branch: `t3code/hosted-deepseek-api-key`

Validation:

- card 045: `effigy validate:focused swallowtail-adapter-deepseek swallowtail-runtime`, `git diff --check`
- card 046: `effigy validate:focused swallowtail-adapter-deepseek swallowtail-runtime swallowtail-host-local`, `git diff --check`
- card 047: `effigy validate:focused swallowtail-adapter-deepseek swallowtail-runtime swallowtail-testkit`, `git diff --check`
- `effigy package:api`
- `effigy check:examples`

PR: https://github.com/inflatable-cookie/swallowtail/pull/13

## Next

Await review. Do not merge without operator authorisation. Hosted OAuth
stays a remaining gate. Compile Claude Agent ACP or llama.cpp attached
only after this proof lands.
