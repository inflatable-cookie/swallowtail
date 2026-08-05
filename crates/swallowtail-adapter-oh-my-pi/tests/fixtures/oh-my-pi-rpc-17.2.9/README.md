# Oh My Pi RPC 17.2.9 corpus

Frozen deterministic evidence for `@oh-my-pi/pi-coding-agent@17.2.9`, accessed
2026-08-05.

Sources:

- https://github.com/can1357/oh-my-pi/tree/f7f8e040ee04710414fbd775431091fa301b9786
- https://github.com/can1357/oh-my-pi/blob/f7f8e040ee04710414fbd775431091fa301b9786/docs/rpc.md
- https://www.npmjs.com/package/@oh-my-pi/pi-coding-agent/v/17.2.9

The corpus binds one exact `oh-my-pi.package` qualification point. It freezes
the initial ready frame, pre-turn command update, RPC v2 negotiation and frame
bounds, explicit model and reasoning control, restrictive process arguments,
agent/message/tool activity, usage, typed extension UI, and terminal
`agent_end.isTerminal` behavior. Session-level model/thinking changes remain
lifecycle-only outside an active turn. Empty `setWidget` frames represent UI
clear operations and carry no portable display content.

Provider, model, executable, working directory, and local OMP authentication
remain opaque host inputs. Fixtures do not install OMP, read a real auth store,
or contact a model provider. Session mutation, host tool injection, subagent
subscription/control, and write-capable tools are outside this first route.

The separate ignored live probe uses only explicit operator gating. It checks
the prepared catalogue and one exact provider/model/reasoning run without
serializing credentials into Swallowtail evidence.
