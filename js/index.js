(async () => {
  window.wasm = await import("../pkg/index.js").catch(console.error);
})();
