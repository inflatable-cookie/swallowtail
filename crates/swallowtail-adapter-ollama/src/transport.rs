use crate::failure::failure;
use crate::protocol::{NativeEvent, Request, Response};
use futures_channel::{mpsc, oneshot};
use futures_core::Stream;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use swallowtail_runtime::{BoxFuture, HostServices, RuntimeFailure, ScopeId};

const STREAM_CAPACITY: usize = 64;

#[derive(Clone, Default)]
pub(crate) struct CurlTransport;

impl CurlTransport {
    pub(crate) async fn request(
        &self,
        scope: ScopeId,
        endpoint: String,
        request: Request,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Response, RuntimeFailure> {
        let blocking = services.blocking_work().cloned().ok_or_else(|| {
            failure(
                "swallowtail.ollama.blocking_service_missing",
                "Ollama HTTP requires a blocking-work service",
            )
        })?;
        let url = request_url(&endpoint, &request)?;
        let (sender, receiver) = oneshot::channel();
        blocking
            .run(
                scope,
                Box::new(move || {
                    let result = perform_request(url, request, cancelled);
                    let _ = sender.send(result);
                    Ok(())
                }),
            )
            .await?;
        receiver.await.map_err(|_| {
            failure(
                "swallowtail.ollama.blocking_result_missing",
                "Ollama blocking HTTP work returned no result",
            )
        })?
    }

    pub(crate) fn subscribe(
        &self,
        scope: ScopeId,
        endpoint: String,
        request: Request,
        model: String,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Subscription, RuntimeFailure> {
        let blocking = services.blocking_work().cloned().ok_or_else(|| {
            failure(
                "swallowtail.ollama.blocking_service_missing",
                "Ollama NDJSON requires a blocking-work service",
            )
        })?;
        let url = request_url(&endpoint, &request)?;
        let (sender, receiver) = mpsc::channel(STREAM_CAPACITY);
        let job_cancelled = Arc::clone(&cancelled);
        let work = blocking.run(
            scope,
            Box::new(move || perform_ndjson(url, request, model, sender, job_cancelled)),
        );
        Ok(Subscription {
            receiver,
            cancelled,
            work: Some(work),
        })
    }
}

pub(crate) struct Subscription {
    receiver: mpsc::Receiver<NativeEvent>,
    cancelled: Arc<AtomicBool>,
    work: Option<BoxFuture<'static, Result<(), RuntimeFailure>>>,
}

impl Subscription {
    pub(crate) async fn close(mut self) -> Result<(), RuntimeFailure> {
        self.cancelled.store(true, Ordering::SeqCst);
        match self.work.take() {
            Some(work) => work.await,
            None => Ok(()),
        }
    }

    pub(crate) fn poll_next(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<NativeEvent, RuntimeFailure>>> {
        if let Poll::Ready(item) = Pin::new(&mut self.receiver).poll_next(context)
            && item.is_some()
        {
            return Poll::Ready(item.map(Ok));
        }
        match self
            .work
            .as_mut()
            .map_or(Poll::Ready(Ok(())), |work| work.as_mut().poll(context))
        {
            Poll::Ready(result) => {
                self.work = None;
                match result {
                    Err(error) => Poll::Ready(Some(Err(error))),
                    Ok(()) => Pin::new(&mut self.receiver)
                        .poll_next(context)
                        .map(|item| item.map(Ok)),
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

include!("transport/io.rs");
