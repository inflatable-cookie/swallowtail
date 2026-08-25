fn encode_json_string(out: &mut Vec<u8>, value: &str) -> Result<(), RuntimeFailure> {
    serde_json::to_writer(&mut *out, value).map_err(|_| history_failure())
}

fn encode_canonical_json(
    out: &mut Vec<u8>,
    value: serde_json::Value,
) -> Result<(), RuntimeFailure> {
    let bytes = serde_json::to_vec(&crate::protocol::canonicalize_json_object_order(value))
        .map_err(|_| history_failure())?;
    out.extend_from_slice(&bytes);
    Ok(())
}

fn encode_user_message(content: &str) -> Result<RedactedBytes, RuntimeFailure> {
    let mut out = Vec::new();
    out.extend(br#"[{"content":"#);
    encode_json_string(&mut out, content)?;
    out.extend(br#","role":"user"}]"#);
    Ok(RedactedBytes::from_vec(out))
}

impl PrivateBlock {
    fn write_json(&self, out: &mut Vec<u8>) -> Result<(), RuntimeFailure> {
        match self {
            Self::Thinking { signature } => {
                out.extend(br#"{"signature":"#);
                encode_json_string(out, signature.as_str()?)?;
                out.extend(br#","thinking":"","type":"thinking"}"#);
            }
            Self::Redacted { data } => {
                out.extend(br#"{"data":"#);
                encode_json_string(out, data.as_str()?)?;
                out.extend(br#","type":"redacted_thinking"}"#);
            }
        }
        Ok(())
    }
}

impl History {
    fn continuation_messages(&self) -> Result<RedactedBytes, RuntimeFailure> {
        let first = self.first.as_ref().ok_or_else(history_failure)?;
        let mut out = Vec::new();
        out.extend(br#"[{"content":"#);
        encode_json_string(&mut out, first.user.as_str()?)?;
        out.extend(br#","role":"user"},{"content":["#);
        for (index, block) in first.private.iter().enumerate() {
            if index > 0 {
                out.push(b',');
            }
            block.write_json(&mut out)?;
        }
        if !first.private.is_empty() {
            out.push(b',');
        }
        out.extend(br#"{"id":"#);
        encode_json_string(&mut out, &first.call_id)?;
        out.extend(br#","input":"#);
        encode_canonical_json(
            &mut out,
            serde_json::from_slice(&first.arguments.0).map_err(|_| history_failure())?,
        )?;
        out.extend(br#","name":"#);
        encode_json_string(&mut out, &first.tool_name)?;
        out.extend(br#","type":"tool_use"}],"role":"assistant"},{"content":[{"content":"#);
        encode_json_string(
            &mut out,
            first
                .result
                .as_ref()
                .ok_or_else(history_failure)?
                .as_str()?,
        )?;
        out.extend(br#","tool_use_id":"#);
        encode_json_string(&mut out, &first.call_id)?;
        out.extend(br#","type":"tool_result"}],"role":"user"}]"#);
        Ok(RedactedBytes::from_vec(out))
    }

    fn later_messages(&self, user: &str) -> Result<RedactedBytes, RuntimeFailure> {
        let mut out = self.continuation_messages()?.into_vec();
        if out.pop() != Some(b']') {
            return Err(history_failure());
        }
        let answer = self
            .first
            .as_ref()
            .and_then(|first| first.answer.as_ref())
            .ok_or_else(history_failure)?
            .as_str()?;
        out.extend(br#",{"content":"#);
        encode_json_string(&mut out, answer)?;
        out.extend(br#","role":"assistant"},{"content":"#);
        encode_json_string(&mut out, user)?;
        out.extend(br#","role":"user"}]"#);
        Ok(RedactedBytes::from_vec(out))
    }
}
