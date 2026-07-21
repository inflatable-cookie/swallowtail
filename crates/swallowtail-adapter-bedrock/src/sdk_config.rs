use aws_sdk_bedrockruntime::config::retry::RetryConfig;

pub(crate) fn one_attempt_retry_config() -> RetryConfig {
    RetryConfig::standard().with_max_attempts(1)
}

pub(crate) fn catalogue_one_attempt_retry_config() -> aws_sdk_bedrock::config::retry::RetryConfig {
    aws_sdk_bedrock::config::retry::RetryConfig::standard().with_max_attempts(1)
}

#[cfg(test)]
mod tests {
    use super::{catalogue_one_attempt_retry_config, one_attempt_retry_config};

    #[test]
    fn generated_sdk_retry_configuration_is_one_attempt() {
        assert_eq!(one_attempt_retry_config().max_attempts(), 1);
        assert_eq!(catalogue_one_attempt_retry_config().max_attempts(), 1);
    }
}
