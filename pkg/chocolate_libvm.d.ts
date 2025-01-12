/* tslint:disable */
/* eslint-disable */
export function greet(name: string): void;
export class Program {
  free(): void;
  constructor();
  push_instruction(inst: string): void;
  get_inst_len(): number;
  run_with_new_vm(): void;
  get_instruction_str(pos: number): string;
  to_raw(): string;
}
export class VMData {
  free(): void;
  constructor();
  /**
   * execute and retusn whether you can continue to execute
   */
  execute_from_program(p: Program): boolean;
  read_reg(reg: number): number;
  read_stack(): number | undefined;
  pc: number;
  next: number;
  sc: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_vmdata_free: (a: number, b: number) => void;
  readonly __wbg_get_vmdata_pc: (a: number) => number;
  readonly __wbg_set_vmdata_pc: (a: number, b: number) => void;
  readonly __wbg_get_vmdata_next: (a: number) => number;
  readonly __wbg_set_vmdata_next: (a: number, b: number) => void;
  readonly __wbg_get_vmdata_sc: (a: number) => number;
  readonly __wbg_set_vmdata_sc: (a: number, b: number) => void;
  readonly vmdata_new: () => number;
  readonly vmdata_execute_from_program: (a: number, b: number) => number;
  readonly vmdata_read_reg: (a: number, b: number) => number;
  readonly vmdata_read_stack: (a: number) => number;
  readonly __wbg_program_free: (a: number, b: number) => void;
  readonly program_new: () => number;
  readonly program_push_instruction: (a: number, b: number, c: number) => void;
  readonly program_get_inst_len: (a: number) => number;
  readonly program_run_with_new_vm: (a: number) => void;
  readonly program_get_instruction_str: (a: number, b: number) => [number, number];
  readonly program_to_raw: (a: number) => [number, number];
  readonly greet: (a: number, b: number) => void;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
