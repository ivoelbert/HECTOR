import factorial from './examples/factorial.tig';
import stringConcat from './examples/stringConcat.tig';
import basicArray from './examples/basicArray.tig';
import fizzBuzz from './examples/fizzbuzz.tig';
import bubbleSort from './examples/bubbleSort.tig';
import queens from './examples/queens.tig';

interface ExamplesByName {
    [name: string]: string;
}

export const EXAMPLES: ExamplesByName = {
    'factorial.tig': factorial,
    'stringConcat.tig': stringConcat,
    'basicArray.tig': basicArray,
    'fizzBuzz.tig': fizzBuzz,
    'bubbleSort.tig': bubbleSort,
    'queens.tig': queens,
};
