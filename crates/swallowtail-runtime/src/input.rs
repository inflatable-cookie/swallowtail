use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputValueRequired {
    field: &'static str,
}

impl InputValueRequired {
    pub(crate) const fn new(field: &'static str) -> Self {
        Self { field }
    }

    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }
}

impl fmt::Display for InputValueRequired {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} must not be empty", self.field)
    }
}

impl Error for InputValueRequired {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputLimitExceeded {
    field: &'static str,
    maximum: usize,
    actual: usize,
}

impl InputLimitExceeded {
    pub(crate) const fn new(field: &'static str, maximum: usize, actual: usize) -> Self {
        Self {
            field,
            maximum,
            actual,
        }
    }

    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }

    #[must_use]
    pub const fn maximum(&self) -> usize {
        self.maximum
    }

    #[must_use]
    pub const fn actual(&self) -> usize {
        self.actual
    }
}

impl fmt::Display for InputLimitExceeded {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} exceeds its {} byte limit",
            self.field, self.maximum
        )
    }
}

impl Error for InputLimitExceeded {}

pub(crate) fn required_text(
    field: &'static str,
    value: impl Into<String>,
) -> Result<String, InputValueRequired> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(InputValueRequired::new(field))
    } else {
        Ok(value)
    }
}
