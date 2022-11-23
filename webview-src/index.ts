import { invoke } from '@tauri-apps/api/tauri';
import type { Area, Options } from './types';

export async function snapshotArea(area: Area, options: Options = {}) {
  const data = await invoke<number[]>('plugin:screen-shot|snapshot', {
    options: {
      ...options,
      region: { area },
    },
  });
  const typedArray = new Uint8Array(data);
  return typedArray;
}
export async function snapshotViewport(options: Options = {}) {
  const data = await invoke<number[]>('plugin:screen-shot|snapshot', {
    options: {
      ...options,
      region: 'viewport',
    },
  });
  const typedArray = new Uint8Array(data);
  return typedArray;
}
export async function snapshotDocument(options?: Options) {
  const data = await invoke<number[]>('plugin:screen-shot|snapshot', {
    options: {
      ...options,
      region: 'document',
    },
  });
  const typedArray = new Uint8Array(data);
  return typedArray;
}
