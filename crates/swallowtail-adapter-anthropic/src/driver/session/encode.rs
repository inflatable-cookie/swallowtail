fn encode_json_string(out: &mut RedactedBytes, value: &str) -> Result<(), RuntimeFailure> {
    serde_json::to_writer(&mut *out, value).map_err(|_| history_failure())
}

fn encode_canonical_json(
    out: &mut RedactedBytes,
    value: serde_json::Value,
) -> Result<(), RuntimeFailure> {
    serde_json::to_writer(
        &mut *out,
        &crate::protocol::canonicalize_json_object_order(value),
    )
    .map_err(|_| history_failure())
}

fn encode_user_message(content: &str) -> Result<RedactedBytes, RuntimeFailure> {
    let mut out = RedactedBytes::from_vec(Vec::new());
    out.extend_from_slice(br#"[{"content":"#);
    encode_json_string(&mut out, content)?;
    out.extend_from_slice(br#","role":"user"}]"#);
    Ok(out)
}

impl PrivateBlock {
    fn write_json(&self, out: &mut RedactedBytes) -> Result<(), RuntimeFailure> {
        match self {
            Self::Thinking { signature } => {
                out.extend_from_slice(br#"{"signature":"#);
                encode_json_string(out, signature.as_str()?)?;
                out.extend_from_slice(br#","thinking":"","type":"thinking"}"#);
            }
            Self::Redacted { data } => {
                out.extend_from_slice(br#"{"data":"#);
                encode_json_string(out, data.as_str()?)?;
                out.extend_from_slice(br#","type":"redacted_thinking"}"#);
            }
        }
        Ok(())
    }
}

impl History {
    fn continuation_messages(&self) -> Result<RedactedBytes, RuntimeFailure> {
        let first = self.first.as_ref().ok_or_else(history_failure)?;
        let mut out = RedactedBytes::from_vec(Vec::new());
        out.extend_from_slice(br#"[{"content":"#);
        encode_json_string(&mut out, first.user.as_str()?)?;
        out.extend_from_slice(br#","role":"user"},{"content":["#);
        for (index, block) in first.private.iter().enumerate() {
            if index > 0 {
                out.push(b',');
            }
            block.write_json(&mut out)?;
        }
        if !first.private.is_empty() {
            out.push(b',');
        }
        out.extend_from_slice(br#"{"id":"#);
        encode_json_string(&mut out, &first.call_id)?;
        out.extend_from_slice(br#","input":"#);
        encode_canonical_json(
            &mut out,
            serde_json::from_slice(&first.arguments.0).map_err(|_| history_failure())?,
        )?;
        out.extend_from_slice(br#","name":"#);
        encode_json_string(&mut out, &first.tool_name)?;
        out.extend_from_slice(
            br#","type":"tool_use"}],"role":"assistant"},{"content":[{"content":"#,
        );
        encode_json_string(
            &mut out,
            first
                .result
                .as_ref()
                .ok_or_else(history_failure)?
                .as_str()?,
        )?;
        out.extend_from_slice(br#","tool_use_id":"#);
        encode_json_string(&mut out, &first.call_id)?;
        out.extend_from_slice(br#","type":"tool_result"}],"role":"user"}]"#);
        Ok(out)
    }

    fn later_messages(&self, user: &str) -> Result<RedactedBytes, RuntimeFailure> {
        let mut out = self.continuation_messages()?;
        if out.pop() != Some(b']') {
            return Err(history_failure());
        }
        let answer = self
            .first
            .as_ref()
            .and_then(|first| first.answer.as_ref())
            .ok_or_else(history_failure)?
            .as_str()?;
        out.extend_from_slice(br#",{"content":"#);
        encode_json_string(&mut out, answer)?;
        out.extend_from_slice(br#","role":"assistant"},{"content":"#);
        encode_json_string(&mut out, user)?;
        out.extend_from_slice(br#","role":"user"}]"#);
        Ok(out)
    }
}
