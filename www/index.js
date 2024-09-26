import init, { World } from "rust_webassembly_with_ts";

init().then(_ => {
    const world = World.new();
    const canvas = document.getElementById("snake_canvas");
    const ctx = canvas.getContext("2d");
})


//------------------------------------------
// async function start() {
//     const wasm = await init();
//     wasm.greet("sage");
//     console.log("OK");
// }

// await start();


