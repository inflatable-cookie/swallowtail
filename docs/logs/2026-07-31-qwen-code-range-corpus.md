# 2026-07-31 Qwen Code Range Corpus

## Changed

- classified every stable Qwen Code point from `0.19.11` through `0.21.2`
- froze exact git, npm-integrity, stream, command, catalogue, and resume evidence
- identified one private catalogue-filter milestone at `0.21.0`
- added four deterministic compatibility-corpus tests

## Current State

All 34 focused Qwen tests pass. The production claim remains exact `0.19.11`;
card 022 owns the range change.

## Next

Execute card 022. Implement the two-segment maintained claim and exact bound
runtime version validation.
