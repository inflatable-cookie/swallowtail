use crate::output::OutputState;
use std::io::{self, Read};
#[cfg(unix)]
use std::os::fd::AsFd;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use swallowtail_runtime::{ProcessOutputChunk, ProcessOutputStream};

pub(crate) struct ReaderControl {
    cancelled: AtomicBool,
}

impl ReaderControl {
    pub(crate) fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

impl Default for ReaderControl {
    fn default() -> Self {
        Self {
            cancelled: AtomicBool::new(false),
        }
    }
}

#[cfg(unix)]
pub(crate) fn spawn_reader<R>(
    name: &str,
    reader: R,
    limit: usize,
    stream: ProcessOutputStream,
    state: Arc<OutputState>,
    control: Arc<ReaderControl>,
) -> std::io::Result<thread::JoinHandle<()>>
where
    R: AsFd + Read + Send + 'static,
{
    set_nonblocking(&reader)?;
    thread::Builder::new()
        .name(name.to_owned())
        .spawn(move || read_output(reader, limit, stream, &state, &control))
}

#[cfg(not(unix))]
pub(crate) fn spawn_reader<R>(
    name: &str,
    reader: R,
    limit: usize,
    stream: ProcessOutputStream,
    state: Arc<OutputState>,
    control: Arc<ReaderControl>,
) -> std::io::Result<thread::JoinHandle<()>>
where
    R: Read + Send + 'static,
{
    thread::Builder::new()
        .name(name.to_owned())
        .spawn(move || read_output(reader, limit, stream, &state, &control))
}

fn read_output<R>(
    mut reader: R,
    limit: usize,
    stream: ProcessOutputStream,
    state: &OutputState,
    control: &ReaderControl,
) where
    R: Read,
{
    let mut buffer = [0_u8; 8192];
    let mut captured = 0_usize;
    let mut overflowed = false;
    loop {
        if control.is_cancelled() {
            break;
        }
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(read) => {
                let remaining = limit.saturating_sub(captured);
                let accepted = read.min(remaining);
                if accepted > 0 {
                    state.push(ProcessOutputChunk::new(stream, buffer[..accepted].to_vec()));
                    captured += accepted;
                }
                if read > accepted && !overflowed {
                    state.fail_limit();
                    overflowed = true;
                }
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(std::time::Duration::from_millis(10));
            }
            Err(_) => {
                state.fail_read();
                break;
            }
        }
    }
    state.close_reader();
}

#[cfg(unix)]
fn set_nonblocking<R: AsFd>(reader: &R) -> io::Result<()> {
    use nix::fcntl::{FcntlArg, OFlag, fcntl};

    let flags = fcntl(reader, FcntlArg::F_GETFL).map_err(io::Error::other)?;
    let flags = OFlag::from_bits_retain(flags) | OFlag::O_NONBLOCK;
    fcntl(reader, FcntlArg::F_SETFL(flags))
        .map(|_| ())
        .map_err(io::Error::other)
}
