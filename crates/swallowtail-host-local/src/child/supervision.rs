use super::{LocalProcessHandle, LocalProcessParts, SharedChild};
use crate::limits::LocalProcessLimits;
use crate::output::{OutputState, failure};
use crate::process_exit::{ExitState, ReaderSupervision, cleanup_owned_process, supervise_child};
use crate::process_reader::{ReaderControl, spawn_reader};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use swallowtail_runtime::{ProcessOutputStream, RuntimeFailure};

impl LocalProcessHandle {
    pub(crate) fn supervise(
        process: LocalProcessParts,
        limits: LocalProcessLimits,
    ) -> Result<Self, RuntimeFailure> {
        let LocalProcessParts {
            mut child,
            mut group_owner,
            stdin,
            stdout,
            stderr,
        } = process;
        let output = Arc::new(OutputState::default());
        let reader_control = Arc::new(ReaderControl::default());
        let stdout_reader = spawn_reader(
            "swallowtail-stdout",
            stdout,
            limits.stdout_bytes(),
            ProcessOutputStream::Stdout,
            Arc::clone(&output),
            Arc::clone(&reader_control),
        );
        let stderr_reader = spawn_reader(
            "swallowtail-stderr",
            stderr,
            limits.stderr_bytes(),
            ProcessOutputStream::Stderr,
            Arc::clone(&output),
            Arc::clone(&reader_control),
        );
        let (stdout_reader, stderr_reader) = match (stdout_reader, stderr_reader) {
            (Ok(stdout_reader), Ok(stderr_reader)) => (stdout_reader, stderr_reader),
            (stdout_reader, stderr_reader) => {
                reader_control.cancel();
                let _ = cleanup_owned_process(&mut child, &mut group_owner);
                if let Ok(reader) = stdout_reader {
                    let _ = reader.join();
                }
                if let Ok(reader) = stderr_reader {
                    let _ = reader.join();
                }
                return Err(failure(
                    "swallowtail.local_process.reader_start_failed",
                    "Local process output supervision could not start",
                ));
            }
        };

        let exit = Arc::new(ExitState::default());
        let (commands, command_receiver) = mpsc::channel();
        let supervisor_exit = Arc::clone(&exit);
        let supervisor_output = Arc::clone(&output);
        let supervision_slot = Arc::new(Mutex::new(Some((
            child,
            group_owner,
            ReaderSupervision {
                stdout: stdout_reader,
                stderr: stderr_reader,
                control: Arc::clone(&reader_control),
            },
        ))));
        let supervisor_parts = Arc::clone(&supervision_slot);
        let supervisor = thread::Builder::new()
            .name("swallowtail-process".to_owned())
            .spawn(move || {
                let (mut child, mut group_owner, readers) = supervisor_parts
                    .lock()
                    .expect("local process supervision lock poisoned")
                    .take()
                    .expect("local process supervision parts must be present");
                supervise_child(
                    &mut child,
                    &mut group_owner,
                    command_receiver,
                    readers,
                    &supervisor_output,
                    &supervisor_exit,
                );
            });
        if supervisor.is_err() {
            if let Some((mut child, mut group_owner, readers)) = supervision_slot
                .lock()
                .expect("local process supervision lock poisoned")
                .take()
            {
                reader_control.cancel();
                let _ = cleanup_owned_process(&mut child, &mut group_owner);
                let _ = readers.stdout.join();
                let _ = readers.stderr.join();
            }
            return Err(failure(
                "swallowtail.local_process.supervisor_start_failed",
                "Local process supervision could not start",
            ));
        }

        Ok(Self {
            shared: Arc::new(SharedChild {
                stdin: Mutex::new(Some(stdin)),
                stdin_bytes: Mutex::new(0),
                stdin_limit: limits.stdin_bytes(),
                output,
                exit,
                commands,
                stop_requested: AtomicBool::new(false),
                force_requested: AtomicBool::new(false),
            }),
        })
    }
}
