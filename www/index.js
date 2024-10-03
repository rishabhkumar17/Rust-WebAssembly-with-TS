import init, { World } from "rust_webassembly_with_ts";

init().then(_ => {
    const CELL_SIZE = 100;
    const world = World.new();
    const worldWidth = world.width();
    const canvas = document.getElementById("snake_canvas");
    const ctx = canvas.getContext("2d");

    canvas.height = worldWidth * CELL_SIZE;
    canvas.width = worldWidth * CELL_SIZE;

    function drawWorld() {
        ctx.beginPath();

        for (let i = 0; i <= worldWidth; i++) {
            //vertical line
            ctx.moveTo(CELL_SIZE * i, 0);
            ctx.lineTo(CELL_SIZE * i, worldWidth * CELL_SIZE);

            //horizontal line
            ctx.moveTo(0, CELL_SIZE * i);
            ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * i);
        }

        ctx.stroke();
    }

    function drawSnake() {
        const snakeIdx = world.snake_head_idx();
        const col = snakeIdx % worldWidth;
        const row = Math.floor(snakeIdx / worldWidth);

        ctx.beginPath();

        ctx.fillRect(
            col * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
        )

        ctx.stroke();
    }

    drawWorld();
    drawSnake()
})


//------------------------------------------
// async function start() {
//     const wasm = await init();
//     wasm.greet("sage");
//     console.log("OK");
// }

// await start();


