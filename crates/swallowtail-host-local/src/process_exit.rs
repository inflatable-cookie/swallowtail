use crate::output::failure;
use std::future::Future;
use std::pin::Pin;
use std::process::Child;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_runtime::RuntimeFailure;

#[cfg(unix)]
use nix::sys::signal::{Signal, killpg};
#[cfg(unix)]
use nix::unistd::Pid;
#[cfg(windows)]
use std::process::{Command, Stdio};

#[cfg(unix)]
mod descendants;
mod supervision;

#[cfg(unix)]
pub(crate) use descendants::DescendantTracker;
pub(crate) use supervision::{ChildCommand, ReaderSupervision, supervise_child};

#[derive(Default)]
struct ExitInner {
    result: Option<Result<swallowtail_runtime::ProcessExit, RuntimeFailure>>,
    waiters: Vec<Waker>,
}

#[derive(Default)]
pub(crate) struct ExitState {
    inner: Mutex<ExitInner>,
}

impl ExitState {
    pub(crate) fn complete(
        &self,
        result: Result<swallowtail_runtime::ProcessExit, RuntimeFailure>,
    ) {
        let mut inner = self.inner.lock().expect("local process exit lock poisoned");
        if inner.result.is_none() {
            inner.result = Some(result);
        }
        for waiter in inner.waiters.drain(..) {
            waiter.wake();
        }
    }

    pub(crate) fn future(self: &Arc<Self>) -> ExitFuture {
        ExitFuture {
            state: Arc::clone(self),
        }
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.inner
            .lock()
            .expect("local process exit lock poisoned")
            .result
            .is_some()
    }
}

pub(crate) struct ExitFuture {
    state: Arc<ExitState>,
}

impl Future for ExitFuture {
    type Output = Result<swallowtail_runtime::ProcessExit, RuntimeFailure>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self
            .state
            .inner
            .lock()
            .expect("local process exit lock poisoned");
        if let Some(result) = &inner.result {
            Poll::Ready(result.clone())
        } else {
            if !inner
                .waiters
                .iter()
                .any(|waiter| waiter.will_wake(context.waker()))
            {
                inner.waiters.push(context.waker().clone());
            }
            Poll::Pending
        }
    }
}

/// Terminates a process tree while its host-owned group owner is still live.
///
/// The group owner is the ownership primitive. Its live child handle keeps
/// the group identity from becoming a reusable numeric target during tree
/// cleanup. Unix callers never signal a bare process-group number.
pub(crate) fn terminate_process_tree(
    group_owner: Option<&Child>,
    root_process_id: u32,
    force: bool,
) -> Result<(), RuntimeFailure> {
    #[cfg(unix)]
    {
        let _ = (root_process_id, force);
        let owner = group_owner.ok_or_else(|| {
            failure(
                "swallowtail.local_process.tree_ownership_unavailable",
                "Local process tree ownership is unavailable",
            )
        })?;
        let group_id = i32::try_from(owner.id()).map_err(|_| {
            failure(
                "swallowtail.local_process.tree_ownership_unavailable",
                "Local process tree identity is outside the supported range",
            )
        })?;
        let signal = if force {
            Signal::SIGKILL
        } else {
            Signal::SIGTERM
        };
        killpg(Pid::from_raw(group_id), signal).map_err(|_| {
            failure(
                "swallowtail.local_process.tree_termination_failed",
                "Local process tree could not be terminated",
            )
        })
    }

    #[cfg(windows)]
    {
        let _ = group_owner;
        let mut command = Command::new(r"C:\Windows\System32\taskkill.exe");
        command
            .args(["/PID", &root_process_id.to_string(), "/T"])
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        if force {
            command.arg("/F");
        }
        if command
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
        {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.local_process.tree_termination_failed",
                "Local process tree could not be terminated",
            ))
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        let _ = (group_owner, root_process_id, force);
        Err(failure(
            "swallowtail.local_process.tree_ownership_unavailable",
            "Local process tree ownership is unavailable",
        ))
    }
}

/// Cleans up a process before its supervisor has reaped the root.
pub(crate) fn cleanup_owned_process(
    child: &mut Child,
    group_owner: &mut Option<Child>,
) -> Result<(), RuntimeFailure> {
    let mut cleanup_error = None;
    if group_owner.is_some() {
        if let Err(error) = terminate_process_tree(group_owner.as_ref(), child.id(), true) {
            cleanup_error = Some(error);
            let _ = child.kill();
        }
    } else if child.kill().is_err() {
        cleanup_error = Some(failure(
            "swallowtail.local_process.force_stop_failed",
            "Local process could not be force-stopped",
        ));
    }
    if child.wait().is_err() && cleanup_error.is_none() {
        cleanup_error = Some(failure(
            "swallowtail.local_process.wait_failed",
            "Local process exit could not be observed",
        ));
    }
    if let Some(owner) = group_owner.as_mut() {
        if cleanup_error.is_some() {
            let _ = owner.kill();
        }
        if owner.wait().is_err() && cleanup_error.is_none() {
            cleanup_error = Some(failure(
                "swallowtail.local_process.tree_join_failed",
                "Local process tree owner could not be joined",
            ));
        }
    }
    cleanup_error.map_or(Ok(()), Err)
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::terminate_process_tree;

    #[test]
    fn tree_termination_requires_a_live_group_owner() {
        let failure = terminate_process_tree(None, 1, true).expect_err("owner is required");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.local_process.tree_ownership_unavailable"
        );
    }

    #[cfg(unix)]
    #[test]
    fn failed_tree_termination_is_reported_without_signalling_a_foreign_group() {
        let mut child = Command::new("/bin/sleep")
            .arg("5")
            .spawn()
            .expect("fixture process starts");
        let failure = terminate_process_tree(Some(&child), child.id(), true)
            .expect_err("a child outside its own process group cannot be tree-terminated");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.local_process.tree_termination_failed"
        );
        child.kill().expect("fixture process stops");
        child.wait().expect("fixture process joins");
    }
}
