import './src/js/wasm_bridge.js'
import { init_scanner } from './src/js/scanner.js'

import('./src/js/ipfs.js').then((module) => {
  // module.start_ipfs();
})

import('./pkg').then((wasm) => {
  wasm.run_app()
  console.log('wasm', wasm)
  console.log('init_scanner', init_scanner)
  init_scanner(wasm)
})
