use crate::diagnostic::{ValueRequired, required_text};

macro_rules! text_identity {
    ($name:ident, $field:literal) => {
        #[doc = concat!("Validated, non-empty ", $field, ".")]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Creates a ", $field, " after rejecting blank text.")]
            pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
                required_text($field, value).map(Self)
            }

            /// Returns the validated text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

text_identity!(AddableRouteId, "addable route id");
text_identity!(CredentialFieldId, "credential field id");
text_identity!(ConfigFieldId, "config field id");
text_identity!(InstanceLabel, "instance label");
text_identity!(FieldLabel, "field label");
text_identity!(EnvironmentVariableName, "environment variable name");
