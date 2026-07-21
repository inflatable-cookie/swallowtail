use serde_json::Value;
use swallowtail_protocol_acp::{Message, NdjsonDecoder};

const GEMINI_CORPUS: [&str; 4] = [
    include_str!("fixtures/acp-v1-gemini-cli-0.51.0/initialize.ndjson"),
    include_str!("fixtures/acp-v1-gemini-cli-0.51.0/new-session.ndjson"),
    include_str!("fixtures/acp-v1-gemini-cli-0.51.0/prompt-success.ndjson"),
    include_str!("fixtures/acp-v1-gemini-cli-0.51.0/filesystem.ndjson"),
];

const KIMI_CORPUS: [&str; 7] = [
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/initialize.ndjson"),
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/new-session.ndjson"),
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/load-success.ndjson"),
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/resume-success.ndjson"),
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/prompt-success.ndjson"),
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/filesystem-write.ndjson"),
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/cancel.ndjson"),
];

#[test]
fn one_provider_neutral_decoder_accepts_both_agent_corpora() {
    let gemini = decode_corpus(GEMINI_CORPUS);
    let kimi = decode_corpus(KIMI_CORPUS);

    for baseline in ["initialize", "session/new", "session/prompt"] {
        assert!(gemini.iter().any(|method| method == baseline));
        assert!(kimi.iter().any(|method| method == baseline));
    }
    assert!(gemini.iter().any(|method| method == "fs/read_text_file"));
    for extension in ["session/load", "session/resume", "fs/write_text_file"] {
        assert!(kimi.iter().any(|method| method == extension));
        assert!(!gemini.iter().any(|method| method == extension));
    }
}

fn decode_corpus<const N: usize>(transcripts: [&str; N]) -> Vec<String> {
    let mut decoder = NdjsonDecoder::default();
    let mut methods = Vec::new();
    for transcript in transcripts {
        for line in transcript.lines() {
            let envelope: Value = serde_json::from_str(line).expect("fixture envelope is JSON");
            let mut frame =
                serde_json::to_vec(&envelope["message"]).expect("fixture ACP message serializes");
            frame.push(b'\n');
            for message in decoder
                .push(&frame)
                .expect("shared ACP decoder accepts frame")
            {
                match message {
                    Message::Request { method, .. } | Message::Notification { method, .. } => {
                        methods.push(method);
                    }
                    Message::Response { .. } => {}
                }
            }
        }
    }
    decoder.finish().expect("corpus ends on a frame boundary");
    methods
}
