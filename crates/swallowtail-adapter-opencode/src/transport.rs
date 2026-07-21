use crate::failure::failure;
use crate::protocol::{Method, Request, Response, SseDecoder};
use curl::easy::{Easy, List, WriteError};
use futures_channel::{mpsc, oneshot};
use futures_core::Stream;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swallowtail_runtime::{BoxFuture, HostServices, RuntimeFailure, ScopeId};
use url::Url;

const RESPONSE_LIMIT: usize = 4 * 1024 * 1024;
const SSE_CAPACITY: usize = 64;

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
                "swallowtail.opencode.blocking_service_missing",
                "OpenCode HTTP requires a blocking-work service",
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
                "swallowtail.opencode.blocking_result_missing",
                "OpenCode blocking HTTP work did not return a result",
            )
        })?
    }

    pub(crate) async fn subscribe(
        &self,
        scope: ScopeId,
        endpoint: String,
        directory: String,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Subscription, RuntimeFailure> {
        let blocking = services.blocking_work().cloned().ok_or_else(|| {
            failure(
                "swallowtail.opencode.blocking_service_missing",
                "OpenCode SSE requires a blocking-work service",
            )
        })?;
        let request = Request::get("/event").with_directory(&directory);
        let url = request_url(&endpoint, &request)?;
        let (sender, receiver) = mpsc::channel(SSE_CAPACITY);
        let job_cancelled = Arc::clone(&cancelled);
        let work = blocking.run(
            scope,
            Box::new(move || perform_sse(url, sender, job_cancelled)),
        );
        Ok(Subscription {
            receiver,
            cancelled,
            work: Some(work),
        })
    }
}

pub(crate) struct Subscription {
    receiver: mpsc::Receiver<Result<Vec<u8>, RuntimeFailure>>,
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
    ) -> Poll<Option<Result<Vec<u8>, RuntimeFailure>>> {
        if let Poll::Ready(item) = Pin::new(&mut self.receiver).poll_next(context)
            && item.is_some()
        {
            return Poll::Ready(item);
        }
        let work = self
            .work
            .as_mut()
            .map_or(Poll::Ready(Ok(())), |work| work.as_mut().poll(context));
        match work {
            Poll::Ready(result) => {
                self.work = None;
                match result {
                    Err(error) => Poll::Ready(Some(Err(error))),
                    Ok(()) => Pin::new(&mut self.receiver).poll_next(context),
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

include!("transport/io.rs");
include!("transport/tests.rs");
