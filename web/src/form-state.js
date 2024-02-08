// in meters
// in kg
// from wkipedia 2024

export const SUN = {
    mass: 1.9885e30,
    diameter: 1.3914e9,
};

export const EARTH = {
    mass: 5.972e24,
    diameter: 12.742e6,
};

const DISTANCE_SUN_EARTH = 149.6e9;

const formState = {
    canvasHeight: 700,
    canvasWidth: 700,
    worldWidth: Math.floor(DISTANCE_SUN_EARTH * 3), // based on height because wS * (cW / cH which is the ratio)
    particleAmount: 100,
    width: 700,
    height: 700,
    mass: SUN.mass, // earth
    massDeviation: 90.0,
    diameter: SUN.diameter,

    gravity: 6.6743e-11,
    epsilon: 5.84e9,
    timeStep: 1e16,
    updateFrequency: 10,
    // export let collision = false;
};

export default formState;
