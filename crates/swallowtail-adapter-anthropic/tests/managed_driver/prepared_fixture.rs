impl Fixture {
    fn preparation_input(&self) -> AnthropicManagedPreparationInput {
        let access =
            swallowtail_adapter_anthropic::anthropic_managed_access_profile(self.credential.clone());
        let status = AccessStatus::new(
            access.id().clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        AnthropicManagedPreparationInput::new(
            self.instance_id.clone(),
            InstanceRevision::new("prepared-1").expect("revision is valid"),
            self.host_id.clone(),
            self.target.clone(),
            access,
            PreparedAccessEvidence::caller_asserted(status),
            ProviderAgentBinding::new(
                ProviderAgentId::new("agent_fixture").expect("agent id is valid"),
                ProviderAgentVersion::new("7").expect("agent version is valid"),
            ),
        )
    }

    fn prepared_run_input(
        &self,
        id: &str,
        tools: impl IntoIterator<Item = ToolDeclaration>,
    ) -> AnthropicManagedAgentRunInput {
        self.prepared_run_input_with_deadline(id, self.deadline(), tools)
    }

    fn prepared_run_input_with_deadline(
        &self,
        id: &str,
        deadline: Deadline,
        tools: impl IntoIterator<Item = ToolDeclaration>,
    ) -> AnthropicManagedAgentRunInput {
        AnthropicManagedAgentRunInput::durable_with_managed_recovery_and_one_reattachment(
            RequestId::new(id).expect("request id is valid"),
            AnthropicManagedModelSelection::new(
                ModelRouteId::new("anthropic-managed-fixture").expect("route id is valid"),
                ModelRouteRevision::new("prepared-1").expect("route revision is valid"),
                ModelId::new("claude-fixture-model").expect("model id is valid"),
            ),
            OperationContent::new("Return the fixture summary.").expect("content is valid"),
            deadline,
            tools,
        )
    }
}
