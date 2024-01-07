<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    const FPS = 30;
    const WIDTH = 50;
    const HEIGHT = 50;
    const PIXEL_SIZE = 18;
    const GRID_COLOR = "#808080";
    const UNCLICKED_COLOR = "#BABABA";
    const CLICKED_COLOR = "#C6C6C6";

    const NUMBER_COLORS = [
        "#C6C6C6", // Nothing
        "#0000FF", // Blue
        "#008000", // Green
        "#FFA500", // Orange
        "#00008B", // Dark Blue
        "#A52A2A", // Brown
        "#00FFFF", // Cyan
        "#000000", // Black
        "#808080", // Grey
    ];

    let current_frame = Array<boolean>();

    let canvas: HTMLCanvasElement;
    let flagImg: HTMLImageElement;
    let ctx: CanvasRenderingContext2D;

    window.onload = () => {
        canvas.width = (PIXEL_SIZE + 1) * WIDTH + 1;
        canvas.height = (PIXEL_SIZE + 1) * HEIGHT + 1;

        ctx = canvas.getContext("2d")!;
        ctx.font = "18px sans-serif";

        renderLoop();
    };

    const updateFrame = () => {
        invoke("next_frame").then(
            (res: any) => (current_frame = res as Array<boolean>)
        );
    };

    const getIndex = (row: number, column: number): number => {
        return row * WIDTH + column;
    };

    const drawGrid = () => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.beginPath();
        ctx.strokeStyle = GRID_COLOR;

        //Vertical lines
        for (let i = 0; i <= WIDTH; i++) {
            ctx.moveTo(i * (PIXEL_SIZE + 1) + 1, 0);
            ctx.lineTo(i * (PIXEL_SIZE + 1) + 1, (PIXEL_SIZE + 1) * HEIGHT + 1);
        }

        //Horizontal lines
        for (let j = 0; j <= HEIGHT; j++) {
            ctx.moveTo(0, j * (PIXEL_SIZE + 1) + 1);
            ctx.lineTo((PIXEL_SIZE + 1) * WIDTH + 1, j * (PIXEL_SIZE + 1) + 1);
        }

        ctx.stroke();
    };

    const get_num_neighbors = (row: number, column: number): number => {
        let count = 0;

        for (let i = -1; i <= 1; i++) {
            for (let j = -1; j <= 1; j++) {
                if (i == 0 && j == 0) continue;
                if (row + i >= 50 || row + i < 0) continue;
                if (column + i >= 50 || column + i < 0) continue;

                let index = getIndex(row + i, column + j);
                if (!current_frame[index]) {
                    count++;
                }
            }
        }

        return count;
    };

    const draw = () => {
        ctx.beginPath();
        for (let row = 0; row < HEIGHT; row++) {
            for (let col = 0; col < WIDTH; col++) {
                const idx = getIndex(row, col);
                const nbCount = get_num_neighbors(row, col);
                const is_flag = !current_frame[idx];

                if (is_flag) {
                    ctx.fillStyle = UNCLICKED_COLOR;
                    ctx.fillRect(
                        col * (PIXEL_SIZE + 1) + 1,
                        row * (PIXEL_SIZE + 1) + 1,
                        PIXEL_SIZE,
                        PIXEL_SIZE
                    );

                    ctx.stroke();

                    //ctx drawImage flag.png
                    ctx.drawImage(
                        flagImg,
                        col * (PIXEL_SIZE + 1) + 1,
                        row * (PIXEL_SIZE + 1) - PIXEL_SIZE,
                        PIXEL_SIZE,
                        PIXEL_SIZE
                    );

                    ctx.stroke();
                    continue;
                }

                ctx.fillStyle = CLICKED_COLOR;
                ctx.fillRect(
                    col * (PIXEL_SIZE + 1) + 1,
                    row * (PIXEL_SIZE + 1) + 1,
                    PIXEL_SIZE,
                    PIXEL_SIZE
                );

                ctx.stroke();

                ctx.fillStyle = NUMBER_COLORS[nbCount];
                ctx.fillText(
                    nbCount.toString(),
                    col * (PIXEL_SIZE + 1) + 5,
                    row * (PIXEL_SIZE + 1) - 3,
                    PIXEL_SIZE
                );

                ctx.stroke();
            }
        }
    };

    const renderLoop = () => {
        const startTime = performance.now();

        updateFrame();
        drawGrid();
        draw();

        const endTime = performance.now();
        const elapsedTime = endTime - startTime;

        const delay = Math.max(0, 1000 / FPS - elapsedTime);

        setTimeout(() => {
            requestAnimationFrame(renderLoop);
        }, delay);
    };
</script>

<div id="game">
    <canvas id="game-canvas" bind:this={canvas} />
    <div hidden>
        <img src="/flag.png" bind:this={flagImg} alt="bad apple" />
    </div>
</div>

<style>
    #game {
        width: 100%;
        height: 100%;
    }
</style>
