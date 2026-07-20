use crate::{EnvironmentRef, ExecutableRef, WorkingResourceRef};
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub struct ProcessRequest {
    executable: ExecutableRef,
    arguments: Vec<String>,
    environment: Vec<EnvironmentRef>,
    working_resource: Option<WorkingResourceRef>,
}

impl ProcessRequest {
    #[must_use]
    pub fn new(executable: ExecutableRef) -> Self {
        Self {
            executable,
            arguments: Vec::new(),
            environment: Vec::new(),
            working_resource: None,
        }
    }

    #[must_use]
    pub fn with_arguments(mut self, arguments: impl IntoIterator<Item = String>) -> Self {
        self.arguments = arguments.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_environment(
        mut self,
        environment: impl IntoIterator<Item = EnvironmentRef>,
    ) -> Self {
        self.environment = environment.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_working_resource(mut self, resource: WorkingResourceRef) -> Self {
        self.working_resource = Some(resource);
        self
    }

    #[must_use]
    pub const fn executable(&self) -> &ExecutableRef {
        &self.executable
    }

    pub fn arguments(&self) -> impl ExactSizeIterator<Item = &str> {
        self.arguments.iter().map(String::as_str)
    }

    pub fn environment(&self) -> impl ExactSizeIterator<Item = &EnvironmentRef> {
        self.environment.iter()
    }

    #[must_use]
    pub const fn working_resource(&self) -> Option<&WorkingResourceRef> {
        self.working_resource.as_ref()
    }
}

impl fmt::Debug for ProcessRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProcessRequest")
            .field("executable", &self.executable)
            .field(
                "arguments",
                &format_args!("<{} values>", self.arguments.len()),
            )
            .field("environment", &self.environment)
            .field("working_resource", &self.working_resource)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::ProcessRequest;
    use crate::ExecutableRef;

    #[test]
    fn process_arguments_are_redacted() {
        let executable = ExecutableRef::new("fixture-executable").expect("reference is valid");
        let request = ProcessRequest::new(executable).with_arguments(["secret-prompt".to_owned()]);

        assert!(!format!("{request:?}").contains("secret-prompt"));
    }
}
