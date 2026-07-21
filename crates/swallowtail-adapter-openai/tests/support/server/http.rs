pub fn exchange(endpoint: &str, request: &[u8]) -> FixtureResponse {
    let mut stream = TcpStream::connect(endpoint.trim_start_matches("http://"))
        .expect("fixture endpoint connects");
    stream.write_all(request).expect("request writes");
    stream
        .shutdown(Shutdown::Write)
        .expect("request write side closes");
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).expect("response reads");
    parse_response(&bytes)
}

fn read_request(stream: &mut TcpStream) -> Option<FixtureRequest> {
    stream.set_read_timeout(Some(Duration::from_secs(2))).ok()?;
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 4096];
    loop {
        let count = stream.read(&mut chunk).ok()?;
        if count == 0 && bytes.is_empty() {
            return None;
        }
        bytes.extend_from_slice(&chunk[..count]);
        let Some(end) = bytes.windows(4).position(|value| value == b"\r\n\r\n") else {
            continue;
        };
        let head = std::str::from_utf8(&bytes[..end]).ok()?;
        let mut lines = head.lines();
        let mut start = lines.next()?.split_whitespace();
        let method = start.next()?.to_owned();
        let target = start.next()?.to_owned();
        let headers = lines
            .filter_map(|line| line.split_once(':'))
            .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().to_owned()))
            .collect::<BTreeMap<_, _>>();
        let length = headers
            .get("content-length")
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(0);
        if bytes.len() < end + 4 + length {
            continue;
        }
        return Some(FixtureRequest {
            method,
            target,
            headers,
            body: bytes[end + 4..end + 4 + length].to_vec(),
        });
    }
}

fn parse_response(bytes: &[u8]) -> FixtureResponse {
    let end = bytes
        .windows(4)
        .position(|value| value == b"\r\n\r\n")
        .expect("response head exists");
    let head = std::str::from_utf8(&bytes[..end]).expect("response head is UTF-8");
    let mut lines = head.lines();
    let status = lines
        .next()
        .expect("status exists")
        .split_whitespace()
        .nth(1)
        .expect("status code exists")
        .parse()
        .expect("status is numeric");
    let headers = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().to_owned()))
        .collect();
    FixtureResponse {
        status,
        headers,
        body: bytes[end + 4..].to_vec(),
    }
}
