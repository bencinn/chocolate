/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const __wbg_vmdata_free: (a: number, b: number) => void;
export const __wbg_get_vmdata_pc: (a: number) => number;
export const __wbg_set_vmdata_pc: (a: number, b: number) => void;
export const __wbg_get_vmdata_next: (a: number) => number;
export const __wbg_set_vmdata_next: (a: number, b: number) => void;
export const __wbg_get_vmdata_sc: (a: number) => number;
export const __wbg_set_vmdata_sc: (a: number, b: number) => void;
export const vmdata_new: () => number;
export const vmdata_execute_from_program: (a: number, b: number) => number;
export const vmdata_read_reg: (a: number, b: number) => number;
export const vmdata_read_stack: (a: number) => number;
export const __wbg_program_free: (a: number, b: number) => void;
export const program_new: () => number;
export const program_push_instruction: (a: number, b: number, c: number) => void;
export const program_get_inst_len: (a: number) => number;
export const program_run_with_new_vm: (a: number) => void;
export const program_get_instruction_str: (a: number, b: number) => [number, number];
export const program_to_raw: (a: number) => [number, number];
export const greet: (a: number, b: number) => void;
export const __wbindgen_export_0: WebAssembly.Table;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_start: () => void;
