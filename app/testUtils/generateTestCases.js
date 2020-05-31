const _fs = require('fs');
const fs = _fs.promises;
const wasm = require('../../testPkg/hector');

const generateTestInputs = async () => {
    console.log('Generating test inputs...');

    const basePath = 'src/interpreter/test/inputs';
    const files = await fs.readdir(basePath);
    const tigFiles = files.filter((file) => file.endsWith('.tig'));

    await Promise.all(
        tigFiles.map(async (file) => {
            const baseName = file.split('.tig')[0];
            const source = await fs.readFile(
                `${basePath}/${baseName}.tig`,
                'utf8'
            );
            const compilation = wasm.compile(source);

            await fs.writeFile(
                `${basePath}/generated/${baseName}.json`,
                JSON.stringify(compilation.canon)
            );
        })
    );
};

generateTestInputs();
