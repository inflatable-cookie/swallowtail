pub(crate) struct RedactedBytes(Vec<u8>);

impl RedactedBytes {
    pub(crate) fn from_vec(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub(crate) fn clone_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub(crate) fn extend_from_slice(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }

    pub(crate) fn push(&mut self, byte: u8) {
        self.0.push(byte);
    }

    pub(crate) fn pop(&mut self) -> Option<u8> {
        self.0.pop()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl Clone for RedactedBytes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for RedactedBytes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for RedactedBytes {}

impl std::ops::Deref for RedactedBytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl std::io::Write for RedactedBytes {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl std::fmt::Debug for RedactedBytes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("[redacted]")
    }
}

impl Drop for RedactedBytes {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

struct ZeroizingBuf(Vec<u8>);

impl Drop for ZeroizingBuf {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

struct ZeroizingJson(serde_json::Value);

impl Drop for ZeroizingJson {
    fn drop(&mut self) {
        zeroize_json(&mut self.0);
    }
}

fn zeroize_json(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::String(text) => {
            let mut bytes = std::mem::take(text).into_bytes();
            bytes.fill(0);
        }
        serde_json::Value::Array(items) => {
            for item in items {
                zeroize_json(item);
            }
        }
        serde_json::Value::Object(map) => {
            for item in map.values_mut() {
                zeroize_json(item);
            }
        }
        _ => {}
    }
}

fn take_secret(
    value: &mut serde_json::Value,
    field: &str,
    subject: &str,
) -> Result<RedactedBytes, RuntimeFailure> {
    match value
        .as_object_mut()
        .and_then(|object| object.remove(field))
    {
        Some(serde_json::Value::String(secret)) if !secret.trim().is_empty() => {
            Ok(RedactedBytes::from_vec(secret.into_bytes()))
        }
        Some(rejected) => {
            drop(ZeroizingJson(rejected));
            Err(protocol_failure(subject))
        }
        None => Err(protocol_failure(subject)),
    }
}

fn splice_direct_body(
    model: &str,
    messages: &[u8],
    tools: &[u8],
    maximum: u32,
    effort: Option<&str>,
    thinking: bool,
) -> Result<RedactedBytes, RuntimeFailure> {
    let mut body = RedactedBytes::from_vec(Vec::new());
    body.extend_from_slice(br#"{"max_tokens":"#);
    body.extend_from_slice(maximum.to_string().as_bytes());
    body.extend_from_slice(br#","messages":"#);
    body.extend_from_slice(messages);
    body.extend_from_slice(br#","model":"#);
    serde_json::to_writer(&mut body, model).map_err(|_| protocol_failure("model"))?;
    if let Some(effort) = effort {
        body.extend_from_slice(br#","output_config":{"effort":"#);
        serde_json::to_writer(&mut body, effort).map_err(|_| protocol_failure("effort"))?;
        body.push(b'}');
    }
    body.extend_from_slice(br#","stream":true"#);
    if thinking {
        body.extend_from_slice(br#","thinking":{"display":"omitted","type":"adaptive"}"#);
    }
    body.extend_from_slice(br#","tool_choice":{"type":"auto"},"tools":"#);
    body.extend_from_slice(tools);
    body.push(b'}');
    Ok(body)
}
