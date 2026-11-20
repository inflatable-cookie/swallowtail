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
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
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

#[cfg(test)]
mod tests {
    use super::*;

    struct InterruptedReader {
        interrupts_remaining: usize,
        payload: &'static [u8],
        offset: usize,
    }

    impl Read for InterruptedReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if self.interrupts_remaining > 0 {
                self.interrupts_remaining -= 1;
                return Err(io::Error::from(io::ErrorKind::Interrupted));
            }
            if self.offset >= self.payload.len() {
                return Ok(0);
            }
            let n = (self.payload.len() - self.offset).min(buf.len());
            buf[..n].copy_from_slice(&self.payload[self.offset..self.offset + n]);
            self.offset += n;
            Ok(n)
        }
    }

    #[test]
    fn retries_eintr_without_failing_read() {
        let state = Arc::new(OutputState::default());
        let control = ReaderControl::default();
        let reader = InterruptedReader {
            interrupts_remaining: 3,
            payload: b"hello world",
            offset: 0,
        };
        read_output(reader, 1024, ProcessOutputStream::Stdout, &state, &control);
        state.close_reader();
        state.complete_supervision();

        let chunk = futures_executor::block_on(state.read())
            .expect("EINTR must be retried")
            .expect("chunk arrives");
        assert_eq!(chunk.bytes(), b"hello world");
        assert!(
            futures_executor::block_on(state.read())
                .expect("clean EOF")
                .is_none()
        );
    }
}
