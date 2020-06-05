const _fs = require('fs');
const fs = _fs.promises;
const wasm = require('../../testPkg/hector');

const basePath = 'test/inputs';
const canonPath = `${basePath}/canon`;
const wasmPath = `${basePath}/wasm`;
const expectedValuesPath = `${basePath}/expectedValues`;

const generateTestInputs = async () => {
    console.log('Generating test inputs...');

    const files = await fs.readdir(basePath);
    const tigFiles = files.filter((file) => file.endsWith('.tig'));

    await Promise.all(tigFiles.map(writeTestCase));
};

const writeTestCase = async (file) => {
    try {
        const baseName = file.split('.tig')[0];
        const source = await fs.readFile(`${basePath}/${baseName}.tig`, 'utf8');
        const expectedValues = getExpectedValues(source);
        const compilation = wasm.compile(source);

        await Promise.all([
            writeCanon(baseName, compilation),
            writeWasm(baseName, compilation),
            writeExpectedValues(baseName, expectedValues),
        ]);
    } catch (err) {
        console.log(`Something went wrong processing file ${file}`);
        console.log(err);
        console.log('\n\n\n');
    }
};

const writeExpectedValues = async (baseName, expectedValues) => {
    await fs.writeFile(`${expectedValuesPath}/${baseName}.json`, JSON.stringify(expectedValues));
};

const writeCanon = async (baseName, compilation) => {
    await fs.writeFile(`${canonPath}/${baseName}.json`, JSON.stringify(compilation.canon));
};

const writeWasm = async (baseName, compilation) => {
    await fs.writeFile(`${wasmPath}/${baseName}.wasm`, Uint8Array.from(compilation.bin), 'binary');
};

const EXPECTED_VALUE_REGEX = /@result:[\s]*([0-9]+)/;
const getExpectedValues = (source) => {
    const regexExecution = EXPECTED_VALUE_REGEX.exec(source);

    let expectedResult = regexExecution?.[1] ?? null;
    if (expectedResult !== null) {
        expectedResult = Number(expectedResult);
    }

    return {
        result: expectedResult,
    };
};

generateTestInputs();
