import { Exp, Frag, FunFrag, Stm, Label, GlobalTemp, LocalTemp } from './treeTypes';
import { assertExists, UnreachableError, IncludeMap } from './utils/utils';
import { accessExpsFromFormals } from './frame';
import { findLabelIndex, evalBinop, flatStms } from './utils/treeUtils';
import { isFunFrag, isStringFrag } from './utils/fragPatterns';
import {
    isMemExp,
    isConstExp,
    isNameExp,
    isBinopExp,
    isCallExp,
    isEseqExp,
    isLocalExp,
    isGlobalExp,
} from './utils/expPatterns';
import {
    isExpStm,
    isMoveStm,
    isJumpStm,
    isCjumpStm,
    isSeqStm,
    isLabelStm,
} from './utils/stmPatterns';
import { MemMap } from './utils/memMap';
import { StringStorage } from './utils/stringStorage';
import { CustomConsole } from './utils/console';
import { Runtime } from './runtime';

const FRAME_POINTER_OFFSET = 1024 * 1024;

export class TreeInterpreter {
    // Map Locals to values, this keeps track of the *current* locals map (per function)
    private locals: Map<LocalTemp, number>;

    // Map Temps to values
    private globals: Map<GlobalTemp, number>;

    // Map Labels to mem locations
    private labels: Map<Label, number>;

    // Map memory location to values
    private mem: MemMap;

    // String storage
    private stringStorage: StringStorage;

    // Fragments corresponding to functions.
    private functions: IncludeMap<FunFrag>;

    // Provides runtime functions
    private runtime: Runtime;

    constructor(fragments: Frag[], private customConsole: CustomConsole) {
        this.locals = new Map();
        this.globals = new Map();
        this.labels = new Map();
        this.mem = new MemMap();
        this.stringStorage = new StringStorage(this.mem, this.labels);
        this.functions = new IncludeMap();

        fragments.filter(isFunFrag).forEach((frag) => {
            this.functions.set(frag.Proc.frame.label, frag);
        });

        fragments.filter(isStringFrag).forEach((frag) => {
            this.stringStorage.storeString(frag);
        });

        this.runtime = new Runtime(this.mem, this.stringStorage, this.customConsole);
    }

    public run = async (): Promise<number> => {
        // A program starts by calling the function _tigermain
        const mainLabel = '_tigermain';
        return await this.evalFunction(mainLabel, [0]);
    };

    private evalFunction = async (name: string, args: number[]): Promise<number> => {
        // Find the function and extract it's body and frame.
        const fragment = assertExists(
            this.functions.get(name),
            `Could not find function named '${name}'`
        );
        const { body, frame } = fragment.Proc;

        // Store locals for the current function
        const localsToRestore = this.locals;
        this.locals = new Map();

        // Move FP very far away from here
        const prevFp = this.globals.get('fp') ?? 0;
        this.globals.set('fp', prevFp + FRAME_POINTER_OFFSET);

        // Set up the formals so we can exec the body
        await this.setupFormals(args, frame.formals);

        // The machine state is ready to run the body, do it.
        await this.execStms(flatStms(body));

        // Restore locals
        this.locals = localsToRestore;

        // Retreive the return value, default to 0
        const rv = this.globals.get('rv') ?? 0;

        this.globals.set('fp', prevFp);

        return rv;
    };

    /**
     *  Execute stms in order, there may be internal jumps
     *  to previous or following stms. This can loop forever!
     */
    private execStms = async (stms: Stm[]): Promise<void> => {
        // Start executing the first stm
        let executedStmIndex = 0;

        while (executedStmIndex < stms.length) {
            const stm = stms[executedStmIndex];

            // Evaluate the current stm
            const maybeLabel: Label | null = await this.evalStm(stm);

            // Find the next stm to evaluate
            if (maybeLabel === null) {
                // If no jump continue executing the next stm
                executedStmIndex++;
            } else if (maybeLabel === 'done') {
                // If the label is 'done', the program finished.
                break;
            } else {
                // We've got a label, find the corresponding stm and continue executing from there
                const nextStmIndex = findLabelIndex(stms, maybeLabel);
                executedStmIndex = nextStmIndex;
            }
        }
    };

    // Store each value from args in the corresponding temp/mem location
    private setupFormals = async (args: number[], formals: [string, boolean][]): Promise<void> => {
        const accessExps = accessExpsFromFormals(formals);

        for (let i = 0; i < args.length; i++) {
            const arg = args[i];
            const access = accessExps[i];

            if (isGlobalExp(access)) {
                // Evaluate the memory location and store the arg there
                this.globals.set(access.GLOBAL, arg);
            } else if (isLocalExp(access)) {
                // Store the argument in the corresponding temp
                this.locals.set(access.LOCAL, arg);
            } else {
                // The access can be either MEM or TEMP.
                throw new UnreachableError();
            }
        }
    };

    /**
     *  Evaluate a Stm.
     *  Return a Label if we need to jump somewhere, null otherwise
     */
    private evalStm = async (stm: Stm): Promise<Label | null> => {
        if (isExpStm(stm)) {
            await this.evalExp(stm.EXP);
            return null;
        }

        if (isMoveStm(stm)) {
            const [toExp, fromExp] = stm.MOVE;
            if (isLocalExp(toExp)) {
                const value: number = await this.evalExp(fromExp);

                this.locals.set(toExp.LOCAL, value);
                return null;
            }

            if (isGlobalExp(toExp)) {
                const value: number = await this.evalExp(fromExp);

                this.globals.set(toExp.GLOBAL, value);
                return null;
            }

            if (isMemExp(toExp)) {
                const location: number = await this.evalExp(toExp.MEM);
                const value: number = await this.evalExp(fromExp);

                this.mem.set(location, value);
                return null;
            }

            throw new UnreachableError(`MOVE to a non Local, Global or Mem expression\n${toExp}\n`);
        }

        if (isJumpStm(stm)) {
            const [where] = stm.JUMP;
            if (isNameExp(where)) {
                return where.NAME;
            }

            throw new UnreachableError(`JUMP to a non Label expression:\n${where}\n`);
        }

        if (isCjumpStm(stm)) {
            const [op, leftExp, rightExp, labelTrue, labelFalse] = stm.CJUMP;

            const leftVal = await this.evalExp(leftExp);
            const rightVal = await this.evalExp(rightExp);

            // 0 means false, everything else means true.
            const condition = evalBinop(op, leftVal, rightVal);
            return condition === 0 ? labelFalse : labelTrue;
        }

        if (isSeqStm(stm)) {
            throw new UnreachableError('Found SEQ, not a canonical tree!');
        }

        if (isLabelStm(stm)) {
            return null;
        }

        // No more cases
        throw new UnreachableError();
    };

    /**
     *  Evaluate an Exp.
     *  Every exp evaluates to a number.
     */
    private evalExp = async (exp: Exp): Promise<number> => {
        if (isConstExp(exp)) {
            return exp.CONST;
        }

        if (isNameExp(exp)) {
            return assertExists(
                this.labels.get(exp.NAME),
                `Could not find label called '${exp.NAME}'`
            );
        }

        if (isLocalExp(exp)) {
            return assertExists(
                this.locals.get(exp.LOCAL),
                `Could not find local called '${exp.LOCAL}'`
            );
        }

        if (isGlobalExp(exp)) {
            return assertExists(
                this.globals.get(exp.GLOBAL),
                `Could not find global called '${exp.GLOBAL}'`
            );
        }

        if (isBinopExp(exp)) {
            const [op, leftExp, rightExp] = exp.BINOP;

            const leftVal = await this.evalExp(leftExp);
            const rightVal = await this.evalExp(rightExp);

            return evalBinop(op, leftVal, rightVal);
        }

        if (isMemExp(exp)) {
            const dir = await this.evalExp(exp.MEM);
            return assertExists(this.mem.get(dir), `Memory location ${dir} is empty!`);
        }

        if (isCallExp(exp)) {
            const [labelExp, args] = exp.CALL;
            if (isNameExp(labelExp)) {
                const name = labelExp.NAME;
                const evaluatedArgs = await Promise.all(args.map(this.evalExp));

                let returnValue: number;
                const runtimeFunction = this.runtime.maybeGetFunction(name);
                if (runtimeFunction !== undefined) {
                    returnValue = await runtimeFunction(evaluatedArgs);
                } else {
                    returnValue = await this.evalFunction(name, evaluatedArgs);
                }

                this.globals.set('rv', returnValue);

                return returnValue;
            } else {
                throw new UnreachableError('Found CALL to non NAME exp');
            }
        }

        if (isEseqExp(exp)) {
            throw new UnreachableError('Found ESEQ, not a canonical tree!');
        }

        // No more cases
        throw new UnreachableError();
    };
}
