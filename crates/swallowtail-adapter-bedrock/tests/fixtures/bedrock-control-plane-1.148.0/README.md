# Bedrock Control-Plane SDK Fixture

This corpus pins the official `aws-sdk-bedrock` crate at `1.148.0` and
constructs its generated `ListFoundationModels` request, output, model summary,
lifecycle, enum, configuration, and error types directly.

It covers the first native, unfiltered, non-paginated, one-request catalogue
subset from Contract 020. Projection is bounded to 1,024 summaries. Known
values become common typed observations. Unknown generated enum values remain
bounded `amazon-bedrock` observations. Missing optional values remain unknown.

The corpus does not contact AWS, resolve ambient credentials, read AWS
configuration, invoke a model, create a model route, use a Bedrock Runtime
grant, call Marketplace, accept provider terms, or exercise the separate
Bedrock Mantle `/models` surface.
