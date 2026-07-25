use super::io::{read_stream, require_http2, require_http2_owned};
use super::{HEADER_CONNECTION_ID, HEADER_SESSION_ID, HttpState};
use crate::error::{RemoteAcpError, protocol_error, transport_error};
use crate::wire;
use reqwest::header::{ACCEPT, CONTENT_TYPE, COOKIE};
use swallowtail_protocol_acp::Message;

impl HttpState {
    pub(super) async fn post(
        &self,
        message: &Message,
        session_id: Option<&str>,
    ) -> Result<reqwest::Response, RemoteAcpError> {
        let connection_id = self.connection_id.as_deref().ok_or_else(protocol_error)?;
        let mut request = self
            .request(reqwest::Method::POST)?
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json, text/event-stream")
            .header(HEADER_CONNECTION_ID, connection_id);
        if let Some(session_id) = session_id {
            request = request.header(HEADER_SESSION_ID, session_id);
        }
        request
            .body(wire::encode(message, self.maximum_frame_bytes())?)
            .send()
            .await
            .map_err(|_| transport_error())
            .and_then(require_http2_owned)
    }

    pub(super) async fn post_without_connection(
        &self,
        message: &Message,
    ) -> Result<reqwest::Response, RemoteAcpError> {
        self.request(reqwest::Method::POST)?
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .body(wire::encode(message, self.maximum_frame_bytes())?)
            .send()
            .await
            .map_err(|_| transport_error())
            .and_then(require_http2_owned)
    }

    pub(super) async fn open_stream(
        &mut self,
        session_id: Option<String>,
    ) -> Result<(), RemoteAcpError> {
        let connection_id = self.connection_id.as_deref().ok_or_else(protocol_error)?;
        let mut request = self
            .request(reqwest::Method::GET)?
            .header(ACCEPT, "text/event-stream")
            .header(HEADER_CONNECTION_ID, connection_id);
        if let Some(session_id) = &session_id {
            request = request.header(HEADER_SESSION_ID, session_id);
        }
        let response = request.send().await.map_err(|_| transport_error())?;
        require_http2(&response)?;
        if !response.status().is_success()
            || !response
                .headers()
                .get(CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| value.starts_with("text/event-stream"))
        {
            return Err(protocol_error());
        }
        self.cookies
            .store_response(response.headers(), &self.config.endpoint)?;
        let maximum = self.maximum_frame_bytes();
        let sender = self.stream_tx.clone();
        self.readers
            .spawn(read_stream(response, session_id.is_some(), maximum, sender));
        Ok(())
    }

    fn request(&self, method: reqwest::Method) -> Result<reqwest::RequestBuilder, RemoteAcpError> {
        let mut request = self.client.request(method, self.config.endpoint.clone());
        if let Some(cookie) = self.cookies.request_header(&self.config.endpoint)? {
            request = request.header(COOKIE, cookie);
        }
        Ok(request)
    }

    pub(super) async fn close(&mut self) -> Result<(), RemoteAcpError> {
        if let Some(connection_id) = &self.connection_id {
            let response = self
                .request(reqwest::Method::DELETE)?
                .header(HEADER_CONNECTION_ID, connection_id)
                .send()
                .await
                .map_err(|_| transport_error())?;
            require_http2(&response)?;
            if response.status() != reqwest::StatusCode::ACCEPTED {
                return Err(protocol_error());
            }
        }
        Ok(())
    }

    pub(super) async fn join_readers(&mut self) {
        self.readers.abort_all();
        while self.readers.join_next().await.is_some() {}
    }

    pub(super) fn maximum_frame_bytes(&self) -> usize {
        usize::try_from(self.config.bounds.maximum_frame_bytes().get())
            .expect("validated frame bound fits usize")
    }
}
