fn compression() -> GeminiLiveContextWindowCompression {
    GeminiLiveContextWindowCompression::sliding_window()
}

fn maximum(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).expect("fixture maximum is non-zero")
}

fn mode(value: &str) -> ReasoningMode {
    ReasoningMode::new(value).expect("fixture reasoning mode is valid")
}

fn setups(frames: &[String]) -> Vec<Value> {
    frames
        .iter()
        .filter_map(|frame| {
            let value: Value = serde_json::from_str(frame).expect("fixture frame is JSON");
            value.get("setup").cloned()
        })
        .collect()
}

fn compression_values(frames: &[String]) -> Vec<Option<Value>> {
    setups(frames)
        .iter()
        .map(|setup| setup.get("contextWindowCompression").cloned())
        .collect()
}

fn handles(frames: &[String]) -> Vec<Option<String>> {
    setups(frames)
        .iter()
        .map(|setup| {
            setup
                .get("sessionResumption")
                .and_then(|resumption| resumption.get("handle"))
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .collect()
}

fn levels(frames: &[String]) -> Vec<Option<String>> {
    setups(frames)
        .iter()
        .map(|setup| {
            setup
                .get("generationConfig")
                .and_then(|generation| generation.get("thinkingConfig"))
                .and_then(|thinking| thinking.get("thinkingLevel"))
                .and_then(Value::as_str)
                .map(str::to_owned)
        })
        .collect()
}

fn maxima(frames: &[String]) -> Vec<Option<u64>> {
    setups(frames)
        .iter()
        .map(|setup| {
            setup
                .get("generationConfig")
                .and_then(|generation| generation.get("maxOutputTokens"))
                .and_then(Value::as_u64)
        })
        .collect()
}

fn raw_setup_frames(frames: &[String]) -> Vec<&str> {
    frames
        .iter()
        .filter(|frame| frame.contains("\"setup\""))
        .map(String::as_str)
        .collect()
}
