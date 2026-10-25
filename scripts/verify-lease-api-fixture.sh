#!/usr/bin/env bash
# Compile the released registered-tool lease facade from outside the workspace.
#
# Provider-free: the fixture declares types and call sites only. It opens no
# lease, starts no provider, spends nothing and reads no credential. Two proofs
# run here. A temporary external consumer workspace, built outside this
# checkout, proves the seven registered-tool lease methods released in v0.5.4
# remain ordinary public API with their released signatures. The generated
# current-version semantic API dump is then checked for those same seven
# entries, so an item that only compiles by accident cannot pass.
#
# The consumer is a temporary workspace outside this checkout, so this driver
# enters once through the shared helper's host-tool mode: one `oxide exec --`
# lease, and its own build directory, so another workspace's intermediates never
# enter this project's shared slot. The run-owned root below is the only
# directory cleaned up.
set -euo pipefail

fixture_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$fixture_repo_root"

# shellcheck source=scripts/release-version-identity.sh
source "$fixture_repo_root/scripts/release-version-identity.sh"
release_load_version_identity

if [[ ${SWALLOWTAIL_OXIDE_EXEC_CHILD:-} != 1 ]]; then
  SWALLOWTAIL_OXIDE_EXEC_CHILD=1 exec bash "$fixture_repo_root/scripts/oxide-rust.sh" \
    --host-tool "$fixture_repo_root/scripts/verify-lease-api-fixture.sh" "$@"
fi

fixture_root=$(mktemp -d)
trap 'rm -rf "$fixture_root"' EXIT
export CARGO_TARGET_DIR="$fixture_root/target"
export CARGO_BUILD_BUILD_DIR="$fixture_root/build"
fixture_consumer="$fixture_root/lease-api-consumer"
mkdir -p "$fixture_consumer/src"

cat > "$fixture_consumer/Cargo.toml" <<EOF
[package]
name = "swallowtail-lease-api-consumer"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
swallowtail-core = { path = "$fixture_repo_root/crates/swallowtail-core" }
swallowtail-runtime = { path = "$fixture_repo_root/crates/swallowtail-runtime" }
EOF

cat > "$fixture_consumer/src/lib.rs" <<'EOF'
//! External consumer compile fixture for the released registered-tool lease API.
//!
//! Nothing here is executed. It proves an ordinary downstream crate can name
//! the seven lease methods released in `v0.5.4` with their released
//! signatures, then calls them from a call site that only has to typecheck.

#![allow(dead_code)]

use swallowtail_core::ConfiguredInstanceId;
use swallowtail_runtime::{
    Deadline, HostServices, PreparedRegisteredToolBinding, RegisteredToolBridgeLease,
    RegisteredToolOpenRequest, RegisteredToolOperationKernel, RegisteredToolPreparation,
    RuntimeFailure, RuntimeTurnId, ScopeId,
};

/// Released signature of every method this fixture covers.
fn released_signatures() {
    let _: fn(&PreparedRegisteredToolBinding) -> Option<Deadline> =
        PreparedRegisteredToolBinding::lease_deadline;
    let _: fn(&RegisteredToolBridgeLease) -> Option<Deadline> =
        RegisteredToolBridgeLease::lease_deadline;
    let _: fn(&RegisteredToolBridgeLease) -> Deadline = RegisteredToolBridgeLease::next_call_deadline;
    let _: fn(&RegisteredToolOpenRequest) -> Option<Deadline> =
        RegisteredToolOpenRequest::lease_deadline;
    let _: fn(RegisteredToolOpenRequest, Option<Deadline>) -> RegisteredToolOpenRequest =
        RegisteredToolOpenRequest::with_lease_deadline;
    let _: fn(&RegisteredToolOperationKernel) -> Deadline =
        RegisteredToolOperationKernel::next_call_deadline;
    let _: fn(
        &RegisteredToolPreparation,
        &HostServices,
        ConfiguredInstanceId,
        ScopeId,
        RuntimeTurnId,
        Deadline,
        Option<Deadline>,
    ) -> Result<PreparedRegisteredToolBinding, RuntimeFailure> =
        RegisteredToolPreparation::prepare_with_lease_deadline;
}

/// The same methods from an ordinary call site.
fn call_sites(
    preparation: &RegisteredToolPreparation,
    hosts: &HostServices,
    configured_instance: ConfiguredInstanceId,
    operation_scope: ScopeId,
    turn: RuntimeTurnId,
    open_deadline: Deadline,
    request: RegisteredToolOpenRequest,
    prepared: &PreparedRegisteredToolBinding,
    lease: &RegisteredToolBridgeLease,
    kernel: &RegisteredToolOperationKernel,
    lease_deadline: Option<Deadline>,
) -> Option<Deadline> {
    let request = request.with_lease_deadline(lease_deadline);
    let _ = request.lease_deadline();
    let _ = prepared.lease_deadline();
    let _ = lease.lease_deadline();
    let _ = lease.next_call_deadline();
    let _ = kernel.next_call_deadline();
    let _ = preparation.prepare_with_lease_deadline(
        hosts,
        configured_instance,
        operation_scope,
        turn,
        open_deadline,
        lease_deadline,
    );
    None
}
EOF

cargo generate-lockfile --manifest-path "$fixture_consumer/Cargo.toml" --offline
cargo check --manifest-path "$fixture_consumer/Cargo.toml" --offline --locked

fixture_dump="release-baselines/public-api-$release_current_version/swallowtail-runtime.txt"
if [[ ! -f $fixture_dump ]]; then
  printf 'missing generated semantic API dump %s\n' "$fixture_dump" >&2
  exit 1
fi

while IFS= read -r fixture_entry; do
  if ! grep -Fxq "$fixture_entry" "$fixture_dump"; then
    printf 'generated API dump omits a released lease method: %s\n' "$fixture_entry" >&2
    exit 1
  fi
done <<'ENTRIES'
pub const fn swallowtail_runtime::PreparedRegisteredToolBinding::lease_deadline(&self) -> core::option::Option<swallowtail_runtime::Deadline>
pub const fn swallowtail_runtime::RegisteredToolBridgeLease::lease_deadline(&self) -> core::option::Option<swallowtail_runtime::Deadline>
pub fn swallowtail_runtime::RegisteredToolBridgeLease::next_call_deadline(&self) -> swallowtail_runtime::Deadline
pub const fn swallowtail_runtime::RegisteredToolOpenRequest::lease_deadline(&self) -> core::option::Option<swallowtail_runtime::Deadline>
pub fn swallowtail_runtime::RegisteredToolOpenRequest::with_lease_deadline(self, core::option::Option<swallowtail_runtime::Deadline>) -> Self
pub fn swallowtail_runtime::RegisteredToolOperationKernel::next_call_deadline(&self) -> swallowtail_runtime::Deadline
pub fn swallowtail_runtime::RegisteredToolPreparation::prepare_with_lease_deadline(&self, &swallowtail_runtime::HostServices, swallowtail_core::runtime_identity::ConfiguredInstanceId, swallowtail_runtime::ScopeId, swallowtail_runtime::RuntimeTurnId, swallowtail_runtime::Deadline, core::option::Option<swallowtail_runtime::Deadline>) -> core::result::Result<swallowtail_runtime::PreparedRegisteredToolBinding, swallowtail_runtime::RuntimeFailure>
ENTRIES

printf 'external lease API fixture passed for 7 released methods at v%s\n' \
  "$release_current_version"
