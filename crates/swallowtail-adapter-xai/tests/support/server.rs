use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde_json::Value;
use tungstenite::handshake::server::{Request, Response};
use tungstenite::{Error, Message, accept_hdr};

use swallowtail_adapter_xai::RESPONSES_WEBSOCKET_PATH;

const FIRST_EVENTS: &str =
    include_str!("../fixtures/xai-responses-websocket-2026-04-23/first-turn-events.ndjson");
const SECOND_EVENTS: &str =
    include_str!("../fixtures/xai-responses-websocket-2026-04-23/second-turn-events.ndjson");
const PREVIOUS_NOT_FOUND: &str =
    include_str!("../fixtures/xai-responses-websocket-2026-04-23/previous-response-not-found.json");
const CONNECTION_LIMIT: &str =
    include_str!("../fixtures/xai-responses-websocket-2026-04-23/connection-limit.json");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServerScenario {
    Success,
    Disconnect,
    WaitForClientClose,
    PreviousResponseNotFound,
    ConnectionLimit,
}

type HandshakeRecord = Arc<Mutex<Option<(String, Option<String>)>>>;

pub struct FixtureServer {
    endpoint: String,
    frames: Arc<Mutex<Vec<String>>>,
    handshake: HandshakeRecord,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl FixtureServer {
    pub fn start(scenario: ServerScenario) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener binds");
        let endpoint = format!("ws://{}", listener.local_addr().expect("address exists"));
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
                .expect("read timeout sets");
            #[expect(
                clippy::result_large_err,
                reason = "the tungstenite handshake callback fixes this response type"
            )]
            let callback = |request: &Request, response: Response| {
                verify_handshake(request, response, &server_handshake)
            };
            let Ok(mut socket) = accept_hdr(stream, callback) else {
                return;
            };

            match scenario {
                ServerScenario::Success => {
                    let first = read_text(&mut socket);
                    validate_turn(&first, false);
                    server_frames
                        .lock()
                        .expect("frame lock available")
                        .push(first);
                    send_events(&mut socket, FIRST_EVENTS);

                    let second = read_text(&mut socket);
                    validate_turn(&second, true);
                    server_frames
                        .lock()
                        .expect("frame lock available")
                        .push(second);
                    send_events(&mut socket, SECOND_EVENTS);
                    let _ = socket.close(None);
                }
                ServerScenario::Disconnect => {
                    let first = read_text(&mut socket);
                    validate_turn(&first, false);
                    server_frames
                        .lock()
                        .expect("frame lock available")
                        .push(first);
                    for event in FIRST_EVENTS.lines().take(2) {
                        socket
                            .send(Message::Text(event.to_owned().into()))
                            .expect("fixture event sends");
                    }
                    drop(socket);
                }
                ServerScenario::WaitForClientClose => {
                    let first = read_text(&mut socket);
                    validate_turn(&first, false);
                    server_frames
                        .lock()
                        .expect("frame lock available")
                        .push(first);
                    loop {
                        match socket.read() {
                            Ok(Message::Text(text)) => server_frames
                                .lock()
                                .expect("frame lock available")
                                .push(text.to_string()),
                            Ok(Message::Close(_))
                            | Err(Error::ConnectionClosed | Error::AlreadyClosed) => break,
                            Err(Error::Protocol(_)) => break,
                            Err(Error::Io(_)) => break,
                            Ok(_) => {}
                            Err(error) => panic!("unexpected fixture read error: {error}"),
                        }
                    }
                }
                ServerScenario::PreviousResponseNotFound => {
                    send_error(&mut socket, &server_frames, PREVIOUS_NOT_FOUND);
                }
                ServerScenario::ConnectionLimit => {
                    send_error(&mut socket, &server_frames, CONNECTION_LIMIT);
                }
            }
        });
        Self {
            endpoint,
            frames,
            handshake,
            stop,
            thread: Some(thread),
        }
    }

    pub fn endpoint(&self) -> String {
        format!("{}{RESPONSES_WEBSOCKET_PATH}", self.endpoint)
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

impl Drop for FixtureServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(self.endpoint.trim_start_matches("ws://"));
        if let Some(thread) = self.thread.take() {
            thread.join().expect("fixture server joins");
        }
    }
}

fn read_text(socket: &mut tungstenite::WebSocket<TcpStream>) -> String {
    loop {
        match socket.read().expect("fixture request reads") {
            Message::Text(text) => return text.to_string(),
            Message::Ping(bytes) => socket
                .send(Message::Pong(bytes))
                .expect("fixture pong sends"),
            message => panic!("unexpected fixture message: {message:?}"),
        }
    }
}

fn validate_turn(frame: &str, chained: bool) {
    let request: Value = serde_json::from_str(frame).expect("request JSON parses");
    assert_eq!(request["type"], "response.create");
    assert_eq!(request["model"], "grok-fixture-exact");
    assert_eq!(request["store"], false);
    assert_eq!(request["tools"], serde_json::json!([]));
    assert!(request.get("stream").is_none());
    assert!(request.get("background").is_none());
    assert!(request.get("generate").is_none());
    if chained {
        assert_eq!(request["previous_response_id"], "resp_fixture_first");
    } else {
        assert!(request.get("previous_response_id").is_none());
    }
}

fn send_events(socket: &mut tungstenite::WebSocket<TcpStream>, events: &str) {
    for event in events.lines() {
        socket
            .send(Message::Text(event.to_owned().into()))
            .expect("fixture event sends");
    }
}

fn send_error(
    socket: &mut tungstenite::WebSocket<TcpStream>,
    frames: &Arc<Mutex<Vec<String>>>,
    error: &str,
) {
    let request = read_text(socket);
    validate_turn(&request, false);
    frames.lock().expect("frame lock available").push(request);
    socket
        .send(Message::Text(error.trim().to_owned().into()))
        .expect("fixture error sends");
    let _ = socket.close(None);
}

#[expect(
    clippy::result_large_err,
    reason = "the tungstenite handshake callback fixes this response type"
)]
fn verify_handshake(
    request: &Request,
    response: Response,
    record: &HandshakeRecord,
) -> Result<Response, tungstenite::handshake::server::ErrorResponse> {
    let path = request.uri().path().to_owned();
    let authorization = request
        .headers()
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    *record.lock().expect("handshake lock available") = Some((path.clone(), authorization.clone()));
    if path != RESPONSES_WEBSOCKET_PATH || authorization.as_deref() != Some("Bearer fixture-secret")
    {
        return Err(http_error());
    }
    Ok(response)
}

fn http_error() -> tungstenite::handshake::server::ErrorResponse {
    tungstenite::http::Response::builder()
        .status(401)
        .body(Some("fixture handshake rejected".to_owned()))
        .expect("fixture rejection builds")
}
