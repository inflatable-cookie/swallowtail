use crate::rpc::failure;
use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, PreflightPlan, ProviderRequestPolicy,
};
use swallowtail_runtime::{RuntimeFailure, SchemaDocument, SessionOptions, ToolDeclaration};

const JSON_SCHEMA_MEDIA_TYPE: &str = "application/schema+json";

pub(crate) struct CodexSessionInput {
    developer_instructions: Option<String>,
    reasoning_effort: Option<String>,
    dynamic_tools: Vec<Value>,
    declared_tools: BTreeSet<String>,
}

pub(crate) struct CodexSessionRuntime {
    pub(crate) reasoning_effort: Option<String>,
    pub(crate) declared_tools: BTreeSet<String>,
    pub(crate) deadline_planned: bool,
    pub(crate) turn_sandbox_policy: Option<Value>,
    pub(crate) provider_requests: ProviderRequestPolicy,
}

impl CodexSessionInput {
    pub(crate) fn for_open(
        plan: &PreflightPlan,
        options: &SessionOptions,
    ) -> Result<Self, RuntimeFailure> {
        Self::prepare(plan, options, true)
    }

    pub(crate) fn for_resume(
        plan: &PreflightPlan,
        options: &SessionOptions,
    ) -> Result<Self, RuntimeFailure> {
        Self::prepare(plan, options, false)
    }

    fn prepare(
        plan: &PreflightPlan,
        options: &SessionOptions,
        allows_tools: bool,
    ) -> Result<Self, RuntimeFailure> {
        let tools = options.tools().collect::<Vec<_>>();
        validate_feature_binding(
            plan,
            options.reasoning_mode().is_some(),
            Capability::ReasoningSelection,
            "reasoning selection",
        )?;
        validate_feature_binding(
            plan,
            !tools.is_empty(),
            Capability::ToolCalls,
            "dynamic tools",
        )?;
        validate_reasoning(plan, options)?;
        validate_tools(plan, &tools)?;
        if !allows_tools && !tools.is_empty() {
            return Err(unsupported(
                "dynamic tools on resumed threads in the current Codex protocol",
            ));
        }

        let mut declared_tools = BTreeSet::new();
        let dynamic_tools = tools
            .into_iter()
            .map(|tool| {
                if !declared_tools.insert(tool.name().to_owned()) {
                    return Err(plan_mismatch("duplicate dynamic tool names"));
                }
                translate_tool(tool)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            developer_instructions: options
                .developer_instructions()
                .map(|content| content.as_str().to_owned()),
            reasoning_effort: options
                .reasoning_mode()
                .map(|mode| mode.as_str().to_owned()),
            dynamic_tools,
            declared_tools,
        })
    }

    pub(crate) fn apply_open(&self, params: &mut Value) {
        let object = params
            .as_object_mut()
            .expect("static thread/start parameters are an object");
        if let Some(instructions) = &self.developer_instructions {
            object.insert(
                "developerInstructions".to_owned(),
                Value::String(instructions.clone()),
            );
        }
        if !self.dynamic_tools.is_empty() {
            object.insert(
                "dynamicTools".to_owned(),
                Value::Array(self.dynamic_tools.clone()),
            );
        }
    }

    pub(crate) fn apply_resume(&self, params: &mut Value) {
        let object = params
            .as_object_mut()
            .expect("static thread/resume parameters are an object");
        if let Some(instructions) = &self.developer_instructions {
            object.insert(
                "developerInstructions".to_owned(),
                Value::String(instructions.clone()),
            );
        }
    }

    pub(crate) fn requires_experimental_api(&self) -> bool {
        !self.dynamic_tools.is_empty()
    }

    pub(crate) fn into_runtime(
        self,
        deadline_planned: bool,
        turn_sandbox_policy: Option<Value>,
        provider_requests: ProviderRequestPolicy,
    ) -> CodexSessionRuntime {
        CodexSessionRuntime {
            reasoning_effort: self.reasoning_effort,
            declared_tools: self.declared_tools,
            deadline_planned,
            turn_sandbox_policy,
            provider_requests,
        }
    }
}

fn validate_reasoning(
    plan: &PreflightPlan,
    options: &SessionOptions,
) -> Result<(), RuntimeFailure> {
    let Some(mode) = options.reasoning_mode() else {
        return Ok(());
    };
    let requirement = capability_requirement(plan, Capability::ReasoningSelection)
        .expect("feature binding was validated");
    if requirement.constraints().any(|constraint| {
        matches!(constraint, CapabilityConstraint::ReasoningMode(value) if value == mode)
    }) {
        Ok(())
    } else {
        Err(plan_mismatch("reasoning mode"))
    }
}

fn validate_tools(plan: &PreflightPlan, tools: &[&ToolDeclaration]) -> Result<(), RuntimeFailure> {
    if tools.is_empty() {
        return Ok(());
    }
    let requirement =
        capability_requirement(plan, Capability::ToolCalls).expect("feature binding was validated");
    let count = u32::try_from(tools.len()).unwrap_or(u32::MAX);
    if !requirement.constraints().any(
        |constraint| matches!(constraint, CapabilityConstraint::ToolMaximumCount(maximum) if count <= *maximum),
    ) {
        return Err(plan_mismatch("dynamic tool count"));
    }
    for tool in tools {
        if tool.schema_media_type() != JSON_SCHEMA_MEDIA_TYPE {
            return Err(unsupported("non-JSON Schema dynamic tools"));
        }
        if !requirement.constraints().any(|constraint| {
            matches!(constraint, CapabilityConstraint::ToolSchemaDialect(value) if value == tool.schema_dialect())
        }) {
            return Err(plan_mismatch("dynamic tool schema dialect"));
        }
        let bytes = match tool.input_schema() {
            SchemaDocument::Inline(bytes) => bytes,
            SchemaDocument::Reference(_) => {
                return Err(unsupported("referenced dynamic tool schemas"));
            }
        };
        let byte_len = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if !requirement.constraints().any(
            |constraint| matches!(constraint, CapabilityConstraint::ToolMaximumSchemaBytes(maximum) if byte_len <= *maximum),
        ) {
            return Err(plan_mismatch("dynamic tool schema size"));
        }
    }
    Ok(())
}

fn translate_tool(tool: &ToolDeclaration) -> Result<Value, RuntimeFailure> {
    let SchemaDocument::Inline(bytes) = tool.input_schema() else {
        return Err(unsupported("referenced dynamic tool schemas"));
    };
    let schema: Value = serde_json::from_slice(bytes).map_err(|_| {
        failure(
            "swallowtail.codex.app_server.tool_schema_invalid",
            "Codex dynamic tool schema is not valid JSON",
        )
    })?;
    Ok(serde_json::json!({
        "type": "function",
        "name": tool.name(),
        "description": tool.description().map_or("", |value| value.as_str()),
        "inputSchema": schema
    }))
}

fn validate_feature_binding(
    plan: &PreflightPlan,
    requested: bool,
    capability: Capability,
    feature: &str,
) -> Result<(), RuntimeFailure> {
    if requested == capability_requirement(plan, capability).is_some() {
        Ok(())
    } else {
        Err(plan_mismatch(feature))
    }
}

fn capability_requirement(
    plan: &PreflightPlan,
    capability: Capability,
) -> Option<&CapabilityRequirement> {
    plan.requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == capability)
}

fn plan_mismatch(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.preflight_mismatch",
        format!("Codex app-server request does not match preflight for {feature}"),
    )
}

fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.unsupported_input",
        format!("Codex app-server does not support {feature}"),
    )
}
