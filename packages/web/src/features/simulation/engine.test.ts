import { describe, it, expect, vi } from 'vitest';
import { SimulationEngine, SimulationParams, WasmEngine } from './engine';
import { SimulationWrapper } from './wasm/nbody_simulator';

describe('SimulationEngine', () => {
  const mockSimulation = {
    free: vi.fn(),
    count: vi.fn(() => 10),
    positions_x_ptr: vi.fn(() => 0),
    positions_y_ptr: vi.fn(() => 4),
    velocities_x_ptr: vi.fn(() => 8),
    velocities_y_ptr: vi.fn(() => 12),
    masses_ptr: vi.fn(() => 16),
    diameters_ptr: vi.fn(() => 20),
    colors_ptr: vi.fn(() => 24),
    step: vi.fn(),
  } as unknown as SimulationWrapper;

  const mockWasm = {
    generate_particles: vi.fn(() => mockSimulation),
    memory: {
      buffer: new ArrayBuffer(1024),
    },
  } as unknown as WasmEngine;

  it('should initialize correctly', () => {
    const engine = new SimulationEngine(mockWasm);
    const params = {
      canvasWidth: 800,
      canvasHeight: 600,
      worldWidth: 1000,
      particleAmount: 10,
      mass: 1,
      massDeviation: 0.1,
      diameter: 2,
    } as SimulationParams;

    engine.init(params);

    expect(mockWasm.generate_particles).toHaveBeenCalledWith(
      10,
      1000,
      (600 * 1000) / 800,
      1,
      0.1,
      2
    );
    expect(engine.count).toBe(10);
    expect(engine.views.posX).toBeInstanceOf(Float32Array);
  });

  it('should step correctly', () => {
    const engine = new SimulationEngine(mockWasm);
    const params = {
      canvasWidth: 800,
      canvasHeight: 600,
      worldWidth: 1000,
      gravity: 1,
      epsilon: 0.1,
      timeStep: 0.01,
      particleAmount: 10,
      mass: 1,
      massDeviation: 0.1,
      diameter: 2,
    } as SimulationParams;

    engine.init(params);
    engine.step(params);

    expect(mockSimulation.step).toHaveBeenCalledWith(
      1000,
      (600 * 1000) / 800,
      1,
      0.1,
      0.01
    );
  });
});
