import './static/reset.scss';
import './static/theme.scss';
import "./js/wasm_bridge.js";


import("./js/ipfs_server.js").then(module => {
  //module.start_ipfs();
});


import("./pkg").then(module => {
  module.run_app();
});
