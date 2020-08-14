/* tslint:disable */
/* eslint-disable */
/**
* @returns {string} 
*/
export function wasm_test_fn(): string;
export class WasmInterface {
  free(): void;
/**
* @returns {WasmInterface} 
*/
  static new(): WasmInterface;
/**
*/
  make_move(): void;
/**
* @param {number} row_idx 
* @param {number} col_idx 
* @returns {number} 
*/
  get_tile_val(row_idx: number, col_idx: number): number;
/**
* @returns {boolean} 
*/
  is_game_over(): boolean;
}
