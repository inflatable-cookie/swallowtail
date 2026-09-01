use super::validation::reject_env_shebang_without_interpreter;
use crate::child::{LocalProcessHandle, LocalProcessParts};
use crate::host::LocalProcessHost;
use crate::output::failure;
use std::process::{Child, Command, Stdio};
use swallowtail_runtime::{ProcessHandle, ProcessRequest, RuntimeFailure, ScopeId};

#[cfg(unix)]
use std::os::unix::process::CommandExt;

impl LocalProcessHost {
    pub(super) fn start_process(
        &self,
        scope: &ScopeId,
        request: ProcessRequest,
    ) -> Result<Box<dyn ProcessHandle>, RuntimeFailure> {
        self.validate_arguments(&request)?;
        let launch = self
            .approvals
            .executables
            .get(request.executable())
            .ok_or_else(|| {
                failure(
                    "swallowtail.local_process.executable_not_approved",
                    "Local executable reference is not approved",
                )
            })?;
        self.validate_launch(launch, &request)?;
        reject_env_shebang_without_interpreter(launch)?;
        let mut command = Command::new(launch.program());
        command
            .args(launch.prefix_arguments())
            .args(request.arguments())
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        command.envs(launch.bootstrap_environment().iter().cloned());
        self.apply_environment(&mut command, &request)?;
        self.apply_working_resource(&mut command, scope, &request)?;
        #[cfg(unix)]
        let mut group_owner = Some(spawn_process_group_owner()?);
        #[cfg(not(unix))]
        let mut group_owner = None;
        #[cfg(unix)]
        {
            let group_id = i32::try_from(
                group_owner
                    .as_ref()
                    .expect("Unix process group owner is present")
                    .id(),
            )
            .map_err(|_| {
                failure(
                    "swallowtail.local_process.tree_ownership_unavailable",
                    "Local process tree identity is outside the supported range",
                )
            });
            let group_id = match group_id {
                Ok(group_id) => group_id,
                Err(error) => {
                    discard_process_group_owner(group_owner.take());
                    return Err(error);
                }
            };
            command.process_group(group_id);
        }
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(_) => {
                discard_process_group_owner(group_owner.take());
                return Err(failure(
                    "swallowtail.local_process.spawn_failed",
                    "Local process could not be started",
                ));
            }
        };
        let stdin = match child.stdin.take() {
            Some(stdin) => stdin,
            None => {
                let _ = crate::process_exit::cleanup_owned_process(&mut child, &mut group_owner);
                return Err(failure(
                    "swallowtail.local_process.stdin_unavailable",
                    "Local process input is unavailable",
                ));
            }
        };
        let stdout = match child.stdout.take() {
            Some(stdout) => stdout,
            None => {
                let _ = crate::process_exit::cleanup_owned_process(&mut child, &mut group_owner);
                return Err(failure(
                    "swallowtail.local_process.stdout_unavailable",
                    "Local process output is unavailable",
                ));
            }
        };
        let stderr = match child.stderr.take() {
            Some(stderr) => stderr,
            None => {
                let _ = crate::process_exit::cleanup_owned_process(&mut child, &mut group_owner);
                return Err(failure(
                    "swallowtail.local_process.stderr_unavailable",
                    "Local process error output is unavailable",
                ));
            }
        };
        LocalProcessHandle::supervise(
            LocalProcessParts {
                child,
                group_owner,
                stdin,
                stdout,
                stderr,
            },
            self.limits,
        )
        .map(|handle| Box::new(handle) as Box<dyn ProcessHandle>)
    }

    fn apply_environment(
        &self,
        command: &mut Command,
        request: &ProcessRequest,
    ) -> Result<(), RuntimeFailure> {
        for reference in request.environment() {
            let values = self.approvals.environments.get(reference).ok_or_else(|| {
                failure(
                    "swallowtail.local_process.environment_not_approved",
                    "Local environment reference is not approved",
                )
            })?;
            command.envs(values.iter().cloned());
        }
        Ok(())
    }

    fn apply_working_resource(
        &self,
        command: &mut Command,
        scope: &ScopeId,
        request: &ProcessRequest,
    ) -> Result<(), RuntimeFailure> {
        if let Some(reference) = request.working_resource() {
            let approved = self.approvals.working_resources.get(reference).cloned();
            let path = approved
                .or_else(|| self.materialization.working_resource_path(scope, reference))
                .ok_or_else(|| {
                    failure(
                        "swallowtail.local_process.working_resource_not_approved",
                        "Local working-resource reference is not approved",
                    )
                })?;
            command.current_dir(path);
        }
        Ok(())
    }
}

#[cfg(unix)]
fn spawn_process_group_owner() -> Result<Child, RuntimeFailure> {
    let mut command = Command::new("/bin/sleep");
    command
        .arg("86400")
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command.process_group(0);
    command.spawn().map_err(|_| {
        failure(
            "swallowtail.local_process.tree_owner_spawn_failed",
            "Local process tree owner could not be started",
        )
    })
}

fn discard_process_group_owner(owner: Option<Child>) {
    if let Some(mut owner) = owner {
        let _ = owner.kill();
        let _ = owner.wait();
    }
}
