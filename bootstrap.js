import './static/reset.scss';
import './static/theme.scss';
import "./js/wasm_bridge.js";
import { init_scanner } from "./js/scanner.js";

import("./js/ipfs.js").then(module => {
  module.start_ipfs();
});


import("./pkg").then(wasm => {
  wasm.run_app();
  console.log("wasm", wasm)
  console.log("init_scanner", init_scanner)
  init_scanner(wasm)
});