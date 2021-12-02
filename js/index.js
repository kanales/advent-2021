let obs = [];
window.onwasm = e => {
  obs.push(e);
};

(async () => {
  window.wasm = await import("../pkg/index.js").catch(console.error);
  obs.map(e => e());
  obs = [];
})();
