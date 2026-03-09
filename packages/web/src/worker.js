import init, { generate_particles } from "./wasm/nbody_simulator.js";
import { SimulationEngine } from "./SimulationEngine.js";
import { Renderer } from "./Renderer.js";

let engine = null;
let renderer = null;
let canvas = null;
let ctx = null;
let params = null;
let isPlaying = false;

onmessage = async (e) => {
    const { type, data } = e.data;

    switch (type) {
        case "INIT": {
            const wasm = await init();
            engine = new SimulationEngine({ ...wasm, generate_particles });
            canvas = data.canvas;
            ctx = canvas.getContext("2d");
            renderer = new Renderer(ctx);
            params = data.params;
            
            updateCanvasSize();
            engine.init(params);
            
            isPlaying = data.isPlaying;
            if (isPlaying) {
                requestAnimationFrame(loop);
            } else {
                renderOnce();
            }
            break;
        }
        case "UPDATE_PARAMS": {
            params = { ...params, ...data };
            updateCanvasSize();
            break;
        }
        case "RESTART": {
            params = { ...params, ...data };
            updateCanvasSize();
            engine.init(params);
            if (!isPlaying) {
                renderOnce();
            }
            break;
        }
        case "PLAY_PAUSE": {
            const oldPlaying = isPlaying;
            isPlaying = data;
            if (isPlaying && !oldPlaying) {
                requestAnimationFrame(loop);
            }
            break;
        }
    }
};

function updateCanvasSize() {
    if (canvas && params) {
        canvas.width = params.canvasWidth;
        canvas.height = params.canvasHeight;
    }
}

function loop() {
    if (!isPlaying) return;

    // Report frame start to main thread for stats
    postMessage({ type: "FRAME_START" });

    engine.step(params);
    renderer.draw(engine, params);

    postMessage({ type: "FRAME_END" });

    requestAnimationFrame(loop);
}

function renderOnce() {
    renderer.draw(engine, params);
}
