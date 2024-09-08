import init, { greet } from "rust_webassembly_with_ts";

init().then(_ => {
    greet("Filip");
    console.log("Yeah!")
})