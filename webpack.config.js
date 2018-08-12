const path = require("path");

module.exports = {
  entry: "./websrc/js/",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
    publicPath: "/websrc/",
  },
  mode: "development",
};
