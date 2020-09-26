import './anna_app/static/reset.scss';
import './anna_app/static/theme.scss';
import "./anna_app/js/wasm_bridge.js";
import { init_scanner } from "./anna_app/js/scanner.js";

import("./anna_app/js/ipfs.js").then(module => {
  // module.start_ipfs();
});


import("./anna_app/pkg").then(wasm => {
  wasm.run_app();
  console.log("wasm", wasm)
  console.log("init_scanner", init_scanner)
  init_scanner(wasm)
});