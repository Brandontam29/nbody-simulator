import appState from "./app-state.js";
import state from "./form-state.js";

const canvas = document.getElementsByTagName("canvas")[0];

/**
 * Register
 */
export function registerPlayPauseButton() {
    const button = document.getElementById("play-pause");

    button.addEventListener("click", () => {
        appState.play = !appState.play;
        window.postToWorker("PLAY_PAUSE", appState.play);
    });
}

export function registerRestartButton() {
    const button = document.getElementById("restart");

    button.addEventListener("click", () => {
        window.postToWorker("RESTART", state);
    });
}

export function registerSaveButton() {
    const button = document.getElementById("save");

    button.addEventListener("click", () => {
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

    parameterForm.addEventListener("submit", (event) => {
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

        // We don't resize the canvas here because transferControlToOffscreen 
        // doesn't allow changing width/height from the main thread directly 
        // after transfer easily, but we can send the new dimensions.
        // Actually, the offscreen canvas dimensions need to be updated.
        window.postToWorker("RESTART", state);
    });
}

export function registerFileUpload() {
    const uploadInput = document.getElementById("parameters-upload");

    uploadInput.addEventListener("change", (event) => {
        const files = event.target.files;

        if (files.length <= 0) {
            return;
        }

        const file = files[0];
        const reader = new FileReader();

        reader.onload = (e) => {
            const json = JSON.parse(e.target.result);
            setFormValues(json);
            // Optionally auto-submit or just let user click submit
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
                const item = inputs[j];
                if (item.value === value) item.checked = true;
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
