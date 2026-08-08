use crate::config::TransportConfig;
use crate::cookies::BoundedCookieStore;
use crate::correlation::CorrelationState;
use crate::error::{RemoteAcpError, capacity_error, protocol_error, transport_error};
use crate::wire;
use crate::worker::{WorkerCommand, WorkerEvent, cancellation_error, race_deadline};
use futures_channel::{mpsc, oneshot};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use swallowtail_protocol_acp::Message;
use swallowtail_runtime::{Deadline, TimeService};
use tokio::task::JoinSet;

mod io;
mod request;

use io::{
    bounded_body, bounded_header, message_session, require_http2, response_connection_id,
    validate_initialize_response,
};

const HEADER_CONNECTION_ID: &str = "acp-connection-id";
const HEADER_SESSION_ID: &str = "acp-session-id";

enum StreamItem {
    Message { session: bool, text: String },
    Failed,
}

struct HttpState {
    config: TransportConfig,
    client: reqwest::Client,
    cookies: BoundedCookieStore,
    connection_id: Option<String>,
    session_id: Option<String>,
    readers: JoinSet<()>,
    stream_tx: tokio::sync::mpsc::Sender<StreamItem>,
    connection_events: u32,
    session_events: u32,
    deadline: Option<Deadline>,
    time: Option<Arc<dyn TimeService>>,
}

pub(crate) async fn run(
    config: TransportConfig,
    mut commands: mpsc::Receiver<WorkerCommand>,
    mut events: mpsc::Sender<WorkerEvent>,
    ready: oneshot::Sender<Result<(), RemoteAcpError>>,
    deadline: Option<Deadline>,
    time: Option<Arc<dyn TimeService>>,
) -> Result<(), RemoteAcpError> {
    let stream_capacity = usize::try_from(
        config
            .bounds
            .maximum_connection_stream_events()
            .get()
            .saturating_add(config.bounds.maximum_session_stream_events().get()),
    )
    .map_err(|_| capacity_error())?;
    let (stream_tx, mut stream_rx) = tokio::sync::mpsc::channel(stream_capacity);
    let mut client_builder = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .no_proxy()
        .http2_adaptive_window(true);
    if config.endpoint.scheme() == "http" {
        client_builder = client_builder.http2_prior_knowledge();
    }
    let client = client_builder.build().map_err(|_| transport_error())?;
    let cookies =
        BoundedCookieStore::new(config.maximum_cookie_count, config.maximum_cookie_bytes)?;
    let maximum_pending_requests = usize::try_from(config.bounds.maximum_pending_requests().get())
        .map_err(|_| capacity_error())?;
    let maximum_pending_callbacks =
        usize::try_from(config.bounds.maximum_pending_callbacks().get())
            .map_err(|_| capacity_error())?;
    let mut state = HttpState {
        config,
        client,
        cookies,
        connection_id: None,
        session_id: None,
        readers: JoinSet::new(),
        stream_tx,
        connection_events: 0,
        session_events: 0,
        deadline,
        time,
    };
    let mut correlation =
        CorrelationState::new(maximum_pending_requests, maximum_pending_callbacks);
    let _ = ready.send(Ok(()));

    let result = loop {
        tokio::select! {
            command = commands.next() => {
                match command {
                    Some(WorkerCommand::Send(message)) => {
                        let deadline = state.deadline;
                        let time = state.time.clone();
                        if let Err(error) = race_deadline(
                            deadline,
                            time.as_deref(),
                            state.send(message, &mut correlation, &mut events),
                        ).await
                        {
                            break Err(error);
                        }
                    }
                    Some(WorkerCommand::Cancel) => {
                        let deadline = state.deadline;
                        let time = state.time.clone();
                        let _ = race_deadline(deadline, time.as_deref(), state.close()).await;
                        break Err(cancellation_error(false));
                    }
                    Some(WorkerCommand::Deadline) => {
                        let deadline = state.deadline;
                        let time = state.time.clone();
                        let _ = race_deadline(deadline, time.as_deref(), state.close()).await;
                        break Err(cancellation_error(true));
                    }
                    Some(WorkerCommand::Close) | None => {
                        let deadline = state.deadline;
                        let time = state.time.clone();
                        match race_deadline(deadline, time.as_deref(), state.close()).await {
                            Ok(Ok(())) => break Ok(()),
                            Ok(Err(error)) | Err(error) => break Err(error),
                        }
                    }
                }
            }
            stream = race_deadline(
                state.deadline,
                state.time.as_deref(),
                stream_rx.recv(),
            ), if state.connection_id.is_some() => {
                match stream {
                    Ok(Some(StreamItem::Message { session, text })) => {
                        let deadline = state.deadline;
                        let time = state.time.clone();
                        if let Err(error) = race_deadline(
                            deadline,
                            time.as_deref(),
                            state.receive_stream(session, text, &mut correlation, &mut events),
                        ).await
                        {
                            break Err(error);
                        }
                    }
                    Ok(Some(StreamItem::Failed)) | Ok(None) => break Err(transport_error()),
                    Err(error) => break Err(error),
                }
            }
        }
    };
    state.join_readers().await;
    result
}

impl HttpState {
    async fn send(
        &mut self,
        message: Message,
        correlation: &mut CorrelationState,
        events: &mut mpsc::Sender<WorkerEvent>,
    ) -> Result<(), RemoteAcpError> {
        let callback_session = match &message {
            Message::Response { id, .. } => correlation.callback_session(id)?.map(str::to_owned),
            _ => None,
        };
        correlation.outbound(&message)?;
        if self.connection_id.is_none() {
            return self.initialize(message, correlation, events).await;
        }
        let session = callback_session.or_else(|| message_session(&message));
        let response = self.post(&message, session.as_deref()).await?;
        if response.status() != reqwest::StatusCode::ACCEPTED {
            return Err(protocol_error());
        }
        self.cookies
            .store_response(response.headers(), &self.config.endpoint)?;
        let _ = bounded_body(response, self.maximum_frame_bytes()).await?;
        Ok(())
    }

    async fn initialize(
        &mut self,
        message: Message,
        correlation: &mut CorrelationState,
        events: &mut mpsc::Sender<WorkerEvent>,
    ) -> Result<(), RemoteAcpError> {
        if !matches!(&message, Message::Request { method, .. } if method == "initialize") {
            return Err(protocol_error());
        }
        let response = self.post_without_connection(&message).await?;
        require_http2(&response)?;
        if response.status() != reqwest::StatusCode::OK {
            return Err(protocol_error());
        }
        self.cookies
            .store_response(response.headers(), &self.config.endpoint)?;
        let connection_id = bounded_header(
            response.headers(),
            HEADER_CONNECTION_ID,
            self.maximum_frame_bytes(),
        )?;
        let body = bounded_body(response, self.maximum_frame_bytes()).await?;
        let response_message = wire::decode(
            std::str::from_utf8(&body).map_err(|_| protocol_error())?,
            self.maximum_frame_bytes(),
        )?;
        let body_connection_id = response_connection_id(&response_message)?;
        if body_connection_id != connection_id {
            return Err(protocol_error());
        }
        let metadata = correlation.inbound(&response_message)?;
        validate_initialize_response(&response_message, metadata.completed_method.as_deref())?;
        self.connection_id = Some(connection_id);
        self.open_stream(None).await?;
        events
            .send(WorkerEvent::Message(response_message))
            .await
            .map_err(|_| transport_error())
    }

    async fn receive_stream(
        &mut self,
        session: bool,
        text: String,
        correlation: &mut CorrelationState,
        events: &mut mpsc::Sender<WorkerEvent>,
    ) -> Result<(), RemoteAcpError> {
        let count = if session {
            &mut self.session_events
        } else {
            &mut self.connection_events
        };
        *count = count.checked_add(1).ok_or_else(capacity_error)?;
        let maximum = if session {
            self.config.bounds.maximum_session_stream_events().get()
        } else {
            self.config.bounds.maximum_connection_stream_events().get()
        };
        if *count > maximum {
            return Err(capacity_error());
        }
        let message = wire::decode(&text, self.maximum_frame_bytes())?;
        let metadata = correlation.inbound(&message)?;
        if let Some(session_id) = metadata.opened_session {
            if self.session_id.is_some() || session_id.len() > self.maximum_frame_bytes() {
                return Err(protocol_error());
            }
            self.session_id = Some(session_id.clone());
            self.open_stream(Some(session_id)).await?;
        }
        events
            .send(WorkerEvent::Message(message))
            .await
            .map_err(|_| transport_error())
    }
}
