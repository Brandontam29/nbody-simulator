import { Stats } from "./stats.js";
import init, {
    generate_particles,
    next_nbody_positions,
} from "./wasm/nbody_simulator.js";

import state from "./form-state.js";
import appState from "./app-state.js";
import { removeElementsByIndices } from "./removeElementsByIndices.js";

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

/**
 * INIT
 */
document.addEventListener("DOMContentLoaded", (_event) => {
    init().then(async () => {
        setupParticles();

        registerNextFrameButton();
        registerPlayPauseButton();
        registerRestartButton();
        registerLogStatsButton();
        registerSaveButton();
        registerFileUpload();
        registerParameterForm();
        registerDefaultValues();

        requestAnimationFrame(render);
        setInterval(async () => {
            if (!appState.play) return;

            appState.particles = await next_nbody_positions(
                appState.particles,
                state.gravity,
                state.epsilon,
                state.timeStep
            );
        }, state.updateFrequency);
    });
});

/**
 * FUNCTIONS
 */
async function render() {
    stats.begin();

    if (!ctx) throw new Error("canvas dead");

    const { canvasWidth, canvasHeight, worldWidth } = state;

    ctx.clearRect(0, 0, canvasWidth, canvasHeight);

    let toRemove = [];

    for (let i = 0; i < appState.particles.length; i++) {
        const p = appState.particles[i];

        const worldHeight = (worldWidth * canvasHeight) / canvasWidth;
        const pcPosition = {
            x: (p.position.x / worldWidth) * canvasWidth,
            y: (p.position.y / worldHeight) * canvasHeight,
        };

        const PAINT_SCALE = 2;
        const pcDiameter = (p.diameter / worldWidth) * canvasWidth * PAINT_SCALE;

        if (
            pcPosition.x > canvasWidth * 1.5 ||
            pcPosition.y > canvasHeight * 1.5 ||
            pcPosition.x < 0 - 0.5 * canvasWidth ||
            pcPosition.y < 0 - 0.5 * canvasHeight
        ) {
            toRemove.push(i);
            continue;
        }

        ctx.fillStyle = `rgb(${Math.floor(p.color[0])}, ${Math.floor(
            p.color[1]
        )}, ${Math.floor(p.color[2])})`;

        ctx.fillRect(
            pcPosition.x - pcDiameter / 2,
            pcPosition.y - pcDiameter / 2,
            pcDiameter,
            pcDiameter
        );

        // console.log(
        //     i,
        //     (p.mass / state.mass).toFixed(2),
        //     formatNumberWithUnderscores(p.diameter),
        //     pcDiameter.toFixed(2)
        // );
        ctx.fill();
    }

    removeElementsByIndices(appState.particles, toRemove);

    stats.end();

    if (appState.play) requestAnimationFrame(render);
}

async function setupParticles() {
    appState.particles = await generate_particles(
        state.particleAmount,
        {
            x: state.worldWidth,
            y: (state.canvasHeight * state.worldWidth) / state.canvasWidth,
        },
        state.mass,
        state.massDeviation,
        state.diameter
    );

    console.log(appState.particles);
}

/**
 * Register
 */
function registerPlayPauseButton() {
    const button = document.getElementById("play-pause");

    button.addEventListener("click", () => {
        appState.play = !appState.play;

        if (appState.play) {
            requestAnimationFrame(render);
        }
    });
}

function registerNextFrameButton() {
    const button = document.getElementById("next-frame");

    button.addEventListener("click", async () => {
        appState.particles = await next_nbody_positions(
            appState.particles,
            state.gravity,
            state.epsilon,
            state.timeStep
        );

        requestAnimationFrame(render);
    });
}

function registerRestartButton() {
    const button = document.getElementById("restart");

    button.addEventListener("click", async () => {
        setupParticles();
    });
}
function registerLogStatsButton() {
    const button = document.getElementById("log-stats");

    button.addEventListener("click", async () => {
        // for (let i = 0; i < appState.particles.length; i++) {
        //     const p = appState.particles[i];
        //     console.log(i, p);
        // }

        console.log(appState.particles);
    });
}

function registerSaveButton() {
    const button = document.getElementById("save");

    button.addEventListener("click", async () => {
        const form = document.getElementById("parameter-form");
        const formData = new FormData(form);

        const formDataObj = {};

        formData.forEach(function (value, key) {
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

function registerParameterForm() {
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

            state[keys[i]] = input.value;
        }

        canvas.width = state.canvasWidth;
        canvas.height = state.canvasHeight;

        setupParticles();
    });
}
function registerFileUpload() {
    const uploadInput = document.getElementById("parameters-upload");

    uploadInput.addEventListener("change", function (event) {
        // Get the file list from the input element
        const files = event.target.files;

        if (files.length < 0) {
            return;
        }

        const file = files[0];

        const reader = new FileReader();

        reader.onload = function (e) {
            console.log(e.target.result);

            const json = JSON.parse(e.target.result);
            setFormValues(json);
        };

        reader.readAsText(file);
    });
}

function setFormValues(obj) {
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

        input.value = obj[key];
    }
}
function registerDefaultValues() {
    // const inputs = document.getElementsByTagName("input");

    // for (let i = 0; i < inputs.length; i++) {
    //     const input = inputs[i];

    //     if (state[input.name] === undefined) continue;

    //     if (input.type === "radio") {
    //         const value = state[input.name];

    //         if (input.value === `${value}`) {
    //             input.checked = true;
    //         }

    //         continue;
    //     }

    //     input.value = state[input.name];
    // }

    setFormValues(state);
}

function formatNumberWithUnderscores(number) {
    return number.toString().replace(/\B(?=(\d{3})+(?!\d))/g, "_");
}
