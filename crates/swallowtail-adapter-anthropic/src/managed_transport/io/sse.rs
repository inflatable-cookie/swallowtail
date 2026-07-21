#[derive(Default)]
struct RawSseDecoder {
    pending: Vec<u8>,
    observed: usize,
}

impl RawSseDecoder {
    fn push(&mut self, chunk: &[u8]) -> Result<Vec<String>, RuntimeFailure> {
        self.observed = self.observed.saturating_add(chunk.len());
        if self.observed > RESPONSE_LIMIT {
            return Err(response_limit());
        }
        self.pending.extend_from_slice(chunk);
        let mut frames = Vec::new();
        while let Some((end, delimiter)) = frame_end(&self.pending) {
            let bytes: Vec<_> = self.pending.drain(..end + delimiter).collect();
            let text = std::str::from_utf8(&bytes).map_err(|_| protocol_failure())?;
            let normalized = text.replace("\r\n", "\n");
            frames.push(normalized);
        }
        Ok(frames)
    }

    fn finish(self) -> Result<(), RuntimeFailure> {
        if self.pending.iter().all(u8::is_ascii_whitespace) {
            Ok(())
        } else {
            Err(protocol_failure())
        }
    }
}

fn frame_end(bytes: &[u8]) -> Option<(usize, usize)> {
    let lf = bytes.windows(2).position(|window| window == b"\n\n").map(|end| (end, 2));
    let crlf = bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|end| (end, 4));
    match (lf, crlf) {
        (Some(left), Some(right)) => Some(if left.0 <= right.0 { left } else { right }),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

fn perform_sse(
    url: Url,
    credential: Vec<u8>,
    request: Request,
    mut sender: mpsc::Sender<ManagedStreamItem>,
    cancelled: Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    let mut easy = Easy::new();
    let credential = SecretCopy(credential);
    configure(&mut easy, &url, &credential.0, &request, &cancelled)?;
    let headers = RefCell::new(BTreeMap::new());
    let status = Cell::new(0_u32);
    let sent_headers = Cell::new(false);
    let failure_slot = RefCell::new(None);
    let error_body = RefCell::new(Vec::new());
    let mut decoder = RawSseDecoder::default();
    {
        let mut transfer = easy.transfer();
        transfer
            .header_function(|line| {
                if line.starts_with(b"HTTP/") {
                    status.set(parse_status(line).unwrap_or(0));
                }
                capture_header(line, &mut headers.borrow_mut());
                true
            })
            .map_err(curl_failure)?;
        transfer
            .write_function(|chunk| {
                if !(200..300).contains(&status.get()) {
                    let mut body = error_body.borrow_mut();
                    if body.len().saturating_add(chunk.len()) > RESPONSE_LIMIT {
                        *failure_slot.borrow_mut() = Some(response_limit());
                        return Ok(0);
                    }
                    body.extend_from_slice(chunk);
                    return Ok(chunk.len());
                }
                if !sent_headers.replace(true)
                    && send(&mut sender, ManagedStreamItem::Headers(headers.borrow().clone()))
                        .is_err()
                {
                    *failure_slot.borrow_mut() = Some(backpressure());
                    return Ok(0);
                }
                match decoder.push(chunk) {
                    Ok(frames) => {
                        for frame in frames {
                            if send(&mut sender, ManagedStreamItem::Frame(frame)).is_err() {
                                *failure_slot.borrow_mut() = Some(backpressure());
                                return Ok(0);
                            }
                        }
                        Ok(chunk.len())
                    }
                    Err(error) => {
                        *failure_slot.borrow_mut() = Some(error);
                        Ok(0)
                    }
                }
            })
            .map_err(curl_failure)?;
        let result = transfer.perform();
        if let Some(error) = failure_slot.borrow_mut().take() {
            return Err(error);
        }
        if cancelled.load(Ordering::SeqCst) {
            return Ok(());
        }
        result.map_err(curl_failure)?;
    }
    if !(200..300).contains(&easy.response_code().map_err(curl_failure)?) {
        return Err(provider_failure("event stream"));
    }
    decoder.finish()
}

fn send(
    sender: &mut mpsc::Sender<ManagedStreamItem>,
    item: ManagedStreamItem,
) -> Result<(), TrySendError<ManagedStreamItem>> {
    sender.try_send(item)
}
