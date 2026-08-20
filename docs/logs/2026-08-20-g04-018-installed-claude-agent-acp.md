# g04.018 Installed Claude Agent ACP

Date: 2026-08-20
Roadmap: `../roadmaps/g04/018-installed-claude-agent-acp.md`
Cards: `../roadmaps/g04/batch-cards/050-claude-agent-acp-addable-descriptor.md`,
`../roadmaps/g04/batch-cards/051-claude-agent-acp-admission-and-prepare.md`,
`../roadmaps/g04/batch-cards/052-claude-agent-acp-refresh-update-and-subject.md`

## Result

`swallowtail-adapter-claude-agent` exports an installed
`AddableRouteDescriptor` for `claude-agent.acp`. Topology is installed, not
`ExecutionLayer`. Config fields are opaque `binary_path` and `environment`.
The local subscription row advertises no credential field and does not
advertise `claude-code.headless` or `claude-code.response-only`.

Admission writes `AdmittedInstanceRecord` through the 057 store with no
secret bytes and no `CredentialRef`. Missing URL-open, loopback, and
device-code ports do not fail this path. `prepare_claude_agent` still
prepares after admission with `LocalUnauthenticated` +
`SubscriptionAllowance`.

Refresh writes host-supplied `AccessStatus` without changing enablement.
Authenticated subject stays Absent. Update observation reuses
`claude_agent_acp_claim` and optional 032 evidence. Overlay does not invent
a Claude Agent catalogue `provider_id`. Session-negotiated models stay
unmarked. `public-api-0.3.3` is unchanged. Additive API is in
`public-api-unreleased`.

Worker worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-ee8f5694`
Worker branch: `t3code/read-claude-agent-acp-handoff`

Validation:

- card 050: `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-runtime`, `git diff --check`
- card 051: `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-runtime swallowtail-host-local`, `git diff --check`
- card 052: `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-runtime swallowtail-testkit`, `git diff --check`
- `effigy package:api`
- `effigy check:examples`

PR: https://github.com/inflatable-cookie/swallowtail/pull/15

## Next

Await review. Do not merge without operator authorisation. Hosted OAuth
stays a remaining gate. Compile llama.cpp attached only after this proof
lands.
