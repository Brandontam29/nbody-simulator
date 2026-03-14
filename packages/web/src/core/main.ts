import { Stats } from '../features/rendering/stats';
import state from '../features/ui/form-state';
import appState from './app-state';
import * as register from '../features/ui/manager';

/**
 * Variables
 */
const canvas = document.getElementsByTagName('canvas')[0] as HTMLCanvasElement;
const container = document.getElementById('stats-container') as HTMLDivElement;

const stats = new Stats();
stats.dom.style.position = 'absolute';
// @ts-ignore - style.left is optional but stats.dom might have it set
delete (stats.dom.style as any).left;
stats.showPanel(0);
container.appendChild(stats.dom);

// Initialize Worker
const worker = new Worker(new URL('../features/simulation/worker.ts', import.meta.url), {
  type: 'module',
});

// Transfer Canvas to Worker
const offscreen = canvas.transferControlToOffscreen();

// UI Event Proxying
export function postToWorker(type: string, data: any) {
  worker.postMessage({ type, data });
}

// Global reference for register.ts to use
(window as any).postToWorker = postToWorker;

worker.onmessage = (e: MessageEvent) => {
  const { type } = e.data;
  if (type === 'FRAME_START') {
    stats.begin();
  } else if (type === 'FRAME_END') {
    stats.end();
  }
};

/**
 * INIT
 */
document.addEventListener('DOMContentLoaded', (_event) => {
  // Send initial configuration to worker
  worker.postMessage(
    {
      type: 'INIT',
      data: {
        canvas: offscreen,
        params: state,
        isPlaying: appState.play,
      },
    },
    [offscreen],
  );

  register.registerPlayPauseButton();
  register.registerRestartButton();
  register.registerSaveButton();
  register.registerLogStatsButton();
  register.registerClearSettingsButton();
  register.registerFileUpload();
  register.registerParameterForm();
  register.registerDefaultValues();
});

// Export empty functions to avoid breaking imports if any other file expects them
export async function setupParticles() {}
