// Private selected-SDK absence probe. Never forwards to an ambient builtin API.
export function sdkStartupBuiltinLookup(...args) {
  if (args.length === 1 && args[0] === "async_hooks") return undefined;
  throw new Error("direct builtin access refused");
}
