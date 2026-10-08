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

The exact artifacts did not reach a permission request. `1.0.80` produced no
initialize result before the bounded timeout; `1.0.81` and `1.0.93` initialized
and reported their frozen versions, but produced no result for `session/new`.
No prompt, permission request, cancellation exchange, or tool call occurred.
The absence of a tool effect in those incomplete runs does not prove a stable
permission boundary. The record therefore keeps permission proof false and
does not support moving any production claim.

To reacquire the artifacts, run `python3 scripts/stage-copilot-acp-offline-artifacts.py`
and retain its output in fresh task-owned scratch. It verifies pinned public
registry metadata and inventories each extracted tree without installing a
package. Use a new task-owned scratch record for the harness `--prepare`
preflight before any exact execution; pass the staging helper's `artifact_root`
to `scripts/copilot-acp-offline-proof.py --execute` only after reviewing the
record. The committed execution record describes the completed run; do not
repeat the exact binary attempts as part of validation.
