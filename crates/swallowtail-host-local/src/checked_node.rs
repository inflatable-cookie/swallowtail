//! Host-owned checked Node registrations. These do not qualify vendor code.

use crate::{LocalCapturedNodeRecipe, LocalTrustedRuntime};
use std::fmt;
use std::io;
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{ExecutableRef, ProcessRequest, RuntimeFailure};

/// One host-approved captured Node invocation and revocable runtime custody.
///
/// Construct this only in the trusted execution host after independently
/// establishing the graph, guard, vendor and launch-policy fingerprint. A
/// fingerprint copied from an untrusted caller does not establish admission.
/// The host must maintain custody of the runtime and its loader closure
/// through spawn. This is not a sandbox for hostile JavaScript or same-user
/// verified execution, and registration does not confer qualification.
#[derive(Clone)]
pub struct LocalCheckedNodeLaunch {
    pub(crate) executable: ExecutableRef,
    pub(crate) runtime: LocalTrustedRuntime,
    pub(crate) recipe: LocalCapturedNodeRecipe,
    arguments: Vec<Vec<String>>,
    revoked: Arc<Mutex<bool>>,
}

impl LocalCheckedNodeLaunch {
    /// Binds a host's approved fingerprint and exact argument vectors.
    ///
    /// The recipe holds captured bytes; the supplied fingerprint must come
    /// from the host's independently reviewed policy. Missing custody,
    /// fingerprint mismatch and empty or oversized argument policies refuse.
    pub fn new(
        executable: ExecutableRef,
        runtime: LocalTrustedRuntime,
        recipe: LocalCapturedNodeRecipe,
        approved_fingerprint: [u8; 32],
        arguments: Vec<Vec<String>>,
    ) -> io::Result<Self> {
        runtime.verify()?;
        if recipe.fingerprint().digest() != &approved_fingerprint
            || !recipe.matches_runtime(&runtime)
            || arguments.is_empty()
            || arguments.len() > 16
            || arguments.iter().any(|args| {
                args.len() > 128 || args.iter().map(String::len).sum::<usize>() > 64 * 1024
            })
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid checked Node policy",
            ));
        }
        Ok(Self {
            executable,
            runtime,
            recipe,
            arguments,
            revoked: Arc::new(Mutex::new(false)),
        })
    }

    /// Prevents subsequent preflights and starts across all host clones.
    /// Existing owned processes retain their ordinary joined cleanup contract.
    pub fn revoke(&self) {
        let mut revoked = self
            .revoked
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        *revoked = true;
    }

    pub(crate) fn during_admission<T>(
        &self,
        request: &ProcessRequest,
        action: impl FnOnce() -> Result<T, RuntimeFailure>,
    ) -> Result<T, RuntimeFailure> {
        let revoked = self.revoked.lock().map_err(|_| denied())?;
        if *revoked
            || request.executable() != &self.executable
            || !self
                .arguments
                .iter()
                .any(|args| request.arguments().eq(args.iter().map(String::as_str)))
        {
            return Err(denied());
        }
        // The lock covers the synchronous spawn, never an await or cleanup.
        action()
    }
}

pub(crate) fn denied() -> RuntimeFailure {
    crate::output::failure(
        "swallowtail.local_process.checked_binding_refused",
        "Checked process binding is absent, changed or revoked",
    )
}

impl fmt::Debug for LocalCheckedNodeLaunch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalCheckedNodeLaunch")
            .finish_non_exhaustive()
    }
}
