# g04.019 Local llama.cpp Attached

Date: 2026-08-20
Roadmap: `../roadmaps/g04/019-local-llama-cpp-attached.md`
Cards: `../roadmaps/g04/batch-cards/053-llama-cpp-attached-addable-descriptor.md`,
`../roadmaps/g04/batch-cards/054-llama-cpp-attached-admission-and-prepare.md`,
`../roadmaps/g04/batch-cards/055-llama-cpp-attached-refresh-update-and-subject.md`

## Result

`swallowtail-adapter-llama-cpp` exports a local-runtime
`AddableRouteDescriptor` for `llama-cpp.attached`. Topology is
local-runtime, not `ExecutionLayer`. Config field is an opaque `endpoint`.
The row advertises no credential field, does not probe `/health`, and does
not advertise `llama-cpp.owned`.

Admission writes `AdmittedInstanceRecord` through the 057 store with no
secret bytes and no `CredentialRef`. Missing URL-open, loopback, and
device-code ports do not fail this path. `prepare_llama_cpp_attached` still
prepares after admission with `llama_cpp_attached_access_profile` and a
host `InstanceTargetRef`. Exact opaque b9910/f5525f7e7 stays prepare-time.

Refresh writes host-supplied `AccessStatus` without changing enablement.
Authenticated subject stays Absent. Update observation reuses
`llama_cpp_attached_runtime_claim`; 032 stays unobserved. Overlay does not
invent a llama.cpp catalogue `provider_id`. Catalogue rows stay unmarked.
`public-api-0.3.3` is unchanged. Additive API is in
`public-api-unreleased`.

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-733f457f`
Worker branch: `t3code/llama-cpp-attached-route`

Validation:

- card 053: `effigy validate:focused swallowtail-adapter-llama-cpp swallowtail-runtime`, `git diff --check`
- card 054: `effigy validate:focused swallowtail-adapter-llama-cpp swallowtail-runtime swallowtail-host-local`, `git diff --check`
- card 055: `effigy validate:focused swallowtail-adapter-llama-cpp swallowtail-runtime swallowtail-testkit`, `git diff --check`
- `effigy package:api`

PR: pending

## Next

Await review. Do not merge without operator authorisation. Hosted OAuth
stays a remaining gate.
