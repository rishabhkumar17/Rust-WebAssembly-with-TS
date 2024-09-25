import init, { World } from "rust_webassembly_with_ts";

// async function start() {
//     const wasm = await init();
//     wasm.greet("sage");
//     console.log("OK");
// }

// await start();


init().then(_ => {
    const world = World.new();
    // console.log(world.width);
    console.log(world.width());
})