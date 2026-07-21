use tungstenite::client::IntoClientRequest;
use tungstenite::http::{HeaderValue, header::AUTHORIZATION};
use tungstenite::{Message, WebSocket, connect, stream::MaybeTlsStream};

use super::{Conversation, Event, parse_event};

pub type FixtureSocket = WebSocket<MaybeTlsStream<std::net::TcpStream>>;

pub fn authenticated_connect(endpoint: &str) -> FixtureSocket {
    connect_request(endpoint, "Bearer fixture-secret").expect("fixture websocket connects")
}

pub fn connect_request(
    endpoint: &str,
    authorization: &str,
) -> Result<FixtureSocket, tungstenite::Error> {
    let mut request = endpoint.into_client_request()?;
    request.headers_mut().insert(
        AUTHORIZATION,
        HeaderValue::from_str(authorization).expect("fixture authorization is valid"),
    );
    connect(request).map(|(socket, _)| socket)
}

pub fn read_turn(socket: &mut FixtureSocket, conversation: &mut Conversation) {
    loop {
        let Message::Text(frame) = socket.read().expect("fixture event reads") else {
            panic!("fixture event is text");
        };
        let terminal = matches!(parse_event(&frame).unwrap(), Event::Completed { .. });
        conversation.apply(parse_event(&frame).unwrap()).unwrap();
        if terminal {
            break;
        }
    }
}
