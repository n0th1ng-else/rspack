const { RawSource, ConcatSource } = require("webpack-sources");

/** @type {import("@rspack/core").Configuration} */
module.exports = {
	target: "web",
	node: {
		__dirname: false,
		__filename: false
	},
	entry: {
		main: "./index.js"
	},
	output: {
		publicPath: "auto",
		cssFilename: "css/[name].css"
	},
	resolve: {
		alias: {
			"@": __dirname
		}
	},
	module: {
		generator: {
			"css/auto": {
				exportsOnly: false
			}
		},
		rules: [
			{
				test: /\.png$/i,
				type: "asset",
				generator: {
					filename: "image/[name][ext]"
				}
			}
		]
	},
	plugins: [
		{
			apply(compiler) {
				compiler.hooks.compilation.tap("compilation", compilation => {
					compilation.hooks.processAssets.tapPromise(
						"polyfill for auto public path in target: 'node'",
						async assets => {
							compilation.updateAsset(
								"bundle0.js",
								new ConcatSource(
									new RawSource(
										`Object.assign(globalThis, { document: { currentScript: { src: "/" } } });\n`
									),
									assets["bundle0.js"]
								)
							);
						}
					);
				});
			}
		}
	],
	experiments: {
		css: true
	}
};
