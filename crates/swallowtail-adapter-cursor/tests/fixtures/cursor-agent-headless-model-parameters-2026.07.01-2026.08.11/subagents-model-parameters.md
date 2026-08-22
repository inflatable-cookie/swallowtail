#### Model parameters

Append square brackets to a model ID to set per-model options like speed, reasoning effort, and context window. Write options as `id=value` pairs, and separate multiple options with commas.

| Example                                   | Behavior                                                                                 |
| :---------------------------------------- | :--------------------------------------------------------------------------------------- |
| `composer-2.5[]`                          | Pins the base model. Empty brackets select the standard variant instead of the fast one. |
| `composer-2.5[fast=false]`                | Selects the standard (non-fast) variant explicitly.                                      |
| `claude-opus-5[effort=high]`              | Sets reasoning effort to `high`.                                                         |
| `claude-opus-5[context=300k]`             | Sets the context window to 300k tokens.                                                  |
| `claude-opus-5[effort=high,context=300k]` | Combines options.                                                                        |

Available options depend on the model, and use the same `id=value` pairs as the SDK's model parameters.
