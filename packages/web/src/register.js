import appState from "./app-state.js";
import state from "./form-state.js";
import { setupParticles } from "./script.js";

const canvas = document.getElementsByTagName("canvas")[0];

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

export function registerNextFrameButton() {
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

export function registerRestartButton() {
    const button = document.getElementById("restart");

    button.addEventListener("click", async () => {
        setupParticles();
    });
}
export function registerLogStatsButton() {
    const button = document.getElementById("log-stats");

    button.addEventListener("click", async () => {
        // for (let i = 0; i < appState.particles.length; i++) {
        //     const p = appState.particles[i];
        //     console.log(i, p);
        // }

        console.log(appState.particles);
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

            state[keys[i]] = input.value;
        }

        canvas.width = state.canvasWidth;
        canvas.height = state.canvasHeight;

        setupParticles();
    });
}
export function registerFileUpload() {
    const uploadInput = document.getElementById("parameters-upload");

    uploadInput.addEventListener("change", (event) => {
        // Get the file list from the input element
        const files = event.target.files;

        if (files.length < 0) {
            return;
        }

        const file = files[0];

        const reader = new FileReader();

        reader.onload = (e) => {
            console.log(e.target.result);

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

        input.value = obj[key];
    }
}
export function registerDefaultValues() {
    setFormValues(state);
}

// export function formatNumberWithUnderscores(number) {
//     return number.toString().replace(/\B(?=(\d{3})+(?!\d))/g, "_");
// }
