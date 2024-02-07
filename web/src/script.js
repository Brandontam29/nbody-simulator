import { Stats } from "./stats.js";
import init, {
    generate_particles,
    next_nbody_positions,
} from "../../nbody_simulator/pkg/nbody_simulator.js";

import variables from "./default.js";

/**
 * Variables
 */
let particles = [];
let play = true;

const canvas = document.getElementsByTagName("canvas")[0];
const container = document.getElementById("stats-container");
const ctx = canvas.getContext("2d");

const stats = new Stats();
stats.dom.style.position = "absolute";
delete stats.dom.style.left;
stats.showPanel(0);

container.appendChild(stats.dom);

canvas.height = variables.height;
canvas.width = variables.width;

/**
 * INIT
 */
document.addEventListener("DOMContentLoaded", (_event) => {
    init().then(() => {
        setupParticles();
        registerNextFrameButton();
        registerPlayPauseButton();
        registerRestartButton();
        registerParameterForm();
        registerDefaultValues();

        requestAnimationFrame(render);
        setInterval(async () => {
            if (!play) return;

            particles = await next_nbody_positions(
                particles,
                variables.gravity,
                variables.epsilon,
                variables.scale
            );
        }, variables.updateFrequency);
    });
});

/**
 * FUNCTIONS
 */
async function render() {
    stats.begin();

    if (!ctx) throw new Error("canvas dead");
    ctx.clearRect(0, 0, variables.width, variables.height);
    for (let i = 0; i < particles.length; i++) {
        const p = particles[i];
        ctx.fillRect(p.position.x, p.position.y, p.diameter * 2, p.diameter * 2);
        ctx.fillStyle = "#000";
        ctx.fill();
    }

    stats.end();

    if (play) requestAnimationFrame(render);
}

async function setupParticles() {
    particles = await generate_particles(
        variables.particleAmount,
        { x: variables.width, y: variables.height },
        variables.mass,
        variables.massDeviation,
        variables.diameter
    );
}

/**
 * Register
 */
function registerPlayPauseButton() {
    const button = document.getElementById("play-pause");

    button.addEventListener("click", () => {
        play = !play;

        if (play) {
            requestAnimationFrame(render);
        }
    });
}

function registerNextFrameButton() {
    const button = document.getElementById("next-frame");

    button.addEventListener("click", async () => {
        particles = await next_nbody_positions(particles);

        requestAnimationFrame(render);
    });
}

function registerRestartButton() {
    const button = document.getElementById("restart");

    button.addEventListener("click", async () => {
        setupParticles();
    });
}

function registerParameterForm() {
    const parameterForm = document.getElementById("parameter-form");

    parameterForm.addEventListener("submit", async (event) => {
        event.preventDefault();

        const keys = Object.keys(variables);

        for (let i = 0; i < keys.length; i++) {
            const input = document.getElementsByName(keys[i])[0];

            if (!input) continue;

            if (input.type === "number") {
                variables[keys[i]] = input.valueAsNumber;
                continue;
            }

            variables[keys[i]] = input.value;
        }

        setupParticles();
    });
}
function registerDefaultValues() {
    const inputs = document.getElementsByTagName("input");

    for (let i = 0; i < inputs.length; i++) {
        const input = inputs[i];

        if (variables[input.name] === undefined) continue;

        if (input.type === "radio") {
            const value = variables[input.name];

            if (input.value === `${value}`) {
                input.checked = true;
            }

            continue;
        }

        input.value = variables[input.name];
    }
}
