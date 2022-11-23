import { invoke } from '@tauri-apps/api/tauri';
import type { Area, Options } from './types';

export async function snapshotArea(area: Area, options?: Options) {
  const data = (await invoke('plugin:screen-shot|snapshot', {
    options,
  })) as number[];
  const typedArray = new Uint8Array(data);
  return typedArray;
}
export async function snapshotViewport(options?: Options) {
  const data = (await invoke('plugin:screen-shot|snapshot', {
    options,
  })) as number[];
  const typedArray = new Uint8Array(data);
  return typedArray;
}
export async function snapshotDocument(options?: Options) {
  const data = (await invoke('plugin:screen-shot|snapshot', {
    options,
  })) as number[];
  const typedArray = new Uint8Array(data);
  return typedArray;
}

// const data = snapshotDocument({ capture: {} });
