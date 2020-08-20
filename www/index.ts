// import * as wasm from "hello-wasm-pack";
import * as wasm from "./../pkg/wasmweb_sample"

wasm.greet()
wasm.log("[WEB] external logging...")

const sum = wasm.just_sum()
console.log("Sum of number", sum)