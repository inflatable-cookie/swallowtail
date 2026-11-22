use crate::binding::{BedrockCredentialProvider, BedrockRegion};
use crate::failure::{failure, provider_failure};
use crate::sdk_config::one_attempt_retry_config;
use crate::stream::{
    StreamDecoder, StreamUpdate, classify_converse_failure, classify_output_failure,
};
use aws_sdk_bedrockruntime::config::{BehaviorVersion, Region};
use aws_sdk_bedrockruntime::types::{
    ContentBlock, ConversationRole, InferenceConfiguration, Message,
};
use futures_channel::mpsc::Sender;
use futures_util::SinkExt;
use swallowtail_runtime::{BoxFuture, RuntimeFailure};
use tokio::sync::watch;

#[derive(Clone)]
pub(crate) struct SdkInvocation {
    pub(crate) endpoint: String,
    pub(crate) region: BedrockRegion,
    pub(crate) provider: BedrockCredentialProvider,
    pub(crate) model: String,
    pub(crate) prompt: String,
    pub(crate) maximum_output_tokens: i32,
}

pub(crate) trait SdkExecutor: Send + Sync {
    fn execute(
        &self,
        invocation: SdkInvocation,
        updates: Sender<Result<StreamUpdate, RuntimeFailure>>,
        cancelled: watch::Receiver<bool>,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>>;
}

#[derive(Default)]
pub(crate) struct AwsSdkExecutor;

impl SdkExecutor for AwsSdkExecutor {
    fn execute(
        &self,
        invocation: SdkInvocation,
        mut updates: Sender<Result<StreamUpdate, RuntimeFailure>>,
        mut cancelled: watch::Receiver<bool>,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            let config = aws_sdk_bedrockruntime::Config::builder()
                .behavior_version(BehaviorVersion::latest())
                .region(Region::new(invocation.region.as_str().to_owned()))
                .endpoint_url(invocation.endpoint)
                .credentials_provider(invocation.provider.as_sdk_provider())
                .retry_config(one_attempt_retry_config())
                .build();
            let client = aws_sdk_bedrockruntime::Client::from_conf(config);
            let message = Message::builder()
                .role(ConversationRole::User)
                .content(ContentBlock::Text(invocation.prompt))
                .build()
                .map_err(|_| {
                    failure(
                        "swallowtail.bedrock.request_build_failed",
                        "Bedrock Runtime request could not be constructed",
                    )
                })?;
            let inference = InferenceConfiguration::builder()
                .max_tokens(invocation.maximum_output_tokens)
                .build();

            let send = client
                .converse_stream()
                .model_id(invocation.model)
                .messages(message)
                .inference_config(inference)
                .send();
            tokio::pin!(send);
            let mut response = tokio::select! {
                biased;
                changed = cancelled.changed() => {
                    let _ = changed;
                    return Ok(());
                }
                response = &mut send => response.map_err(|error| {
                    error.as_service_error().map_or_else(
                        || failure("swallowtail.bedrock.transport_failed", "Bedrock Runtime transport failed"),
                        |service| provider_failure(classify_converse_failure(service)),
                    )
                })?,
            };

            let mut decoder = StreamDecoder::default();
            loop {
                let received = tokio::select! {
                    biased;
                    changed = cancelled.changed() => {
                        let _ = changed;
                        return Ok(());
                    }
                    received = response.stream.recv() => received,
                };
                match received {
                    Ok(Some(event)) => {
                        let update = decoder.push(event).map_err(|_| {
                            failure(
                                "swallowtail.bedrock.stream_decode_failed",
                                "Bedrock Runtime stream semantics were invalid or unsupported",
                            )
                        })?;
                        if !send_update_or_cancel(&mut updates, &mut cancelled, update).await? {
                            return Ok(());
                        }
                    }
                    Ok(None) if decoder.is_complete() => return Ok(()),
                    Ok(None) => {
                        return Err(failure(
                            "swallowtail.bedrock.stream_disconnected",
                            "Bedrock Runtime stream closed before message completion",
                        ));
                    }
                    Err(error) => {
                        return Err(error.as_service_error().map_or_else(
                            || {
                                failure(
                                    "swallowtail.bedrock.transport_failed",
                                    "Bedrock Runtime transport failed",
                                )
                            },
                            |service| provider_failure(classify_output_failure(service)),
                        ));
                    }
                }
            }
        })
    }
}

async fn send_update_or_cancel(
    updates: &mut Sender<Result<StreamUpdate, RuntimeFailure>>,
    cancelled: &mut watch::Receiver<bool>,
    update: StreamUpdate,
) -> Result<bool, RuntimeFailure> {
    if *cancelled.borrow() {
        return Ok(false);
    }
    let sent = tokio::select! {
        biased;
        changed = cancelled.changed() => {
            let _ = changed;
            return Ok(false);
        }
        sent = updates.send(Ok(update)) => sent,
    };
    sent.map_err(|_| {
        failure(
            "swallowtail.bedrock.event_receiver_closed",
            "Bedrock Runtime event receiver closed before completion",
        )
    })?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_channel::mpsc;
    use std::future::Future;
    use std::task::{Context, Poll, Waker};

    #[test]
    fn send_update_or_cancel_returns_on_cancellation_when_channel_is_full() {
        let (mut sender, mut receiver) = mpsc::channel(0);
        sender
            .try_send(Ok(StreamUpdate::MessageStarted))
            .expect("channel accepts initial buffered update");
        assert!(
            sender
                .try_send(Ok(StreamUpdate::ContentBlockStopped))
                .expect_err("channel must now be full")
                .is_full()
        );

        let (cancel_sender, mut cancel_receiver) = watch::channel(false);
        let mut send_future = Box::pin(send_update_or_cancel(
            &mut sender,
            &mut cancel_receiver,
            StreamUpdate::ContentBlockStopped,
        ));

        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        assert!(
            matches!(send_future.as_mut().poll(&mut context), Poll::Pending),
            "send must block while channel is full and receiver is alive"
        );

        cancel_sender.send(true).expect("cancellation signal sends");
        let outcome = send_future.as_mut().poll(&mut context);
        assert!(
            matches!(outcome, Poll::Ready(Ok(false))),
            "cancellation must unblock pending send cleanly"
        );
        drop(send_future);

        // Receiver must still hold only the original unconsumed update.
        let first = receiver
            .try_recv()
            .expect("initial update remains unconsumed")
            .expect("initial update is Ok");
        assert!(matches!(first, StreamUpdate::MessageStarted));
        assert!(
            receiver.try_recv().is_err(),
            "cancelled send must not enqueue a second update"
        );
    }
}
