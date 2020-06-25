import factorial from './examples/factorial.tig';
import stringConcat from './examples/stringConcat.tig';
import basicArray from './examples/basicArray.tig';
import fizzBuzz from './examples/fizzbuzz.tig';
import bubbleSort from './examples/bubbleSort.tig';
import queens from './examples/queens.tig';
import { useLocalStorageState } from '../../hooks/useLocalStorageState';

interface ExamplesByName {
    [name: string]: string;
}

const EXAMPLES: ExamplesByName = {
    'factorial.tig': factorial,
    'stringConcat.tig': stringConcat,
    'basicArray.tig': basicArray,
    'fizzBuzz.tig': fizzBuzz,
    'bubbleSort.tig': bubbleSort,
    'queens.tig': queens
};

const EXAMPLES_KEY = 'hector_examples';

type SaveExampleAction = (name: string, code: string) => void;
type DeleteExampleAction = (name: string, code: string) => void;

export const useExamples = (): [ExamplesByName, SaveExampleAction, DeleteExampleAction] => {
    const [examples, setExamples] = useLocalStorageState<ExamplesByName>(EXAMPLES_KEY, EXAMPLES);

    const saveExample = (name: string, code: string): void => {
        setExamples({
            ...examples,
            [name]: code,
        });
    };

    const deleteExample = (name: string): void => {
        const { [name]: omitted, ...newExamples } = examples;

        setExamples(newExamples);
    };

    return [examples, saveExample, deleteExample];
};
