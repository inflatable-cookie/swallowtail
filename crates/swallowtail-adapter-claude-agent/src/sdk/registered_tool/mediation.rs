//! Route-local mediation between the carrier's MCP wire and the kernel.
//!
//! Every admitted `tools/call` becomes exactly one
//! [`RegisteredToolCallRequest`] issued through
//! [`RegisteredToolBridgeLease::call`], so the Contract 063 kernel owns the
//! serialized admission point, the live consumer verdict, correlation, the
//! effective deadline, exactly-once settlement, and the honest execution
//! disposition. The mediator adds no second registry, lease, listener, or
//! admission model, and it never mints a validated binding.
//!
//! `canUseTool` stays permission admission only. The mediator refuses to issue
//! a call the route's own admission seam did not allow, and a recorded denial
//! never reaches the linked dispatcher.

use super::carrier::ClaudeAgentSdkRegisteredToolCarrier;
use super::version::{
    CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION, CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_REVISION,
    claude_agent_sdk_mcp_protocol_version_known,
};
use super::wire::{
    ClaudeAgentSdkMcpMethod, ClaudeAgentSdkMcpRequest, ClaudeAgentSdkMcpRequestId,
    MCP_ERROR_INVALID_PARAMS, MCP_ERROR_INVALID_REQUEST, MCP_ERROR_METHOD_NOT_FOUND, mcp_error,
    mcp_result,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;
use swallowtail_runtime::{
    RegisteredToolBounds, RegisteredToolBridgeLease, RegisteredToolCallId,
    RegisteredToolDeclaration, RegisteredToolEffectPosture, RegisteredToolExecutionDisposition,
    RegisteredToolFailureKind, RegisteredToolLimits, RegisteredToolOutcome, RegisteredToolPayload,
    RegisteredToolResult, RuntimeFailure,
};

/// Exact one-shot decision the route's `canUseTool` seam produced.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ClaudeAgentSdkRegisteredToolDecision {
    /// One exact call may proceed.
    Allow,
    /// One exact call is denied and never reaches the linked dispatcher.
    Deny,
}

/// What the mediator did with one carrier record.
///
/// The response is the exact bytes to write back on the carrier's stdio wire.
/// Keeping it as bytes rather than a parsed document keeps `serde_json` out of
/// this route's public surface and leaves the carrier as the only encoder.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClaudeAgentSdkMcpReply {
    /// One encoded JSON-RPC response to write back to the provider.
    Respond(Vec<u8>),
    /// A notification was accepted; nothing is written back.
    Accepted,
}

impl ClaudeAgentSdkMcpReply {
    /// Returns the encoded response bytes when one exists.
    #[must_use]
    pub fn response_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Respond(bytes) => Some(bytes),
            Self::Accepted => None,
        }
    }

    /// Encodes one response document, or reports the encoding failure.
    fn respond(document: Value) -> Self {
        // `serde_json::to_vec` on a document this module built cannot fail; the
        // fallback keeps the carrier total rather than panicking on a wire path.
        serde_json::to_vec(&document).map_or(Self::Accepted, Self::Respond)
    }
}

#[derive(Default)]
struct MediatorState {
    initialized: bool,
    handshake_done: bool,
    outstanding: BTreeMap<String, ClaudeAgentSdkRegisteredToolDecision>,
    seen_ids: BTreeSet<String>,
    cancelled_ids: BTreeSet<String>,
}

/// Route-local mediator binding one carrier to one open Contract 063 lease.
pub struct ClaudeAgentSdkRegisteredToolMediator {
    carrier: ClaudeAgentSdkRegisteredToolCarrier,
    lease: RegisteredToolBridgeLease,
    bounds: RegisteredToolBounds,
    state: Mutex<MediatorState>,
}

impl ClaudeAgentSdkRegisteredToolMediator {
    /// Binds one carrier to the lease its own selection opened.
    ///
    /// A lease opened for another selection is rejected here: the carrier's
    /// provider spelling would otherwise present identities the lease never
    /// bound.
    pub fn new(
        carrier: ClaudeAgentSdkRegisteredToolCarrier,
        lease: RegisteredToolBridgeLease,
    ) -> Result<Self, RuntimeFailure> {
        if lease.selection() != carrier.selection() {
            return Err(kind_failure(RegisteredToolFailureKind::ForeignCorrelation));
        }
        let bounds = carrier.selection().effective_bounds();
        Ok(Self {
            carrier,
            lease,
            bounds,
            state: Mutex::new(MediatorState::default()),
        })
    }

    /// Mirrors the consumer narrowing the lease was opened with.
    ///
    /// Card 114's lease publishes its selection but not the effective bounds it
    /// was opened under, so a route cannot read the narrowing back. This
    /// builder lets the caller state it, which only ever narrows: the kernel
    /// re-checks every bound at the serialized admission point and stays
    /// authoritative, so a mediator given a wider value cannot widen anything.
    #[must_use]
    pub fn with_consumer_limits(mut self, limits: RegisteredToolLimits) -> Self {
        self.bounds = self.bounds.narrowed(limits.bounds());
        self
    }

    /// Returns the effective argument and result bounds this mediator enforces.
    #[must_use]
    pub const fn effective_bounds(&self) -> RegisteredToolBounds {
        self.bounds
    }

    /// Returns the carrier this mediator presents.
    #[must_use]
    pub const fn carrier(&self) -> &ClaudeAgentSdkRegisteredToolCarrier {
        &self.carrier
    }

    /// Returns the open lease every admitted call is issued through.
    #[must_use]
    pub const fn lease(&self) -> &RegisteredToolBridgeLease {
        &self.lease
    }

    /// Records one `canUseTool` decision for an exact provider tool name.
    ///
    /// This is the permission seam only. An `Allow` authorizes at most one
    /// subsequent call of that exact tool; it never becomes a persistent
    /// policy, and a second outstanding decision for the same tool is rejected
    /// rather than silently replacing the first.
    pub fn record_admission(
        &self,
        provider_tool_name: &str,
        decision: ClaudeAgentSdkRegisteredToolDecision,
    ) -> Result<(), RuntimeFailure> {
        if self
            .carrier
            .resolve_provider_tool(provider_tool_name)
            .is_none()
        {
            return Err(kind_failure(RegisteredToolFailureKind::UnsupportedTool));
        }
        let mut state = self.locked();
        if state
            .outstanding
            .insert(provider_tool_name.to_owned(), decision)
            .is_some()
        {
            return Err(kind_failure(
                RegisteredToolFailureKind::DuplicateCorrelation,
            ));
        }
        Ok(())
    }

    /// Mediates one bounded carrier record.
    ///
    /// A record that cannot be decoded fails before any state changes; nothing
    /// undecodable ever reaches the kernel.
    pub async fn mediate(&self, record: &[u8]) -> Result<ClaudeAgentSdkMcpReply, RuntimeFailure> {
        let request = ClaudeAgentSdkMcpRequest::decode(record)?;
        match request.method() {
            ClaudeAgentSdkMcpMethod::Initialize => Ok(self.initialize(&request)),
            ClaudeAgentSdkMcpMethod::Initialized => {
                self.locked().initialized = true;
                Ok(ClaudeAgentSdkMcpReply::Accepted)
            }
            ClaudeAgentSdkMcpMethod::Cancelled => {
                if let Some(id) = request.string_param("requestId") {
                    self.locked().cancelled_ids.insert(format!("s{id}"));
                } else if let Some(id) = request.params().get("requestId").and_then(Value::as_i64) {
                    self.locked().cancelled_ids.insert(format!("n{id}"));
                }
                Ok(ClaudeAgentSdkMcpReply::Accepted)
            }
            ClaudeAgentSdkMcpMethod::ToolsList => Ok(self.list_tools(&request)),
            ClaudeAgentSdkMcpMethod::ToolsCall => self.call_tool(&request).await,
        }
    }

    fn locked(&self) -> std::sync::MutexGuard<'_, MediatorState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn initialize(&self, request: &ClaudeAgentSdkMcpRequest) -> ClaudeAgentSdkMcpReply {
        let id = request.id().expect("initialize is a request").clone();
        let offered = request.string_param("protocolVersion").unwrap_or_default();
        if !claude_agent_sdk_mcp_protocol_version_known(offered) {
            return ClaudeAgentSdkMcpReply::respond(mcp_error(
                &id,
                MCP_ERROR_INVALID_PARAMS,
                RegisteredToolFailureKind::UnsupportedProtocolVersion.code(),
            ));
        }
        {
            let mut state = self.locked();
            if state.handshake_done {
                return ClaudeAgentSdkMcpReply::respond(mcp_error(
                    &id,
                    MCP_ERROR_INVALID_REQUEST,
                    RegisteredToolFailureKind::DuplicateCorrelation.code(),
                ));
            }
            state.handshake_done = true;
        }
        ClaudeAgentSdkMcpReply::respond(mcp_result(
            &id,
            json!({
                "protocolVersion": CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION,
                "capabilities": {"tools": {}},
                "serverInfo": {
                    "name": self.carrier.server_name(),
                    "version": CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_REVISION,
                },
            }),
        ))
    }

    fn list_tools(&self, request: &ClaudeAgentSdkMcpRequest) -> ClaudeAgentSdkMcpReply {
        let id = request.id().expect("tools/list is a request").clone();
        if !self.locked().initialized {
            return ClaudeAgentSdkMcpReply::respond(mcp_error(
                &id,
                MCP_ERROR_INVALID_REQUEST,
                RegisteredToolFailureKind::NotReady.code(),
            ));
        }
        let snapshot = self.carrier.selection().snapshot();
        let mut tools = Vec::with_capacity(self.carrier.tools().len());
        for tool in self.carrier.tools() {
            let Some(declaration) = snapshot.declaration(tool.id()) else {
                return ClaudeAgentSdkMcpReply::respond(mcp_error(
                    &id,
                    MCP_ERROR_METHOD_NOT_FOUND,
                    RegisteredToolFailureKind::UnsupportedTool.code(),
                ));
            };
            tools.push(tool_descriptor(tool.carrier_tool_name(), declaration));
        }
        ClaudeAgentSdkMcpReply::respond(mcp_result(&id, json!({"tools": tools})))
    }

    async fn call_tool(
        &self,
        request: &ClaudeAgentSdkMcpRequest,
    ) -> Result<ClaudeAgentSdkMcpReply, RuntimeFailure> {
        let id = request.id().expect("tools/call is a request").clone();
        let correlation = id.as_correlation_text();
        if !self.locked().initialized {
            return Ok(respond_error(
                &id,
                MCP_ERROR_INVALID_REQUEST,
                RegisteredToolFailureKind::NotReady,
            ));
        }
        if !self.locked().seen_ids.insert(correlation.clone()) {
            return Ok(respond_error(
                &id,
                MCP_ERROR_INVALID_REQUEST,
                RegisteredToolFailureKind::DuplicateCorrelation,
            ));
        }
        let Some(name) = request.string_param("name") else {
            return Ok(respond_error(
                &id,
                MCP_ERROR_INVALID_PARAMS,
                RegisteredToolFailureKind::UnsupportedTool,
            ));
        };
        let Some(tool) = self.carrier.resolve_carrier_tool(name) else {
            return Ok(respond_error(
                &id,
                MCP_ERROR_INVALID_PARAMS,
                RegisteredToolFailureKind::UnsupportedTool,
            ));
        };
        let snapshot = self.carrier.selection().snapshot();
        let Some(declaration) = snapshot.declaration(tool.id()) else {
            return Ok(respond_error(
                &id,
                MCP_ERROR_INVALID_PARAMS,
                RegisteredToolFailureKind::UnsupportedTool,
            ));
        };
        // The consumer's own one-shot decision gates dispatch. An absent
        // decision is a call the admission seam never saw, which fails closed
        // exactly like an explicit denial rather than being admitted by
        // default.
        let decision = self
            .locked()
            .outstanding
            .remove(tool.provider_tool_name())
            .ok_or(ClaudeAgentSdkRegisteredToolDecision::Deny)
            .unwrap_or(ClaudeAgentSdkRegisteredToolDecision::Deny);
        if decision == ClaudeAgentSdkRegisteredToolDecision::Deny {
            return Ok(tool_error(&id, RegisteredToolFailureKind::ConsumerDenied));
        }
        if self.locked().cancelled_ids.contains(&correlation) {
            return Ok(tool_error(&id, RegisteredToolFailureKind::Cancelled));
        }
        let arguments = match request.params().get("arguments") {
            None | Some(Value::Null) => Value::Object(serde_json::Map::new()),
            Some(Value::Object(arguments)) => Value::Object(arguments.clone()),
            Some(_) => {
                return Ok(respond_error(
                    &id,
                    MCP_ERROR_INVALID_PARAMS,
                    RegisteredToolFailureKind::InvalidResult,
                ));
            }
        };
        let body = serde_json::to_vec(&arguments)
            .map_err(|_| kind_failure(RegisteredToolFailureKind::InvalidResult))?;
        let payload = match RegisteredToolPayload::new(
            declaration.input_schema().media_type().clone(),
            body,
            self.bounds.max_argument_bytes(),
        ) {
            Ok(payload) => payload,
            Err(failure) => return Ok(tool_error(&id, failure.kind())),
        };
        let Ok(call_id) = RegisteredToolCallId::new(format!(
            "{}:{}:{correlation}",
            self.lease.turn().as_str(),
            self.carrier.server_name()
        )) else {
            return Ok(respond_error(
                &id,
                MCP_ERROR_INVALID_REQUEST,
                RegisteredToolFailureKind::IdentityRejected,
            ));
        };
        let outcome = self
            .lease
            .call(swallowtail_runtime::RegisteredToolCallRequest::new(
                call_id,
                tool.id().clone(),
                payload,
                self.lease.deadline(),
            ))
            .await;
        Ok(match outcome {
            Ok(outcome) => ClaudeAgentSdkMcpReply::respond(mcp_result(
                &id,
                call_tool_result(&outcome, declaration),
            )),
            // A kernel-level rejection never reached a dispatcher, so it is a
            // tool error carrying its safe code rather than a fabricated
            // result. The exact code is preserved; nothing is normalized into a
            // generic failure.
            Err(failure) => ClaudeAgentSdkMcpReply::respond(mcp_result(
                &id,
                json!({
                    "content": [{"type": "text", "text": failure.diagnostic().code()}],
                    "isError": true,
                }),
            )),
        })
    }
}

/// Projects one settled outcome onto an exact MCP `CallToolResult`.
///
/// A successful result carries the linked host's bounded body verbatim, and its
/// declared output-schema digest is published beside it so a consumer can bind
/// the result to the exact declaration it selected. A failure carries the safe
/// Contract 063 code and the honest execution disposition; the disposition is
/// never downgraded into a claim that the effect did not happen.
fn call_tool_result(
    outcome: &RegisteredToolOutcome,
    declaration: &RegisteredToolDeclaration,
) -> Value {
    match outcome.result() {
        Some(result) => json!({
            "content": [{"type": "text", "text": result_text(result)}],
            "isError": false,
            "_meta": {
                "swallowtail/outputSchemaDigest": result.schema_digest().as_str(),
                "swallowtail/outputSchemaRevision": declaration.output_schema().revision().as_str(),
            },
        }),
        None => json!({
            "content": [{
                "type": "text",
                "text": outcome
                    .failure()
                    .map_or_else(
                        || RegisteredToolFailureKind::InvalidResult.code(),
                        |failure| failure.kind().code(),
                    ),
            }],
            "isError": true,
            "_meta": {
                "swallowtail/executionDisposition": disposition_label(outcome.disposition()),
                "swallowtail/permitsExplicitRetry": outcome.permits_explicit_retry(),
            },
        }),
    }
}

/// Renders one bounded result body for the provider.
///
/// The linked host owns the body. It is passed through unchanged when it is
/// valid UTF-8 and replaced with a safe code otherwise, so a non-textual body
/// can never become an invalid provider record.
fn result_text(result: &RegisteredToolResult) -> String {
    std::str::from_utf8(result.payload().expose_for_execution())
        .unwrap_or_else(|_| RegisteredToolFailureKind::InvalidResult.code())
        .to_owned()
}

const fn disposition_label(disposition: RegisteredToolExecutionDisposition) -> &'static str {
    disposition.as_str()
}

/// Builds one bounded MCP tool descriptor from a Contract 063 declaration.
///
/// The descriptor carries the declared input-schema document and its digest.
/// No effect, retry, credential, host, or bound material beyond the read-only
/// annotation crosses to the provider.
fn tool_descriptor(name: &str, declaration: &RegisteredToolDeclaration) -> Value {
    let input_schema =
        serde_json::from_str::<Value>(declaration.input_schema().document().expose_for_execution())
            .unwrap_or_else(|_| json!({"type": "object"}));
    json!({
        "name": name,
        "description": declaration.id().to_string(),
        "inputSchema": input_schema,
        "annotations": {
            "readOnlyHint": declaration.effect() == RegisteredToolEffectPosture::ReadOnly,
        },
        "_meta": {
            "swallowtail/inputSchemaDigest": declaration.input_schema().digest().as_str(),
            "swallowtail/outputSchemaDigest": declaration.output_schema().digest().as_str(),
            "swallowtail/executionKind": declaration.kind().as_str(),
        },
    })
}

fn respond_error(
    id: &ClaudeAgentSdkMcpRequestId,
    code: i64,
    kind: RegisteredToolFailureKind,
) -> ClaudeAgentSdkMcpReply {
    ClaudeAgentSdkMcpReply::respond(mcp_error(id, code, kind.code()))
}

/// Reports one Contract 063 refusal as a tool error rather than a wire error.
///
/// A refusal the consumer's own policy produced is visible to the model as a
/// failed tool call; it is not a malformed request, and it never reaches the
/// linked dispatcher.
fn tool_error(
    id: &ClaudeAgentSdkMcpRequestId,
    kind: RegisteredToolFailureKind,
) -> ClaudeAgentSdkMcpReply {
    ClaudeAgentSdkMcpReply::respond(mcp_result(
        id,
        json!({
            "content": [{"type": "text", "text": kind.code()}],
            "isError": true,
            "_meta": {
                "swallowtail/executionDisposition":
                    RegisteredToolExecutionDisposition::NotExecuted.as_str(),
            },
        }),
    ))
}

fn kind_failure(kind: RegisteredToolFailureKind) -> RuntimeFailure {
    swallowtail_runtime::RegisteredToolFailure::new(kind).into_runtime_failure()
}
