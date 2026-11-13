mod launch;
mod validation;

use crate::host::LocalProcessHost;
use swallowtail_runtime::{
    BoxFuture, ProcessHandle, ProcessRequest, ProcessService, RuntimeFailure, ScopeId,
};

impl LocalProcessHost {
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
