import init, { generate_particles } from "./wasm/nbody_simulator.js";
import { SimulationEngine, SimulationParams } from "./engine";
import { Renderer } from "../rendering/renderer";

let engine: SimulationEngine | null = null;
let renderer: Renderer | null = null;
let canvas: OffscreenCanvas | null = null;
let ctx: OffscreenCanvasRenderingContext2D | null = null;
let params: SimulationParams | null = null;
let isPlaying = false;

onmessage = async (e: MessageEvent) => {
    const { type, data } = e.data;

    switch (type) {
        case "INIT": {
            const wasm = await init();
            // Use type assertion here since SimulationEngine constructor expects WasmEngine
            // and the wasm object from init() might not perfectly match WasmEngine interface
            // as defined in engine.ts.
            engine = new SimulationEngine({ ...wasm, generate_particles } as any);
            canvas = data.canvas as OffscreenCanvas;
            ctx = canvas.getContext("2d") as OffscreenCanvasRenderingContext2D;
            renderer = new Renderer(ctx);
            params = data.params;
            
            updateCanvasSize();
            if (engine && params) {
                engine.init(params);
            }
            
            isPlaying = data.isPlaying;
            if (isPlaying) {
                requestAnimationFrame(loop);
            } else {
                renderOnce();
            }
            break;
        }
        case "UPDATE_PARAMS": {
            if (params) {
                params = { ...params, ...data };
            }
            updateCanvasSize();
            break;
        }
        case "RESTART": {
            if (params) {
                params = { ...params, ...data };
            }
            updateCanvasSize();
            if (engine && params) {
                engine.init(params);
            }
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
    if (!isPlaying || !engine || !renderer || !params) return;

    // Report frame start to main thread for stats
    postMessage({ type: "FRAME_START" });

    engine.step(params);
    renderer.draw(engine, params);

    postMessage({ type: "FRAME_END" });

    requestAnimationFrame(loop);
}

function renderOnce() {
    if (engine && renderer && params) {
        renderer.draw(engine, params);
    }
}
