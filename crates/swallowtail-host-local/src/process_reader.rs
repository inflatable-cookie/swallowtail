use crate::output::OutputState;
use std::io::Read;
use std::sync::Arc;
use std::thread;
use swallowtail_runtime::{ProcessOutputChunk, ProcessOutputStream};

pub(crate) fn spawn_reader<R>(
    name: &str,
    reader: R,
    limit: usize,
    stream: ProcessOutputStream,
    state: Arc<OutputState>,
) -> std::io::Result<thread::JoinHandle<()>>
where
    R: Read + Send + 'static,
{
    thread::Builder::new()
        .name(name.to_owned())
        .spawn(move || read_output(reader, limit, stream, &state))
}

fn read_output<R>(mut reader: R, limit: usize, stream: ProcessOutputStream, state: &OutputState)
where
    R: Read,
{
    let mut buffer = [0_u8; 8192];
    let mut captured = 0_usize;
    let mut overflowed = false;
    loop {
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
            Err(_) => {
                state.fail_read();
                break;
            }
        }
    }
    state.close_reader();
}
