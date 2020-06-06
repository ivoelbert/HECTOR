import { CustomConsole } from '../utils/console';
import binaryen from 'binaryen';
import { RuntimeFactory } from './runtime';

export interface Evaluator {
    run(): number;
}

type TigerMain = () => number;

export interface EvaluatorRuntime {}

export class WasmEvaluator implements Evaluator {
    private main: TigerMain;

    constructor(binaryWasm: Uint8Array, customConsole: CustomConsole) {
        const runtime = new RuntimeFactory(customConsole);
        const wasmModule = binaryen.readBinary(binaryWasm);
        binaryen.setOptimizeLevel(1);
        wasmModule.runPasses(['asyncify']);
        const binary = wasmModule.emitBinary();
        const compiled = new WebAssembly.Module(binary);
        const instance = new WebAssembly.Instance(compiled, { externals: runtime.build() });

        this.main = instance.exports.tigermain_wrapper as TigerMain;
    }

    run = () => {
        return this.main();
    };
}
