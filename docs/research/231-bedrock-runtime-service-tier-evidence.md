# 231 Bedrock Runtime Service-Tier Evidence

Status: reserved
Owner: Tom
Created: 2026-08-27
Card: g04.082 / 230

## Question

Which exact `bedrock.runtime` `performanceConfig.latency` or `serviceTier`
rows can be selected without flattening account-, region-, capacity-, or
model-dependent behavior into a generic Fast control?

## Required Decision

Promote a closed deliver-now table or an honest empty set. First reconcile the
evidence point represented by public SDK constant `1.136.0` and the locked
dependency `1.139.0` without changing either. Separate request, SDK-build,
service-acceptance, returned-tier, billing, and observed-latency truth.

## Starting Evidence

Research 013 selected the SDK-native route. Research 127 and 159 record the
existing SDK claim/pin mismatch. The current driver sends only model, messages,
and `inferenceConfig.maxTokens` through `ConverseStream`.
