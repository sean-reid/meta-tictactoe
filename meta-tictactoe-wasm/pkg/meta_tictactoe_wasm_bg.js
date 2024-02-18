let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
*/
export const GamePlayer = Object.freeze({ X:1,"1":"X",O:2,"2":"O", });
/**
*/
export const Cell = Object.freeze({ Empty:0,"0":"Empty",X:1,"1":"X",O:2,"2":"O", });

const GameStateWrapperFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_gamestatewrapper_free(ptr >>> 0));
/**
*/
export class GameStateWrapper {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GameStateWrapper.prototype);
        obj.__wbg_ptr = ptr;
        GameStateWrapperFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GameStateWrapperFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gamestatewrapper_free(ptr);
    }
    /**
    * @returns {GameStateWrapper}
    */
    static new() {
        const ret = wasm.gamestatewrapper_new();
        return GameStateWrapper.__wrap(ret);
    }
    /**
    * @returns {any}
    */
    get_possible_moves() {
        const ret = wasm.gamestatewrapper_get_possible_moves(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {MetaMove} mov
    */
    apply_move(mov) {
        _assertClass(mov, MetaMove);
        wasm.gamestatewrapper_apply_move(this.__wbg_ptr, mov.__wbg_ptr);
    }
    /**
    * @param {MetaMove} mov
    */
    undo_move(mov) {
        _assertClass(mov, MetaMove);
        wasm.gamestatewrapper_undo_move(this.__wbg_ptr, mov.__wbg_ptr);
    }
    /**
    * @returns {number}
    */
    evaluate() {
        const ret = wasm.gamestatewrapper_evaluate(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {boolean}
    */
    is_game_over() {
        const ret = wasm.gamestatewrapper_is_game_over(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @returns {number}
    */
    current_player() {
        const ret = wasm.gamestatewrapper_current_player(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {any}
    */
    get_board_state() {
        const ret = wasm.gamestatewrapper_get_board_state(this.__wbg_ptr);
        return takeObject(ret);
    }
}

const MetaGameStateFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_metagamestate_free(ptr >>> 0));
/**
*/
export class MetaGameState {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MetaGameStateFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_metagamestate_free(ptr);
    }
}

const MetaMoveFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_metamove_free(ptr >>> 0));
/**
*/
export class MetaMove {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MetaMoveFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_metamove_free(ptr);
    }
}

const MiniBoardFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_miniboard_free(ptr >>> 0));
/**
*/
export class MiniBoard {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MiniBoardFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_miniboard_free(ptr);
    }
}

const MinimaxFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_minimax_free(ptr >>> 0));
/**
*/
export class Minimax {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MinimaxFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_minimax_free(ptr);
    }
    /**
    * @param {GameStateWrapper} game_state
    * @param {number} depth
    * @returns {any}
    */
    static find_best_move(game_state, depth) {
        _assertClass(game_state, GameStateWrapper);
        const ret = wasm.minimax_find_best_move(game_state.__wbg_ptr, depth);
        return takeObject(ret);
    }
    /**
    * @param {GameStateWrapper} game_state
    * @param {number} depth
    * @param {number} alpha
    * @param {number} beta
    * @param {number} maximizing_player
    * @returns {number}
    */
    static minimax(game_state, depth, alpha, beta, maximizing_player) {
        _assertClass(game_state, GameStateWrapper);
        const ret = wasm.minimax_minimax(game_state.__wbg_ptr, depth, alpha, beta, maximizing_player);
        return ret;
    }
}

export function __wbindgen_json_serialize(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = JSON.stringify(obj === undefined ? null : obj);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbindgen_json_parse(arg0, arg1) {
    const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

