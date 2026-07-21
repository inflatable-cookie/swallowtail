use crate::InputValueRequired;
use crate::input::required_text;
use std::fmt;
use swallowtail_core::InstanceTargetRef;

macro_rules! opaque_host_reference {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, InputValueRequired> {
                required_text($field, value).map(Self)
            }

            #[must_use]
            pub fn as_host_value(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"<opaque>")
                    .finish()
            }
        }
    };
}

opaque_host_reference!(ExecutableRef, "executable reference");
opaque_host_reference!(EnvironmentRef, "environment reference");
opaque_host_reference!(EndpointRef, "endpoint reference");
opaque_host_reference!(WorkingResourceRef, "working resource reference");
opaque_host_reference!(AttachmentRef, "attachment reference");
opaque_host_reference!(SchemaRef, "schema reference");
opaque_host_reference!(MaterializedFileRef, "materialized file reference");
opaque_host_reference!(MaterializedResourceRef, "materialized resource reference");
opaque_host_reference!(
    MaterializedModelArtifactRef,
    "materialized model artifact reference"
);

impl ExecutableRef {
    /// Preserves the opaque configured-instance target for host resolution.
    pub fn from_instance_target(reference: &InstanceTargetRef) -> Self {
        Self(reference.as_host_value().to_owned())
    }
}

impl EndpointRef {
    /// Preserves the preflight-bound configured-instance target for host resolution.
    pub fn from_instance_target(reference: &InstanceTargetRef) -> Self {
        Self(reference.as_host_value().to_owned())
    }
}

impl MaterializedFileRef {
    /// Gives a driver the host-authorized file value while keeping formatting redacted.
    #[must_use]
    pub fn as_driver_value(&self) -> &str {
        self.as_host_value()
    }
}

impl MaterializedResourceRef {
    /// Gives a driver the host-authorized filesystem root while keeping formatting redacted.
    #[must_use]
    pub fn as_driver_value(&self) -> &str {
        self.as_host_value()
    }
}

impl MaterializedModelArtifactRef {
    /// Gives a serving driver the host-authorized artifact value while keeping formatting redacted.
    #[must_use]
    pub fn as_driver_value(&self) -> &str {
        self.as_host_value()
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutableRef;

    #[test]
    fn host_reference_is_redacted() {
        let reference =
            ExecutableRef::new("/private/provider/bin").expect("executable reference is valid");

        assert!(!format!("{reference:?}").contains(reference.as_host_value()));
    }
}
