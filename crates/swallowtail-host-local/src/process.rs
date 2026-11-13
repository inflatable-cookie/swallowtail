mod launch;
mod validation;

use crate::host::LocalProcessHost;
use swallowtail_runtime::{
    BoxFuture, ProcessHandle, ProcessRequest, ProcessService, RuntimeFailure, ScopeId,
};

impl LocalProcessHost {
    /// Starts an ordinarily approved candidate while serializing host runtime
    /// revocation with spawn and writing its captured-source bootstrap.
    ///
    /// This is not a checked-process grant: graph, guard and policy authority
    /// must be independently established by the host before calling it. The
    /// host must protect the selected runtime path from replacement through
    /// spawn. It does not provide same-user-resistant verified execution.
    #[cfg(unix)]
    pub async fn start_trusted_candidate(
        &self,
        scope: ScopeId,
        request: ProcessRequest,
        runtime: &crate::LocalTrustedRuntime,
        recipe: &crate::LocalCapturedNodeRecipe,
        timeout: std::time::Duration,
    ) -> Result<Box<dyn ProcessHandle>, RuntimeFailure> {
        if timeout.is_zero() || timeout > std::time::Duration::from_secs(30) {
            return Err(crate::output::failure(
                "swallowtail.local_process.bootstrap_limit",
                "Bootstrap input exceeds its host limit",
            ));
        }
        let launch = self
            .approvals
            .executables
            .get(request.executable())
            .ok_or_else(|| {
                crate::output::failure(
                    "swallowtail.local_process.executable_not_approved",
                    "Local executable reference is not approved",
                )
            })?;
        recipe.verify_trusted_launch(runtime, launch).map_err(|_| {
            crate::output::failure(
                "swallowtail.local_process.runtime_custody_mismatch",
                "Trusted runtime custody could not be verified",
            )
        })?;
        let process = runtime
            .during_spawn(launch, || self.start_process(&scope, request))
            .map_err(|_| {
                crate::output::failure(
                    "swallowtail.local_process.runtime_custody_mismatch",
                    "Trusted runtime custody could not be verified",
                )
            })??;
        if let Err(error) = process.bootstrap(recipe.bootstrap(), timeout).await {
            let stopped = process.force_stop().await;
            let joined = process.wait().await;
            stopped?;
            joined?;
            return Err(error);
        }
        Ok(Box::new(process))
    }

    /// Starts an ordinarily approved process with a separate bounded input prelude.
    ///
    /// This does not establish checked-launch identity or interpreter custody.
    /// The caller remains responsible for admitting any guarded recipe. Failure
    /// stops and joins the child; dropping the future requests owned cleanup.
    #[cfg(unix)]
    pub async fn start_with_bootstrap(
        &self,
        scope: ScopeId,
        request: ProcessRequest,
        bootstrap: &[u8],
        timeout: std::time::Duration,
    ) -> Result<Box<dyn ProcessHandle>, RuntimeFailure> {
        if bootstrap.len() > 32 * 1024 * 1024 + 5
            || timeout.is_zero()
            || timeout > std::time::Duration::from_secs(30)
        {
            return Err(crate::output::failure(
                "swallowtail.local_process.bootstrap_limit",
                "Bootstrap input exceeds its host limit",
            ));
        }
        let process = self.start_process(&scope, request)?;
        if let Err(error) = process.bootstrap(bootstrap, timeout).await {
            let stopped = process.force_stop().await;
            let joined = process.wait().await;
            stopped?;
            joined?;
            return Err(error);
        }
        Ok(Box::new(process))
    }
}

impl ProcessService for LocalProcessHost {
    fn start(
        &self,
        scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        let result = self
            .start_process(&scope, request)
            .map(|handle| Box::new(handle) as Box<dyn ProcessHandle>);
        Box::pin(async move { result })
    }
}
