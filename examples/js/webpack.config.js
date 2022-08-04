const path = require('path');
const webpack = require('webpack');

module.exports = {
  mode: 'development',
  entry: './src/index.js',
  output: {
    filename: 'main.js',
    path: path.resolve(__dirname, 'dist')
  },
  devServer: {
    static: {
      directory: path.join(__dirname, 'dist'),
    },
    compress: true,
    port: 9000
  },
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true
  },
  resolve: {
        fallback: {
            buffer: require.resolve('buffer/'),
        },
    },
    plugins: [
        new webpack.ProvidePlugin({
            Buffer: ['buffer', 'Buffer'],
        }),
    ],
  // plugins: [
  //   // Work around for Buffer is undefined:
  //   // https://github.com/webpack/changelog-v5/issues/10
  //   new webpack.ProvidePlugin({
  //       Buffer: ['buffer', 'Buffer'],
  //   }),
  //   new webpack.ProvidePlugin({
  //       process: 'process/browser',
  //   }),
  // ],
  // resolve: {
  //       extensions: [ '.ts', '.js' ],
  //       fallback: {
  //           // "stream": require.resolve("stream-browserify"),
  //           // "buffer": require.resolve("buffer")
  //       }
  //   },
  externals: {
    'wasmer_wasi_js_bg.wasm': true
  }
}
