const WRAPPED_EXPORTS = new WeakMap();

function isPromise(obj) {
    return (
        !!obj &&
        (typeof obj === 'object' || typeof obj === 'function') &&
        typeof obj.then === 'function'
    );
}

function proxyGet(obj, transform) {
    return new Proxy(obj, {
        get: (obj, name) => transform(obj[name]),
    });
}

class Asyncify {
    constructor(dataAddr, dataEnd) {
        this.dataAddr = dataAddr;
        this.dataEnd = dataEnd;
        this.dataStart = this.dataAddr + 8;
        this.state = { type: 'Loading' };
        this.exports = null;
    }

    assertNoneState() {
        if (this.state.type !== 'None') {
            throw new Error(`Invalid async state ${this.state.type}`);
        }
    }

    wrapImportFn(fn) {
        return (...args) => {
            if (this.state.type === 'Rewinding') {
                let { value } = this.state;
                this.state = { type: 'None' };
                this.exports.asyncify_stop_rewind();
                return value;
            }
            this.assertNoneState();
            let value = fn(...args);
            if (!isPromise(value)) {
                return value;
            }
            this.exports.asyncify_start_unwind(this.dataAddr);
            this.state = {
                type: 'Unwinding',
                promise: value,
            };
        };
    }

    wrapModuleImports(module) {
        return proxyGet(module, (value) => {
            if (typeof value === 'function') {
                return this.wrapImportFn(value);
            }
            return value;
        });
    }

    wrapImports(imports) {
        if (imports === undefined) return;

        return proxyGet(imports, (moduleImports) => this.wrapModuleImports(moduleImports));
    }

    wrapExportFn(fn) {
        let newExport = WRAPPED_EXPORTS.get(fn);

        if (newExport !== undefined) {
            return newExport;
        }

        newExport = async (...args) => {
            this.assertNoneState();

            let result = fn(...args);

            while (this.state.type === 'Unwinding') {
                let { promise } = this.state;
                this.state = { type: 'None' };
                this.exports.asyncify_stop_unwind();
                let value = await promise;
                this.assertNoneState();
                this.exports.asyncify_start_rewind(this.dataAddr);
                this.state = {
                    type: 'Rewinding',
                    value,
                };
                result = fn();
            }

            this.assertNoneState();

            return result;
        };

        WRAPPED_EXPORTS.set(fn, newExport);

        return newExport;
    }

    wrapExports(exports) {
        let newExports = Object.create(null);

        for (let exportName in exports) {
            let value = exports[exportName];
            if (typeof value === 'function' && !exportName.startsWith('asyncify_')) {
                value = this.wrapExportFn(value);
            }
            Object.defineProperty(newExports, exportName, {
                enumerable: true,
                value,
            });
        }

        WRAPPED_EXPORTS.set(exports, newExports);

        return newExports;
    }

    init(instance, imports) {
        const { exports } = instance;

        const memory = exports.memory || (imports.mem && imports.mem.memory);

        new Int32Array(memory.buffer, this.dataAddr).set([this.dataStart, this.dataEnd]);

        this.state = { type: 'None' };

        this.exports = this.wrapExports(exports);

        Object.setPrototypeOf(instance, Instance.prototype);
    }
}

export class Instance extends WebAssembly.Instance {
    constructor(module, dataAddr, dataEnd, imports) {
        let state = new Asyncify(dataAddr, dataEnd);
        super(module, state.wrapImports(imports));
        state.init(this, imports);
    }

    get exports() {
        return WRAPPED_EXPORTS.get(super.exports);
    }
}

Object.defineProperty(Instance.prototype, 'exports', { enumerable: true });
