use serde_json::{Value, json};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use tungstenite::handshake::server::{Request, Response};
use tungstenite::{Error, Message, WebSocket, accept_hdr};

const LIVE_PATH: &str =
    "/ws/google.ai.generativelanguage.v1beta.GenerativeService.BidiGenerateContent";

#[derive(Clone, Copy)]
pub enum LiveScenario {
    TwoTurnsRollover,
    MissingHandle,
    ReplacementFailure,
    SecondGoAway,
    Disconnect,
    ProviderFailure,
    UnknownEvent,
    FormatDrift,
    Wait,
}

pub struct LiveFixtureServer {
    endpoint: String,
    address: String,
    frames: Arc<Mutex<Vec<String>>>,
    handshakes: Arc<Mutex<Vec<String>>>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl LiveFixtureServer {
    pub fn start(scenario: LiveScenario) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fixture listener binds");
        let address = listener.local_addr().expect("address exists").to_string();
        let endpoint = format!("ws://{address}{LIVE_PATH}");
        let frames = Arc::new(Mutex::new(Vec::new()));
        let handshakes = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let thread_frames = Arc::clone(&frames);
        let thread_handshakes = Arc::clone(&handshakes);
        let thread_stop = Arc::clone(&stop);
        let thread = thread::spawn(move || {
            run(
                listener,
                scenario,
                &thread_frames,
                &thread_handshakes,
                &thread_stop,
            );
        });
        Self {
            endpoint,
            address,
            frames,
            handshakes,
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

    pub fn handshakes(&self) -> Vec<String> {
        self.handshakes
            .lock()
            .expect("handshake lock available")
            .clone()
    }

    pub fn wait_for_frames(&self, count: usize) {
        let deadline = Instant::now() + Duration::from_secs(2);
        while self.frames().len() < count {
            assert!(Instant::now() < deadline, "fixture frame did not arrive");
            thread::yield_now();
        }
    }
}

impl Drop for LiveFixtureServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(&self.address);
        if let Some(thread) = self.thread.take() {
            thread.join().expect("fixture server joins");
        }
    }
}

fn run(
    listener: TcpListener,
    scenario: LiveScenario,
    frames: &Arc<Mutex<Vec<String>>>,
    handshakes: &Arc<Mutex<Vec<String>>>,
    stop: &AtomicBool,
) {
    let Some(mut first) = accept(&listener, handshakes, stop) else {
        return;
    };
    record(&mut first, frames);
    send(&mut first, json!({"setupComplete": {}}));
    read_turn(&mut first, frames);

    match scenario {
        LiveScenario::Disconnect => (),
        LiveScenario::Wait => wait_for_close(&mut first),
        LiveScenario::ProviderFailure => {
            send(
                &mut first,
                json!({"error":{"code":429,"message":"fixture private provider failure"}}),
            );
            wait_for_close(&mut first);
        }
        LiveScenario::UnknownEvent => {
            send(&mut first, json!({"futureEvent":{"hidden":"private"}}));
            wait_for_close(&mut first);
        }
        LiveScenario::FormatDrift => {
            send(
                &mut first,
                json!({"serverContent":{"modelTurn":{"parts":[{"inlineData":{"mimeType":"audio/pcm;rate=16000","data":"AQIDBA=="}}]}}}),
            );
            wait_for_close(&mut first);
        }
        LiveScenario::MissingHandle => {
            send(
                &mut first,
                json!({"sessionResumptionUpdate":{"resumable":false}}),
            );
            send(&mut first, json!({"goAway":{"timeLeft":"15s"}}));
            send_success(&mut first, 1);
            wait_for_close(&mut first);
        }
        scenario => {
            send(
                &mut first,
                json!({"sessionResumptionUpdate":{"resumable":true,"newHandle":"fixture-private-stale-handle"}}),
            );
            send(
                &mut first,
                json!({"sessionResumptionUpdate":{"resumable":true,"newHandle":"fixture-private-handle-2"}}),
            );
            send(&mut first, json!({"goAway":{"timeLeft":"15s"}}));
            send_success(&mut first, 1);
            let Some(mut second) = accept(&listener, handshakes, stop) else {
                return;
            };
            record(&mut second, frames);
            if matches!(scenario, LiveScenario::ReplacementFailure) {
                drop(second);
                wait_for_close(&mut first);
                return;
            }
            send(&mut second, json!({"setupComplete": {}}));
            wait_for_close(&mut first);
            read_turn(&mut second, frames);
            if matches!(scenario, LiveScenario::SecondGoAway) {
                send(&mut second, json!({"goAway":{"timeLeft":"10s"}}));
            }
            send_success(&mut second, 2);
            wait_for_close(&mut second);
        }
    }
}

fn accept(
    listener: &TcpListener,
    handshakes: &Arc<Mutex<Vec<String>>>,
    stop: &AtomicBool,
) -> Option<WebSocket<TcpStream>> {
    let (stream, _) = listener.accept().ok()?;
    if stop.load(Ordering::SeqCst) {
        return None;
    }
    stream.set_read_timeout(Some(Duration::from_secs(2))).ok()?;
    let seen = Arc::clone(handshakes);
    #[expect(
        clippy::result_large_err,
        reason = "the tungstenite callback fixes the response type"
    )]
    let callback = move |request: &Request, response: Response| {
        let target = request
            .uri()
            .path_and_query()
            .map(ToString::to_string)
            .unwrap_or_default();
        seen.lock().expect("handshake lock available").push(target);
        Ok(response)
    };
    accept_hdr(stream, callback).ok()
}

fn read_turn(socket: &mut WebSocket<TcpStream>, frames: &Arc<Mutex<Vec<String>>>) {
    for expected in ["activityStart", "audio", "activityEnd"] {
        let frame: Value = serde_json::from_str(&record(socket, frames)).expect("client JSON");
        assert!(
            frame["realtimeInput"].get(expected).is_some(),
            "expected {expected} frame"
        );
    }
}

fn send_success(socket: &mut WebSocket<TcpStream>, turn: u32) {
    for frame in [
        json!({"serverContent":{"modelTurn":{"parts":[{"inlineData":{"mimeType":"audio/pcm;rate=24000","data":"AQIDBA=="}}]}}}),
        json!({"serverContent":{"outputTranscription":{"text":format!("fixture transcript {turn}")}}}),
        json!({"usageMetadata":{"promptTokenCount":12,"responseTokenCount":7}}),
        json!({"serverContent":{"generationComplete":true}}),
        json!({"serverContent":{"turnComplete":true}}),
    ] {
        send(socket, frame);
    }
}

fn record(socket: &mut WebSocket<TcpStream>, frames: &Arc<Mutex<Vec<String>>>) -> String {
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
            Message::Ping(bytes) => socket.send(Message::Pong(bytes)).unwrap(),
            message => panic!("unexpected fixture message: {message:?}"),
        }
    }
}

fn send(socket: &mut WebSocket<TcpStream>, frame: Value) {
    socket
        .send(Message::Text(frame.to_string().into()))
        .expect("fixture event sends");
}

fn wait_for_close(socket: &mut WebSocket<TcpStream>) {
    loop {
        match socket.read() {
            Ok(Message::Close(_)) | Err(Error::ConnectionClosed | Error::AlreadyClosed) => return,
            Err(Error::Io(_) | Error::Protocol(_)) => return,
            Ok(_) => {}
            Err(error) => panic!("unexpected fixture read error: {error}"),
        }
    }
}
