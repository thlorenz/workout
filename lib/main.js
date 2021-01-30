import init, { add } from '../pkg/wasm_no_bundler.js'

async function run() {
  await init()

  const result = add(1, 2)
  console.log(`1 + 2 = ${result}`)
  if (result !== 3) throw new Error("wasm addition doesn't work!")
}

run()
