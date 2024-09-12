import init, { greet } from "rust_webassembly_with_ts";

// async function start() {
//     const wasm = await init();
//     wasm.greet("sage");
//     console.log("OK");
// }

// await start();


init().then(wasm => {
    wasm.greet("Filip");
    console.log("Yeah!")
})