use futures_core::Stream;
use std::pin::Pin;
use std::sync::{Arc, Weak};
use std::task::{Context, Poll};
use swallowtail_runtime::{
    BoxEventStream, OperationContent, ProviderObservation, RuntimeEvent, RuntimeEventKind,
    RuntimeEventSender, RuntimeEventStream, RuntimeFailure, TerminalStatus, TokenUsage,
    runtime_event_channel,
};

pub(crate) struct TurnEvents {
    sender: RuntimeEventSender,
    consumer: Weak<()>,
}

struct ObservedEventStream {
    inner: RuntimeEventStream,
    _consumer: Arc<()>,
}

impl Stream for ObservedEventStream {
    type Item = Result<RuntimeEvent, RuntimeFailure>;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.get_mut().inner).poll_next(context)
    }
}

impl TurnEvents {
    pub(crate) fn new(capacity: usize) -> Result<(Self, BoxEventStream), RuntimeFailure> {
        let (sender, inner) = runtime_event_channel(capacity)?;
        let consumer = Arc::new(());
        Ok((
            Self {
                sender,
                consumer: Arc::downgrade(&consumer),
            },
            Box::pin(ObservedEventStream {
                inner,
                _consumer: consumer,
            }),
        ))
    }

    pub(crate) fn send(&self, event: RuntimeEvent) -> Result<(), RuntimeFailure> {
        self.sender.send(event)
    }

    pub(crate) fn finish(
        &self,
        status: TerminalStatus,
        output: Option<&OperationContent>,
        usage: Option<TokenUsage>,
        next_sequence: impl Fn() -> u64,
    ) -> TerminalStatus {
        let emitted = (|| {
            if let Some(output) = output {
                self.send(RuntimeEvent::with_content(
                    next_sequence(),
                    RuntimeEventKind::OutputAvailable,
                    output.clone(),
                ))?;
            }
            if let Some(usage) = usage {
                self.send(RuntimeEvent::new(
                    next_sequence(),
                    RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage)),
                ))?;
            }
            Ok::<(), RuntimeFailure>(())
        })();
        self.sender.mark_terminal();
        match emitted {
            Err(error) if self.consumer.upgrade().is_some() => {
                TerminalStatus::RuntimeFailed(error.diagnostic().clone())
            }
            _ => status,
        }
    }
}
