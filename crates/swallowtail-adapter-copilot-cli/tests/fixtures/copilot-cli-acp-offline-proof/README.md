# Copilot CLI ACP offline artifact proof

`artifact-inventory.json` freezes the official npm wrapper and Darwin ARM64
native package trees for `@github/copilot` `1.0.80`, `1.0.81`, and `1.0.93`.
It records registry integrity, archive digests, every extracted file's digest,
size and mode, and the observed stable tags. The archives and executables are
not committed, installed, or copied into the host CLI location.

`execution-record.json` is the secret-free record. The task scratch path is
deliberately omitted. It records that fake containment passed before exact
artifact execution, including denied loopback/external networking, denied
host-home reads and keychain access, record-before-process ordering, fake ACP
permission cancellation, and absence of a tool effect. Exact artifact
invocations used only a synthetic auth placeholder and the `copilot --acp
--stdio` entry point under the same deny-network profile.

The harness allows twenty seconds for each ACP response and retains sanitized
JSON-RPC error evidence. All three exact artifacts initialized, reported their
frozen versions, and advertised `copilot-login`. Each returned `-32000`
`Authentication required` from `session/new` under the synthetic token
placeholder. Provider-supplied error data is omitted. The no-network task did
not attempt authentication, so no session, prompt, permission request,
cancellation exchange, or tool call occurred. The record keeps permission
proof false and does not support moving any production claim.

To reacquire the artifacts, run `python3 scripts/stage-copilot-acp-offline-artifacts.py`
and retain its output in fresh task-owned scratch. It verifies pinned public
registry metadata and inventories each extracted tree without installing a
package. Use a new task-owned scratch record for the harness `--prepare`
preflight before any exact execution; pass the staging helper's `artifact_root`
to `scripts/copilot-acp-offline-proof.py --execute` only after reviewing the
record. A matching `Authentication required` response is the stop condition
when running without auth or network. The committed execution record describes
the completed run; do not repeat the exact binary attempts as part of routine
validation.

The authenticated-proof harness also runs a separate fake-only pre-prompt
discovery path. It accepts initialize, authenticate, and session creation,
records only synthetic model/configuration identifiers, sends no prompt, then
closes and joins the child. Its negative controls reject prompt and
permission/tool callbacks, unapproved origins, wrong account or keychain
service, excess invocation/time, auth/config writes, and host-home/repository
escape. This path does not execute or authorize any frozen artifact and does
not change the disabled status in `authenticated-proof-plan.json`.

## Host-login permission proof attempt

`permission-proof-plan.json` binds the separately approved host-login/Auto
boundaries, exact artifact identities, default-deny official-destination proxy,
one invocation and one prompt per version, and the shared three-invocation
ceiling. `permission-proof-preflight-record.json` captures the fake-only
containment and permission-cancellation controls before original start.

`permission-proof-execution-record.json` is the secret-free original attempt.
It binds the harness digest and a fsynced pre-execution record. The
`sandbox-exec` launcher for exact `1.0.93` returned code 1 before ACP
`initialize`; the record does not establish whether the vendor binary reached
startup. No egress-proxy destination, ACP authentication request, session,
prompt, permission, model identity or effect was observed. Stderr was
discarded, so the cause is unknown. The one invocation was consumed, and the
harness stopped without starting `1.0.81` or `1.0.80`. This is not permission
evidence or a qualification change. Any new `1.0.93` invocation requires
separate authority.

Before staging artifacts or starting an original, the harness validates this
committed record against its pinned SHA-256 and refuses every version whose
invocation is marked consumed. Missing, changed or unsafe record paths fail
closed. Both `--permission-proof` and legacy `--execute` use this guard. The
fake self-test covers refusal of consumed `1.0.93` before legacy staging and
the missing and changed-record stops; it does not launch an original.
