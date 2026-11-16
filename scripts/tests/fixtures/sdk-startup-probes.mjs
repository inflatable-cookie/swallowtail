// First-party source-shaped startup probes. No vendor source is evaluated here.
{
  const sdkOwnedAsyncResource = globalThis.process?.getBuiltinModule?.("async_hooks");
  const sdkOwnedBind = sdkOwnedAsyncResource
    ? (callback) => sdkOwnedAsyncResource.AsyncResource.bind(callback)
    : (callback) => callback;
  const sdkOwnedCallback = () => "owned_callback";
  if (sdkOwnedAsyncResource !== undefined || sdkOwnedBind(sdkOwnedCallback) !== sdkOwnedCallback) {
    throw new Error("owned callback fallback changed");
  }
  if (sdkOwnedBind(sdkOwnedCallback)() !== "owned_callback") {
    throw new Error("owned callback identity changed");
  }

  const sdkOwnedAsyncStorage = globalThis.process?.getBuiltinModule?.("async_hooks");
  const sdkOwnedStorage = sdkOwnedAsyncStorage
    ? new sdkOwnedAsyncStorage.AsyncLocalStorage()
    : { run: (_store, callback) => callback(), getStore: () => undefined };
  let sdkOwnedRunCount = 0;
  const sdkOwnedRunResult = sdkOwnedStorage.run({ owned: true }, () => {
    sdkOwnedRunCount += 1;
    if (sdkOwnedStorage.getStore() !== undefined) throw new Error("owned store escaped");
    return "owned_run";
  });
  if (sdkOwnedAsyncStorage !== undefined || sdkOwnedRunResult !== "owned_run" || sdkOwnedRunCount !== 1) {
    throw new Error("owned storage fallback changed");
  }

  const sdkOwnedLookup = process.getBuiltinModule;
  let sdkOwnedCoercion = false;
  let sdkOwnedForbiddenEffect = false;
  const sdkOwnedCoercible = {
    toString() { sdkOwnedCoercion = true; return "async_hooks"; },
    [Symbol.toPrimitive]() { sdkOwnedCoercion = true; return "async_hooks"; }
  };
  const sdkOwnedInvalidCalls = [
    [], ["async_hooks", undefined], ["async_hooks", "fs"], ["async_hooks", undefined, undefined],
    ["node:async_hooks"], ["fs"], ["child_process"], ["module"], ["worker_threads"],
    ["process"], ["vm"], ["buffer"], ["definitely_missing_builtin"],
    [""], ["Async_hooks"], ["async_hooks "], ["async_hooks\0"],
    [undefined], [null], [1], [true], [0n], [Symbol("async_hooks")], [NaN],
    [[]], [{}], [new String("async_hooks")], [sdkOwnedCoercible], [() => "async_hooks"]
  ];
  for (const sdkOwnedArguments of sdkOwnedInvalidCalls) {
    let sdkOwnedDenied = false;
    try {
      sdkOwnedLookup(...sdkOwnedArguments);
      sdkOwnedForbiddenEffect = true;
    } catch (error) {
      sdkOwnedDenied = error?.message === "direct builtin access refused";
    }
    if (!sdkOwnedDenied) throw new Error("owned invalid lookup was admitted");
  }
  if (sdkOwnedCoercion || sdkOwnedForbiddenEffect) throw new Error("owned lookup effect reached");

  const sdkOwnedDescriptor = Object.getOwnPropertyDescriptor(process, "getBuiltinModule");
  if (sdkOwnedDescriptor.writable || sdkOwnedDescriptor.configurable || sdkOwnedDescriptor.value !== sdkOwnedLookup) {
    throw new Error("owned lookup property is mutable");
  }
  if (Reflect.set(process, "getBuiltinModule", () => process) || Reflect.deleteProperty(process, "getBuiltinModule")) {
    throw new Error("owned lookup replacement admitted");
  }
  let sdkOwnedRedefinitionDenied = false;
  try {
    Object.defineProperty(process, "getBuiltinModule", { value: () => process });
  } catch {
    sdkOwnedRedefinitionDenied = true;
  }
  if (!sdkOwnedRedefinitionDenied || process.getBuiltinModule !== sdkOwnedLookup) {
    throw new Error("owned lookup definition changed");
  }
  if (globalThis.sdkOwnedAmbientLookupCalls !== undefined && globalThis.sdkOwnedAmbientLookupCalls !== 0) {
    throw new Error("owned ambient lookup was forwarded");
  }
}
