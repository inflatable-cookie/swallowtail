use crate::{InputValueRequired, OperationContent, SchemaDocument};
use swallowtail_core::ReasoningMode;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolDeclaration {
    name: String,
    description: Option<OperationContent>,
    input_schema: SchemaDocument,
    schema_media_type: String,
    schema_dialect: String,
}

impl ToolDeclaration {
    pub fn new(
        name: impl Into<String>,
        input_schema: SchemaDocument,
        schema_media_type: impl Into<String>,
        schema_dialect: impl Into<String>,
    ) -> Result<Self, InputValueRequired> {
        Ok(Self {
            name: crate::input::required_text("tool name", name)?,
            description: None,
            input_schema,
            schema_media_type: crate::input::required_text(
                "tool schema media type",
                schema_media_type,
            )?,
            schema_dialect: crate::input::required_text("tool schema dialect", schema_dialect)?,
        })
    }

    #[must_use]
    pub fn with_description(mut self, description: OperationContent) -> Self {
        self.description = Some(description);
        self
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn description(&self) -> Option<&OperationContent> {
        self.description.as_ref()
    }

    #[must_use]
    pub const fn input_schema(&self) -> &SchemaDocument {
        &self.input_schema
    }

    #[must_use]
    pub fn schema_media_type(&self) -> &str {
        &self.schema_media_type
    }

    #[must_use]
    pub fn schema_dialect(&self) -> &str {
        &self.schema_dialect
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SessionOptions {
    developer_instructions: Option<OperationContent>,
    reasoning_mode: Option<ReasoningMode>,
    tools: Vec<ToolDeclaration>,
}

impl SessionOptions {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.developer_instructions.is_none()
            && self.reasoning_mode.is_none()
            && self.tools.is_empty()
    }

    #[must_use]
    pub fn with_developer_instructions(mut self, instructions: OperationContent) -> Self {
        self.developer_instructions = Some(instructions);
        self
    }

    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    #[must_use]
    pub fn with_tools(mut self, tools: impl IntoIterator<Item = ToolDeclaration>) -> Self {
        self.tools = tools.into_iter().collect();
        self
    }

    #[must_use]
    pub const fn developer_instructions(&self) -> Option<&OperationContent> {
        self.developer_instructions.as_ref()
    }

    #[must_use]
    pub const fn reasoning_mode(&self) -> Option<&ReasoningMode> {
        self.reasoning_mode.as_ref()
    }

    pub fn tools(&self) -> impl ExactSizeIterator<Item = &ToolDeclaration> {
        self.tools.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{SessionOptions, ToolDeclaration};
    use crate::{OperationContent, SchemaDocument};
    use swallowtail_core::ReasoningMode;

    #[test]
    fn session_options_keep_consumer_content_redacted() {
        let tool = ToolDeclaration::new(
            "task_ledger",
            SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 128)
                .expect("schema is bounded"),
            "application/schema+json",
            "json-schema-2020-12",
        )
        .expect("declaration is valid")
        .with_description(OperationContent::new("private tool description").expect("valid"));
        let options = SessionOptions::default()
            .with_developer_instructions(
                OperationContent::new("private instructions").expect("valid"),
            )
            .with_reasoning_mode(ReasoningMode::new("low").expect("valid"))
            .with_tools([tool]);

        let rendered = format!("{options:?}");
        assert!(!rendered.contains("private instructions"));
        assert!(!rendered.contains("private tool description"));
        assert!(!rendered.contains("\"type\""));
        assert_eq!(options.tools().len(), 1);
    }

    #[test]
    fn tool_declaration_rejects_blank_transport_metadata() {
        let schema = SchemaDocument::inline(b"{}".to_vec(), 16).expect("schema is bounded");
        assert!(ToolDeclaration::new(" ", schema, "application/schema+json", "draft").is_err());
    }
}
