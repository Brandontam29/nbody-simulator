import { SimulationWrapper, InitOutput } from './wasm/nbody_simulator';

export interface SimulationParams {
    canvasWidth: number;
    canvasHeight: number;
    worldWidth: number;
    particleAmount: number;
    mass: number;
    massDeviation: number;
    diameter: number;
    gravity: number;
    epsilon: number;
    timeStep: number;
}

export interface SimulationViews {
    posX: Float32Array | null;
    posY: Float32Array | null;
    velX: Float32Array | null;
    velY: Float32Array | null;
    masses: Float32Array | null;
    diameters: Float32Array | null;
    colors: Float32Array | null;
}

export interface WasmEngine {
    generate_particles: (
        particleAmount: number,
        worldWidth: number,
        worldHeight: number,
        mass: number,
        massDeviation: number,
        diameter: number
    ) => SimulationWrapper;
    memory: WebAssembly.Memory;
}

export class SimulationEngine {
    private wasm: WasmEngine;
    private simulation: SimulationWrapper | null;
    public views: SimulationViews;

    constructor(wasmModule: WasmEngine) {
        this.wasm = wasmModule;
        this.simulation = null;
        this.views = {
            posX: null,
            posY: null,
            velX: null,
            velY: null,
            masses: null,
            diameters: null,
            colors: null,
        };
    }

    init(params: SimulationParams): void {
        if (this.simulation) {
            this.simulation.free();
        }

        const worldHeight = (params.canvasHeight * params.worldWidth) / params.canvasWidth;

        this.simulation = this.wasm.generate_particles(
            params.particleAmount,
            params.worldWidth,
            worldHeight,
            params.mass,
            params.massDeviation,
            params.diameter
        );

        this.updateViews();
    }

    updateViews(): void {
        if (!this.simulation || !this.wasm.memory) return;

        const count = this.simulation.count();
        const buffer = this.wasm.memory.buffer;

        this.views.posX = new Float32Array(buffer, this.simulation.positions_x_ptr(), count);
        this.views.posY = new Float32Array(buffer, this.simulation.positions_y_ptr(), count);
        this.views.velX = new Float32Array(buffer, this.simulation.velocities_x_ptr(), count);
        this.views.velY = new Float32Array(buffer, this.simulation.velocities_y_ptr(), count);
        this.views.masses = new Float32Array(buffer, this.simulation.masses_ptr(), count);
        this.views.diameters = new Float32Array(buffer, this.simulation.diameters_ptr(), count);
        this.views.colors = new Float32Array(buffer, this.simulation.colors_ptr(), count * 3);
    }

    step(params: SimulationParams): void {
        if (!this.simulation) return;

        const worldHeight = (params.canvasHeight * params.worldWidth) / params.canvasWidth;

        this.simulation.step(
            params.worldWidth,
            worldHeight,
            params.gravity,
            params.epsilon,
            params.timeStep
        );
        
        // In case memory grew
        this.updateViews();
    }

    get count(): number {
        return this.simulation ? this.simulation.count() : 0;
    }
}
