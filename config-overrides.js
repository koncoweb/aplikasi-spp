const { override } = require('customize-cra');

module.exports = override((config) => {
  // Only modify webpack dev server config in development
  if (process.env.NODE_ENV === 'development' && config.devServer) {
    // Remove deprecated options
    delete config.devServer.onAfterSetupMiddleware;
    delete config.devServer.onBeforeSetupMiddleware;
    
    // Configure dev server using new API
    config.devServer = {
      ...config.devServer,
      setupMiddlewares: (middlewares, devServer) => {
        if (!devServer) {
          throw new Error('webpack-dev-server is not defined');
        }
        return middlewares;
      },
      open: true, // Opens browser automatically
      port: 3000,
      host: 'localhost',
      historyApiFallback: true,
      hot: true,
    };
  }
  
  return config;
});
