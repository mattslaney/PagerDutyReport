import init, { log_msg } from "/pkg/wasm.js";
init().then(() => {
  log_msg("WASM+TS");
});
