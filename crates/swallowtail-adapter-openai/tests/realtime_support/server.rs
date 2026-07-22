use serde_json::{Value, json};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tungstenite::handshake::server::{Request, Response};
use tungstenite::http::HeaderValue;
use tungstenite::{Error, Message, accept_hdr};

const SESSION_EVENTS: &str =
    include_str!("../fixtures/openai-realtime-2026-07-22/success-events.jsonl");
const FORMAT_DRIFT: &str = include_str!("../fixtures/openai-realtime-2026-07-22/format-drift.json");
const PROVIDER_ERROR: &str =
    include_str!("../fixtures/openai-realtime-2026-07-22/provider-error.json");

mod scenario;

#[derive(Clone, Copy)]
pub enum RealtimeScenario {
    TwoTurns,
    Cancel,
    CancelDisconnect,
    Disconnect,
    FormatDrift,
    ProviderFailed,
    Unknown,
}

type Handshake = Arc<Mutex<Option<(String, Option<String>)>>>;

pub struct RealtimeFixtureServer {
    endpoint: String,
    frames: Arc<Mutex<Vec<String>>>,
    handshake: Handshake,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl RealtimeFixtureServer {
    pub fn start(scenario: RealtimeScenario) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener binds");
        let base = format!("ws://{}", listener.local_addr().expect("address exists"));
        let endpoint = format!("{base}/v1/realtime");
        let frames = Arc::new(Mutex::new(Vec::new()));
        let handshake = Arc::new(Mutex::new(None));
        let stop = Arc::new(AtomicBool::new(false));
        let server_frames = Arc::clone(&frames);
        let server_handshake = Arc::clone(&handshake);
        let server_stop = Arc::clone(&stop);
        let thread = thread::spawn(move || {
            let (stream, _) = listener.accept().expect("fixture connection arrives");
            if server_stop.load(Ordering::SeqCst) {
                return;
            }
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .expect("fixture timeout sets");
            #[expect(
                clippy::result_large_err,
                reason = "the tungstenite callback fixes the response type"
            )]
            let callback = |request: &Request, mut response: Response| {
                let target = request
                    .uri()
                    .path_and_query()
                    .map(ToString::to_string)
                    .unwrap_or_default();
                let authorization = request
                    .headers()
                    .get("authorization")
                    .and_then(|value| value.to_str().ok())
                    .map(str::to_owned);
                *server_handshake.lock().expect("handshake lock available") =
                    Some((target, authorization));
                response.headers_mut().insert(
                    "x-request-id",
                    HeaderValue::from_static("req_realtime_fixture"),
                );
                Ok(response)
            };
            let Ok(mut socket) = accept_hdr(stream, callback) else {
                return;
            };
            scenario::run(scenario, &mut socket, &server_frames);
        });
        Self {
            endpoint,
            frames,
            handshake,
            stop,
            thread: Some(thread),
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn frames(&self) -> Vec<String> {
        self.frames.lock().expect("frame lock available").clone()
    }

    pub fn wait_for_frames(&self, count: usize) {
        let deadline = Instant::now() + Duration::from_secs(2);
        while self.frames().len() < count {
            assert!(Instant::now() < deadline, "fixture frame did not arrive");
            thread::yield_now();
        }
    }

    pub fn handshake(&self) -> Option<(String, Option<String>)> {
        self.handshake
            .lock()
            .expect("handshake lock available")
            .clone()
    }
}

impl Drop for RealtimeFixtureServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let address = self
            .endpoint
            .trim_start_matches("ws://")
            .trim_end_matches("/v1/realtime");
        let _ = TcpStream::connect(address);
        if let Some(thread) = self.thread.take() {
            thread.join().expect("fixture server joins");
        }
    }
}

pub(super) fn read_turn_frames(
    socket: &mut tungstenite::WebSocket<TcpStream>,
    frames: &Arc<Mutex<Vec<String>>>,
) {
    loop {
        let frame = record(socket, frames);
        let value: Value = serde_json::from_str(&frame).expect("client frame is JSON");
        match value["type"].as_str() {
            Some("input_audio_buffer.commit") => send(
                socket,
                r#"{"type":"input_audio_buffer.committed","item_id":"item_private"}"#,
            ),
            Some("response.create") => return,
            Some("input_audio_buffer.append") => {}
            other => panic!("unexpected client frame: {other:?}"),
        }
    }
}

pub(super) fn send_success(socket: &mut tungstenite::WebSocket<TcpStream>, turn: u32) {
    let response = format!("resp_private_{turn}");
    for event in [
        json!({"type":"response.created","response":{"id":response,"status":"in_progress"}}),
        json!({"type":"response.output_item.added","response_id":response}),
        json!({"type":"rate_limits.updated","rate_limits":[{"name":"requests","limit":1000,"remaining":999,"reset_seconds":60.0}]}),
        json!({"type":"response.output_audio.delta","response_id":response,"delta":"CQgHBg=="}),
        json!({"type":"response.output_audio_transcript.delta","response_id":response,"delta":"Hello"}),
        json!({"type":"response.output_audio.done","response_id":response}),
        json!({"type":"response.output_audio_transcript.done","response_id":response,"transcript":"Hello"}),
        json!({"type":"response.done","response":{"id":response,"status":"completed","usage":{"input_tokens":4,"output_tokens":3}}}),
    ] {
        send(socket, &event.to_string());
    }
}

pub(super) fn record(
    socket: &mut tungstenite::WebSocket<TcpStream>,
    frames: &Arc<Mutex<Vec<String>>>,
) -> String {
    loop {
        match socket.read().expect("fixture client frame reads") {
            Message::Text(frame) => {
                let frame = frame.to_string();
                frames
                    .lock()
                    .expect("frame lock available")
                    .push(frame.clone());
                return frame;
            }
            Message::Ping(bytes) => socket
                .send(Message::Pong(bytes))
                .expect("fixture pong sends"),
            message => panic!("unexpected fixture message: {message:?}"),
        }
    }
}

pub(super) fn send(socket: &mut tungstenite::WebSocket<TcpStream>, frame: &str) {
    socket
        .send(Message::Text(frame.to_owned().into()))
        .expect("fixture event sends");
}

pub(super) fn wait_for_close(
    socket: &mut tungstenite::WebSocket<TcpStream>,
    frames: &Arc<Mutex<Vec<String>>>,
) {
    loop {
        match socket.read() {
            Ok(Message::Text(frame)) => frames
                .lock()
                .expect("frame lock available")
                .push(frame.to_string()),
            Ok(Message::Close(_)) | Err(Error::ConnectionClosed | Error::AlreadyClosed) => return,
            Err(Error::Io(_)) | Err(Error::Protocol(_)) => return,
            Ok(_) => {}
            Err(error) => panic!("unexpected fixture read error: {error}"),
        }
    }
}
