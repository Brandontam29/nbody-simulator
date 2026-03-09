import { Stats } from "./stats.js";
import init, {
    generate_particles,
    SimulationWrapper,
} from "./wasm/nbody_simulator.js";

import state from "./form-state.js";
import appState from "./app-state.js";

/**
 * Variables
 */
const canvas = document.getElementsByTagName("canvas")[0];
const container = document.getElementById("stats-container");
const ctx = canvas.getContext("2d");

const stats = new Stats();
stats.dom.style.position = "absolute";
delete stats.dom.style.left;
stats.showPanel(0);
container.appendChild(stats.dom);

canvas.height = state.canvasHeight;
canvas.width = state.canvasWidth;

let wasmMemory = null;
let simulation = null;

// Views into WASM memory
let views = {
    posX: null,
    posY: null,
    velX: null,
    velY: null,
    masses: null,
    diameters: null,
    colors: null,
};

/**
 * INIT
 */
document.addEventListener("DOMContentLoaded", (_event) => {
    init().then(async (wasm) => {
        wasmMemory = wasm.memory;
        await setupParticles();

        registerPlayPauseButton();
        registerRestartButton();
        registerSaveButton();
        registerFileUpload();
        registerParameterForm();
        registerDefaultValues();

        requestAnimationFrame(render);
    });
});

/**
 * FUNCTIONS
 */
function updateViews() {
    if (!simulation || !wasmMemory) return;
    
    const count = simulation.count();
    const buffer = wasmMemory.buffer;
    
    views.posX = new Float32Array(buffer, simulation.positions_x_ptr(), count);
    views.posY = new Float32Array(buffer, simulation.positions_y_ptr(), count);
    views.velX = new Float32Array(buffer, simulation.velocities_x_ptr(), count);
    views.velY = new Float32Array(buffer, simulation.velocities_y_ptr(), count);
    views.masses = new Float32Array(buffer, simulation.masses_ptr(), count);
    views.diameters = new Float32Array(buffer, simulation.diameters_ptr(), count);
    views.colors = new Float32Array(buffer, simulation.colors_ptr(), count * 3);
}

async function render() {
    if (!appState.play) return;

    stats.begin();

    if (!ctx) throw new Error("canvas dead");

    // Simulation Step
    if (simulation) {
        simulation.step(
            state.worldWidth,
            (state.canvasHeight * state.worldWidth) / state.canvasWidth,
            state.gravity,
            state.epsilon,
            state.timeStep
        );
        updateViews();
    }

    const { canvasWidth, canvasHeight, worldWidth } = state;
    const worldHeight = (worldWidth * canvasHeight) / canvasWidth;

    ctx.clearRect(0, 0, canvasWidth, canvasHeight);

    const count = simulation ? simulation.count() : 0;
    const PAINT_SCALE = 2;

    // Original behavior: use WASM colors
    for (let i = 0; i < count; i++) {
        const x = views.posX[i];
        const y = views.posY[i];

        // Basic Culling
        if (x < -worldWidth * 0.5 || x > worldWidth * 1.5 || y < -worldHeight * 0.5 || y > worldHeight * 1.5) {
            continue;
        }

        const diameter = views.diameters[i];
        const pcX = (x / worldWidth) * canvasWidth;
        const pcY = (y / worldHeight) * canvasHeight;
        const pcDiameter = (diameter / worldWidth) * canvasWidth * PAINT_SCALE;

        const r = Math.floor(views.colors[i * 3]);
        const g = Math.floor(views.colors[i * 3 + 1]);
        const b = Math.floor(views.colors[i * 3 + 2]);

        ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
        ctx.fillRect(
            pcX - pcDiameter / 2,
            pcY - pcDiameter / 2,
            pcDiameter,
            pcDiameter
        );
    }

    stats.end();

    if (appState.play) requestAnimationFrame(render);
}

export async function setupParticles() {
    const worldWidth = state.worldWidth;
    const worldHeight = (state.canvasHeight * state.worldWidth) / state.canvasWidth;

    const initialParticles = generate_particles(
        state.particleAmount,
        worldWidth,
        worldHeight,
        state.mass,
        state.massDeviation,
        state.diameter
    );

    if (simulation) {
        simulation.free();
    }
    
    simulation = new SimulationWrapper(initialParticles);
    updateViews();
}

/**
 * Register
 */
export function registerPlayPauseButton() {
    const button = document.getElementById("play-pause");
    button.addEventListener("click", () => {
        appState.play = !appState.play;
        if (appState.play) {
            requestAnimationFrame(render);
        }
    });
}

export function registerRestartButton() {
    const button = document.getElementById("restart");
    button.addEventListener("click", async () => {
        await setupParticles();
    });
}

export function registerSaveButton() {
    const button = document.getElementById("save");
    button.addEventListener("click", async () => {
        const form = document.getElementById("parameter-form");
        const formData = new FormData(form);
        const formDataObj = {};
        formData.forEach((value, key) => {
            formDataObj[key] = value;
        });
        const jsonData = JSON.stringify(formDataObj);
        const blob = new Blob([jsonData], { type: "application/json" });
        const url = URL.createObjectURL(blob);
        const link = document.createElement("a");
        link.href = url;
        link.download = "nbody-parameters.json";
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    });
}

export function registerParameterForm() {
    const parameterForm = document.getElementById("parameter-form");
    parameterForm.addEventListener("submit", async (event) => {
        event.preventDefault();
        const keys = Object.keys(state);
        for (let i = 0; i < keys.length; i++) {
            const input = document.getElementsByName(keys[i])[0];
            if (!input) continue;
            if (input.type === "number") {
                state[keys[i]] = input.valueAsNumber;
                continue;
            }
            if (input.type === "checkbox") {
                state[keys[i]] = input.checked;
                continue;
            }
            state[keys[i]] = input.value;
        }
        canvas.width = state.canvasWidth;
        canvas.height = state.canvasHeight;
        await setupParticles();
    });
}

export function registerFileUpload() {
    const uploadInput = document.getElementById("parameters-upload");
    uploadInput.addEventListener("change", (event) => {
        const files = event.target.files;
        if (files.length <= 0) return;
        const file = files[0];
        const reader = new FileReader();
        reader.onload = (e) => {
            const json = JSON.parse(e.target.result);
            setFormValues(json);
        };
        reader.readAsText(file);
    });
}

export function setFormValues(obj) {
    const keys = Object.keys(obj);
    for (let i = 0; i < keys.length; i++) {
        const key = keys[i];
        const inputs = document.getElementsByName(key);
        if (inputs.length === 0) continue;
        if (inputs.length > 1 && inputs[0].type === "radio") {
            const value = `${obj[key]}`;
            for (let j = 0; j < inputs.length; j++) {
                const i = inputs[j];
                if (i.value === value) i.checked = true;
            }
            continue;
        }
                const input = inputs[0];
        
                if (input.type === "checkbox") {
                    input.checked = !!obj[key];
                    continue;
                }
        
                input.value = obj[key];
            }
        }

export function registerDefaultValues() {
    setFormValues(state);
}
