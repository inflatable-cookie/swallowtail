# Bedrock Runtime Service-Tier Evidence

Frozen secret-free corpus for g04.082 card 230 / Research 231.

It pins exact AWS API, user-guide, and `aws-sdk-bedrockruntime` generated
shapes for `ConverseStream` `performanceConfig.latency` and `serviceTier`
without contacting AWS, reading credentials, or invoking a model.

The public adapter constant claims Runtime SDK `1.136.0`; Cargo locks
`=1.139.0`. Both versions expose identical enum members and wire keys for the
fields audited here. The mismatch is recorded; this corpus does not choose a
canonical version.
