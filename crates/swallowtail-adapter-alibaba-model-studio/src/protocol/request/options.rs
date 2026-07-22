use crate::failure::AlibabaProtocolFailure;
use crate::selection::EXACT_MODEL_ID;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TurnOptions {
    model: String,
    stream: bool,
    store: bool,
    reasoning_effort: String,
    tools: usize,
    session_cache: bool,
    background: bool,
    retries: u32,
    previous_response: bool,
    maximum_output_tokens: Option<u64>,
    fallback: bool,
}

impl TurnOptions {
    #[must_use]
    pub fn frozen() -> Self {
        Self {
            model: EXACT_MODEL_ID.to_owned(),
            stream: true,
            store: false,
            reasoning_effort: "none".to_owned(),
            tools: 0,
            session_cache: false,
            background: false,
            retries: 0,
            previous_response: false,
            maximum_output_tokens: None,
            fallback: false,
        }
    }

    #[must_use]
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    #[must_use]
    pub const fn with_stream(mut self, value: bool) -> Self {
        self.stream = value;
        self
    }

    #[must_use]
    pub const fn with_store(mut self, value: bool) -> Self {
        self.store = value;
        self
    }

    #[must_use]
    pub fn with_reasoning_effort(mut self, value: impl Into<String>) -> Self {
        self.reasoning_effort = value.into();
        self
    }

    #[must_use]
    pub const fn with_tools(mut self, value: usize) -> Self {
        self.tools = value;
        self
    }

    #[must_use]
    pub const fn with_session_cache(mut self, value: bool) -> Self {
        self.session_cache = value;
        self
    }

    #[must_use]
    pub const fn with_background(mut self, value: bool) -> Self {
        self.background = value;
        self
    }

    #[must_use]
    pub const fn with_retries(mut self, value: u32) -> Self {
        self.retries = value;
        self
    }

    #[must_use]
    pub const fn with_previous_response(mut self, value: bool) -> Self {
        self.previous_response = value;
        self
    }

    #[must_use]
    pub const fn with_maximum_output_tokens(mut self, value: u64) -> Self {
        self.maximum_output_tokens = Some(value);
        self
    }

    #[must_use]
    pub const fn with_fallback(mut self, value: bool) -> Self {
        self.fallback = value;
        self
    }

    pub(super) fn validate(&self) -> Result<(), AlibabaProtocolFailure> {
        let unsupported = if self.model != EXACT_MODEL_ID {
            Some("model aliases or substitutions")
        } else if !self.stream {
            Some("non-streaming turns")
        } else if self.store {
            Some("provider response storage")
        } else if self.reasoning_effort != "none" {
            Some("reasoning output")
        } else if self.tools != 0 {
            Some("tools")
        } else if self.session_cache {
            Some("session cache")
        } else if self.background {
            Some("background execution")
        } else if self.retries != 0 {
            Some("automatic retry")
        } else if self.previous_response {
            Some("previous_response_id")
        } else if self.maximum_output_tokens.is_some() {
            Some("output bounds")
        } else if self.fallback {
            Some("provider fallback")
        } else {
            None
        };
        unsupported.map_or(Ok(()), |feature| {
            Err(AlibabaProtocolFailure::unsupported(feature))
        })
    }
}

impl Default for TurnOptions {
    fn default() -> Self {
        Self::frozen()
    }
}
