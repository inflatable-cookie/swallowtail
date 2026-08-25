fn tool_specs(
    request: &OpenDirectContinuationSessionRequest,
) -> Result<Vec<ToolSpec>, RuntimeFailure> {
    request
        .options()
        .tools()
        .map(|tool| {
            if tool.schema_media_type() != "application/schema+json"
                || tool.schema_dialect() != "json-schema-2020-12"
            {
                return Err(unsupported("non-JSON-Schema tool declarations"));
            }
            let bytes = tool
                .input_schema()
                .inline_bytes()
                .ok_or_else(|| unsupported("referenced tool schemas"))?;
            let input_schema = serde_json::from_slice(bytes).map_err(|_| {
                failure(
                    "swallowtail.anthropic.tool_schema_invalid",
                    "Anthropic tool schema was not valid JSON",
                )
            })?;
            Ok(ToolSpec {
                name: tool.name().to_owned(),
                description: tool
                    .description()
                    .map_or_else(String::new, |value| value.as_str().to_owned()),
                input_schema,
            })
        })
        .collect()
}

fn build_user_messages(
    history: &Arc<Mutex<History>>,
    request: &DirectContinuationTurnRequest,
    attempt: &DirectInferenceAttempt,
) -> Result<RedactedBytes, RuntimeFailure> {
    match attempt.ordinal().get() {
        1 => encode_user_message(request.content().as_str()),
        3 => history
            .lock()
            .expect("history lock poisoned")
            .later_messages(request.content().as_str()),
        _ => Err(failure(
            "swallowtail.anthropic.attempt_sequence_invalid",
            "Anthropic user turn authorized an invalid attempt ordinal",
        )),
    }
}

struct History {
    maximum_bytes: u64,
    maximum_private_bytes: u64,
    first: Option<FirstHistory>,
}

struct FirstHistory {
    user: SecretText,
    call_id: String,
    tool_name: String,
    arguments: SecretText,
    result: Option<SecretText>,
    answer: Option<SecretText>,
    private: Vec<PrivateBlock>,
}

enum PrivateBlock {
    Thinking { signature: SecretBytes },
    Redacted { data: SecretBytes },
}

impl PrivateBlock {
    fn len(&self) -> usize {
        match self {
            Self::Thinking { signature } => signature.len(),
            Self::Redacted { data } => data.len(),
        }
    }
}

impl History {
    fn new(maximum_bytes: u64, maximum_private_bytes: u64) -> Self {
        Self {
            maximum_bytes,
            maximum_private_bytes,
            first: None,
        }
    }

    fn clear(&mut self) {
        self.first = None;
    }

    fn record_tool(
        &mut self,
        request: &DirectContinuationTurnRequest,
        call: &DirectToolCall,
        private: Vec<PrivateBlock>,
    ) -> Result<(), RuntimeFailure> {
        if self.first.is_some() {
            return Err(history_failure());
        }
        self.first = Some(FirstHistory {
            user: SecretText::new(request.content().as_str()),
            call_id: call.call_id().as_str().to_owned(),
            tool_name: call.tool_name().to_owned(),
            arguments: SecretText(call.arguments().as_bytes().to_vec()),
            result: None,
            answer: None,
            private,
        });
        self.require_bound()
    }

    fn record_result(&mut self, result: &DirectToolResult) -> Result<(), RuntimeFailure> {
        let first = self.first.as_mut().ok_or_else(history_failure)?;
        if first.result.is_some() || result.call_id().as_str() != first.call_id {
            return Err(history_failure());
        }
        first.result = Some(SecretText(result.content().as_bytes().to_vec()));
        self.require_bound()
    }

    fn record_answer(&mut self, answer: &str) -> Result<(), RuntimeFailure> {
        let first = self.first.as_mut().ok_or_else(history_failure)?;
        if first.result.is_none() || first.answer.is_some() {
            return Err(history_failure());
        }
        first.answer = Some(SecretText::new(answer));
        self.require_bound()
    }

    fn require_bound(&self) -> Result<(), RuntimeFailure> {
        let bytes = self.first.as_ref().map_or(0, |first| {
            first.user.0.len()
                + first.call_id.len()
                + first.tool_name.len()
                + first.arguments.0.len()
                + first.result.as_ref().map_or(0, |value| value.0.len())
                + first.answer.as_ref().map_or(0, |value| value.0.len())
                + first.private.iter().map(PrivateBlock::len).sum::<usize>()
        });
        let private = self
            .first
            .as_ref()
            .map_or(0, |first| first.private.iter().map(PrivateBlock::len).sum::<usize>());
        if bytes as u64 > self.maximum_bytes || private as u64 > self.maximum_private_bytes {
            Err(failure(
                "swallowtail.anthropic.history_bound_exceeded",
                "Anthropic private session history exceeded its selected bound",
            ))
        } else {
            Ok(())
        }
    }
}

struct SecretText(Vec<u8>);

impl SecretText {
    fn new(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }

    fn as_str(&self) -> Result<&str, RuntimeFailure> {
        std::str::from_utf8(&self.0).map_err(|_| history_failure())
    }
}

impl std::fmt::Debug for SecretText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("[redacted]")
    }
}

impl Drop for SecretText {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

struct SecretBytes(Vec<u8>);

impl SecretBytes {
    fn from_redacted(bytes: &RedactedBytes) -> Self {
        Self(bytes.clone_bytes())
    }

    fn as_str(&self) -> Result<&str, RuntimeFailure> {
        std::str::from_utf8(&self.0).map_err(|_| history_failure())
    }

    fn copy(&self) -> Vec<u8> {
        self.0.clone()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Drop for SecretBytes {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

impl std::fmt::Debug for SecretBytes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("[redacted]")
    }
}

