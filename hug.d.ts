/* tslint:disable */
/* eslint-disable */
/**
* @param {string} output
*/
export function on_output(output: string): void;
/**
* @param {string} name
*/
export function on_name_change(name: string): void;
/**
*/
export function on_click_random(): void;
/**
*/
export function on_click_room(): void;
/**
*/
export function on_click_share(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly on_output: (a: number, b: number) => void;
  readonly on_name_change: (a: number, b: number) => void;
  readonly on_click_random: () => void;
  readonly on_click_room: () => void;
  readonly on_click_share: () => void;
  readonly main: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly wasm_bindgen__convert__closures__invoke1_mut__h1a6c65301dfa05d5: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures__invoke0_mut__h913bb59da714ffac: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures__invoke1_mut__h62bd1ae715296cd2: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        