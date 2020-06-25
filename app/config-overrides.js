const path = require('path');

module.exports = function override(config, env) {
    const wasmExtensionRegExp = /\.wasm$/;
    const tigExtensionRegExp = /\.tig$/;

    config.resolve.extensions.push('.wasm');
    config.resolve.extensions.push('.tig');

    config.module.rules.forEach((rule) => {
        (rule.oneOf || []).forEach((oneOf) => {
            if (oneOf.loader && oneOf.loader.indexOf('file-loader') >= 0) {
                // Make file-loader ignore WASM and TIG files
                oneOf.exclude.push(wasmExtensionRegExp);
                oneOf.exclude.push(tigExtensionRegExp);
            }
        });
    });

    // Add a dedicated loader for WASM
    config.module.rules.push({
        test: wasmExtensionRegExp,
        include: path.resolve(__dirname, 'src'),
        use: [{ loader: require.resolve('wasm-loader'), options: {} }],
    });

    config.module.rules.push({
        test: tigExtensionRegExp,
        include: path.resolve(__dirname, 'src'),
        use: [{ loader: require.resolve('raw-loader'), options: {} }],
    });

    return config;
};
