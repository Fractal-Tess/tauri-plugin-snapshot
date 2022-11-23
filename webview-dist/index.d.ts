import type { Area, Options } from './types';
export declare function snapshotArea(area: Area, options?: Options): Promise<Uint8Array>;
export declare function snapshotViewport(options?: Options): Promise<Uint8Array>;
export declare function snapshotDocument(options?: Options): Promise<Uint8Array>;
