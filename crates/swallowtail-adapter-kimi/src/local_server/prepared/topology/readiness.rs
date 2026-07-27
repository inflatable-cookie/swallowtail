use super::super::{KimiLocalServerPreparationProbe, preparation_failure};
use std::future::poll_fn;
use std::task::Poll;
use swallowtail_runtime::{
    HostServices, PreparationFailure, PreparationStage, ProcessHandle, ProcessOutputChunk,
    RuntimeFailure,
};

const READY_PREFIX: &[u8] = b"http://127.0.0.1:";
const MAXIMUM_STARTUP_BYTES: usize = 32 * 1024;

pub(super) async fn observe_ready_origin(
    process: &dyn ProcessHandle,
    expected_port: u16,
    probe: &KimiLocalServerPreparationProbe,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    let mut parser = ReadyOriginParser::default();
    loop {
        match wait_for_output_or_terminal(process.read_output(), probe, services).await {
            ReadinessSignal::Output(Ok(Some(chunk))) => {
                if parser.push(&chunk)? == Some(expected_port) {
                    return Ok(());
                }
                if parser.observed_port().is_some() {
                    return Err(preparation_failure(
                        PreparationStage::BoundedOutput,
                        "swallowtail.kimi.local_server.preparation.ready_endpoint_mismatch",
                        "Owned Kimi local server selected a different endpoint",
                    ));
                }
            }
            ReadinessSignal::Output(Ok(None)) => {
                return Err(preparation_failure(
                    PreparationStage::ProcessExit,
                    "swallowtail.kimi.local_server.preparation.process_exited",
                    "Owned Kimi local server exited before readiness",
                ));
            }
            ReadinessSignal::Output(Err(error)) => {
                return Err(super::super::runtime_preparation_failure(
                    PreparationStage::BoundedOutput,
                    error,
                ));
            }
            ReadinessSignal::Cancelled => {
                return Err(preparation_failure(
                    PreparationStage::BoundedOutput,
                    "swallowtail.kimi.local_server.preparation.cancelled",
                    "Owned Kimi local-server preparation was cancelled",
                ));
            }
            ReadinessSignal::TimedOut => {
                return Err(preparation_failure(
                    PreparationStage::BoundedOutput,
                    "swallowtail.kimi.local_server.preparation.timed_out",
                    "Owned Kimi local-server readiness timed out",
                ));
            }
        }
    }
}

async fn wait_for_output_or_terminal(
    output: swallowtail_runtime::BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>>,
    probe: &KimiLocalServerPreparationProbe,
    services: &HostServices,
) -> ReadinessSignal {
    let mut output = Box::pin(output);
    let mut cancellation = probe.cancellation.wait_requested();
    let mut deadline = services
        .time()
        .expect("validated time service")
        .wait_until(probe.deadline);
    poll_fn(|context| {
        if let Poll::Ready(output) = output.as_mut().poll(context) {
            return Poll::Ready(ReadinessSignal::Output(output));
        }
        if cancellation.as_mut().poll(context).is_ready() {
            return Poll::Ready(ReadinessSignal::Cancelled);
        }
        if deadline.as_mut().poll(context).is_ready() {
            return Poll::Ready(ReadinessSignal::TimedOut);
        }
        Poll::Pending
    })
    .await
}

enum ReadinessSignal {
    Output(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Cancelled,
    TimedOut,
}

#[derive(Default)]
struct ReadyOriginParser {
    matched_prefix: usize,
    digits: Vec<u8>,
    observed_port: Option<u16>,
    total_bytes: usize,
}

impl ReadyOriginParser {
    fn push(&mut self, chunk: &ProcessOutputChunk) -> Result<Option<u16>, PreparationFailure> {
        self.total_bytes = self.total_bytes.saturating_add(chunk.bytes().len());
        if self.total_bytes > MAXIMUM_STARTUP_BYTES {
            return Err(preparation_failure(
                PreparationStage::BoundedOutput,
                "swallowtail.kimi.local_server.preparation.output_limit",
                "Owned Kimi local-server startup output exceeded its bounded limit",
            ));
        }
        for byte in chunk.bytes() {
            if self.observed_port.is_some() {
                break;
            }
            if self.matched_prefix < READY_PREFIX.len() {
                if *byte == READY_PREFIX[self.matched_prefix] {
                    self.matched_prefix += 1;
                } else {
                    self.matched_prefix = usize::from(*byte == READY_PREFIX[0]);
                }
                continue;
            }
            if byte.is_ascii_digit() {
                if self.digits.len() >= 5 {
                    return Err(malformed_ready_origin());
                }
                self.digits.push(*byte);
                continue;
            }
            if self.digits.is_empty() || !matches!(*byte, b'/' | b'#' | b' ' | b'\r' | b'\n') {
                return Err(malformed_ready_origin());
            }
            let digits = std::str::from_utf8(&self.digits).map_err(|_| malformed_ready_origin())?;
            let port = digits
                .parse::<u16>()
                .map_err(|_| malformed_ready_origin())?;
            if port == 0 {
                return Err(malformed_ready_origin());
            }
            self.observed_port = Some(port);
        }
        Ok(self.observed_port)
    }

    const fn observed_port(&self) -> Option<u16> {
        self.observed_port
    }
}

fn malformed_ready_origin() -> PreparationFailure {
    preparation_failure(
        PreparationStage::BoundedOutput,
        "swallowtail.kimi.local_server.preparation.ready_origin_malformed",
        "Owned Kimi local server reported an invalid readiness endpoint",
    )
}

#[cfg(test)]
mod tests {
    use super::ReadyOriginParser;
    use swallowtail_runtime::{ProcessOutputChunk, ProcessOutputStream};

    #[test]
    fn readiness_parser_keeps_only_origin_shape_and_port() {
        let mut parser = ReadyOriginParser::default();
        let first = ProcessOutputChunk::new(
            ProcessOutputStream::Stdout,
            b"Kimi server: http://127.0.".to_vec(),
        );
        let second = ProcessOutputChunk::new(
            ProcessOutputStream::Stdout,
            b"0.1:54999/#token=do-not-retain".to_vec(),
        );
        assert_eq!(parser.push(&first).expect("prefix is accepted"), None);
        assert_eq!(
            parser.push(&second).expect("origin is accepted"),
            Some(54999)
        );
        assert_eq!(parser.digits, b"54999");
    }
}
