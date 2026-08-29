use crate::child::{LocalProcessHandle, LocalProcessParts};
use crate::executable_launch::{
    LocalExecutableLaunch, MAXIMUM_BOOTSTRAP_ENVIRONMENT_BINDINGS,
    MAXIMUM_BOOTSTRAP_ENVIRONMENT_BYTES,
};
use crate::output::failure;
use std::io::Read;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use swallowtail_runtime::{
    BoxFuture, ProcessHandle, ProcessRequest, ProcessService, RuntimeFailure, ScopeId,
};

#[cfg(unix)]
use std::os::unix::process::CommandExt;

use crate::host::LocalProcessHost;

impl LocalProcessHost {
    fn start_process(
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

    fn validate_arguments(&self, request: &ProcessRequest) -> Result<(), RuntimeFailure> {
        let count = request.arguments().len();
        let bytes = request.arguments().map(str::len).sum::<usize>();
        if count > self.limits.arguments() || bytes > self.limits.argument_bytes() {
            Err(failure(
                "swallowtail.local_process.argument_limit_exceeded",
                "Local process arguments exceeded host-approved limits",
            ))
        } else {
            Ok(())
        }
    }

    fn validate_launch(
        &self,
        launch: &LocalExecutableLaunch,
        request: &ProcessRequest,
    ) -> Result<(), RuntimeFailure> {
        let count = launch
            .prefix_arguments()
            .len()
            .saturating_add(request.arguments().len());
        let prefix_bytes = launch
            .prefix_arguments()
            .iter()
            .map(|argument| argument.as_os_str().as_encoded_bytes().len())
            .fold(0usize, usize::saturating_add);
        let argument_bytes = request
            .arguments()
            .map(str::len)
            .fold(prefix_bytes, usize::saturating_add);
        if count > self.limits.arguments() || argument_bytes > self.limits.argument_bytes() {
            return Err(failure(
                "swallowtail.local_process.argument_limit_exceeded",
                "Local process arguments exceeded host-approved limits",
            ));
        }

        let environment = launch.bootstrap_environment();
        let environment_bytes = environment
            .iter()
            .map(|(name, value)| {
                name.as_os_str()
                    .as_encoded_bytes()
                    .len()
                    .saturating_add(value.as_os_str().as_encoded_bytes().len())
            })
            .fold(0usize, usize::saturating_add);
        if environment.len() > MAXIMUM_BOOTSTRAP_ENVIRONMENT_BINDINGS
            || environment_bytes > MAXIMUM_BOOTSTRAP_ENVIRONMENT_BYTES
        {
            return Err(failure(
                "swallowtail.local_process.bootstrap_environment_limit_exceeded",
                "Local process bootstrap environment exceeded host-approved limits",
            ));
        }
        Ok(())
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

const MAXIMUM_SHEBANG_BYTES: u64 = 256;

/// npm and similar launchers use `#!/usr/bin/env …`. Spawning that path with a
/// cleared environment cannot find the interpreter. Refuse the zero-prefix
/// case with an explicit recipe hint instead of a silent discovery failure.
fn reject_env_shebang_without_interpreter(
    launch: &LocalExecutableLaunch,
) -> Result<(), RuntimeFailure> {
    if !launch.prefix_arguments().is_empty() {
        return Ok(());
    }
    if !uses_env_shebang(launch.program()) {
        return Ok(());
    }
    Err(failure(
        "swallowtail.local_process.interpreted_launcher_requires_host_recipe",
        "Approved executable uses #!/usr/bin/env; approve an interpreted launch \
         with the exact interpreter and script prefix. Ambient PATH is cleared.",
    ))
}

fn uses_env_shebang(program: &Path) -> bool {
    let mut bytes = Vec::new();
    let Ok(file) = std::fs::File::open(program) else {
        return false;
    };
    if file
        .take(MAXIMUM_SHEBANG_BYTES)
        .read_to_end(&mut bytes)
        .is_err()
    {
        return false;
    }
    let first_line = bytes.split(|byte| *byte == b'\n').next().unwrap_or(&bytes);
    let Ok(line) = std::str::from_utf8(first_line) else {
        return false;
    };
    let line = line.trim_end_matches('\r');
    line.starts_with("#!/usr/bin/env ") || line.starts_with("#!/bin/env ")
}

impl ProcessService for LocalProcessHost {
    fn start(
        &self,
        scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        let result = self.start_process(&scope, request);
        Box::pin(async move { result })
    }
}
