import { CustomConsole } from '../utils/console';
//import binaryen from 'binaryen';
import { Runtime } from './runtime';

export interface Evaluator {
    run(): number;
}

export class WasmEvaluator implements Evaluator {
    private runtime: Runtime;

    constructor(binaryWasm: Uint8Array, customConsole: CustomConsole) {
        // const wasmModule = binaryen.readBinary(binaryWasm);
        // binaryen.setOptimizeLevel(1);
        // wasmModule.runPasses(['asyncify']);
        // const binary = wasmModule.emitBinary();
        const compiledModule = new WebAssembly.Module(binaryWasm);

        this.runtime = new Runtime(compiledModule, customConsole);
    }

    run = () => {
        return this.runtime.run();
    };
}
