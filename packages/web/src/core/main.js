import { Stats } from "../features/rendering/stats.js";
import state from "../features/ui/form-state.js";
import appState from "./app-state.js";
import * as register from "../features/ui/manager.js";

/**
 * Variables
 */
const canvas = document.getElementsByTagName("canvas")[0];
const container = document.getElementById("stats-container");

const stats = new Stats();
stats.dom.style.position = "absolute";
delete stats.dom.style.left;
stats.showPanel(0);
container.appendChild(stats.dom);

// Initialize Worker
const worker = new Worker(new URL("../features/simulation/worker.js", import.meta.url), {
    type: "module",
});

// Transfer Canvas to Worker
const offscreen = canvas.transferControlToOffscreen();

// UI Event Proxying
export function postToWorker(type, data) {
    worker.postMessage({ type, data });
}

// Global reference for register.js to use
window.postToWorker = postToWorker;

worker.onmessage = (e) => {
    const { type } = e.data;
    if (type === "FRAME_START") {
        stats.begin();
    } else if (type === "FRAME_END") {
        stats.end();
    }
};

/**
 * INIT
 */
document.addEventListener("DOMContentLoaded", (_event) => {
    // Send initial configuration to worker
    worker.postMessage(
        {
            type: "INIT",
            data: {
                canvas: offscreen,
                params: state,
                isPlaying: appState.play,
            },
        },
        [offscreen]
    );

    register.registerPlayPauseButton();
    register.registerRestartButton();
    register.registerSaveButton();
    register.registerFileUpload();
    register.registerParameterForm();
    register.registerDefaultValues();
});

// Export empty functions to avoid breaking imports if any other file expects them
export async function setupParticles() {}
