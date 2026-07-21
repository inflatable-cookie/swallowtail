fn perform_sse(
    url: Url,
    credential: Vec<u8>,
    request: Request,
    mut sender: mpsc::Sender<StreamItem>,
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
    let mut decoder = SseDecoder::default();
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
                    && send(&mut sender, StreamItem::Headers(headers.borrow().clone())).is_err()
                {
                    *failure_slot.borrow_mut() = Some(backpressure());
                    return Ok(0);
                }
                match decoder.push(chunk) {
                    Ok(frames) => {
                        for frame in frames {
                            if send(&mut sender, StreamItem::Frame(frame)).is_err() {
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
    let response = Response {
        status: easy.response_code().map_err(curl_failure)?,
        headers: headers.into_inner(),
        body: error_body.into_inner(),
    };
    require_success(&response)?;
    decoder.finish()
}

fn send(
    sender: &mut mpsc::Sender<StreamItem>,
    item: StreamItem,
) -> Result<(), TrySendError<StreamItem>> {
    sender.try_send(item)
}
