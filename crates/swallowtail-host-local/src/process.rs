mod launch;
mod validation;

use crate::host::LocalProcessHost;
use swallowtail_runtime::{
    BoxFuture, CheckedProcessRef, ProcessHandle, ProcessRequest, ProcessService, RuntimeFailure,
    ScopeId,
};

impl LocalProcessHost {
    /// Starts an ordinarily approved candidate while serializing host runtime
    /// revocation with spawn, then writing its captured-source bootstrap.
    /// Only the checked path rechecks admission through the final bootstrap byte.
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
        let process = self.spawn_trusted_candidate(&scope, request, runtime, recipe)?;
        if let Err(error) = process.bootstrap(recipe.bootstrap(), timeout).await {
            let stopped = process.force_stop().await;
            let joined = process.wait().await;
            stopped?;
            joined?;
            return Err(error);
        }
        Ok(Box::new(process))
    }

    #[cfg(unix)]
    fn spawn_trusted_candidate(
        &self,
        scope: &ScopeId,
        request: ProcessRequest,
        runtime: &crate::LocalTrustedRuntime,
        recipe: &crate::LocalCapturedNodeRecipe,
    ) -> Result<crate::child::LocalProcessHandle, RuntimeFailure> {
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
        let _native_custody = recipe
            .native
            .as_ref()
            .map(crate::LocalTrustedRuntime::acquire_custody)
            .transpose()
            .map_err(|_| crate::checked_node::denied())?;
        let _ps_custody = recipe
            .ps
            .as_ref()
            .map(crate::LocalTrustedRuntime::acquire_custody)
            .transpose()
            .map_err(|_| crate::checked_node::denied())?;
        runtime
            .during_spawn(launch, || self.start_process(scope, request))
            .map_err(|_| {
                crate::output::failure(
                    "swallowtail.local_process.runtime_custody_mismatch",
                    "Trusted runtime custody could not be verified",
                )
            })?
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
        if self
            .approvals
            .checked_nodes
            .values()
            .any(|checked| checked.executable == *request.executable())
        {
            return Box::pin(async { Err(crate::checked_node::denied()) });
        }
        let result = self
            .start_process(&scope, request)
            .map(|handle| Box::new(handle) as Box<dyn ProcessHandle>);
        Box::pin(async move { result })
    }

    fn validate_checked(
        &self,
        _scope: ScopeId,
        binding: CheckedProcessRef,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let result = self
            .approvals
            .checked_nodes
            .get(&binding)
            .ok_or_else(crate::checked_node::denied)
            .and_then(|checked| {
                checked.during_admission(&request, || {
                    let launch = self
                        .approvals
                        .executables
                        .get(request.executable())
                        .ok_or_else(crate::checked_node::denied)?;
                    checked
                        .recipe
                        .verify_trusted_launch(&checked.runtime, launch)
                        .map_err(|_| crate::checked_node::denied())?;
                    self.validate_arguments(&request)?;
                    self.validate_launch(launch, &request)
                })
            });
        Box::pin(async move { result })
    }

    #[cfg(unix)]
    fn start_checked(
        &self,
        scope: ScopeId,
        binding: CheckedProcessRef,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        // Resolve and spawn synchronously while both custody locks are held.
        // No checked path calls the ordinary ProcessService::start fallback.
        let checked = self.approvals.checked_nodes.get(&binding).cloned();
        let process = checked
            .as_ref()
            .ok_or_else(crate::checked_node::denied)
            .and_then(|checked| {
                checked.during_admission(&request, || {
                    self.spawn_trusted_candidate(
                        &scope,
                        request.clone(),
                        &checked.runtime,
                        &checked.recipe,
                    )
                })
            });
        let launch = self
            .approvals
            .executables
            .get(request.executable())
            .cloned();
        Box::pin(async move {
            let process = process?;
            let checked = checked.ok_or_else(crate::checked_node::denied)?;
            let launch = launch.ok_or_else(crate::checked_node::denied)?;
            // Recheck before sending any graph bytes, then serialize the final
            // admission-byte write with both registration and runtime revocation.
            // The callback never holds either synchronous lock across an await.
            let admit = |write: &mut dyn FnMut() -> std::io::Result<usize>| {
                checked
                    .during_admission(&request, || {
                        let _native = checked
                            .recipe
                            .native
                            .as_ref()
                            .map(crate::LocalTrustedRuntime::acquire_custody)
                            .transpose()
                            .map_err(|_| crate::checked_node::denied())?;
                        let _ps = checked
                            .recipe
                            .ps
                            .as_ref()
                            .map(crate::LocalTrustedRuntime::acquire_custody)
                            .transpose()
                            .map_err(|_| crate::checked_node::denied())?;
                        checked
                            .runtime
                            .during_spawn(&launch, write)
                            .map_err(|_| crate::checked_node::denied())
                    })
                    .map_err(std::io::Error::other)?
            };
            let result = match admit(&mut || Ok(0)) {
                Err(_) => Err(crate::checked_node::denied()),
                Ok(_) => {
                    process
                        .bootstrap_with_admission(
                            checked.recipe.bootstrap(),
                            std::time::Duration::from_secs(5),
                            &admit,
                        )
                        .await
                }
            };
            if let Err(error) = result {
                let stopped = process.force_stop().await;
                let joined = process.wait().await;
                stopped?;
                joined?;
                return Err(error);
            }
            Ok(Box::new(process) as Box<dyn ProcessHandle>)
        })
    }
}
