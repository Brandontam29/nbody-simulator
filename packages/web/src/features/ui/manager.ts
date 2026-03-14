import appState from '../../core/app-state';
import state, { FormState } from './form-state';

declare global {
  interface Window {
    postToWorker: (type: string, data: any) => void;
  }
}

/**
 * Register
 */
export function registerPlayPauseButton() {
  const button = document.getElementById('play-pause') as HTMLButtonElement | null;
  if (!button) return;

  button.addEventListener('click', () => {
    appState.play = !appState.play;
    window.postToWorker('PLAY_PAUSE', appState.play);
  });
}

export function registerRestartButton() {
  const button = document.getElementById('restart') as HTMLButtonElement | null;
  if (!button) return;

  button.addEventListener('click', () => {
    window.postToWorker('RESTART', state);
  });
}

export function registerSaveButton() {
  const button = document.getElementById('save') as HTMLButtonElement | null;
  if (!button) return;

  button.addEventListener('click', () => {
    const form = document.getElementById('parameter-form') as HTMLFormElement | null;
    if (!form) return;
    const formData = new FormData(form);

    const formDataObj: Record<string, any> = {};

    formData.forEach((value, key) => {
      formDataObj[key] = value;
    });

    const jsonData = JSON.stringify(formDataObj);

    const blob = new Blob([jsonData], { type: 'application/json' });

    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = 'nbody-parameters.json';

    document.body.appendChild(link);
    link.click();

    document.body.removeChild(link);
  });
}

export function registerParameterForm() {
  const parameterForm = document.getElementById('parameter-form') as HTMLFormElement | null;
  if (!parameterForm) return;

  parameterForm.addEventListener('submit', (event) => {
    event.preventDefault();

    const keys = Object.keys(state) as (keyof FormState)[];

    for (let i = 0; i < keys.length; i++) {
      const key = keys[i];
      const inputElements = document.getElementsByName(key);
      if (inputElements.length === 0) continue;

      const input = inputElements[0] as HTMLInputElement;

      if (input.type === 'number') {
        (state as any)[key] = input.valueAsNumber;
        continue;
      }

      if (input.type === 'checkbox') {
        (state as any)[key] = input.checked;
        continue;
      }

      (state as any)[key] = input.value;
    }

    // We don't resize the canvas here because transferControlToOffscreen
    // doesn't allow changing width/height from the main thread directly
    // after transfer easily, but we can send the new dimensions.
    // Actually, the offscreen canvas dimensions need to be updated.
    window.postToWorker('RESTART', state);
  });
}

export function registerFileUpload() {
  const uploadInput = document.getElementById('parameters-upload') as HTMLInputElement | null;
  if (!uploadInput) return;

  uploadInput.addEventListener('change', (event: Event) => {
    const target = event.target as HTMLInputElement;
    const files = target.files;

    if (!files || files.length <= 0) {
      return;
    }

    const file = files[0];
    const reader = new FileReader();

    reader.onload = (e: ProgressEvent<FileReader>) => {
      if (e.target && typeof e.target.result === 'string') {
        const json = JSON.parse(e.target.result);
        setFormValues(json);
      }
    };

    reader.readAsText(file);
  });
}

export function setFormValues(obj: Record<string, any>) {
  const keys = Object.keys(obj);

  for (let i = 0; i < keys.length; i++) {
    const key = keys[i];
    const inputs = document.getElementsByName(key);
    if (inputs.length === 0) continue;

    if (inputs.length > 1 && (inputs[0] as HTMLInputElement).type === 'radio') {
      const value = `${obj[key]}`;

      for (let j = 0; j < inputs.length; j++) {
        const item = inputs[j] as HTMLInputElement;
        if (item.value === value) item.checked = true;
      }
      continue;
    }

    const input = inputs[0] as HTMLInputElement;

    if (input.type === 'checkbox') {
      input.checked = !!obj[key];
      continue;
    }

    input.value = obj[key];
  }
}

export function registerDefaultValues() {
  setFormValues(state);
}
