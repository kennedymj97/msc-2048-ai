import * as wasm from './msc_2048_ai_bg.wasm';

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const u32CvtShim = new Uint32Array(2);

const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);
/**
*/
export class WasmExpectimax {

    static __wrap(ptr) {
        const obj = Object.create(WasmExpectimax.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_wasmexpectimax_free(ptr);
    }
    /**
    * @returns {WasmExpectimax}
    */
    static new() {
        var ret = wasm.wasmexpectimax_new();
        return WasmExpectimax.__wrap(ret);
    }
    /**
    * @param {BigInt} board
    * @returns {number}
    */
    get_next_move(board) {
        uint64CvtShim[0] = board;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        var ret = wasm.wasmexpectimax_get_next_move(this.ptr, low0, high0);
        return ret;
    }
}
/**
*/
export class WasmSnake {

    static __wrap(ptr) {
        const obj = Object.create(WasmSnake.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_wasmsnake_free(ptr);
    }
    /**
    * @returns {WasmSnake}
    */
    static new() {
        var ret = wasm.wasmsnake_new();
        return WasmSnake.__wrap(ret);
    }
    /**
    * @param {BigInt} board
    * @returns {number}
    */
    get_next_move(board) {
        uint64CvtShim[0] = board;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        var ret = wasm.wasmsnake_get_next_move(this.ptr, low0, high0);
        return ret;
    }
}

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

