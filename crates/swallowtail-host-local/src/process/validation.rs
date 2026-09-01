use crate::executable_launch::{
    LocalExecutableLaunch, MAXIMUM_BOOTSTRAP_ENVIRONMENT_BINDINGS,
    MAXIMUM_BOOTSTRAP_ENVIRONMENT_BYTES,
};
use crate::host::LocalProcessHost;
use crate::output::failure;
use std::io::Read;
use std::path::Path;
use swallowtail_runtime::{ProcessRequest, RuntimeFailure};

impl LocalProcessHost {
    pub(super) fn validate_arguments(
        &self,
        request: &ProcessRequest,
    ) -> Result<(), RuntimeFailure> {
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

    pub(super) fn validate_launch(
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
}

const MAXIMUM_SHEBANG_BYTES: u64 = 256;

/// npm and similar launchers use `#!/usr/bin/env …`. Spawning that path with a
/// cleared environment cannot find the interpreter. Refuse the zero-prefix
/// case with an explicit recipe hint instead of a silent discovery failure.
pub(super) fn reject_env_shebang_without_interpreter(
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
