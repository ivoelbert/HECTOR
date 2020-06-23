import { CustomConsole } from '../utils/console';
import binaryen from 'binaryen';
import { Runtime } from './runtime';

export interface Evaluator {
    run(): Promise<number>;
}

export class WasmEvaluator implements Evaluator {
    private runtime: Runtime;

    constructor(binaryWasm: Uint8Array, customConsole: CustomConsole) {
        const wasmModule = binaryen.readBinary(binaryWasm);
        binaryen.setOptimizeLevel(1);
        wasmModule.runPasses(['asyncify']);
        const binary = wasmModule.emitBinary();

        this.runtime = new Runtime(binary, customConsole);
    }

    run = async () => {
        return this.runtime.run();
    };
}
