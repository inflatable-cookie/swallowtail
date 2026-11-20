//! Host ports for the mediated-stdio registered-tool courier.
//!
//! A route binding names the exact selection. The host resolves the approved
//! courier target, mints the one-shot rendezvous, and hands back a launch for
//! the provider to spawn. Nothing here starts a process, opens a listener, or
//! grants a lease, so an adapter can describe the seam without naming a
//! local-host type.

use crate::failure::RuntimeFailure;
use crate::host_reference::{EnvironmentRef, ExecutableRef};
use crate::process_input::ProcessRequest;
use crate::registered_tool::RegisteredToolBridgeLease;
use std::ffi::OsString;
use std::path::Path;

/// Host composition that resolves a courier target and mints its launch.
///
/// The two resolution methods are pure lookups against host-approved tables.
/// `registered_tool_proxy_launch` is the operation: it consumes the opened
/// lease's one-shot rendezvous, so it can succeed at most once per lease.
pub trait RegisteredToolCourierService: Send + Sync {
    /// Returns the approved native program path for one executable reference.
    fn approved_executable_path(&self, reference: &ExecutableRef) -> Option<&Path>;

    /// Returns the approved environment bindings for one environment reference.
    fn approved_environment(&self, reference: &EnvironmentRef) -> Option<&[(OsString, OsString)]>;

    /// Materializes one operation-scoped mediated-stdio courier launch.
    ///
    /// The returned launch is one-shot. Its process arguments carry only the
    /// fixed wire tag and a non-authoritative rendezvous path; endpoint,
    /// bearer, and generations stay in the host's private rendezvous file.
    fn registered_tool_proxy_launch(
        &self,
        lease: &RegisteredToolBridgeLease,
    ) -> Result<Box<dyn RegisteredToolCourierLaunch>, RuntimeFailure>;
}

/// One-shot courier launch materialized by the host for one open lease.
///
/// The provider spawns this process; Swallowtail never holds its handle. The
/// ready barrier is the courier's authenticated handshake, not provider
/// progress and not lease completion.
pub trait RegisteredToolCourierLaunch: Send {
    /// Returns the host-approved process request for the provider's launcher.
    fn process_request(&self) -> &ProcessRequest;

    /// Returns the non-authoritative rendezvous path passed to the courier.
    fn rendezvous_path(&self) -> &Path;

    /// Waits for the courier to authenticate within the lease's open budget.
    fn wait_until_ready(&mut self) -> Result<(), RuntimeFailure>;
}

#[cfg(test)]
mod tests {
    use super::{RegisteredToolCourierLaunch, RegisteredToolCourierService};
    use crate::failure::RuntimeFailure;
    use crate::host_reference::{EnvironmentRef, ExecutableRef};
    use crate::process_input::ProcessRequest;
    use crate::registered_tool::RegisteredToolBridgeLease;
    use std::ffi::OsString;
    use std::path::{Path, PathBuf};

    struct Launch {
        request: ProcessRequest,
        rendezvous: PathBuf,
    }

    impl RegisteredToolCourierLaunch for Launch {
        fn process_request(&self) -> &ProcessRequest {
            &self.request
        }

        fn rendezvous_path(&self) -> &Path {
            &self.rendezvous
        }

        fn wait_until_ready(&mut self) -> Result<(), RuntimeFailure> {
            Ok(())
        }
    }

    struct Host {
        executable: PathBuf,
        environment: Vec<(OsString, OsString)>,
    }

    impl RegisteredToolCourierService for Host {
        fn approved_executable_path(&self, _reference: &ExecutableRef) -> Option<&Path> {
            Some(&self.executable)
        }

        fn approved_environment(
            &self,
            _reference: &EnvironmentRef,
        ) -> Option<&[(OsString, OsString)]> {
            Some(&self.environment)
        }

        fn registered_tool_proxy_launch(
            &self,
            _lease: &RegisteredToolBridgeLease,
        ) -> Result<Box<dyn RegisteredToolCourierLaunch>, RuntimeFailure> {
            Ok(Box::new(Launch {
                request: ProcessRequest::new(
                    ExecutableRef::new("fixture.courier").expect("reference"),
                ),
                rendezvous: self.executable.clone(),
            }))
        }
    }

    #[test]
    fn a_port_host_resolves_and_mints_one_launch() {
        let host = Host {
            executable: PathBuf::from("/fixture/courier"),
            environment: vec![(OsString::from("KEY"), OsString::from("value"))],
        };
        let executable = ExecutableRef::new("fixture.courier").expect("reference");
        let environment = EnvironmentRef::new("fixture.environment").expect("reference");
        assert_eq!(
            host.approved_executable_path(&executable),
            Some(Path::new("/fixture/courier"))
        );
        assert_eq!(
            host.approved_environment(&environment).map(<[_]>::len),
            Some(1)
        );
        let request = ProcessRequest::new(executable);
        assert_eq!(request.arguments().len(), 0);
    }
}
