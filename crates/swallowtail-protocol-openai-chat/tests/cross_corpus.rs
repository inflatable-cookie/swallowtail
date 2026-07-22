use swallowtail_protocol_openai_chat::{
    CodecLimits, Payload, SseDecoder, SseRecord, decode_payload,
};

const LLAMA_SUCCESS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../swallowtail-adapter-llama-cpp/tests/fixtures/llama-cpp-b9910-openai-chat/success.sse"
));
const KIMI_SUCCESS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/kimi-platform-k3-2026-07-21/success.sse"
));

#[test]
fn independent_llama_cpp_and_kimi_corpora_pass_the_same_structural_decoder() {
    let llama = decode_fragmented(LLAMA_SUCCESS);
    let kimi = decode_fragmented(KIMI_SUCCESS);
    assert!(
        llama
            .iter()
            .any(|payload| matches!(payload, Payload::Chunk(_)))
    );
    assert!(
        kimi.iter()
            .any(|payload| matches!(payload, Payload::Chunk(_)))
    );
}

fn decode_fragmented(bytes: &[u8]) -> Vec<Payload> {
    let mut decoder = SseDecoder::default();
    let mut records = Vec::new();
    for fragment in bytes.chunks(7) {
        records.extend(decoder.push(fragment).expect("SSE fragment decodes"));
    }
    decoder.finish().expect("SSE stream completes");
    assert!(matches!(records.last(), Some(SseRecord::Done)));
    records
        .into_iter()
        .filter_map(|record| match record {
            SseRecord::Data(data) => {
                Some(decode_payload(&data, CodecLimits::default()).expect("payload decodes"))
            }
            SseRecord::Done => None,
        })
        .collect()
}
